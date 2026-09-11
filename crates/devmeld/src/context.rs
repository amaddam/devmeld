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
    storage::local_path(cwd)?;
    if !cwd.is_absolute() {
        return Err(crate::error("input base directory must be absolute"));
    }
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

/// Adapt a typed native input to a portable in-context or absolute local reference.
pub(crate) fn reference(base: &Path, root: &Path, path: &Path) -> Result<String> {
    let target = storage::resolve(
        base,
        path.to_str()
            .ok_or_else(|| crate::error("non-Unicode input path"))?,
    )?;
    let reference = target.strip_prefix(root).unwrap_or(&target);
    Ok(if reference.as_os_str().is_empty() {
        ".".into()
    } else {
        let text = reference
            .to_str()
            .ok_or_else(|| crate::error("non-Unicode input path"))?;
        // Ordinary relative paths use a readable separator on Windows. Do not
        // rewrite verbatim absolute paths: that can change filesystem semantics.
        #[cfg(windows)]
        if !reference.is_absolute() {
            return Ok(text.replace('\\', "/"));
        }
        text.into()
    })
}
