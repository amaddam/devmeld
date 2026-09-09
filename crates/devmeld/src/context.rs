//! Local context selection. A present marker is a boundary, not proof of validity.
use crate::{Result, error, storage};
use std::path::{Path, PathBuf};

pub(crate) fn marker_exists(root: &Path) -> Result<bool> {
    let marker = root.join(".devmeld");
    match std::fs::symlink_metadata(&marker) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(error(format!("cannot inspect {}: {e}", marker.display()))),
    }
}

pub(crate) fn select(cwd: &Path, explicit: Option<&Path>) -> Result<PathBuf> {
    let cwd = storage::resolve(cwd, ".")?;
    if let Some(path) = explicit {
        return storage::resolve(
            &cwd,
            path.to_str()
                .ok_or_else(|| error("non-Unicode context path"))?,
        );
    }
    for directory in cwd.ancestors() {
        if marker_exists(directory)? {
            return Ok(directory.to_owned());
        }
    }
    Ok(cwd.to_owned())
}

pub(crate) fn operands(cwd: &Path, root: &Path, args: &[String]) -> Result<Vec<String>> {
    let mut args = args.to_vec();
    let command: Vec<_> = args.iter().map(String::as_str).collect();
    let mut paths = Vec::new();
    match command.as_slice() {
        ["resource", "add", _, options @ ..] => {
            paths.push(2);
            let mut i = 0;
            while i < options.len() {
                if options[i] == "--schema" && i + 1 < options.len() {
                    paths.push(4 + i);
                }
                i += if crate::annotation_args::takes_no_value(options[i]) {
                    1
                } else {
                    2
                };
            }
        }
        ["entry", "add" | "remove", _, ..] => paths.push(2),
        ["output", _] => paths.push(1),
        ["init", options @ ..] => {
            for (i, pair) in options.chunks(2).enumerate() {
                if matches!(pair, ["--entry" | "--instruction-entry" | "--output", _]) {
                    paths.push(2 + i * 2);
                }
            }
        }
        _ => (),
    }
    for index in paths {
        let target = storage::resolve(cwd, &args[index])?;
        // Keep in-context references portable; outside inputs keep their explicit
        // local location, including another drive. Never move or copy the source.
        let reference = target.strip_prefix(root).unwrap_or(&target);
        args[index] = if reference.as_os_str().is_empty() {
            ".".into()
        } else {
            reference
                .to_str()
                .ok_or_else(|| error("non-Unicode input path"))?
                .into()
        };
    }
    Ok(args)
}
