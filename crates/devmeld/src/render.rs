use crate::{
    Result,
    declarations::Config,
    error,
    language::Messages,
    markdown::{self, Document, Inline, authored, literal, prose, text},
    storage::{Plan, resolve},
};
use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag, TagEnd};
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf, Prefix};

struct FileLink {
    destination: String,
    local_path: Option<String>,
}

// Publication consumes the domain's effective values, not configuration controls.
// CLI inspection has its own detailed rendering of local values and saved choices.
fn context_metadata(
    organization: &devmeld_resources::organization::Organization,
    path: &devmeld_resources::organization::OrganizationPath,
    messages: &Messages,
) -> Result<Vec<Inline>> {
    let effective = organization.effective_annotations(path)?;
    let mut result = Vec::new();
    if !effective.tags().is_empty() {
        let mut tags = vec![text(&format!("{}:", messages.annotation_tags))];
        tags.extend(markdown::list(
            effective.tags().keys().map(|tag| vec![literal(tag)]),
        ));
        result.push(tags);
    }
    for (key, field) in effective.fields() {
        result.push(vec![literal(key), text(": "), authored(field.value())]);
    }
    Ok(result)
}

impl FileLink {
    fn events(&self, label: &str, messages: &Messages) -> Inline {
        let mut result = vec![
            Event::Start(Tag::Link {
                link_type: LinkType::Inline,
                dest_url: self.destination.clone().into(),
                title: "".into(),
                id: "".into(),
            }),
            // Brackets are structural even when the same text is plain prose.
            // A code span keeps the entire label literal inside a Markdown link.
            if label.contains(['[', ']']) {
                literal(label)
            } else {
                authored(label)
            },
            Event::End(TagEnd::Link),
        ];
        if let Some(path) = &self.local_path {
            result.extend([
                text(&format!(" ({}: ", messages.local_path)),
                literal(path),
                text(")"),
            ]);
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
        let mut document = Document::default();
        document.paragraph(link.events(
            "[ssh] <http>",
            crate::language::OutputLanguage::English.messages(),
        ));
        let rendered = document.finish().unwrap();
        assert_eq!(
            rendered,
            "[`[ssh] <http>`](file:///D:/a%60%5B%23%5D.md) (Local path: ``D:/a`[#].md``)\n"
        );
    }

    #[test]
    fn authored_link_labels_roundtrip_as_one_link_with_the_exact_destination() {
        for label in [
            "use_when",
            "x]",
            "a [b]",
            "[ssh] <http>",
            "`a`",
            "hello!",
            "a &amp; b",
            "*go*",
            "_private",
            "notes #",
        ] {
            let link = FileLink {
                destination: "../test_app%20%23.md".into(),
                local_path: None,
            };
            let mut document = Document::default();
            document
                .paragraph(link.events(label, crate::language::OutputLanguage::English.messages()));
            let output = document.finish().unwrap();
            let mut count = 0;
            let mut actual = String::new();
            for event in pulldown_cmark::Parser::new(&output) {
                match event {
                    Event::Start(Tag::Link { dest_url, .. }) => {
                        assert_eq!(dest_url.as_ref(), link.destination, "{output}");
                        count += 1;
                    }
                    Event::Text(value) | Event::Code(value) => actual.push_str(&value),
                    Event::Start(Tag::Paragraph) | Event::End(TagEnd::Paragraph | TagEnd::Link) => {
                        ()
                    }
                    other => panic!("{label:?} became {other:?}: {output}"),
                }
            }
            assert_eq!(count, 1, "{output}");
            assert_eq!(actual, label, "{output}");
        }
    }
}
fn children(
    content: &mut Document,
    groups: Option<Vec<Inline>>,
    resources: Option<Vec<Inline>>,
    messages: &Messages,
) {
    for (heading, items) in [
        (messages.groups_heading, groups),
        (messages.resources_heading, resources),
    ] {
        if let Some(items) = items {
            content.heading(HeadingLevel::H2, heading);
            content.list(items);
        }
    }
}

pub(crate) fn publication(
    plan: &mut Plan,
    config: &Config,
    resources: &devmeld_resources::Resources,
) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    let directory = resolve(plan.root(), &config.publication.directory)?;
    let index = directory.join("index.md");
    let mut files = BTreeMap::new();
    let messages = config.publication.language.messages();
    let mut navigation = Document::default();
    navigation.heading(HeadingLevel::H1, messages.context_heading);
    navigation.paragraph(prose(messages.navigation_notice));
    navigation.paragraph(prose(messages.maintenance_notice));
    let organization = config.organization()?;
    let pages = crate::page_paths::pages(&directory, &organization)?;
    let mut child_resources = BTreeMap::<_, Vec<Inline>>::new();
    let mut child_groups = BTreeMap::<_, Vec<Inline>>::new();
    for (address, identity) in organization.resources() {
        let resource = resources
            .get(identity)
            .ok_or_else(|| error("missing organized resource"))?;
        let id = resource.id().as_str();
        let page = pages
            .resources
            .get(id)
            .ok_or_else(|| error("missing resource page"))?;
        let description = organization
            .annotations(address)
            .ok_or_else(|| error("missing node annotations"))?
            .description();
        let summary = description.or_else(|| resource.summary());
        let label = if address.parent().is_some() {
            address.as_str()
        } else {
            resource.title()
        };
        let parent = address.parent();
        let container = match &parent {
            Some(parent) => pages
                .groups
                .get(parent)
                .ok_or_else(|| error("missing parent group page"))?,
            None => &index,
        };
        let mut item = link(container, page)?.events(label, messages);
        if let Some(summary) = summary {
            item.extend([text(" — "), authored(summary)]);
        }
        child_resources.entry(parent).or_default().push(item);
        let metadata = context_metadata(&organization, address, messages)?;
        let mut content = Document::default();
        content.ownership(messages.resource_marker);
        content.heading(HeadingLevel::H1, resource.title());
        if let Some(summary) = resource.summary() {
            content.paragraph([authored(summary)]);
        }
        if let Some(description) = description {
            if resource.summary().is_some() {
                content.heading(HeadingLevel::H2, messages.context_notes);
            }
            content.paragraph([authored(description)]);
        }
        content
            .paragraph(link(page, resource.source())?.events(messages.original_source, messages));
        if !resource.attributes().is_empty() {
            content.heading(HeadingLevel::H2, messages.source_attributes);
            content.list(
                resource
                    .attributes()
                    .iter()
                    .map(|(key, value)| vec![literal(key), text(": "), authored(value)]),
            );
        }
        if !metadata.is_empty() {
            if !resource.attributes().is_empty() {
                content.heading(HeadingLevel::H2, messages.context_information);
            }
            content.list(metadata);
        }
        if !resource.references().is_empty() {
            content.heading(HeadingLevel::H2, messages.references_heading);
        }
        let mut references = Vec::new();
        for (label, target) in resource.references() {
            references.push(link(page, target)?.events(label, messages));
        }
        content.list(references);
        let mut tools: Vec<_> = config
            .access
            .iter()
            .filter(|a| a.resource == id)
            .map(|a| a.tool.as_str())
            .collect();
        tools.sort();
        if !tools.is_empty() {
            content.heading(HeadingLevel::H2, messages.access_heading);
            content.paragraph(prose(messages.access_guidance));
            let mut associations = Vec::new();
            for tool in tools {
                let address = config
                    .resources
                    .iter()
                    .find(|r| r.id == tool)
                    .ok_or_else(|| error("missing associated resource"))?
                    .address();
                associations.push(
                    link(
                        page,
                        pages
                            .resources
                            .get(tool)
                            .ok_or_else(|| error("missing associated page"))?,
                    )?
                    .events(
                        &format!("{}: {address}", messages.associated_resource),
                        messages,
                    ),
                );
            }
            content.list(associations);
        }
        files.insert(page.clone(), content.finish()?.into_bytes());
    }
    // Build direct-child links before rendering parents; lexical order must not
    // decide whether a parent can discover its children.
    for group in organization.groups() {
        let parent = group.parent();
        let container = match &parent {
            Some(parent) => pages
                .groups
                .get(parent)
                .ok_or_else(|| error("missing parent group page"))?,
            None => &index,
        };
        let page = pages
            .groups
            .get(group)
            .ok_or_else(|| error("missing group page"))?;
        let description = organization
            .annotations(group)
            .ok_or_else(|| error("missing group annotations"))?
            .description();
        let mut item = link(container, page)?.events(group.as_str(), messages);
        if let Some(description) = description {
            item.extend([text(" — "), authored(description)]);
        }
        child_groups.entry(parent).or_default().push(item);
    }
    for group in organization.groups() {
        let page = pages
            .groups
            .get(group)
            .ok_or_else(|| error("missing group page"))?;
        let mut content = Document::default();
        content.ownership(messages.resource_marker);
        content.heading(HeadingLevel::H1, group.as_str());
        if let Some(description) = organization
            .annotations(group)
            .ok_or_else(|| error("missing group annotations"))?
            .description()
        {
            content.paragraph([authored(description)]);
        }
        let metadata = context_metadata(&organization, group, messages)?;
        content.list(metadata);
        let parent = group.parent();
        let (container, label) = match &parent {
            Some(parent) => (
                pages
                    .groups
                    .get(parent)
                    .ok_or_else(|| error("missing parent group page"))?,
                parent.as_str(),
            ),
            None => (&index, messages.navigation_label),
        };
        content.paragraph(link(page, container)?.events(label, messages));
        children(
            &mut content,
            child_groups.remove(&Some(group.clone())),
            child_resources.remove(&Some(group.clone())),
            messages,
        );
        files.insert(page.clone(), content.finish()?.into_bytes());
    }
    children(
        &mut navigation,
        child_groups.remove(&None),
        child_resources.remove(&None),
        messages,
    );
    files.insert(index.clone(), navigation.finish()?.into_bytes());
    for entry in &config.publication.entries {
        if entry.kind == "instructions" {
            continue;
        }
        if entry.kind != "file" {
            return Err(error("unsupported entry kind"));
        }
        let path = resolve(plan.root(), &entry.path)?;
        let content = entry_document(&path, &index, messages, HeadingLevel::H1)?.finish()?;
        if files.insert(path, content.into_bytes()).is_some() {
            return Err(error("duplicate publication target"));
        }
    }
    Ok(files)
}

pub(crate) fn instruction_entry(root: &Path, config: &Config, path: &Path) -> Result<String> {
    let messages = config.publication.language.messages();
    let index = resolve(root, &config.publication.directory)?.join("index.md");
    let mut content = entry_document(path, &index, messages, HeadingLevel::H2)?;
    content.paragraph(prose(messages.shared_maintenance));
    content.finish()
}

fn entry_document(
    path: &Path,
    index: &Path,
    messages: &Messages,
    level: HeadingLevel,
) -> Result<Document> {
    let mut content = Document::default();
    content.heading(level, messages.project_heading);
    let mut introduction = prose(messages.entry_intro);
    introduction.extend(link(path, index)?.events(messages.navigation_label, messages));
    introduction.extend(prose(messages.entry_after_link));
    content.paragraph(introduction);
    content.paragraph(prose(messages.maintenance_notice));
    Ok(content)
}
