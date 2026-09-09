use crate::{Result, error};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const FILE_LIMIT: u64 = 8 * 1024 * 1024;
const PLAN_LIMIT: usize = 128 * 1024 * 1024;

// Validate path syntax before filesystem access. This is not a detector for
// remote filesystems hidden behind local mounts or drive mappings.
pub(crate) fn local_path(path: &Path) -> Result<()> {
    if let Some((scheme, _)) = path.to_str().and_then(|value| value.split_once("://")) {
        if scheme.len() > 1
            && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
            && scheme
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"+-.".contains(&b))
        {
            return Err(error(
                "expected a native local path, not a URL; store remote addresses in resource attributes",
            ));
        }
    }
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};
        if let Some(Component::Prefix(prefix)) = path.components().next() {
            if !matches!(prefix.kind(), Prefix::Disk(_) | Prefix::VerbatimDisk(_)) {
                return Err(error(
                    "expected a native local path; UNC and device paths are not indexing inputs",
                ));
            }
            if !path.is_absolute() {
                return Err(error(
                    "expected a native local path with an absolute drive root, not a drive-relative path",
                ));
            }
        } else if path.has_root() {
            return Err(error(
                "expected a native local path with an explicit drive root",
            ));
        }
    }
    Ok(())
}

pub(crate) fn resolve(root: &Path, value: &str) -> Result<PathBuf> {
    if value.trim().is_empty() {
        return Err(error("empty path"));
    }
    local_path(Path::new(value))?;
    safe_components(&root.join(value))?;
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

    #[test]
    fn remote_index_inputs_fail_with_local_path_guidance_before_io() {
        let f = Fixture::new();
        for value in [
            "https://remote.invalid/index.md",
            "file:///D:/notes.md",
            "ssh://remote.invalid/notes",
        ] {
            let failure = resolve(&f.0, value).unwrap_err().to_string();
            assert!(failure.contains("native local path"), "{failure}");
            let failure = crate::prepare(Path::new(value), &["init".into()])
                .err()
                .unwrap()
                .to_string();
            assert!(failure.contains("native local path"), "{failure}");
        }
        assert_eq!(fs::read_dir(&f.0).unwrap().count(), 0);
    }

    #[cfg(windows)]
    #[test]
    fn windows_network_device_and_ambiguous_paths_are_not_index_inputs() {
        for value in [
            r"\\server.invalid\share\index.md",
            r"\\?\UNC\server.invalid\share\index.md",
            r"\\.\C:\index.md",
            r"D:notes.md",
            r"\notes.md",
        ] {
            assert!(
                local_path(Path::new(value))
                    .unwrap_err()
                    .to_string()
                    .contains("native local path")
            );
        }
        for value in [
            r"D:\知识\notes.md",
            r"\\?\D:\知识\notes.md",
            "D:/notes.md",
            "../knowledge/notes.md",
        ] {
            local_path(Path::new(value)).unwrap();
        }
    }
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            Self::new_in(&std::env::temp_dir())
        }
        fn new_in(parent: &Path) -> Self {
            let p = parent.join(format!(
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

    #[cfg(windows)]
    #[test]
    #[ignore = "requires DEVMELD_TEST_OTHER_ROOT on a second local Windows drive"]
    fn cross_drive_relocation_recovers_at_each_mutation_boundary() {
        let second_root = PathBuf::from(
            std::env::var_os("DEVMELD_TEST_OTHER_ROOT")
                .expect("set the second local-drive test directory"),
        );
        // Six output/entry changes and one ownership record, including both
        // removals and creations on different volumes. Stop before commit.
        for stop in 0..=7 {
            let f = Fixture::new();
            let second = Fixture::new_in(&second_root);
            assert_ne!(f.0.components().next(), second.0.components().next());
            let prepare = |args: &[&str]| {
                crate::prepare(
                    &f.0,
                    &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                )
                .unwrap()
            };
            let other_entry = second.0.join("ENTRY.md");
            prepare(&[
                "init",
                "--entry",
                "ENTRY.md",
                "--entry",
                other_entry.to_str().unwrap(),
            ])
            .apply()
            .unwrap();
            fs::write(f.0.join("source.md"), "original knowledge").unwrap();
            prepare(&["resource", "add", "notes", "--document", "source.md"])
                .apply()
                .unwrap();
            prepare(&["sync"]).apply().unwrap();
            let new_output = second.0.join("output");
            prepare(&["output", new_output.to_str().unwrap()])
                .apply()
                .unwrap();
            prepare(&["language", "zh-CN"]).apply().unwrap();
            let paths = [
                f.0.join(".devmeld/output/index.md"),
                f.0.join(".devmeld/output/r-notes.md"),
                f.0.join("ENTRY.md"),
                other_entry,
                f.0.join(".devmeld/context.json"),
                f.0.join(".devmeld/state/owned.json"),
            ];
            let before = paths
                .iter()
                .map(|p| observe(p).unwrap())
                .collect::<Vec<_>>();
            let publication = prepare(&["sync"]);
            assert_eq!(publication.changes.len(), 6);
            assert!(
                publication
                    .apply_until(Some(stop))
                    .unwrap_err()
                    .to_string()
                    .contains("injected interruption")
            );
            assert!(crate::prepare(&f.0, &["sync".into()]).is_err());
            prepare(&["recover"]).apply().unwrap();
            assert_eq!(
                paths
                    .iter()
                    .map(|p| observe(p).unwrap())
                    .collect::<Vec<_>>(),
                before
            );
            assert!(!new_output.join("index.md").exists());
            assert!(!new_output.join("r-notes.md").exists());
            assert_eq!(
                fs::read_to_string(f.0.join("source.md")).unwrap(),
                "original knowledge"
            );
            prepare(&["sync"]).apply().unwrap();
            assert!(prepare(&["sync"]).is_empty());
        }
    }
    #[test]
    fn language_configuration_and_publication_use_existing_interruption_recovery() {
        for stop in 0..=4 {
            let f = Fixture::new();
            let prepare = |args: &[&str]| {
                crate::prepare(
                    &f.0,
                    &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                )
                .unwrap()
            };
            prepare(&["init", "--entry", "project/CONTEXT.md"])
                .apply()
                .unwrap();
            fs::write(f.0.join("doc.md"), "ssh / http").unwrap();
            prepare(&["resource", "add", "doc", "--document", "doc.md"])
                .apply()
                .unwrap();
            prepare(&["sync"]).apply().unwrap();
            let config_path = f.0.join(".devmeld/context.json");
            let english_config = fs::read(&config_path).unwrap();
            let files = [
                ".devmeld/output/index.md",
                ".devmeld/output/r-doc.md",
                "project/CONTEXT.md",
            ];
            let english: Vec<_> = files
                .iter()
                .map(|p| fs::read(f.0.join(p)).unwrap())
                .collect();
            let failure = prepare(&["language", "zh-CN"])
                .apply_until(Some(1))
                .unwrap_err();
            assert!(failure.to_string().contains("injected interruption"));
            assert!(crate::prepare(&f.0, &["sync".into()]).is_err());
            prepare(&["recover"]).apply().unwrap();
            assert_eq!(fs::read(&config_path).unwrap(), english_config);
            prepare(&["language", "zh-CN"]).apply().unwrap();
            let chinese_config = fs::read(&config_path).unwrap();
            let failure = prepare(&["sync"]).apply_until(Some(stop)).unwrap_err();
            assert!(failure.to_string().contains("injected interruption"));
            assert!(crate::prepare(&f.0, &["language".into(), "en".into()]).is_err());
            prepare(&["recover"]).apply().unwrap();
            for (file, before) in files.iter().zip(&english) {
                assert_eq!(fs::read(f.0.join(file)).unwrap(), *before);
            }
            assert_eq!(fs::read(&config_path).unwrap(), chinese_config);
            assert_eq!(
                fs::read_to_string(f.0.join("doc.md")).unwrap(),
                "ssh / http"
            );
            prepare(&["sync"]).apply().unwrap();
            assert!(
                fs::read_to_string(f.0.join(".devmeld/output/index.md"))
                    .unwrap()
                    .starts_with("# 上下文\n")
            );
            assert!(prepare(&["sync"]).is_empty());
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
    #[test]
    fn allowed_large_text_is_not_expanded_into_an_unreadable_receipt() {
        let f = Fixture::new();
        let bytes = vec![b'x'; 3 * 1024 * 1024];
        let mut plan = Plan::new(f.0.clone()).unwrap();
        plan.set(f.0.join("large.md"), Some(bytes.clone())).unwrap();
        plan.apply().unwrap();
        let mut repeat = Plan::new(f.0.clone()).unwrap();
        repeat.set(f.0.join("large.md"), Some(bytes)).unwrap();
        assert!(repeat.is_empty());
    }
    #[test]
    fn recovery_can_itself_be_interrupted_and_retried() {
        for stop in 0..=3 {
            let f = Fixture::new();
            let a = f.0.join("a.md");
            let b = f.0.join("b.md");
            let mut initial = Plan::new(f.0.clone()).unwrap();
            initial.set(a.clone(), Some(b"old".to_vec())).unwrap();
            initial.apply().unwrap();
            let mut change = Plan::new(f.0.clone()).unwrap();
            change.set(a.clone(), Some(b"new".to_vec())).unwrap();
            change.set(b.clone(), Some(b"created".to_vec())).unwrap();
            assert!(change.apply_until(Some(3)).is_err());
            let journal_path = f.0.join(".devmeld/state/pending.json");
            let journal: Journal =
                serde_json::from_slice(&fs::read(&journal_path).unwrap()).unwrap();
            assert!(recover_until(&journal, &journal_path, Some(stop)).is_err());
            Plan::recovery(f.0.clone()).unwrap().apply().unwrap();
            assert_eq!(fs::read(&a).unwrap(), b"old");
            assert!(!b.exists());
            Plan::new(f.0.clone())
                .unwrap()
                .set(a.clone(), Some(b"old".to_vec()))
                .unwrap();
        }
    }
    #[test]
    fn a_partial_commit_record_does_not_make_rollback_unrecoverable() {
        let f = Fixture::new();
        let target = f.0.join("a.md");
        let mut plan = Plan::new(f.0.clone()).unwrap();
        plan.set(target.clone(), Some(b"new".to_vec())).unwrap();
        assert!(plan.apply_until(Some(1)).is_err());
        let pending = f.0.join(".devmeld/state/pending.json");
        let journal: Journal = serde_json::from_slice(&fs::read(&pending).unwrap()).unwrap();
        fs::write(&journal.commit_file, b"{partial").unwrap();
        Plan::recovery(f.0.clone()).unwrap().apply().unwrap();
        assert!(!target.exists());
        assert!(Plan::new(f.0.clone()).is_ok());
    }
    #[test]
    fn committed_recovery_only_cleans_up_and_does_not_undo_later_edits() {
        let f = Fixture::new();
        let target = f.0.join("a.md");
        let mut plan = Plan::new(f.0.clone()).unwrap();
        plan.set(target.clone(), Some(b"committed".to_vec()))
            .unwrap();
        assert!(plan.apply_until(Some(3)).is_err());
        fs::write(&target, "later edit").unwrap();
        let recovery = Plan::recovery(f.0.clone()).unwrap();
        assert!(!recovery.preview().contains("Restore:"));
        recovery.apply().unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"later edit");
        assert!(Plan::new(f.0.clone()).is_ok());
    }

    #[test]
    fn current_initialization_recovers_without_a_complete_config_or_receipt() {
        for stop in 0..=3 {
            let f = Fixture::new();
            let plan = crate::prepare(
                &f.0,
                &[
                    "init".into(),
                    "--instruction-entry".into(),
                    "AGENTS.md".into(),
                ],
            )
            .unwrap();
            assert!(plan.apply_until(Some(stop)).is_err());
            crate::prepare(&f.0, &["recover".into()])
                .unwrap()
                .apply()
                .unwrap();
            if stop < 3 {
                assert!(!f.0.join(".devmeld/context.json").exists());
                assert!(!f.0.join(".devmeld/state/owned.json").exists());
                crate::prepare(&f.0, &["init".into()])
                    .unwrap()
                    .apply()
                    .unwrap();
            } else {
                crate::prepare(&f.0, &["sync".into()])
                    .unwrap()
                    .apply()
                    .unwrap();
                assert!(f.0.join("AGENTS.md").exists());
            }
        }
    }

    #[test]
    fn instruction_create_update_detach_recover_at_every_step_and_commit_boundary() {
        for operation in ["create", "attach", "update", "detach"] {
            // Discover the real number of operation steps; do not hard-code target counts.
            for stop in 0.. {
                let f = Fixture::new();
                let prepare = |args: &[&str]| {
                    crate::prepare(
                        &f.0,
                        &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                    )
                    .unwrap()
                };
                prepare(&["init", "--instruction-entry", "AGENTS.md"])
                    .apply()
                    .unwrap();
                if operation != "create" {
                    fs::write(f.0.join("AGENTS.md"), b"\xef\xbb\xbfAuthor rules\r\n").unwrap();
                }
                if matches!(operation, "update" | "detach") {
                    prepare(&["sync"]).apply().unwrap();
                    if operation == "update" {
                        prepare(&["language", "zh-CN"]).apply().unwrap();
                    } else {
                        prepare(&["entry", "remove", "AGENTS.md"]).apply().unwrap();
                    }
                }
                let config = observe(&f.0.join(".devmeld/context.json")).unwrap();
                let plan = prepare(&["sync"]);
                let boundaries = plan.changes.len() + 2; // ownership step + committed record
                if stop > boundaries {
                    break;
                }
                let before: Vec<_> = plan
                    .changes
                    .keys()
                    .chain(std::iter::once(&f.0.join(".devmeld/state/owned.json")))
                    .map(|p| (p.clone(), observe(p).unwrap()))
                    .collect();
                assert!(
                    plan.apply_until(Some(stop)).is_err(),
                    "{operation} at {stop}"
                );
                prepare(&["recover"]).apply().unwrap();
                assert_eq!(observe(&f.0.join(".devmeld/context.json")).unwrap(), config);
                if stop < boundaries {
                    for (path, expected) in before {
                        assert_eq!(
                            observe(&path).unwrap(),
                            expected,
                            "{operation} at {stop}: {}",
                            path.display()
                        );
                    }
                    prepare(&["sync"]).apply().unwrap();
                }
                if operation == "detach" {
                    assert_eq!(
                        fs::read(f.0.join("AGENTS.md")).unwrap(),
                        b"\xef\xbb\xbfAuthor rules\r\n"
                    );
                }
                assert!(prepare(&["sync"]).is_empty());
            }
        }
    }

    #[test]
    fn shared_host_external_edits_block_uncommitted_rollback_but_not_committed_cleanup() {
        for committed in [false, true] {
            let f = Fixture::new();
            crate::prepare(
                &f.0,
                &[
                    "init".into(),
                    "--instruction-entry".into(),
                    "AGENTS.md".into(),
                ],
            )
            .unwrap()
            .apply()
            .unwrap();
            let plan = crate::prepare(&f.0, &["sync".into()]).unwrap();
            let stop = plan.changes.len() + usize::from(committed) + 1;
            assert!(plan.apply_until(Some(stop)).is_err());
            let target = f.0.join("AGENTS.md");
            let mut bytes = fs::read(&target).unwrap();
            bytes.extend_from_slice(b"Author edits after interruption");
            fs::write(&target, &bytes).unwrap();
            let result = crate::prepare(&f.0, &["recover".into()]).unwrap().apply();
            assert_eq!(result.is_ok(), committed);
            assert_eq!(fs::read(target).unwrap(), bytes);
            assert_eq!(f.0.join(".devmeld/state/pending.json").exists(), !committed);
        }
    }

    #[test]
    fn pre_journal_staging_preserves_targets_and_reports_remnants_without_adoption() {
        let f = Fixture::new();
        fs::write(f.0.join("AGENTS.md"), "Author instructions").unwrap();
        crate::prepare(
            &f.0,
            &[
                "init".into(),
                "--instruction-entry".into(),
                "AGENTS.md".into(),
            ],
        )
        .unwrap()
        .apply()
        .unwrap();
        let host = observe(&f.0.join("AGENTS.md")).unwrap();
        let receipt = observe(&f.0.join(".devmeld/state/owned.json")).unwrap();
        let plan = crate::prepare(&f.0, &["sync".into()]).unwrap();
        let error = plan.apply_until(Some(usize::MAX)).unwrap_err().to_string();
        assert!(
            error.contains("targets unchanged")
                && error.contains("staging")
                && error.contains("no automatic orphan cleanup"),
            "{error}"
        );
        assert_eq!(observe(&f.0.join("AGENTS.md")).unwrap(), host);
        assert_eq!(
            observe(&f.0.join(".devmeld/state/owned.json")).unwrap(),
            receipt
        );
        assert!(!f.0.join(".devmeld/state/pending.json").exists());
        let remnants: Vec<_> = fs::read_dir(&f.0)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(".devmeld-")
            })
            .collect();
        assert!(!remnants.is_empty());
        crate::prepare(&f.0, &["recover".into()])
            .unwrap()
            .apply()
            .unwrap();
        assert!(remnants.iter().all(|p| p.exists()));
    }

    #[test]
    fn shared_recovery_can_resume_after_each_rollback_step_and_intermediate_swap() {
        for stop in 0..=2 {
            let f = Fixture::new();
            let host = f.0.join("AGENTS.md");
            fs::write(&host, "Authored").unwrap();
            crate::prepare(
                &f.0,
                &[
                    "init".into(),
                    "--instruction-entry".into(),
                    "AGENTS.md".into(),
                ],
            )
            .unwrap()
            .apply()
            .unwrap();
            crate::prepare(&f.0, &["sync".into()])
                .unwrap()
                .apply()
                .unwrap();
            crate::prepare(&f.0, &["entry".into(), "remove".into(), "AGENTS.md".into()])
                .unwrap()
                .apply()
                .unwrap();
            let before = observe(&host).unwrap();
            let plan = crate::prepare(&f.0, &["sync".into()]).unwrap();
            let steps = plan.changes.len() + 1;
            assert_eq!(steps, 2);
            assert!(plan.apply_until(Some(steps)).is_err());
            let pending = f.0.join(".devmeld/state/pending.json");
            let journal: Journal = serde_json::from_slice(&fs::read(&pending).unwrap()).unwrap();
            assert!(recover_until(&journal, &pending, Some(stop)).is_err());
            // A stopped restore can leave its verified before-image swap before rename.
            let step = journal.steps.iter().find(|s| s.path == host).unwrap();
            if !step.swap.exists() {
                fs::hard_link(&step.backup, &step.swap).unwrap();
            }
            crate::prepare(&f.0, &["recover".into()])
                .unwrap()
                .apply()
                .unwrap();
            assert_eq!(observe(&host).unwrap(), before);
        }

        let f = Fixture::new();
        fs::write(f.0.join("AGENTS.md"), "Authored").unwrap();
        crate::prepare(
            &f.0,
            &[
                "init".into(),
                "--instruction-entry".into(),
                "AGENTS.md".into(),
            ],
        )
        .unwrap()
        .apply()
        .unwrap();
        let plan = crate::prepare(&f.0, &["sync".into()]).unwrap();
        assert!(plan.apply_until(Some(0)).is_err());
        let pending = f.0.join(".devmeld/state/pending.json");
        let journal: Journal = serde_json::from_slice(&fs::read(&pending).unwrap()).unwrap();
        let step = journal
            .steps
            .iter()
            .find(|s| s.path == f.0.join("AGENTS.md"))
            .unwrap();
        // Replacement interrupted after installing its swap link but before target rename.
        fs::hard_link(&step.stage, &step.swap).unwrap();
        crate::prepare(&f.0, &["recover".into()])
            .unwrap()
            .apply()
            .unwrap();
        assert_eq!(fs::read(f.0.join("AGENTS.md")).unwrap(), b"Authored");
    }
}

fn safe_components(path: &Path) -> Result<()> {
    local_path(path)?;
    let mut prefix = PathBuf::new();
    for component in path.components() {
        prefix.push(component);
        if matches!(component, std::path::Component::Prefix(_)) {
            continue;
        }
        match fs::symlink_metadata(&prefix) {
            Ok(metadata) => {
                let redirected = metadata.file_type().is_symlink();
                #[cfg(windows)]
                let redirected = {
                    use std::os::windows::fs::MetadataExt;
                    redirected || metadata.file_attributes() & 0x400 != 0
                };
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
    #[serde(with = "text_bytes")]
    bytes: Vec<u8>,
    identity: String,
}

// Managed files are UTF-8 text, not JSON arrays of byte numbers.
mod text_bytes {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        let text = std::str::from_utf8(bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(text)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        Ok(String::deserialize(deserializer)?.into_bytes())
    }
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
    observe_bounded(path, PLAN_LIMIT as u64)
}
fn observe_bounded(path: &Path, limit: u64) -> Result<Option<Observed>> {
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
    File::open(path)?.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(error(format!(
            "file exceeds its read bound: {}",
            path.display()
        )));
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
    context_root: PathBuf,
    #[serde(deserialize_with = "unique_surfaces")]
    surfaces: BTreeMap<PathBuf, Claim>,
}

fn unique_surfaces<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<BTreeMap<PathBuf, Claim>, D::Error> {
    struct Targets;
    impl<'de> serde::de::Visitor<'de> for Targets {
        type Value = BTreeMap<PathBuf, Claim>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unique ownership targets")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(
            self,
            mut map: M,
        ) -> std::result::Result<Self::Value, M::Error> {
            let mut result = BTreeMap::new();
            while let Some((path, claim)) = map.next_entry::<PathBuf, Claim>()? {
                if result.insert(path.clone(), claim).is_some() {
                    return Err(serde::de::Error::custom(format!(
                        "duplicate ownership target: {}",
                        path.display()
                    )));
                }
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(Targets)
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum Claim {
    WholeFile {
        observed: Observed,
    },
    InstructionEntry {
        #[serde(with = "text_bytes")]
        insertion: Vec<u8>,
    },
}
impl Receipt {
    fn new(context_root: PathBuf) -> Self {
        Self {
            format_version: 0,
            context_root,
            surfaces: BTreeMap::new(),
        }
    }
}

enum OwnershipEffect {
    WholeFile,
    Insertion(Vec<u8>),
    Release,
}
struct Mutation {
    after: Option<Vec<u8>>,
    ownership: OwnershipEffect,
}

fn current_format(bytes: &[u8], path: &Path) -> Result<()> {
    let envelope: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| error(format!("{}: {e}", path.display())))?;
    if envelope
        .get("format_version")
        .and_then(serde_json::Value::as_u64)
        != Some(0)
    {
        return Err(error(format!(
            "unsupported maintenance format: {}; expected v0; records left intact, no automatic migration",
            path.display()
        )));
    }
    Ok(())
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
    context_root: PathBuf,
    committed: bool,
    steps: Vec<Step>,
    manifest_file: PathBuf,
    manifest_identity: String,
    commit_file: PathBuf,
    commit_identity: String,
}

/// An inspectable captured operation; apply rechecks its inputs before writing.
pub struct Plan {
    root: PathBuf,
    basis: BTreeMap<PathBuf, Option<Observed>>,
    changes: BTreeMap<PathBuf, Mutation>,
    receipt: Receipt,
    sources: BTreeSet<PathBuf>,
    recovery: Option<Journal>,
}
impl Plan {
    pub(crate) fn new(root: PathBuf) -> Result<Self> {
        let mut plan = Self::empty(root);
        if plan
            .capture(&plan.root.join(".devmeld/state/pending.json"))?
            .is_some()
        {
            return Err(error("pending recovery; run recover before new operations"));
        }
        let state = plan.root.join(".devmeld/state/owned.json");
        if let Some(bytes) = plan.capture(&state)? {
            current_format(&bytes, &state)?;
            plan.receipt = serde_json::from_slice(&bytes)?;
            if !same_path(&plan.receipt.context_root, &plan.root) {
                return Err(error(format!(
                    "ownership context does not match: {}",
                    state.display()
                )));
            }
            let mut targets: Vec<PathBuf> = Vec::new();
            for (path, claim) in &plan.receipt.surfaces {
                let value = path
                    .to_str()
                    .ok_or_else(|| error("non-Unicode ownership target"))?;
                let resolved = resolve(&plan.root, value)?;
                if !path.is_absolute()
                    || !same_path(path, &resolved)
                    || targets.iter().any(|p| overlaps(p, path))
                    || overlaps(path, &plan.root.join(".devmeld/state"))
                {
                    return Err(error(format!(
                        "invalid or aliased ownership target: {}",
                        path.display()
                    )));
                }
                if let Claim::InstructionEntry { insertion } = claim {
                    if same_path(path, &plan.root.join(".devmeld/context.json")) {
                        return Err(error("configuration cannot have insertion ownership"));
                    }
                    crate::instructions::validate_evidence(insertion)
                        .map_err(|e| error(format!("{e}: {}", path.display())))?;
                }
                targets.push(path.clone());
            }
        }
        Ok(plan)
    }
    fn empty(root: PathBuf) -> Self {
        Self {
            receipt: Receipt::new(root.clone()),
            root,
            basis: BTreeMap::new(),
            changes: BTreeMap::new(),
            sources: BTreeSet::new(),
            recovery: None,
        }
    }
    pub(crate) fn recovery(root: PathBuf) -> Result<Self> {
        let mut plan = Self::empty(root);
        let path = plan.root.join(".devmeld/state/pending.json");
        if let Some(bytes) = plan.capture(&path)? {
            current_format(&bytes, &path)?;
            let journal: Journal = serde_json::from_slice(&bytes)?;
            if !same_path(&journal.context_root, &plan.root) {
                return Err(error(format!(
                    "recovery context does not match: {}",
                    path.display()
                )));
            }
            plan.recovery = Some(journal);
        } else {
            return Self::new(plan.root);
        }
        Ok(plan)
    }
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
    pub(crate) fn owned_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.receipt.surfaces.keys()
    }
    pub(crate) fn require_owned_config(&self) -> Result<()> {
        let path = self.root.join(".devmeld/context.json");
        match (
            self.receipt.surfaces.get(&path),
            self.basis.get(&path).and_then(Option::as_ref),
        ) {
            (Some(Claim::WholeFile { observed }), Some(current)) if observed == current => Ok(()),
            _ => Err(error(format!(
                "missing or mismatched configuration ownership: {}; records left intact",
                path.display()
            ))),
        }
    }
    pub(crate) fn check_entry_kind(&self, path: &Path, kind: &str) -> Result<()> {
        match self.receipt.surfaces.get(path) {
            Some(Claim::WholeFile { .. }) if kind != "file" => Err(error(format!(
                "incompatible ownership mode: {}; detach before changing kind",
                path.display()
            ))),
            Some(Claim::InstructionEntry { .. }) if kind != "instructions" => Err(error(format!(
                "incompatible ownership mode: {}; detach before changing kind",
                path.display()
            ))),
            _ => Ok(()),
        }
    }
    pub(crate) fn validate_targets(
        &mut self,
        desired: impl Iterator<Item = PathBuf>,
    ) -> Result<()> {
        let mut paths: BTreeSet<_> = desired
            .chain(self.receipt.surfaces.keys().cloned())
            .collect();
        paths.insert(self.root.join(".devmeld/context.json"));
        paths.insert(self.root.join(".devmeld/state/owned.json"));
        // The cooperative lock is an identity-only exclusion, not a content input:
        // Windows forbids reopening its bytes while our own exclusive lock is held.
        let lock_path = self.root.join(".devmeld/state/lock");
        safe_components(&lock_path)?;
        let lock_id = if lock_path.try_exists()? {
            Some(identity(&lock_path)?)
        } else {
            None
        };
        for path in &paths {
            self.capture(path)?;
        }
        let paths: Vec<_> = paths.into_iter().collect();
        for (i, path) in paths.iter().enumerate() {
            let current = self.basis.get(path).and_then(Option::as_ref);
            if current.is_some_and(|c| Some(&c.identity) == lock_id.as_ref()) {
                return Err(error(format!(
                    "publication target aliases maintenance lock: {}",
                    path.display()
                )));
            }
            for other in paths[..i].iter().chain(self.sources.iter()) {
                let alias = current.is_some_and(|c| {
                    self.basis
                        .get(other)
                        .and_then(Option::as_ref)
                        .is_some_and(|o| c.identity == o.identity)
                });
                if overlaps(path, other) || alias {
                    return Err(error(format!(
                        "publication target overlaps another target or input source (path/physical alias): {} and {}",
                        path.display(),
                        other.display()
                    )));
                }
            }
        }
        Ok(())
    }
    pub(crate) fn source(&mut self, path: &Path) -> Result<Vec<u8>> {
        if overlaps(path, &self.root.join(".devmeld/state"))
            || same_path(path, &self.root.join(".devmeld/context.json"))
        {
            return Err(error("source overlaps reserved maintenance data"));
        }
        self.sources.insert(path.to_owned());
        let bytes = self
            .capture(path)?
            .ok_or_else(|| error(format!("missing source: {}", path.display())))?;
        for reserved in [
            self.root.join(".devmeld/context.json"),
            self.root.join(".devmeld/state/owned.json"),
        ] {
            if let Some(record) = observe(&reserved)? {
                if self
                    .basis
                    .get(path)
                    .and_then(Option::as_ref)
                    .is_some_and(|source| source.identity == record.identity)
                {
                    return Err(error("source aliases reserved maintenance data"));
                }
            }
        }
        Ok(bytes)
    }
    pub(crate) fn capture(&mut self, path: &Path) -> Result<Option<Vec<u8>>> {
        let internal_record = path == self.root.join(".devmeld/state/owned.json")
            || path == self.root.join(".devmeld/state/pending.json");
        let observed = observe_bounded(
            path,
            if internal_record {
                PLAN_LIMIT as u64
            } else {
                FILE_LIMIT
            },
        )?;
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
        let owned = match self.receipt.surfaces.get(&path) {
            Some(Claim::WholeFile { observed }) => Some(observed),
            Some(Claim::InstructionEntry { .. }) => {
                return Err(error(format!(
                    "incompatible ownership mode: {}",
                    path.display()
                )));
            }
            None => None,
        };
        devmeld_publication::classify(
            before.as_deref(),
            after.as_deref(),
            owned.is_some(),
            owned == current,
            overlaps,
        )
        .map_err(|e| error(format!("{e}: {}", path.display())))?;
        if after.as_ref().is_some_and(|b| b.len() as u64 > FILE_LIMIT) {
            return Err(error("generated file exceeds 8 MiB"));
        }
        if before != after {
            let ownership = if after.is_some() {
                OwnershipEffect::WholeFile
            } else {
                OwnershipEffect::Release
            };
            self.changes.insert(path, Mutation { after, ownership });
        }
        self.check_size()
    }
    fn check_size(&self) -> Result<()> {
        if self
            .basis
            .values()
            .flatten()
            .map(|o| o.bytes.len())
            .sum::<usize>()
            + self
                .changes
                .values()
                .filter_map(|m| m.after.as_ref())
                .map(Vec::len)
                .sum::<usize>()
            > PLAN_LIMIT
        {
            return Err(error("operation exceeds 128 MiB"));
        }
        Ok(())
    }
    pub(crate) fn set_entry(&mut self, path: PathBuf, body: Option<&str>) -> Result<()> {
        use devmeld_publication::EntryEvidence;
        let before = self.capture(&path)?;
        let current = self.basis.get(&path).and_then(Option::as_ref);
        let overlap = self.sources.iter().any(|source| {
            same_path(source, &path)
                || current.is_some_and(|c| {
                    self.basis
                        .get(source)
                        .and_then(Option::as_ref)
                        .is_some_and(|s| s.identity == c.identity)
                })
        });
        let recorded = match self.receipt.surfaces.get(&path) {
            Some(Claim::InstructionEntry { insertion }) => Some(insertion.as_slice()),
            Some(Claim::WholeFile { .. }) => {
                return Err(error(format!(
                    "incompatible ownership mode: {}",
                    path.display()
                )));
            }
            None => None,
        };
        let prepared = crate::instructions::prepare(before.as_deref(), recorded, body)
            .map_err(|e| error(format!("{e}: {}", path.display())))?;
        devmeld_publication::authorize_entry(
            before.is_some(),
            if recorded.is_some() {
                EntryEvidence::MatchingInsertion
            } else {
                EntryEvidence::UnclaimedClean
            },
            body.is_none(),
            overlap,
        )
        .map_err(|e| error(format!("{e}: {}", path.display())))?;
        if prepared.bytes.len() as u64 > FILE_LIMIT {
            return Err(error(format!(
                "instruction host exceeds 8 MiB: {}",
                path.display()
            )));
        }
        if before.as_deref() != Some(&prepared.bytes) {
            let ownership = match prepared.insertion {
                Some(bytes) => OwnershipEffect::Insertion(bytes),
                None => OwnershipEffect::Release,
            };
            self.changes.insert(
                path,
                Mutation {
                    after: Some(prepared.bytes),
                    ownership,
                },
            );
        }
        self.check_size()
    }
    pub(crate) fn withdraw(&mut self, path: PathBuf) -> Result<()> {
        if matches!(
            self.receipt.surfaces.get(&path),
            Some(Claim::InstructionEntry { .. })
        ) {
            self.set_entry(path, None)
        } else {
            self.set(path, None)
        }
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
                if journal.committed {
                    output.push_str(&format!(
                        "\nLeave committed target unchanged: {}\n",
                        step.path.display()
                    ));
                    continue;
                }
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
        if self
            .changes
            .contains_key(&self.root.join(".devmeld/context.json"))
        {
            output.push_str(
                "Registration/configuration only; run sync separately to publish entry changes.\n",
            );
        }
        if !self.changes.is_empty() {
            output.push_str(
                "Applying also records local ownership/recovery evidence under .devmeld/state.\n",
            );
        }
        for (path, mutation) in &self.changes {
            if matches!(mutation.ownership, OwnershipEffect::Insertion(_))
                || matches!(
                    self.receipt.surfaces.get(path),
                    Some(Claim::InstructionEntry { .. })
                )
            {
                output.push_str(&format!(
                    "\nInstruction insertion: {} ({})\n",
                    path.display(),
                    if mutation.after.is_some()
                        && matches!(mutation.ownership, OwnershipEffect::Release)
                    {
                        "detach; retain host"
                    } else if self.basis.get(path).is_some_and(Option::is_none) {
                        "attach; create host"
                    } else {
                        "attach/update; preserve outside text"
                    }
                ));
            }
            output.push_str(&format!(
                "\nTarget: {}\nBefore:\n{}\nAfter:\n{}\n",
                path.display(),
                self.basis
                    .get(path)
                    .and_then(Option::as_ref)
                    .map(|o| String::from_utf8_lossy(&o.bytes).into_owned())
                    .unwrap_or_else(|| "<absent>".into()),
                mutation
                    .after
                    .as_ref()
                    .map(|b| String::from_utf8_lossy(b).into_owned())
                    .unwrap_or_else(|| "<absent>".into())
            ));
        }
        output
    }
    fn recheck(&self) -> Result<()> {
        for (path, expected) in &self.basis {
            let limit = expected
                .as_ref()
                .map(|o| o.bytes.len() as u64)
                .unwrap_or(0)
                .max(FILE_LIMIT);
            if observe_bounded(path, limit)? != *expected {
                return Err(error(format!("stale preview: {}", path.display())));
            }
        }
        Ok(())
    }
    pub fn apply(self) -> Result<()> {
        let pending = self.root.join(".devmeld/state/pending.json");
        self.apply_until(None).map_err(|cause| {
            if pending.exists() {
                error(format!(
                    "{cause}; unfinished maintenance remains, inspect and run recover"
                ))
            } else {
                cause
            }
        })
    }

    // Fault injection remains private to storage tests; it is not a CLI executor.
    fn apply_until(mut self, stop_after: Option<usize>) -> Result<()> {
        self.recheck()?;
        if self.is_empty() {
            return Ok(());
        }
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
        let mut journal = (|| -> Result<Journal> {
        let nonce = format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        );
        let manifest_file = state_dir.join(format!(".devmeld-{nonce}.manifest"));
        let commit_file = state_dir.join(format!(".devmeld-{nonce}.commit"));
        write_new(&manifest_file, b"")?;
        write_new(&commit_file, b"")?;
        let mut journal = Journal {
            format_version: 0,
            context_root: self.root.clone(),
            committed: false,
            steps: Vec::new(),
            manifest_identity: identity(&manifest_file)?,
            commit_identity: identity(&commit_file)?,
            manifest_file,
            commit_file,
        };
        for (path, mutation) in &self.changes {
            let step = stage_step(
                path,
                self.basis.get(path).cloned().flatten(),
                mutation.after.clone(),
                &nonce,
                journal.steps.len(),
            )?;
            match &mutation.ownership {
                OwnershipEffect::WholeFile => {
                    let observed = step
                        .after
                        .clone()
                        .ok_or_else(|| error("whole-file claim requires a present target"))?;
                    self.receipt
                        .surfaces
                        .insert(path.clone(), Claim::WholeFile { observed });
                }
                OwnershipEffect::Release => {
                    self.receipt.surfaces.remove(path);
                }
                OwnershipEffect::Insertion(insertion) => {
                    self.receipt.surfaces.insert(
                        path.clone(),
                        Claim::InstructionEntry {
                            insertion: insertion.clone(),
                        },
                    );
                }
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
        let journal_bytes = super::declarations::encode(&journal)?;
        if journal_bytes.len() > PLAN_LIMIT {
            return Err(error(
                "recovery journal exceeds 128 MiB; no target changes applied",
            ));
        }
        write_owned_record(
            &journal.manifest_file,
            &journal.manifest_identity,
            &journal_bytes,
        )?;
        if stop_after == Some(usize::MAX) {
            return Err(error("injected pre-journal interruption"));
        }
        Ok(journal)
        })().map_err(|e| {
            let parents: BTreeSet<_> = self.changes.keys().filter_map(|p| p.parent()).chain(std::iter::once(state_dir.as_path())).collect();
            error(format!("{e}; before journal publication: targets unchanged; staging files/directories may remain under {parents:?}; no automatic orphan cleanup"))
        })?;
        // Expose only a complete journal; a partial preparation is never a pending operation.
        fs::hard_link(&journal.manifest_file, &journal_path)?;
        // The absence captured by the preview is now replaced by our own journal.
        self.basis
            .insert(journal_path.clone(), observe(&journal_path)?);
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
        write_owned_record(
            &journal.commit_file,
            &journal.commit_identity,
            &super::declarations::encode(&journal)?,
        )?;
        fs::rename(&journal.commit_file, &journal_path)?;
        if stop_after == Some(journal.steps.len() + 1) {
            return Err(error("injected committed-operation interruption"));
        }
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
fn write_owned_record(path: &Path, expected_identity: &str, bytes: &[u8]) -> Result<()> {
    safe_components(path)?;
    if identity(path)? != expected_identity {
        return Err(error("internal record identity changed"));
    }
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
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
    recover_until(journal, journal_path, None)
}
fn recover_until(journal: &Journal, journal_path: &Path, stop_after: Option<usize>) -> Result<()> {
    if !journal.committed {
        // Preflight the entire rollback before restoring any file.
        for step in &journal.steps {
            recovery_state(step)?;
        }
        if stop_after == Some(0) {
            return Err(error("injected recovery interruption"));
        }
        for (index, step) in journal.steps.iter().rev().enumerate() {
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
            if stop_after == Some(index + 1) {
                return Err(error("injected recovery interruption"));
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
    for (path, expected_identity) in [
        (&journal.manifest_file, &journal.manifest_identity),
        (&journal.commit_file, &journal.commit_identity),
    ] {
        safe_components(path)?;
        if path.try_exists()? {
            if identity(path)? != *expected_identity {
                return Err(error("internal record cleanup conflict"));
            }
            fs::remove_file(path)?;
        }
    }
    fs::remove_file(journal_path)?;
    Ok(())
}
