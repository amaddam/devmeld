use crate::{Result, cargo, successful};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub const CORES: [&str; 4] = ["shared-kernel", "catalog", "local-context", "knowledge"];
const MEMBERS: [(&str, &str); 5] = [
    ("devmeld-shared-kernel", "crates/shared-kernel"),
    ("devmeld-catalog", "crates/catalog"),
    ("devmeld-local-context", "crates/local-context"),
    ("devmeld-knowledge", "crates/knowledge"),
    ("xtask", "tools/xtask"),
];

fn field<'a>(value: &'a Value, name: &str) -> Result<&'a Value> {
    value
        .get(name)
        .ok_or_else(|| format!("ARCH_SCHEMA: missing {name}").into())
}
fn text(value: &Value) -> Result<&str> {
    value
        .as_str()
        .ok_or_else(|| "ARCH_SCHEMA: expected string".into())
}
fn array(value: &Value) -> Result<&Vec<Value>> {
    value
        .as_array()
        .ok_or_else(|| "ARCH_SCHEMA: expected array".into())
}
fn boolean(value: &Value) -> Result<bool> {
    value
        .as_bool()
        .ok_or_else(|| "ARCH_SCHEMA: expected boolean".into())
}
fn path(value: &Value) -> Result<PathBuf> {
    Path::new(text(value)?)
        .canonicalize()
        .map_err(|e| format!("ARCH_PATH: {e}").into())
}

pub fn parse(bytes: &[u8]) -> Result<Value> {
    serde_json::from_slice(bytes).map_err(|e| format!("ARCH_METADATA_JSON: {e}").into())
}
pub fn metadata(root: &Path) -> Result<Value> {
    let output = cargo(
        root,
        &[
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--locked",
            "--offline",
        ],
    )
    .arg("--manifest-path")
    .arg(root.join("Cargo.toml"))
    .output()
    .map_err(|e| format!("ARCH_METADATA_COMMAND: {e}"))?;
    parse(&successful(output, "ARCH_METADATA_COMMAND")?.stdout)
}

fn redirected(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    // Junctions are not ordinary Unix symlinks; reject all Windows reparse
    // points before recursing. No OS-specific shell or external program.
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if metadata.file_attributes() & 0x400 != 0 {
            return true;
        }
    }
    false
}

pub fn reject_redirects(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if redirected(&metadata) {
        return Err(format!("ARCH_PATH: redirected source {}", path.display()).into());
    }
    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            reject_redirects(&entry?.path())?;
        }
    }
    Ok(())
}

fn names(directory: &Path) -> Result<BTreeSet<String>> {
    fs::read_dir(directory)?
        .map(|entry| {
            entry?
                .file_name()
                .into_string()
                .map_err(|_| "ARCH_SCOPE: non-Unicode entry".into())
        })
        .collect()
}

fn layout(root: &Path) -> Result<()> {
    if root.join("src").exists()
        || names(&root.join("crates"))? != CORES.into_iter().map(str::to_owned).collect()
        || names(&root.join("tools"))? != BTreeSet::from(["xtask".to_owned()])
    {
        return Err("ARCH_SCOPE: unexpected root facade/core/tool directory".into());
    }
    // Scope of the current delivery, not a permanent directory quota. Keep
    // these checks independent of semantic source/IO review.
    for (_, relative) in MEMBERS {
        for entry in fs::read_dir(root.join(relative))? {
            let entry = entry?;
            let name = entry.file_name();
            if entry.file_type()?.is_dir() {
                if name != "src" && name != "tests" {
                    return Err("ARCH_SCOPE: unplanned crate directory".into());
                }
                for source in fs::read_dir(entry.path())? {
                    let source = source?;
                    if !source.file_type()?.is_file()
                        || source.path().extension().is_none_or(|e| e != "rs")
                    {
                        return Err("ARCH_SCOPE: unplanned source layer/artifact".into());
                    }
                }
            } else if name != "Cargo.toml" {
                return Err("ARCH_SCOPE: unplanned crate artifact".into());
            }
        }
    }
    Ok(())
}

