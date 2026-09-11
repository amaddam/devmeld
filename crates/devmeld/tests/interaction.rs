use std::{
    fs,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "devmeld-interaction-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self(root)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_devmeld"))
            .current_dir(&self.0)
            .args(args)
            .stdin(Stdio::null())
            .output()
            .unwrap()
    }
    fn ok(&self, args: &[&str]) -> String {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }
    fn read(&self, path: &str) -> Vec<u8> {
        fs::read(self.0.join(path)).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn help_distinguishes_native_files_directories_and_logical_addresses() {
    let f = Fixture::new();
    let root = f.ok(&["--help"]);
    let cases: &[(&[&str], &str)] = &[
        (&["init"], "init [CONTEXT_DIR]"),
        (&["resource", "add"], "resource add <SOURCE_FILE> [OPTIONS]"),
        (&["resource", "update"], "resource update <RESOURCE_PATH>"),
        (&["resource", "list"], "resource list [GROUP_PATH]"),
        (&["resource", "show"], "resource show <RESOURCE_PATH>"),
        (
            &["resource", "move"],
            "resource move <FROM_RESOURCE_PATH> <TO_RESOURCE_PATH>",
        ),
        (&["resource", "remove"], "resource remove <RESOURCE_PATH>"),
        (&["group", "add"], "group add <GROUP_PATH>"),
        (&["group", "update"], "group update <GROUP_PATH>"),
        (&["group", "list"], "group list [GROUP_PATH]"),
        (&["group", "show"], "group show <GROUP_PATH>"),
        (
            &["group", "move"],
            "group move <FROM_GROUP_PATH> <TO_GROUP_PATH>",
        ),
        (&["group", "remove"], "group remove <GROUP_PATH>"),
        (
            &["access", "add"],
            "access add <RESOURCE_PATH> <TOOL_RESOURCE_PATH>",
        ),
        (
            &["access", "remove"],
            "access remove <RESOURCE_PATH> <TOOL_RESOURCE_PATH>",
        ),
        (&["entry", "attach"], "entry attach <ENTRY_FILE>"),
        (&["entry", "create"], "entry create <ENTRY_FILE>"),
        (&["entry", "remove"], "entry remove <ENTRY_FILE>"),
        (&["output"], "output <OUTPUT_DIR>"),
    ];
    let mut reports = vec![root.clone()];
    for (topic, usage) in cases {
        assert!(!root.contains(usage), "root expands a leaf: {usage}");
        let mut args = topic.to_vec();
        args.push("--help");
        let operation = f.ok(&args);
        assert!(operation.contains(usage), "operation missing {usage}");
        if topic.len() == 2 {
            let group = f.ok(&[topic[0], "--help"]);
            assert!(group.contains(topic[1]), "group missing operation");
            assert!(!group.contains(usage), "group expands a leaf: {usage}");
            reports.push(group);
        }
        reports.push(operation);
    }
    for report in reports {
        assert!(report.contains("[--context <CONTEXT_DIR>]"));
        assert!(report.contains("Global options:"));
        assert!(
            !report
                .split(|c: char| !c.is_ascii_alphabetic() && c != '_')
                .any(|word| word == "PATH")
        );
    }
    assert!(root.contains("<VALUE> is required; [VALUE] is optional"));
    let leaf = f.ok(&["group", "list", "--help"]);
    for unrelated in [
        "Notation:",
        "TOOL_RESOURCE_PATH",
        "SOURCE_FILE",
        "--schema",
        "--inherit",
        "--yes",
        "ANNOTATIONS",
        "Do not type",
    ] {
        assert!(!leaf.contains(unrelated), "unrelated {unrelated}: {leaf}");
    }
    assert!(leaf.contains("Arguments:"));
    assert!(leaf.contains("Examples:"));
    assert!(leaf.contains("descendants"));
    assert!(leaf.contains("excluding"));
    assert_eq!(leaf, f.ok(&["group", "list", "-h"]));
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 0);
}

#[test]
fn root_help_describes_saves_and_never_advertises_removed_confirmation() {
    let f = Fixture::new();
    let help = f.ok(&["--help"]);
    assert!(!help.contains("--apply"));
    assert!(!help.contains("type apply"));
    assert!(help.contains("save directly"));
    assert!(help.contains("sync"));
    assert!(help.contains("recover"));
    assert!(!help.contains("config set language"));
    assert!(!help.contains("SOURCE_FILE"));
    assert!(!f.0.join(".devmeld").exists());
}

