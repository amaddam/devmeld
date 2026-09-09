use crate::{
    Result,
    declarations::Config,
    error,
    language::Messages,
    storage::{Plan, resolve},
};
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf, Prefix};

pub(crate) fn text(value: &str) -> String {
    value
        .chars()
        .flat_map(|c| match c {
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '<' => "&lt;".chars().collect(),
            '>' => "&gt;".chars().collect(),
            '\n' | '\r' => " ".chars().collect(),
            '\\' | '`' | '*' | '_' | '[' | ']' | '(' | ')' | '#' | '!' | '|' => vec!['\\', c],
            _ if c.is_control() => vec![' '],
            _ => vec![c],
        })
        .collect()
}
struct FileLink {
    destination: String,
    local_path: Option<String>,
}

impl FileLink {
    fn markdown(&self, label: &str, messages: &Messages) -> String {
        let mut result = format!("[{}]({})", text(label), self.destination);
        if let Some(path) = &self.local_path {
            result.push_str(&format!(" ({}: {})", messages.local_path, text(path)));
        }
        result
    }
}

fn encode_segment(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn disk(component: Component<'_>) -> Option<u8> {
    if let Component::Prefix(prefix) = component {
        match prefix.kind() {
            Prefix::Disk(drive) | Prefix::VerbatimDisk(drive) => Some(drive.to_ascii_uppercase()),
            _ => None,
        }
    } else {
        None
    }
}

fn same_component(a: &Component<'_>, b: &Component<'_>) -> bool {
    a == b || matches!((disk(*a), disk(*b)), (Some(x), Some(y)) if x == y)
}

fn link(from: &Path, to: &Path) -> Result<FileLink> {
    if !from.is_absolute() || !to.is_absolute() {
        return Err(error("rendered file links require resolved absolute paths"));
    }
    let parent = from.parent().ok_or_else(|| error("link has no parent"))?;
    let a: Vec<_> = parent.components().collect();
    let b: Vec<_> = to.components().collect();
    if !same_component(&a[0], &b[0]) {
        let drive = disk(b[0]).ok_or_else(|| error("unsupported local file root"))?;
        let mut names = Vec::new();
        for component in &b[1..] {
            match component {
                Component::RootDir => (),
                Component::Normal(value) => names.push(value.to_str().ok_or_else(|| {
                    error("non-Unicode paths are not supported in Markdown links")
                })?),
                _ => return Err(error("file URI requires a resolved local disk path")),
            }
        }
        let encoded = names
            .iter()
            .map(|name| encode_segment(name))
            .collect::<Vec<_>>()
            .join("/");
        return Ok(FileLink {
            destination: format!("file:///{}:/{encoded}", drive as char),
            local_path: Some(format!("{}:/{}", drive as char, names.join("/"))),
        });
    }
    let common = a
        .iter()
        .zip(&b)
        .take_while(|(a, b)| same_component(a, b))
        .count();
    let mut parts = vec!["..".to_owned(); a.len() - common];
    for component in &b[common..] {
        if let Component::Normal(value) = component {
            let value = value
                .to_str()
                .ok_or_else(|| error("non-Unicode paths are not supported in Markdown links"))?;
            parts.push(encode_segment(value));
        }
    }
    Ok(FileLink {
        destination: parts.join("/"),
        local_path: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn cross_drive_links_encode_native_paths_as_local_file_uris() {
        for target in [
            r"D:\知识 #100% (ssh)\notes.md",
            r"\\?\D:\知识 #100% (ssh)\notes.md",
        ] {
            assert_eq!(
                link(Path::new(r"C:\context\index.md"), Path::new(target))
                    .unwrap()
                    .destination,
                "file:///D:/%E7%9F%A5%E8%AF%86%20%23100%25%20%28ssh%29/notes.md"
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn same_drive_links_treat_normal_and_verbatim_disk_prefixes_equally() {
        let link = link(
            Path::new(r"c:\context\index.md"),
            Path::new(r"\\?\C:\source\a #%.md"),
        )
        .unwrap();
        assert_eq!(link.destination, "../source/a%20%23%25.md");
        assert!(link.local_path.is_none());
    }

    #[cfg(unix)]
    #[test]
    fn unix_links_keep_backslashes_as_filename_characters() {
        let link = link(
            Path::new("/context/index.md"),
            Path::new("/source/知识 #100% (ssh)\\?.md"),
        )
        .unwrap();
        assert_eq!(
            link.destination,
            "../source/%E7%9F%A5%E8%AF%86%20%23100%25%20%28ssh%29%5C%3F.md"
        );
        assert!(link.local_path.is_none());
    }

    #[test]
    fn link_text_does_not_interpret_authored_markdown_or_html() {
        let link = FileLink {
            destination: "file:///D:/a%60%5B%23%5D.md".into(),
            local_path: Some("D:/a`[#].md".into()),
        };
        let rendered = link.markdown(
            "[ssh] <http>",
            crate::language::OutputLanguage::English.messages(),
        );
        assert_eq!(
            rendered,
            "[\\[ssh\\] &lt;http&gt;](file:///D:/a%60%5B%23%5D.md) (Local path: D:/a\\`\\[\\#\\].md)"
        );
    }
}
pub(crate) fn publication(
    plan: &mut Plan,
    config: &Config,
    resources: &devmeld_resources::Resources,
) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    let directory = resolve(plan.root(), &config.publication.directory)?;
    let index = directory.join("index.md");
    let config_path = plan.root().join(".devmeld/context.json");
    let mut files = BTreeMap::new();
    let messages = config.publication.language.messages();
    let mut navigation = format!(
        "# {}\n\n{}\n\n",
        messages.context_heading, messages.navigation_notice
    );
    for resource in resources.iter() {
        let id = resource.id().as_str();
        let page = directory.join(format!("r-{id}.md"));
        let summary = resource.summary().unwrap_or(messages.original_document);
        navigation.push_str(&format!(
            "- {} — {}\n",
            link(&index, &page)?.markdown(resource.title(), messages),
            text(summary)
        ));
        let mut content = format!(
            "# {}\n\n{}\n\n{}\n\n{}\n\n{}\n",
            text(resource.title()),
            text(summary),
            messages.resource_notice,
            link(&page, resource.source())?.markdown(messages.original_source, messages),
            link(&page, &config_path)?.markdown(messages.managed_registration, messages)
        );
        for (key, value) in resource.attributes() {
            content.push_str(&format!("\n- {}: {}", text(key), text(value)));
        }
        content.push_str("\n\n");
        for (label, target) in resource.references() {
            content.push_str(&format!(
                "- {}\n",
                link(&page, target)?.markdown(label, messages)
            ));
        }
        let mut tools: Vec<_> = config
            .access
            .iter()
            .filter(|a| a.resource == id)
            .map(|a| a.tool.as_str())
            .collect();
        tools.sort();
        if !tools.is_empty() {
            content.push_str(&format!("\n## {}\n\n", messages.access_heading));
            content.push_str(messages.access_guidance);
            content.push_str("\n\n");
            for tool in tools {
                content.push_str(&format!(
                    "- {}\n",
                    link(&page, &directory.join(format!("r-{tool}.md")))?.markdown(
                        &format!("{}: {tool}", messages.associated_resource),
                        messages
                    )
                ));
            }
        }
        files.insert(page, content.into_bytes());
    }
    navigation.push_str(&format!(
        "\n{}\n",
        link(&index, &config_path)?.markdown(messages.managed_registration, messages)
    ));
    files.insert(index.clone(), navigation.into_bytes());
    for entry in &config.publication.entries {
        if entry.kind == "instructions" {
            continue;
        }
        if entry.kind != "file" {
            return Err(error("unsupported entry kind"));
        }
        let path = resolve(plan.root(), &entry.path)?;
        let content = format!(
            "# {}\n\n{}{}{}\n",
            messages.project_heading,
            messages.entry_intro,
            link(&path, &index)?.markdown(messages.navigation_label, messages),
            messages.entry_after_link,
        );
        if files.insert(path, content.into_bytes()).is_some() {
            return Err(error("duplicate publication target"));
        }
    }
    Ok(files)
}

pub(crate) fn instruction_entry(root: &Path, config: &Config, path: &Path) -> Result<String> {
    let messages = config.publication.language.messages();
    let index = resolve(root, &config.publication.directory)?.join("index.md");
    Ok(format!(
        "## {}\n\n{}{}{}\n\n{}\n\n{}\n",
        messages.project_heading,
        messages.entry_intro,
        link(path, &index)?.markdown(messages.navigation_label, messages),
        messages.entry_after_link,
        messages.shared_maintenance,
        link(path, &root.join(".devmeld/context.json"))?
            .markdown(messages.managed_registration, messages)
    ))
}