pub fn validate(root: &Path, metadata: &Value) -> Result<usize> {
    if field(metadata, "version")?.as_u64() != Some(1) {
        return Err("ARCH_SCHEMA: unexpected metadata version".into());
    }
    let root = root.canonicalize()?;
    if path(field(metadata, "workspace_root")?)? != root {
        return Err("ARCH_SCHEMA: workspace root mismatch".into());
    }
    let members = array(field(metadata, "workspace_members")?)?;
    let packages = array(field(metadata, "packages")?)?;
    if members.len() != MEMBERS.len() || packages.len() != MEMBERS.len() {
        return Err("ARCH_MEMBERS: expected four core libraries and xtask".into());
    }
    let mut ids = BTreeSet::new();
    for id in members {
        if !ids.insert(text(id)?) {
            return Err("ARCH_MEMBERS: duplicate opaque identity".into());
        }
    }
    let mut by_name = BTreeMap::new();
    for package in packages {
        let name = text(field(package, "name")?)?;
        let Some((_, relative)) = MEMBERS.iter().find(|(expected, _)| *expected == name) else {
            return Err("ARCH_MEMBERS: unknown package".into());
        };
        if by_name.insert(name, package).is_some() || !ids.remove(text(field(package, "id")?)?) {
            return Err("ARCH_MEMBERS: unknown or duplicate identity".into());
        }
        if path(field(package, "manifest_path")?)?
            != root.join(relative).join("Cargo.toml").canonicalize()?
        {
            return Err("ARCH_PATH: unexpected member manifest".into());
        }
        if !field(package, "source")?.is_null() || !array(field(package, "publish")?)?.is_empty() {
            return Err("ARCH_PACKAGE: member must be local and unpublished".into());
        }
    }
    if !ids.is_empty() {
        return Err("ARCH_MEMBERS: unresolved identity".into());
    }
    reject_redirects(&root.join("crates"))?;
    reject_redirects(&root.join("tools"))?;
    let mut edges = 0;
    for (name, package) in &by_name {
        let tool = *name == "xtask";
        let package_root = path(field(package, "manifest_path")?)?
            .parent()
            .ok_or("ARCH_PATH: manifest parent")?
            .to_owned();
        let mut primary = 0;
        for target in array(field(package, "targets")?)? {
            let kind = array(field(target, "kind")?)?;
            let expected = if tool { "bin" } else { "lib" };
            let source = path(field(target, "src_path")?)?;
            if kind.len() == 1 && kind[0] == expected {
                primary += 1;
                let entrypoint = if tool { "src/main.rs" } else { "src/lib.rs" };
                if source != package_root.join(entrypoint).canonicalize()?
                    || field(target, "crate_types")? != &serde_json::json!([expected])
                    || !boolean(field(target, "test")?)?
                    || (!tool && !boolean(field(target, "doctest")?)?)
                {
                    return Err("ARCH_TARGET: unexpected entrypoint/test policy".into());
                }
            } else if !tool && kind.len() == 1 && kind[0] == "test" {
                if !source.starts_with(package_root.join("tests").canonicalize()?) {
                    return Err("ARCH_TARGET: test source escapes owner".into());
                }
            } else {
                return Err("ARCH_TARGET: forbidden target kind".into());
            }
        }
        if primary != 1 || package_root.join("build.rs").exists() {
            return Err("ARCH_TARGET: expected one primary target and no build script".into());
        }
        let dependencies = array(field(package, "dependencies")?)?;
        if tool && dependencies.len() != 1 {
            return Err("ARCH_EDGE: xtask requires only serde_json".into());
        }
        let mut normal_kernel = 0;
        for dependency in dependencies {
            let kind = field(dependency, "kind")?;
            let kind = if kind.is_null() {
                "normal"
            } else {
                text(kind)?
            };
            if !["normal", "dev", "build"].contains(&kind) {
                return Err("ARCH_KIND: unknown dependency kind".into());
            }
            let destination = text(field(dependency, "name")?)?;
            let optional = boolean(field(dependency, "optional")?)?;
            let target = field(dependency, "target")?;
            let rename = field(dependency, "rename")?;
            if !target.is_null() {
                text(target)?;
            }
            if !rename.is_null() {
                text(rename)?;
            }
            let source = field(dependency, "source")?;
            if tool {
                if kind != "normal"
                    || destination != "serde_json"
                    || optional
                    || !target.is_null()
                    || !rename.is_null()
                    || source != "registry+https://github.com/rust-lang/crates.io-index"
                    || dependency.get("path").is_some_and(|p| !p.is_null())
                {
                    return Err("ARCH_EDGE: xtask dependency not admitted".into());
                }
            } else {
                let allowed = kind == "normal"
                    && *name != "devmeld-shared-kernel"
                    && destination == "devmeld-shared-kernel"
                    || kind == "dev"
                        && *name == "devmeld-knowledge"
                        && ["devmeld-catalog", "devmeld-local-context"].contains(&destination);
                if !allowed || !source.is_null() {
                    return Err(
                        format!("ARCH_EDGE: forbidden {name} --{kind}--> {destination}").into(),
                    );
                }
                if kind == "normal" && (optional || !target.is_null() || !rename.is_null()) {
                    return Err(
                        "ARCH_EDGE: identity dependency must be unconditional, required and unrenamed"
                            .into(),
                    );
                }
                let destination_package = by_name
                    .get(destination)
                    .ok_or("ARCH_EDGE: unknown destination")?;
                let expected = path(field(destination_package, "manifest_path")?)?;
                if path(field(dependency, "path")?)?
                    != expected.parent().ok_or("ARCH_PATH: dependency parent")?
                {
                    return Err("ARCH_PATH: redirected dependency".into());
                }
                if kind == "normal" {
                    normal_kernel += 1;
                }
            }
            edges += 1;
        }
        if !tool && *name != "devmeld-shared-kernel" && normal_kernel != 1 {
            return Err("ARCH_EDGE: expected one unconditional identity dependency".into());
        }
    }
    layout(&root)?;
    Ok(edges)
}

pub fn check(root: &Path) -> Result<()> {
    let edges = validate(root, &metadata(root)?)?;
    println!(
        "Architecture PASS: four core libraries + developer xtask; {edges} admitted declarations"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_metadata_fails_closed_with_schema_diagnostic() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let error = validate(&root, &serde_json::json!({})).unwrap_err();
        assert!(error.to_string().contains("ARCH_SCHEMA"));
    }
}