#[test]
fn command_help_is_a_quick_reference_not_a_manual() {
    let f = Fixture::new();
    for topic in [
        vec!["resource", "add"],
        vec!["group", "update"],
        vec!["init"],
        vec!["config", "set"],
        vec!["status"],
        vec!["recover"],
    ] {
        let mut args = topic.clone();
        args.push("--help");
        let help = f.ok(&args);
        let purpose = help.split("\n\n").next().unwrap();
        assert_eq!(
            purpose.lines().count(),
            1,
            "{topic:?} starts with a tutorial: {help}"
        );
        assert!(!help.contains("\nAnnotations:"));
        assert!(!help.contains("\nInheritance:"));
        assert!(!help.contains("Tags union"));
        assert!(
            !help.contains("http://") && !help.contains("https://"),
            "no website exists yet"
        );
        if let Some(examples) = help.split("Examples:\n").nth(1) {
            assert_eq!(examples.split("\n\n").next().unwrap().lines().count(), 1);
        }
    }
    let add = f.ok(&["resource", "add", "--help"]);
    for required in [
        "--as <RESOURCE_PATH>",
        "--kind <document|description>",
        "--schema <SCHEMA_FILE>",
        "requires --kind description",
        "default document",
        "sync",
    ] {
        assert!(add.contains(required), "missing {required}: {add}");
    }
    let remove = f.ok(&["group", "remove", "--help"]);
    assert!(remove.contains("empty") && remove.contains("source"));
    let sync = f.ok(&["sync", "--help"]);
    assert!(sync.contains("--yes") && sync.contains("conflicts") && sync.contains("stale"));
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 0);
}

#[test]
fn status_and_yes_preserve_external_conflicts_and_recover_noop_does_not_prompt() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    f.ok(&["resource", "add", "notes.md"]);
    f.ok(&["sync", "--yes"]);
    let receipt = f.read(".devmeld/state/owned.toml");
    // A document body is read independently. Status compares generated references,
    // not a historical source snapshot; it must state this limitation explicitly.
    fs::write(f.0.join("notes.md"), "updated authored source").unwrap();
    let status = f.ok(&["status"]);
    assert!(status.contains("Publication: up to date"));
    assert!(status.contains("not a historical source-freshness receipt"));
    assert_eq!(f.read(".devmeld/state/owned.toml"), receipt);
    fs::write(f.0.join(".devmeld/output/index.md"), "external edit").unwrap();
    for args in [
        vec!["status"],
        vec!["sync", "--yes"],
        vec!["sync", "--dry-run"],
    ] {
        assert!(!f.run(&args).status.success(), "{args:?}");
        assert_eq!(f.read(".devmeld/output/index.md"), b"external edit");
        assert_eq!(f.read(".devmeld/state/owned.toml"), receipt);
        assert!(!f.0.join(".devmeld/state/pending.toml").exists());
    }
    let recovery = f.ok(&["recover"]);
    assert!(recovery.contains("0 changed target(s)"));
    assert!(!recovery.contains("[y/N]"));
    assert_eq!(f.read(".devmeld/output/index.md"), b"external edit");
    assert_eq!(f.read("notes.md"), b"updated authored source");
}

#[test]
fn optional_init_path_selects_only_that_location_and_rejects_double_selection() {
    let f = Fixture::new();
    f.ok(&["init", "separate", "--dry-run"]);
    assert!(!f.0.join("separate").exists());
    assert!(!f.0.join(".devmeld").exists());
    f.ok(&["init", "separate"]);
    assert!(f.0.join("separate/.devmeld/context.toml").is_file());
    assert!(!f.0.join(".devmeld").exists());
    assert!(!f.0.join("separate/AGENTS.md").exists());
    assert!(!f.run(&["init", "separate"]).status.success());
    assert!(
        !f.run(&["--context", "other", "init", "third"])
            .status
            .success()
    );
    assert!(!f.0.join("other").exists());
    assert!(!f.0.join("third").exists());
}

#[test]
fn entry_attach_and_create_are_explicit_registrations_with_separate_publication() {
    let f = Fixture::new();
    let authored = b"\xef\xbb\xbf# Rules\r\nUse ssh / HTTP.\nKeep this ending";
    fs::write(f.0.join("AGENTS.md"), authored).unwrap();
    f.ok(&["group", "add", "team"]);
    f.ok(&["entry", "attach", "AGENTS.md"]);
    f.ok(&["entry", "create", "CONTEXT.md"]);
    assert!(!f.run(&["entry", "add", "OLD.md"]).status.success());
    assert_eq!(f.read("AGENTS.md"), authored);
    assert!(!f.0.join("CONTEXT.md").exists());
    let pending = f.ok(&["status"]);
    assert!(pending.contains("Configured entries: 2"));
    assert_eq!(pending.matches("not published").count(), 2);
    f.ok(&["sync", "--yes"]);
    let published = f.ok(&["status"]);
    assert_eq!(
        published
            .matches("published; matches current generated content")
            .count(),
        2
    );
    assert!(published.contains("Client consumption: unverified"));
    let host = f.read("AGENTS.md");
    f.ok(&["entry", "remove", "AGENTS.md"]);
    assert_eq!(f.read("AGENTS.md"), host);
    assert!(f.ok(&["status"]).contains("Publication: pending"));
    f.ok(&["sync", "--yes"]);
    assert_eq!(f.read("AGENTS.md"), authored);
    assert!(f.0.join("CONTEXT.md").is_file());
}

