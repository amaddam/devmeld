use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "devmeld-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn run(&self, args: &[&str], apply: bool) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_devmeld"));
        command
            .arg("--context")
            .arg(&self.0)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if apply {
            command.arg("--apply");
        }
        let mut child = command.spawn().unwrap();
        if apply {
            child.stdin.take().unwrap().write_all(b"apply\n").unwrap();
        }
        child.wait_with_output().unwrap()
    }
    fn ok(&self, args: &[&str]) -> Output {
        let output = self.run(args, true);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }
    fn assert_local_links(&self, file: &str) {
        let path = self.0.join(file);
        let markdown = fs::read_to_string(&path).unwrap();
        for suffix in markdown.split("](").skip(1) {
            let destination = suffix.split(')').next().unwrap();
            let mut decoded = Vec::new();
            let mut bytes = destination.as_bytes().iter().copied();
            while let Some(byte) = bytes.next() {
                if byte == b'%' {
                    let hex = [bytes.next().unwrap(), bytes.next().unwrap()];
                    decoded
                        .push(u8::from_str_radix(std::str::from_utf8(&hex).unwrap(), 16).unwrap());
                } else {
                    decoded.push(byte);
                }
            }
            let destination = String::from_utf8(decoded).unwrap();
            let resolved = path
                .parent()
                .unwrap()
                .join(&destination)
                .canonicalize()
                .unwrap_or_else(|e| panic!("broken link {file} -> {destination}: {e}"));
            assert!(
                resolved.starts_with(self.0.canonicalize().unwrap()),
                "unexpected target: {}",
                resolved.display()
            );
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn initialization_is_previewed_confirmed_and_not_adopted_twice() {
    let f = Fixture::new();
    let preview = f.run(&["init"], false);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 0);
    f.ok(&["init"]);
    let config = f.0.join(".devmeld/context.json");
    let before = fs::read(&config).unwrap();
    assert!(!f.run(&["init"], true).status.success());
    assert_eq!(fs::read(config).unwrap(), before);
}

#[test]
fn documents_publish_to_offline_entry_and_unchanged_sync_is_noop() {
    let f = Fixture::new();
    fs::write(
        f.0.join("团队 notes.md"),
        "# Original knowledge\nNever rewrite me.\n",
    )
    .unwrap();
    f.ok(&["init", "--entry", "project/CONTEXT.md"]);
    f.ok(&["resource", "add", "notes", "--document", "团队 notes.md"]);
    assert!(!f.0.join(".devmeld/output/index.md").exists());
    f.ok(&["sync"]);
    let index = f.0.join(".devmeld/output/index.md");
    let first = fs::read(&index).unwrap();
    let time = fs::metadata(&index).unwrap().modified().unwrap();
    assert!(
        String::from_utf8(first.clone())
            .unwrap()
            .contains("r-notes.md")
    );
    assert!(
        fs::read_to_string(f.0.join("project/CONTEXT.md"))
            .unwrap()
            .contains("../.devmeld/output/index.md")
    );
    assert!(
        fs::read_to_string(f.0.join(".devmeld/output/r-notes.md"))
            .unwrap()
            .contains("%20notes.md")
    );
    let repeat = f.ok(&["sync"]);
    assert!(String::from_utf8_lossy(&repeat.stdout).contains("0 changed target(s)"));
    assert_eq!(fs::read(index).unwrap(), first);
    assert_eq!(
        fs::metadata(f.0.join(".devmeld/output/index.md"))
            .unwrap()
            .modified()
            .unwrap(),
        time
    );
    assert_eq!(
        fs::read_to_string(f.0.join("团队 notes.md")).unwrap(),
        "# Original knowledge\nNever rewrite me.\n"
    );
}

#[test]
fn publication_cannot_overwrite_a_registered_source_even_if_it_owned_the_file_before() {
    let f = Fixture::new();
    f.ok(&["init"]);
    f.ok(&["sync"]);
    f.ok(&[
        "resource",
        "add",
        "alias",
        "--document",
        ".devmeld/output/index.md",
    ]);
    let before = fs::read(f.0.join(".devmeld/output/index.md")).unwrap();
    let output = f.run(&["sync"], true);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("source"));
    assert_eq!(
        fs::read(f.0.join(".devmeld/output/index.md")).unwrap(),
        before
    );
}

