use crate::{Result, error};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const FILE_LIMIT: u64 = 8 * 1024 * 1024;
const PLAN_LIMIT: usize = 128 * 1024 * 1024;

pub(crate) fn resolve(root: &Path, value: &str) -> Result<PathBuf> {
    if value.trim().is_empty() {
        return Err(error("empty path"));
    }
    let mut path = PathBuf::new();
    for part in root.join(value).components() {
        match part {
            std::path::Component::CurDir => (),
            std::path::Component::ParentDir => {
                if !path.pop() {
                    return Err(error("path escapes filesystem root"));
                }
            }
            _ => path.push(part),
        }
    }
    safe_components(&path)?;
    let mut parent = path.clone();
    let mut suffix = Vec::new();
    while !parent.try_exists()? {
        suffix.push(
            parent
                .file_name()
                .ok_or_else(|| error("invalid path"))?
                .to_owned(),
        );
        if !parent.pop() {
            return Err(error("no existing path ancestor"));
        }
    }
    let mut physical = parent.canonicalize()?;
    for part in suffix.iter().rev() {
        physical.push(part);
    }
    Ok(physical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "devmeld-storage-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&p).unwrap();
            Self(p.canonicalize().unwrap())
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn interrupted_create_recovers_and_preserves_external_conflicts() {
        let f = Fixture::new();
        let target = f.0.join("first.md");
        let mut plan = Plan::new(f.0.clone()).unwrap();
        plan.set(target.clone(), Some(b"generated".to_vec()))
            .unwrap();
        assert!(plan.apply_until(Some(1)).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"generated");
        assert!(Plan::new(f.0.clone()).is_err());
        fs::write(&target, "external").unwrap();
        assert!(Plan::recovery(f.0.clone()).unwrap().apply().is_err());
        assert_eq!(fs::read(&target).unwrap(), b"external");
        fs::write(&target, "generated").unwrap();
        Plan::recovery(f.0.clone()).unwrap().apply().unwrap();
        assert!(!target.exists());
        assert!(Plan::recovery(f.0.clone()).unwrap().is_empty());
    }
    #[test]
    fn replacement_and_deletion_restore_original_identity_at_every_boundary() {
        for stop in 0..=3 {
            let f = Fixture::new();
            let a = f.0.join("a.md");
            let b = f.0.join("b.md");
            let mut initial = Plan::new(f.0.clone()).unwrap();
            initial
                .set(a.clone(), Some(b"original a".to_vec()))
                .unwrap();
            initial
                .set(b.clone(), Some(b"original b".to_vec()))
                .unwrap();
            initial.apply().unwrap();
            let a_before = observe(&a).unwrap();
            let b_before = observe(&b).unwrap();
            let mut change = Plan::new(f.0.clone()).unwrap();
            change.set(a.clone(), Some(b"changed".to_vec())).unwrap();
            change.set(b.clone(), None).unwrap();
            assert!(change.apply_until(Some(stop)).is_err());
            Plan::recovery(f.0.clone()).unwrap().apply().unwrap();
            assert_eq!(observe(&a).unwrap(), a_before);
            assert_eq!(observe(&b).unwrap(), b_before);
            let mut next = Plan::new(f.0.clone()).unwrap();
            next.set(a.clone(), Some(b"original a".to_vec())).unwrap();
            assert!(next.is_empty());
        }
    }
    #[test]
    fn an_identical_external_creation_is_not_owned_by_the_journal() {
        let f = Fixture::new();
        let target = f.0.join("a.md");
        let mut plan = Plan::new(f.0.clone()).unwrap();
        plan.set(target.clone(), Some(b"same bytes".to_vec()))
            .unwrap();
        assert!(plan.apply_until(Some(0)).is_err());
        fs::write(&target, b"same bytes").unwrap();
        assert!(Plan::recovery(f.0.clone()).unwrap().apply().is_err());
        assert_eq!(fs::read(target).unwrap(), b"same bytes");
    }
    #[test]
    fn cooperative_lock_rejects_a_second_writer() {
        let f = Fixture::new();
        fs::create_dir_all(f.0.join(".devmeld/state")).unwrap();
        let lock = OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(f.0.join(".devmeld/state/lock"))
            .unwrap();
        lock.try_lock().unwrap();
        let mut plan = Plan::new(f.0.clone()).unwrap();
        plan.set(f.0.join("a.md"), Some(b"data".to_vec())).unwrap();
        assert!(plan.apply().unwrap_err().to_string().contains("locked"));
        assert!(!f.0.join("a.md").exists());
    }
}

fn safe_components(path: &Path) -> Result<()> {
    let mut prefix = PathBuf::new();
    for component in path.components() {
        prefix.push(component);
        if matches!(component, std::path::Component::Prefix(_)) {
            continue;
        }
        match fs::symlink_metadata(&prefix) {
            Ok(metadata) => {
                let mut redirected = metadata.file_type().is_symlink();
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    redirected |= metadata.file_attributes() & 0x400 != 0;
                }
                if redirected {
                    return Err(error(format!(
                        "redirected path is not supported: {}",
                        prefix.display()
                    )));
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
fn same_path(a: &Path, b: &Path) -> bool {
    #[cfg(windows)]
    {
        a.to_string_lossy()
            .eq_ignore_ascii_case(&b.to_string_lossy())
    }
    #[cfg(not(windows))]
    {
        a == b
    }
}
pub(crate) fn overlaps(a: &Path, b: &Path) -> bool {
    a.ancestors().any(|p| same_path(p, b)) || b.ancestors().any(|p| same_path(p, a))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Observed {
    bytes: Vec<u8>,
    identity: String,
}

fn identity(path: &Path) -> Result<String> {
    use file_id::FileId;
    Ok(match file_id::get_file_id(path)? {
        FileId::Inode {
            device_id,
            inode_number,
        } => format!("unix:{device_id}:{inode_number}"),
        FileId::LowRes {
            volume_serial_number,
            file_index,
        } => format!("win-low:{volume_serial_number}:{file_index}"),
        FileId::HighRes {
            volume_serial_number,
            file_id,
        } => format!("win-high:{volume_serial_number}:{file_id}"),
    })
}
fn observe(path: &Path) -> Result<Option<Observed>> {
    safe_components(path)?;
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
        Ok(m) if !m.is_file() || m.file_type().is_symlink() => {
            return Err(error(format!("not an ordinary file: {}", path.display())));
        }
        _ => (),
    }
    let before = identity(path)?;
    let mut bytes = Vec::new();
    File::open(path)?
        .take(FILE_LIMIT + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > FILE_LIMIT {
        return Err(error(format!("file exceeds 8 MiB: {}", path.display())));
    }
    if before != identity(path)? {
        return Err(error("file changed during observation"));
    }
    Ok(Some(Observed {
        bytes,
        identity: before,
    }))
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    format_version: u32,
    files: BTreeMap<PathBuf, Observed>,
}
impl Default for Receipt {
    fn default() -> Self {
        Self {
            format_version: 1,
            files: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Step {
    path: PathBuf,
    before: Option<Observed>,
    after: Option<Observed>,
    backup: PathBuf,
    stage: PathBuf,
    swap: PathBuf,
    removed: PathBuf,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    format_version: u32,
    committed: bool,
    steps: Vec<Step>,
}

/// An inspectable captured operation; apply rechecks its inputs before writing.
pub struct Plan {
    root: PathBuf,
    basis: BTreeMap<PathBuf, Option<Observed>>,
    changes: BTreeMap<PathBuf, Option<Vec<u8>>>,
    receipt: Receipt,
    sources: BTreeSet<PathBuf>,
    recovery: Option<Journal>,
}
impl Plan {
    pub(crate) fn new(root: PathBuf) -> Result<Self> {
        let mut plan = Self::empty(root);
        if plan.root.join(".devmeld/state/pending.json").try_exists()? {
            return Err(error("pending recovery; run recover before new operations"));
        }
        let state = plan.root.join(".devmeld/state/owned.json");
        if let Some(bytes) = plan.capture(&state)? {
            plan.receipt = serde_json::from_slice(&bytes)?;
            if plan.receipt.format_version != 1 {
                return Err(error("unsupported ownership format_version"));
            }
        }
        Ok(plan)
    }
    fn empty(root: PathBuf) -> Self {
        Self {
            root,
            basis: BTreeMap::new(),
            changes: BTreeMap::new(),
            receipt: Receipt::default(),
            sources: BTreeSet::new(),
            recovery: None,
        }
    }
    pub(crate) fn recovery(root: PathBuf) -> Result<Self> {
        let mut plan = Self::empty(root);
        let path = plan.root.join(".devmeld/state/pending.json");
        if let Some(bytes) = plan.capture(&path)? {
            let journal: Journal = serde_json::from_slice(&bytes)?;
            if journal.format_version != 1 {
                return Err(error("unsupported journal format_version"));
            }
            plan.recovery = Some(journal);
        }
        Ok(plan)
    }
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
    pub(crate) fn owned_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.receipt.files.keys()
    }
    pub(crate) fn source(&mut self, path: &Path) -> Result<Vec<u8>> {
        self.sources.insert(path.to_owned());
        self.capture(path)?
            .ok_or_else(|| error(format!("missing source: {}", path.display())))
    }
    pub(crate) fn capture(&mut self, path: &Path) -> Result<Option<Vec<u8>>> {
        let observed = observe(path)?;
        let bytes = observed.as_ref().map(|o| o.bytes.clone());
        if let Some(old) = self.basis.insert(path.to_owned(), observed.clone()) {
            if old != observed {
                return Err(error("input changed while preparing preview"));
            }
        }
        if self
            .basis
            .values()
            .flatten()
            .map(|v| v.bytes.len())
            .sum::<usize>()
            > PLAN_LIMIT
        {
            return Err(error("operation exceeds 128 MiB"));
        }
        Ok(bytes)
    }
    pub(crate) fn set(&mut self, path: PathBuf, after: Option<Vec<u8>>) -> Result<()> {
        let before = self.capture(&path)?;
        let current = self.basis.get(&path).and_then(Option::as_ref);
        let overlaps = self.sources.iter().any(|source| {
            same_path(source, &path)
                || current.is_some_and(|c| {
                    self.basis
                        .get(source)
                        .and_then(Option::as_ref)
                        .is_some_and(|s| s.identity == c.identity)
                })
        });
        devmeld_publication::classify(
            before.as_deref(),
            after.as_deref(),
            self.receipt.files.contains_key(&path),
            self.receipt.files.get(&path) == current,
            overlaps,
        )
        .map_err(|e| error(format!("{e}: {}", path.display())))?;
        if after.as_ref().is_some_and(|b| b.len() as u64 > FILE_LIMIT) {
            return Err(error("generated file exceeds 8 MiB"));
        }
        if before != after {
            self.changes.insert(path, after);
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty() && self.recovery.is_none()
    }
    pub fn preview(&self) -> String {
        if let Some(journal) = &self.recovery {
            let mut output = format!(
                "Recovery: {}\n",
                if journal.committed {
                    "finish committed-operation cleanup"
                } else {
                    "restore unfinished operation"
                }
            );
            for step in &journal.steps {
                output.push_str(&format!(
                    "\nTarget: {}\nRestore:\n{}\n",
                    step.path.display(),
                    step.before
                        .as_ref()
                        .map(|b| String::from_utf8_lossy(&b.bytes).into_owned())
                        .unwrap_or_else(|| "<absent>".into())
                ));
            }
            return output;
        }
        let mut output = format!("{} changed target(s)\n", self.changes.len());
        for (path, after) in &self.changes {
            output.push_str(&format!(
                "\nTarget: {}\nBefore:\n{}\nAfter:\n{}\n",
                path.display(),
                self.basis
                    .get(path)
                    .and_then(Option::as_ref)
                    .map(|o| String::from_utf8_lossy(&o.bytes).into_owned())
                    .unwrap_or_else(|| "<absent>".into()),
                after
                    .as_ref()
                    .map(|b| String::from_utf8_lossy(b).into_owned())
                    .unwrap_or_else(|| "<absent>".into())
            ));
        }
        output
    }
    fn recheck(&self) -> Result<()> {
        for (path, expected) in &self.basis {
            if observe(path)? != *expected {
                return Err(error(format!("stale preview: {}", path.display())));
            }
        }
        Ok(())
    }
    pub fn apply(self) -> Result<()> {
        self.apply_until(None)
    }

    // Fault injection remains private to storage tests; it is not a CLI executor.
    fn apply_until(mut self, stop_after: Option<usize>) -> Result<()> {
        if self.is_empty() {
            return Ok(());
        }
        self.recheck()?;
        let state_dir = self.root.join(".devmeld/state");
        safe_components(&state_dir)?;
        fs::create_dir_all(&state_dir)?;
        let lock_path = state_dir.join("lock");
        safe_components(&lock_path)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(lock_path)?;
        lock.try_lock()
            .map_err(|e| error(format!("context is locked by another writer: {e}")))?;
        self.recheck()?;
        let journal_path = state_dir.join("pending.json");
        if let Some(journal) = self.recovery.take() {
            recover(&journal, &journal_path)?;
            return Ok(());
        }
        if journal_path.try_exists()? {
            return Err(error("pending recovery; run recover"));
        }
        let nonce = format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        );
        let mut journal = Journal {
            format_version: 1,
            committed: false,
            steps: Vec::new(),
        };
        for (path, bytes) in &self.changes {
            let step = stage_step(
                path,
                self.basis.get(path).cloned().flatten(),
                bytes.clone(),
                &nonce,
                journal.steps.len(),
            )?;
            if let Some(after) = &step.after {
                self.receipt.files.insert(path.clone(), after.clone());
            } else {
                self.receipt.files.remove(path);
            }
            journal.steps.push(step);
        }
        let receipt_path = state_dir.join("owned.json");
        let receipt_bytes = super::declarations::encode(&self.receipt)?;
        let receipt_step = stage_step(
            &receipt_path,
            self.basis.get(&receipt_path).cloned().flatten(),
            Some(receipt_bytes),
            &nonce,
            journal.steps.len(),
        )?;
        journal.steps.push(receipt_step);
        write_new(&journal_path, &super::declarations::encode(&journal)?)?;
        self.recheck()?;
        if stop_after == Some(0) {
            return Err(error("injected interruption"));
        }
        for (index, step) in journal.steps.iter().enumerate() {
            install(step)?;
            if stop_after == Some(index + 1) {
                return Err(error("injected interruption"));
            }
        }
        journal.committed = true;
        let next = journal_path.with_extension("next");
        write_new(&next, &super::declarations::encode(&journal)?)?;
        fs::rename(&next, &journal_path)?;
        cleanup(&journal, &journal_path)?;
        Ok(())
    }
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    safe_components(path)?;
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
fn stage_step(
    path: &Path,
    before: Option<Observed>,
    after: Option<Vec<u8>>,
    nonce: &str,
    index: usize,
) -> Result<Step> {
    safe_components(path)?;
    let parent = path.parent().ok_or_else(|| error("target has no parent"))?;
    fs::create_dir_all(parent)?;
    let prefix = format!(".devmeld-{nonce}-{index}");
    let mut step = Step {
        path: path.into(),
        before,
        after: None,
        backup: parent.join(format!("{prefix}.before")),
        stage: parent.join(format!("{prefix}.after")),
        swap: parent.join(format!("{prefix}.swap")),
        removed: parent.join(format!("{prefix}.removed")),
    };
    if observe(path)? != step.before {
        return Err(error(format!("stale target: {}", path.display())));
    }
    if step.before.is_some() {
        fs::hard_link(path, &step.backup)?;
    }
    if let Some(bytes) = after {
        write_new(&step.stage, &bytes)?;
        step.after = observe(&step.stage)?;
    }
    Ok(step)
}
fn install(step: &Step) -> Result<()> {
    if observe(&step.path)? != step.before {
        return Err(error(format!("stale target: {}", step.path.display())));
    }
    match (&step.before, &step.after) {
        (None, Some(_)) => fs::hard_link(&step.stage, &step.path)?,
        (Some(_), Some(_)) => {
            fs::hard_link(&step.stage, &step.swap)?;
            fs::rename(&step.swap, &step.path)?;
        }
        (Some(_), None) => fs::rename(&step.path, &step.removed)?,
        (None, None) => (),
    }
    Ok(())
}
fn recover(journal: &Journal, journal_path: &Path) -> Result<()> {
    if !journal.committed {
        // Preflight the entire rollback before restoring any file.
        for step in &journal.steps {
            recovery_state(step)?;
        }
        for step in journal.steps.iter().rev() {
            if !recovery_state(step)? {
                continue;
            }
            match (&step.before, &step.after) {
                (None, Some(_)) => fs::remove_file(&step.path)?,
                (Some(_), None) => {
                    fs::hard_link(&step.removed, &step.path)?;
                }
                (Some(_), Some(_)) => {
                    if step.swap.try_exists()? {
                        remove_anchor(&step.swap, step.before.as_ref(), step.after.as_ref())?;
                    }
                    fs::hard_link(&step.backup, &step.swap)?;
                    fs::rename(&step.swap, &step.path)?;
                }
                (None, None) => (),
            }
        }
    }
    cleanup(journal, journal_path)
}
fn recovery_state(step: &Step) -> Result<bool> {
    let current = observe(&step.path)?;
    if current == step.before {
        return Ok(false);
    }
    if current != step.after {
        return Err(error(format!("recovery conflict: {}", step.path.display())));
    }
    if step.after.is_none() && observe(&step.removed)? != step.before {
        return Err(error(
            "recovery conflict: deletion has no matching tombstone",
        ));
    }
    if step.before.is_some() && observe(&step.backup)? != step.before {
        return Err(error("recovery conflict: original backup changed"));
    }
    Ok(true)
}
fn remove_anchor(path: &Path, before: Option<&Observed>, after: Option<&Observed>) -> Result<()> {
    if let Some(current) = observe(path)? {
        if ![before, after]
            .into_iter()
            .flatten()
            .any(|o| o.identity == current.identity)
        {
            return Err(error(format!(
                "staging cleanup conflict: {}",
                path.display()
            )));
        }
        fs::remove_file(path)?;
    }
    Ok(())
}
fn cleanup(journal: &Journal, journal_path: &Path) -> Result<()> {
    for step in &journal.steps {
        for path in [&step.backup, &step.stage, &step.swap, &step.removed] {
            remove_anchor(path, step.before.as_ref(), step.after.as_ref())?;
        }
    }
    let next = journal_path.with_extension("next");
    if let Some(value) = observe(&next)? {
        let proposed: Journal = serde_json::from_slice(&value.bytes)?;
        if proposed.steps.iter().map(|s| &s.path).collect::<Vec<_>>()
            != journal.steps.iter().map(|s| &s.path).collect::<Vec<_>>()
        {
            return Err(error("unexpected pending journal update"));
        }
        fs::remove_file(next)?;
    }
    fs::remove_file(journal_path)?;
    Ok(())
}