#[test]
fn configuration_language_saves_without_publishing_and_old_language_command_is_removed() {
    let f = Fixture::new();
    f.ok(&["group", "add", "team"]);
    f.ok(&["sync", "--yes"]);
    let before = f.read(".devmeld/output/index.md");
    f.ok(&["config", "set", "language", "zh-CN"]);
    assert_eq!(f.read(".devmeld/output/index.md"), before);
    assert!(f.ok(&["status"]).contains("Publication: pending"));
    assert!(!f.run(&["language", "en"]).status.success());
    f.ok(&["sync", "--yes"]);
    assert!(
        String::from_utf8(f.read(".devmeld/output/index.md"))
            .unwrap()
            .contains("# 上下文")
    );
}

#[test]
fn status_compares_current_publication_without_writing_or_claiming_client_consumption() {
    let f = Fixture::new();
    assert!(!f.run(&["status"]).status.success());
    assert!(!f.0.join(".devmeld").exists());
    fs::write(f.0.join("notes.md"), "source").unwrap();
    f.ok(&["resource", "add", "notes.md", "--as", "knowledge/notes"]);
    let before = f.read(".devmeld/context.toml");
    let receipt = f.read(".devmeld/state/owned.toml");
    let pending = f.ok(&["status"]);
    assert!(pending.contains("Publication: pending"), "{pending}");
    assert!(pending.contains("Configured entries: 0"), "{pending}");
    assert!(
        pending.contains("Client consumption: unverified"),
        "{pending}"
    );
    assert_eq!(f.read(".devmeld/context.toml"), before);
    assert_eq!(f.read(".devmeld/state/owned.toml"), receipt);
    assert!(!f.0.join(".devmeld/output").exists());
    f.ok(&["sync", "--yes"]);
    let index = f.read(".devmeld/output/index.md");
    let current = f.ok(&["status"]);
    assert!(current.contains("Publication: up to date"), "{current}");
    assert!(current.contains("Client consumption: unverified"));
    f.ok(&["group", "update", "knowledge", "--environment", "test"]);
    assert!(f.ok(&["status"]).contains("Publication: pending"));
    assert_eq!(f.read(".devmeld/output/index.md"), index);
    fs::remove_file(f.0.join("notes.md")).unwrap();
    let blocked = f.run(&["status"]);
    assert!(!blocked.status.success());
    assert!(String::from_utf8_lossy(&blocked.stderr).contains("blocked/unverified"));
    assert_eq!(f.read(".devmeld/output/index.md"), index);
}

#[test]
fn publication_requires_explicit_noninteractive_confirmation_but_noop_does_not() {
    let f = Fixture::new();
    f.ok(&["group", "add", "knowledge"]);
    let refused = f.run(&["sync"]);
    assert!(
        !refused.status.success(),
        "sync without a terminal/--yes must not silently succeed"
    );
    assert!(String::from_utf8_lossy(&refused.stderr).contains("--yes"));
    assert!(!f.0.join(".devmeld/output").exists());
    f.ok(&["sync", "--dry-run"]);
    assert!(!f.0.join(".devmeld/output").exists());
    f.ok(&["sync", "--yes"]);
    let path = f.0.join(".devmeld/output/index.md");
    let modified = fs::metadata(&path).unwrap().modified().unwrap();
    let noop = f.ok(&["sync"]);
    assert!(noop.contains("0 changed target(s)"));
    assert!(!noop.contains("[y/N]"));
    assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), modified);
    for command in [
        vec!["sync", "--yes", "--dry-run"],
        vec!["sync", "--yes", "--yes"],
        vec!["sync", "--apply"],
        vec!["group", "add", "bad", "--yes"],
    ] {
        assert!(!f.run(&command).status.success(), "{command:?}");
    }
}

#[test]
fn scoped_save_needs_no_confirmation_and_dry_run_never_creates_storage() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    f.ok(&[
        "resource",
        "add",
        "notes.md",
        "--as",
        "knowledge/notes",
        "--dry-run",
    ]);
    assert!(!f.0.join(".devmeld").exists());
    let output = f.ok(&["resource", "add", "notes.md", "--as", "knowledge/notes"]);
    assert!(f.0.join(".devmeld/context.toml").exists(), "{output}");
    assert!(output.contains("Configuration saved"), "{output}");
    assert!(output.contains("publication"), "{output}");
    assert!(!output.contains("confirm"), "{output}");
    assert!(!f.0.join(".devmeld/output").exists());
    assert!(!f.0.join("AGENTS.md").exists());
    assert_eq!(f.read("notes.md"), b"source");
}