#[test]
fn reserved_surfaces_and_duplicate_entries_are_rejected_before_initialization() {
    let f = Fixture::new();
    for args in [
        vec!["init", "--output", ".devmeld/state"],
        vec!["init", "--entry", ".devmeld/context.json"],
        vec!["init", "--entry", ".devmeld/output/custom.md"],
        vec!["init", "--entry", "entry.md", "--entry", "entry.md"],
    ] {
        assert!(!f.run(&args, false).status.success(), "accepted {args:?}");
        assert_eq!(fs::read_dir(&f.0).unwrap().count(), 0);
    }
}

#[test]
fn stale_preview_and_external_edits_preserve_external_bytes() {
    let f = Fixture::new();
    f.ok(&["init"]);
    fs::write(f.0.join("doc.md"), "first").unwrap();
    f.ok(&["resource", "add", "doc", "--document", "doc.md"]);
    let plan = devmeld::prepare(&f.0, &["sync".into()]).unwrap();
    fs::write(f.0.join("doc.md"), "second").unwrap();
    assert!(plan.apply().unwrap_err().to_string().contains("stale"));
    assert!(!f.0.join(".devmeld/output/index.md").exists());
    f.ok(&["sync"]);
    fs::write(f.0.join(".devmeld/output/index.md"), "external edit").unwrap();
    assert!(!f.run(&["sync"], true).status.success());
    assert_eq!(
        fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap(),
        "external edit"
    );
}

#[test]
fn registration_entry_and_output_changes_unpublish_only_owned_files() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "original").unwrap();
    f.ok(&["init", "--entry", "project/CONTEXT.md"]);
    f.ok(&["resource", "add", "notes", "--document", "notes.md"]);
    f.ok(&["sync"]);
    f.ok(&["entry", "remove", "project/CONTEXT.md"]);
    f.ok(&["entry", "add", "other/CONTEXT.md"]);
    f.ok(&["output", "published"]);
    f.ok(&["sync"]);
    assert!(!f.0.join("project/CONTEXT.md").exists());
    assert!(!f.0.join(".devmeld/output/index.md").exists());
    assert!(f.0.join("published/r-notes.md").exists());
    assert!(f.0.join("other/CONTEXT.md").exists());
    f.ok(&["resource", "remove", "notes"]);
    let config = fs::read(f.0.join(".devmeld/context.json")).unwrap();
    fs::write(f.0.join("published/index.md"), "external").unwrap();
    assert!(!f.run(&["sync"], true).status.success());
    assert_eq!(fs::read(f.0.join(".devmeld/context.json")).unwrap(), config);
    assert_eq!(
        fs::read_to_string(f.0.join("notes.md")).unwrap(),
        "original"
    );
}

#[test]
fn descriptions_validate_custom_attributes_offline_without_changing_old_output() {
    let f = Fixture::new();
    fs::write(f.0.join("service.json"), r#"{"title":"Team service","summary":"Shared test endpoint","attributes":{"address":"https://local.invalid","custom":"allowed"}}"#).unwrap();
    fs::write(f.0.join("schema.json"), r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","required":["address"],"properties":{"address":{"type":"string","minLength":1}},"additionalProperties":{"type":"string"}}"#).unwrap();
    f.ok(&["init"]);
    f.ok(&[
        "resource",
        "add",
        "service",
        "--description",
        "service.json",
        "--schema",
        "schema.json",
    ]);
    f.ok(&["sync"]);
    let page = f.0.join(".devmeld/output/r-service.md");
    let before = fs::read(&page).unwrap();
    let text = String::from_utf8(before.clone()).unwrap();
    assert!(
        text.contains("Team service")
            && text.contains("https://local.invalid")
            && text.contains("custom")
    );
    let updated = r#"{"title":"Team service","summary":"Shared test endpoint","attributes":{"address":"https://changed.invalid","custom":"allowed","team":"new field"}}"#;
    fs::write(f.0.join("service.json"), updated).unwrap();
    f.ok(&["sync"]);
    let before = fs::read(&page).unwrap();
    let text = String::from_utf8(before.clone()).unwrap();
    assert!(
        text.contains("https://changed.invalid")
            && text.contains("new field")
            && !text.contains("https://local.invalid")
    );
    assert_eq!(
        fs::read_to_string(f.0.join("service.json")).unwrap(),
        updated
    );
    fs::write(f.0.join("service.json"), r#"{"title":"Team service","summary":"Shared test endpoint","attributes":{"custom":"allowed"}}"#).unwrap();
    assert!(!f.run(&["sync"], true).status.success());
    assert_eq!(fs::read(&page).unwrap(), before);
}

#[test]
fn help_explains_command_options_and_missing_confirmation_is_read_only() {
    let help = Command::new(env!("CARGO_BIN_EXE_devmeld"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("--description PATH"));
    let f = Fixture::new();
    let cancelled = Command::new(env!("CARGO_BIN_EXE_devmeld"))
        .arg("--context")
        .arg(&f.0)
        .args(["init", "--apply"])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!cancelled.status.success());
    assert!(String::from_utf8_lossy(&cancelled.stderr).contains("cancelled"));
    assert_eq!(fs::read_dir(&f.0).unwrap().count(), 0);
}

#[test]
fn invalid_ids_envelopes_versions_and_oversize_sources_are_explicit_errors() {
    let f = Fixture::new();
    f.ok(&["init"]);
    fs::write(f.0.join("doc.md"), "original").unwrap();
    assert!(
        !f.run(&["resource", "add", "Bad ID", "--document", "doc.md"], true)
            .status
            .success()
    );
    f.ok(&["resource", "add", "doc", "--document", "doc.md"]);
    assert!(
        !f.run(&["resource", "add", "doc", "--document", "doc.md"], true)
            .status
            .success()
    );
    fs::write(
        f.0.join("bad.json"),
        r#"{"title":"Bad","summary":"Invalid control","unknownControl":true}"#,
    )
    .unwrap();
    assert!(
        !f.run(
            &["resource", "add", "bad", "--description", "bad.json"],
            true
        )
        .status
        .success()
    );
    fs::write(f.0.join("large.md"), vec![b'x'; 8 * 1024 * 1024 + 1]).unwrap();
    assert!(
        !f.run(
            &["resource", "add", "large", "--document", "large.md"],
            true
        )
        .status
        .success()
    );
    let config_path = f.0.join(".devmeld/context.json");
    let mut config: serde_json::Value =
        serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    config["format_version"] = 999.into();
    fs::write(config_path, serde_json::to_vec(&config).unwrap()).unwrap();
    let result = f.run(&["sync"], true);
    assert!(
        !result.status.success()
            && String::from_utf8_lossy(&result.stderr)
                .contains("unsupported configuration format_version")
    );
    assert!(!f.0.join(".devmeld/output").exists());
}

#[cfg(unix)]
#[test]
fn non_unicode_arguments_are_errors_not_panics() {
    use std::os::unix::ffi::OsStringExt;
    let result = Command::new(env!("CARGO_BIN_EXE_devmeld"))
        .arg(std::ffi::OsString::from_vec(vec![0xff]))
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !result.status.success()
            && stderr.contains("must be Unicode")
            && !stderr.contains("panicked")
    );
}

#[test]
fn access_guidance_links_existing_tools_without_granting_execution_authority() {
    let f = Fixture::new();
    fs::write(
        f.0.join("notes.md"),
        "# Team knowledge\nAuthored knowledge, not generated.",
    )
    .unwrap();
    fs::write(
        f.0.join("tool.py"),
        "raise RuntimeError('must never execute')",
    )
    .unwrap();
    fs::write(
        f.0.join("pyproject.toml"),
        "# Original dependency declarations",
    )
    .unwrap();
    fs::write(
        f.0.join("service.json"),
        r#"{"title":"Service","summary":"A team service"}"#,
    )
    .unwrap();
    fs::write(f.0.join("tool.json"), r#"{"title":"Existing tool","summary":"Use according to team instructions","references":[{"label":"Script","path":"tool.py"},{"label":"Dependencies","path":"pyproject.toml"}]}"#).unwrap();
    f.ok(&["init", "--entry", "project/CONTEXT.md"]);
    f.ok(&["resource", "add", "notes", "--document", "notes.md"]);
    f.ok(&[
        "resource",
        "add",
        "service",
        "--description",
        "service.json",
    ]);
    f.ok(&["resource", "add", "tool", "--description", "tool.json"]);
    f.ok(&["access", "add", "service", "tool"]);
    f.ok(&["sync"]);
    for file in [
        "project/CONTEXT.md",
        ".devmeld/output/index.md",
        ".devmeld/output/r-notes.md",
        ".devmeld/output/r-service.md",
        ".devmeld/output/r-tool.md",
    ] {
        f.assert_local_links(file);
    }
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-service.md")).unwrap();
    assert!(
        page.contains("r-tool.md")
            && page.contains("does not authorize")
            && page.contains("verified local/system")
    );
    let tool_page = fs::read_to_string(f.0.join(".devmeld/output/r-tool.md")).unwrap();
    assert!(tool_page.contains("tool.py") && tool_page.contains("pyproject.toml"));
    assert!(
        !f.run(&["resource", "remove", "tool"], true)
            .status
            .success()
    );
    assert!(
        !f.run(&["access", "add", "service", "missing"], true)
            .status
            .success()
    );
    f.ok(&["access", "remove", "service", "tool"]);
    f.ok(&["resource", "remove", "tool"]);
    f.ok(&["sync"]);
    assert!(!f.0.join(".devmeld/output/r-tool.md").exists());
    assert!(f.0.join("tool.py").exists());
    assert!(f.0.join("pyproject.toml").exists());
}

#[test]
fn schema_controls_reject_remote_and_unknown_semantics_but_not_literal_data() {
    let f = Fixture::new();
    fs::write(f.0.join("description.json"), r#"{"title":"Custom","summary":"No domain whitelist","attributes":{"$schema":"a literal value"}}"#).unwrap();
    f.ok(&["init"]);
    for invalid in [
        r#"{"$ref":"https://example.invalid/schema"}"#,
        r#"{"$ref":"file:///private/schema.json"}"#,
        r#"{"$schema":"http://json-schema.org/draft-07/schema#"}"#,
        r#"{"$defs":{"nested":{"$schema":"unknown"}}}"#,
        r#"{"$vocabulary":{"https://example.invalid/custom":true}}"#,
        r#"{"mysteryConstraint":true}"#,
        r#"{"required":"wrong type"}"#,
    ] {
        fs::write(f.0.join("schema.json"), invalid).unwrap();
        let result = f.run(
            &[
                "resource",
                "add",
                "custom",
                "--description",
                "description.json",
                "--schema",
                "schema.json",
            ],
            true,
        );
        assert!(
            !result.status.success(),
            "accepted unsupported schema: {invalid}"
        );
    }
    fs::write(
        f.0.join("schema.json"),
        r##"{"$defs":{"attrs":{"const":{"$schema":"a literal value"}}},"$ref":"#/$defs/attrs"}"##,
    )
    .unwrap();
    f.ok(&[
        "resource",
        "add",
        "custom",
        "--description",
        "description.json",
        "--schema",
        "schema.json",
    ]);
    f.ok(&["sync"]);
    assert!(
        fs::read_to_string(f.0.join(".devmeld/output/r-custom.md"))
            .unwrap()
            .contains("a literal value")
    );
}

#[test]
fn hardlinked_sources_and_missing_ownership_do_not_authorize_republication() {
    let f = Fixture::new();
    f.ok(&["init"]);
    f.ok(&["sync"]);
    let index = f.0.join(".devmeld/output/index.md");
    fs::hard_link(&index, f.0.join("alias.md")).unwrap();
    f.ok(&["resource", "add", "alias", "--document", "alias.md"]);
    assert!(!f.run(&["sync"], true).status.success());
    f.ok(&["resource", "remove", "alias"]);
    let before = fs::read(&index).unwrap();
    fs::remove_file(f.0.join(".devmeld/state/owned.json")).unwrap();
    assert!(!f.run(&["sync"], true).status.success());
    assert_eq!(fs::read(index).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn redirected_paths_including_parent_traversal_are_rejected() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    fs::create_dir(f.0.join("real")).unwrap();
    symlink(f.0.join("real"), f.0.join("redirect")).unwrap();
    for path in ["redirect/entry.md", "redirect/../entry.md"] {
        assert!(!f.run(&["init", "--entry", path], false).status.success());
    }
    assert!(!f.0.join(".devmeld").exists());
}
