use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        Self::new_in(&std::env::temp_dir())
    }
    fn new_in(parent: &std::path::Path) -> Self {
        let root = parent.join(format!(
            "devmeld-instructions-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self(root.canonicalize().unwrap())
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
        let _ = child.stdin.take().unwrap().write_all(b"apply\n");
        child.wait_with_output().unwrap()
    }
    fn ok(&self, args: &[&str]) -> String {
        let output = self.run(args, true);
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
    fn prepare(&self, args: &[&str]) -> devmeld::Result<devmeld::Plan> {
        devmeld::prepare(
            &self.0,
            &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        )
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

const BEGIN: &str = "<!-- devmeld:entry:v0:begin -->";
const END: &str = "<!-- devmeld:entry:v0:end -->";

#[test]
fn explicit_registration_and_confirmed_sync_attach_only_a_small_entry() {
    let f = Fixture::new();
    let authored = b"# Team rules\nKeep source ownership. SSH / HTTP";
    fs::write(f.0.join("AGENTS.md"), authored).unwrap();
    let preview = f.run(&["init", "--instruction-entry", "AGENTS.md"], false);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert!(!f.0.join(".devmeld").exists());
    f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
    assert_eq!(f.read("AGENTS.md"), authored);
    fs::write(f.0.join("facts.md"), "Unique fixture fact.").unwrap();
    f.ok(&["resource", "add", "facts", "--document", "facts.md"]);
    let preview = f.run(&["sync"], false);
    assert!(preview.status.success());
    assert_eq!(f.read("AGENTS.md"), authored);
    f.ok(&["sync"]);
    let host = String::from_utf8(f.read("AGENTS.md")).unwrap();
    assert!(host.starts_with(BEGIN));
    assert!(host.ends_with(std::str::from_utf8(authored).unwrap()));
    assert!(host.contains("[context navigation](.devmeld/output/index.md)"));
    assert!(host.contains("[Managed registration](.devmeld/context.json)"));
    assert!(!host.contains("Unique fixture fact"));
    assert!(!host.contains("r-facts.md"));
    let receipt: serde_json::Value =
        serde_json::from_slice(&f.read(".devmeld/state/owned.json")).unwrap();
    let claim = &receipt["surfaces"][f.0.join("AGENTS.md").to_str().unwrap()];
    assert_eq!(claim["kind"], "instruction_entry");
    assert!(claim.get("observed").is_none());
    assert!(
        claim["insertion"]
            .as_str()
            .unwrap()
            .ends_with(&format!("{END}\n\n"))
    );
}

#[test]
fn attachment_preserves_bom_mixed_endings_and_unterminated_author_text() {
    for authored in [
        b"".as_slice(),
        b"no newline",
        b"a\r\nb\nc",
        b"a\nb\r\nc",
        b"\xef\xbb\xbf# Heading\r\nlast",
    ] {
        let f = Fixture::new();
        fs::write(f.0.join("AGENTS.md"), authored).unwrap();
        f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
        f.ok(&["sync"]);
        let after = f.read("AGENTS.md");
        let bom = if authored.starts_with(b"\xef\xbb\xbf") {
            3
        } else {
            0
        };
        assert_eq!(&after[..bom], &authored[..bom]);
        let suffix = &authored[bom..];
        assert!(after.ends_with(suffix));
        let insertion = &after[bom..after.len() - suffix.len()];
        let crlf = authored.windows(2).any(|w| w == b"\r\n")
            && authored
                .iter()
                .position(|b| *b == b'\n')
                .is_some_and(|i| i > 0 && authored[i - 1] == b'\r');
        let newline = if crlf { "\r\n" } else { "\n" };
        assert!(insertion.starts_with(format!("{BEGIN}{newline}").as_bytes()));
        assert!(insertion.ends_with(format!("{END}{newline}{newline}").as_bytes()));
        f.ok(&["entry", "remove", "AGENTS.md"]);
        f.ok(&["sync"]);
        assert_eq!(f.read("AGENTS.md"), authored);
    }
}

#[test]
fn invalid_encoding_and_raw_reserved_markers_are_never_adopted() {
    for authored in [
        b"\xff".as_slice(),
        b"\xff\xfea\0",
        b"a\0b",
        b"```\n<!-- devmeld:entry:v99:begin -->\n```",
        b"<!-- devmeld:entry:",
    ] {
        let f = Fixture::new();
        fs::write(f.0.join("AGENTS.md"), authored).unwrap();
        f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
        let receipt = f.read(".devmeld/state/owned.json");
        let result = f.run(&["sync"], true);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("AGENTS.md"));
        assert_eq!(f.read("AGENTS.md"), authored);
        assert_eq!(f.read(".devmeld/state/owned.json"), receipt);
        assert!(!f.0.join(".devmeld/output").exists());
    }
}

#[test]
fn outside_atomic_saves_are_preserved_and_unchanged_sync_does_not_touch_host() {
    let f = Fixture::new();
    f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
    f.ok(&["sync"]);
    let insertion = f.read("AGENTS.md");
    let mut edited = b"# New author prefix\r\n".to_vec();
    edited.extend_from_slice(&insertion);
    edited.extend_from_slice(b"Author suffix without newline");
    fs::write(f.0.join("save.tmp"), &edited).unwrap();
    fs::remove_file(f.0.join("AGENTS.md")).unwrap();
    fs::rename(f.0.join("save.tmp"), f.0.join("AGENTS.md")).unwrap();
    let mtime = fs::metadata(f.0.join("AGENTS.md"))
        .unwrap()
        .modified()
        .unwrap();
    let plan = f.prepare(&["sync"]).unwrap();
    assert!(plan.is_empty());
    plan.apply().unwrap();
    assert_eq!(f.read("AGENTS.md"), edited);
    assert_eq!(
        fs::metadata(f.0.join("AGENTS.md"))
            .unwrap()
            .modified()
            .unwrap(),
        mtime
    );
    f.ok(&["language", "zh-CN"]);
    f.ok(&["sync"]);
    let updated = String::from_utf8(f.read("AGENTS.md")).unwrap();
    assert!(updated.starts_with("# New author prefix\r\n"));
    assert!(updated.ends_with("Author suffix without newline"));
    assert!(updated.contains("## 项目上下文\n"));
    assert_eq!(updated.matches(BEGIN).count(), 1);
}

#[test]
fn multiple_hosts_detach_independently_and_registration_roundtrips_need_no_publication() {
    let f = Fixture::new();
    f.ok(&[
        "init",
        "--entry",
        "CONTEXT.md",
        "--instruction-entry",
        "AGENTS.md",
    ]);
    f.ok(&["entry", "add", "CLAUDE.md", "--kind", "instructions"]);
    f.ok(&["entry", "remove", "CLAUDE.md"]);
    f.ok(&["sync"]);
    assert!(!f.0.join("CLAUDE.md").exists());
    let agents = f.read("AGENTS.md");
    f.ok(&["entry", "remove", "AGENTS.md"]);
    f.ok(&["entry", "add", "AGENTS.md", "--kind", "instructions"]);
    assert!(f.prepare(&["sync"]).unwrap().is_empty());
    f.ok(&["entry", "add", "CLAUDE.md", "--kind", "instructions"]);
    f.ok(&["sync"]);
    assert_eq!(f.read("AGENTS.md"), agents);
    let claude = f.read("CLAUDE.md");
    f.ok(&["entry", "remove", "AGENTS.md"]);
    assert_eq!(f.read("AGENTS.md"), agents);
    f.ok(&["sync"]);
    assert_eq!(f.read("AGENTS.md"), b"");
    assert_eq!(f.read("CLAUDE.md"), claude);
    assert!(f.0.join("CONTEXT.md").is_file());
    assert!(f.0.join(".devmeld/output/index.md").is_file());
    let receipt: serde_json::Value =
        serde_json::from_slice(&f.read(".devmeld/state/owned.json")).unwrap();
    assert!(
        receipt["surfaces"]
            .get(f.0.join("AGENTS.md").to_str().unwrap())
            .is_none()
    );
    f.ok(&["entry", "add", "AGENTS.md", "--kind", "instructions"]);
    f.ok(&["sync"]);
    assert_eq!(f.read("AGENTS.md"), agents);
}

#[test]
fn relocated_navigation_and_language_share_sources_without_translating_authored_text() {
    let f = Fixture::new();
    fs::create_dir(f.0.join("project")).unwrap();
    fs::write(f.0.join("project/AGENTS.md"), "SSH / HTTP author rules").unwrap();
    fs::write(f.0.join("知识 #100%.md"), "SSH / HTTP facts").unwrap();
    f.ok(&[
        "init",
        "--instruction-entry",
        "project/AGENTS.md",
        "--entry",
        "CONTEXT.md",
    ]);
    f.ok(&["resource", "add", "ssh-http", "--document", "知识 #100%.md"]);
    f.ok(&["sync"]);
    f.ok(&["output", "navigation #new"]);
    f.ok(&["language", "zh-CN"]);
    f.ok(&["sync"]);
    let host = String::from_utf8(f.read("project/AGENTS.md")).unwrap();
    assert!(host.contains("[上下文索引](../navigation%20%23new/index.md)"));
    assert!(host.ends_with("SSH / HTTP author rules"));
    let page = String::from_utf8(f.read("navigation #new/r-ssh-http.md")).unwrap();
    assert!(page.contains("../%E7%9F%A5%E8%AF%86%20%23100%25.md"));
    assert_eq!(f.read("知识 #100%.md"), b"SSH / HTTP facts");
    assert!(!f.0.join(".devmeld/output/index.md").exists());
    assert!(
        String::from_utf8(f.read("CONTEXT.md"))
            .unwrap()
            .contains("navigation%20%23new/index.md")
    );
    f.ok(&["language", "en"]);
    f.ok(&["sync"]);
    assert!(
        String::from_utf8(f.read("project/AGENTS.md"))
            .unwrap()
            .contains("## Project context")
    );
}

#[test]
fn even_noop_apply_rechecks_all_captured_inputs() {
    for changed in [
        "AGENTS.md",
        "facts.md",
        ".devmeld/context.json",
        ".devmeld/state/owned.json",
    ] {
        let f = Fixture::new();
        fs::write(f.0.join("facts.md"), "facts").unwrap();
        f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
        f.ok(&["resource", "add", "facts", "--document", "facts.md"]);
        f.ok(&["sync"]);
        let plan = f.prepare(&["sync"]).unwrap();
        assert!(plan.is_empty());
        let target = f.0.join(changed);
        let mut bytes = fs::read(&target).unwrap();
        bytes.extend_from_slice(b"\nExternal edit");
        fs::write(&target, &bytes).unwrap();
        let error = plan.apply().expect_err(changed).to_string();
        assert!(error.contains("stale preview"), "{error}");
        assert_eq!(fs::read(target).unwrap(), bytes);
        assert!(!f.0.join(".devmeld/state/pending.json").exists());
    }
}

#[test]
fn missing_receipt_never_turns_a_managed_config_into_fresh_authority() {
    let f = Fixture::new();
    f.ok(&["init"]);
    fs::remove_file(f.0.join(".devmeld/state/owned.json")).unwrap();
    for args in [&["sync"][..], &["recover"], &["language", "zh-CN"]] {
        let result = f.run(args, true);
        assert!(
            !result.status.success(),
            "accepted missing receipt: {args:?}"
        );
        assert!(!f.0.join(".devmeld/output").exists());
        assert!(!f.0.join(".devmeld/state/owned.json").exists());
    }
}

#[test]
fn shared_host_cannot_alias_another_surface_or_maintenance_record() {
    for target in [
        "other.md",
        ".devmeld/context.json",
        ".devmeld/state/owned.json",
        ".devmeld/state/lock",
        ".devmeld/output/index.md",
    ] {
        let f = Fixture::new();
        f.ok(&[
            "init",
            "--instruction-entry",
            "AGENTS.md",
            "--instruction-entry",
            "other.md",
        ]);
        if target == "other.md" {
            fs::write(f.0.join(target), "author").unwrap();
        }
        if target == ".devmeld/output/index.md" {
            fs::create_dir_all(f.0.join(".devmeld/output")).unwrap();
            fs::write(f.0.join(target), "unowned output").unwrap();
        }
        fs::hard_link(f.0.join(target), f.0.join("AGENTS.md")).unwrap();
        let before = f.read(target);
        let result = f.run(&["sync"], true);
        assert!(!result.status.success(), "accepted alias of {target}");
        assert_eq!(f.read(target), before);
        assert_eq!(f.read("AGENTS.md"), before);
    }
}

#[test]
fn receipts_reject_duplicate_target_keys_and_invalid_insertion_evidence() {
    for duplicate in [true, false] {
        let f = Fixture::new();
        f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
        f.ok(&["sync"]);
        let path = f.0.join(".devmeld/state/owned.json");
        let mut receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let host = f.0.join("AGENTS.md");
        if duplicate {
            let surfaces = receipt["surfaces"].as_object().unwrap();
            let key = serde_json::to_string(host.to_str().unwrap()).unwrap();
            let claim = serde_json::to_string(&surfaces[host.to_str().unwrap()]).unwrap();
            let bytes = String::from_utf8(serde_json::to_vec(&receipt).unwrap()).unwrap();
            fs::write(
                &path,
                bytes.replacen(
                    "\"surfaces\":{",
                    &format!("\"surfaces\":{{{key}:{claim},"),
                    1,
                ),
            )
            .unwrap();
        } else {
            receipt["surfaces"][host.to_str().unwrap()]["insertion"] = "not an insertion".into();
            fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
        }
        let record = fs::read(&path).unwrap();
        for args in [&["recover"][..], &["language", "zh-CN"], &["sync"]] {
            let result = f.run(args, true);
            assert!(
                !result.status.success(),
                "accepted invalid receipt: duplicate={duplicate}, {args:?}"
            );
            assert_eq!(fs::read(&path).unwrap(), record);
        }
    }
}

#[test]
fn owned_entry_mode_cannot_be_changed_by_remove_then_add_before_detachment() {
    for (initial, next) in [("file", "instructions"), ("instructions", "file")] {
        let f = Fixture::new();
        f.ok(&["init"]);
        f.ok(&["entry", "add", "ENTRY.md", "--kind", initial]);
        f.ok(&["sync"]);
        let host = f.read("ENTRY.md");
        f.ok(&["entry", "remove", "ENTRY.md"]);
        let config = f.read(".devmeld/context.json");
        let result = f.run(&["entry", "add", "ENTRY.md", "--kind", next], true);
        assert!(
            !result.status.success(),
            "accepted mode conversion {initial} to {next}"
        );
        assert_eq!(f.read("ENTRY.md"), host);
        assert_eq!(f.read(".devmeld/context.json"), config);
    }
}

#[test]
fn changed_missing_and_copied_insertions_do_not_authorize_repair_or_adoption() {
    for change in [
        "edit",
        "duplicate",
        "nested",
        "truncated",
        "unknown",
        "missing",
        "copied",
        "receipt",
    ] {
        let f = Fixture::new();
        f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
        f.ok(&["sync"]);
        let original = String::from_utf8(f.read("AGENTS.md")).unwrap();
        match change {
            "edit" => fs::write(
                f.0.join("AGENTS.md"),
                original.replace("Project context", "Edited"),
            )
            .unwrap(),
            "duplicate" => fs::write(f.0.join("AGENTS.md"), original.repeat(2)).unwrap(),
            "nested" => fs::write(
                f.0.join("AGENTS.md"),
                original.replace("## Project context", BEGIN),
            )
            .unwrap(),
            "truncated" => {
                fs::write(f.0.join("AGENTS.md"), &original[..original.len() - 1]).unwrap()
            }
            "unknown" => fs::write(
                f.0.join("AGENTS.md"),
                original.replace("entry:v0", "entry:v9"),
            )
            .unwrap(),
            "missing" => fs::remove_file(f.0.join("AGENTS.md")).unwrap(),
            "receipt" | "copied" => {
                let other = Fixture::new();
                other.ok(&["init", "--instruction-entry", "AGENTS.md"]);
                if change == "receipt" {
                    fs::write(
                        other.0.join(".devmeld/state/owned.json"),
                        f.read(".devmeld/state/owned.json"),
                    )
                    .unwrap();
                }
                fs::write(other.0.join("AGENTS.md"), &original).unwrap();
                assert!(!other.run(&["sync"], true).status.success());
                assert_eq!(other.read("AGENTS.md"), original.as_bytes());
                continue;
            }
            _ => unreachable!(),
        }
        let before = fs::read(f.0.join("AGENTS.md")).ok();
        assert!(!f.run(&["sync"], true).status.success(), "{change}");
        assert_eq!(fs::read(f.0.join("AGENTS.md")).ok(), before);
        f.ok(&["entry", "remove", "AGENTS.md"]);
        assert!(!f.run(&["sync"], true).status.success(), "detach: {change}");
        assert_eq!(fs::read(f.0.join("AGENTS.md")).ok(), before);
    }
}

#[test]
fn sources_and_obsolete_targets_remain_in_the_alias_check() {
    let f = Fixture::new();
    fs::write(f.0.join("facts.md"), "authored").unwrap();
    fs::hard_link(f.0.join("facts.md"), f.0.join("AGENTS.md")).unwrap();
    f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
    f.ok(&["resource", "add", "facts", "--document", "facts.md"]);
    assert!(!f.run(&["sync"], true).status.success());
    assert_eq!(f.read("facts.md"), b"authored");

    let f = Fixture::new();
    f.ok(&["init", "--instruction-entry", "old.md"]);
    f.ok(&["sync"]);
    f.ok(&["entry", "remove", "old.md"]);
    f.ok(&["entry", "add", "new.md", "--kind", "instructions"]);
    fs::hard_link(f.0.join("old.md"), f.0.join("new.md")).unwrap();
    let before = f.read("old.md");
    assert!(!f.run(&["sync"], true).status.success());
    assert_eq!(f.read("old.md"), before);
    assert_eq!(f.read("new.md"), before);
}

#[cfg(windows)]
#[test]
#[ignore = "requires DEVMELD_TEST_OTHER_ROOT on a second local Windows drive"]
fn cross_drive_instruction_entries_follow_real_sources_after_relocation_and_detach() {
    fn targets(path: &std::path::Path) -> Vec<PathBuf> {
        fs::read_to_string(path)
            .unwrap()
            .split("](")
            .skip(1)
            .map(|part| {
                let href = part.split(')').next().unwrap();
                let mut bytes = href.strip_prefix("file:///").unwrap_or(href).bytes();
                let mut decoded = Vec::new();
                while let Some(byte) = bytes.next() {
                    if byte == b'%' {
                        let hex = [bytes.next().unwrap(), bytes.next().unwrap()];
                        decoded.push(
                            u8::from_str_radix(std::str::from_utf8(&hex).unwrap(), 16).unwrap(),
                        );
                    } else {
                        decoded.push(byte);
                    }
                }
                path.parent()
                    .unwrap()
                    .join(String::from_utf8(decoded).unwrap())
                    .canonicalize()
                    .unwrap()
            })
            .collect()
    }
    let f = Fixture::new();
    let second = Fixture::new_in(&PathBuf::from(
        std::env::var_os("DEVMELD_TEST_OTHER_ROOT").expect("second local drive required"),
    ));
    assert_ne!(f.0.components().next(), second.0.components().next());
    let first_host = f.0.join("AGENTS.md");
    let second_host = second.0.join("项目 #100% AGENTS.md");
    let source = second.0.join("知识 SSH #100%.md");
    fs::write(&source, "HTTP / SSH original answer").unwrap();
    fs::write(&second_host, "Author instructions\r\n").unwrap();
    f.ok(&[
        "init",
        "--instruction-entry",
        "AGENTS.md",
        "--instruction-entry",
        second_host.to_str().unwrap(),
    ]);
    f.ok(&[
        "resource",
        "add",
        "notes",
        "--document",
        source.to_str().unwrap(),
    ]);
    for (output, language) in [
        (f.0.join(".devmeld/output"), "en"),
        (second.0.join("发布 #100%"), "zh-CN"),
    ] {
        f.ok(&["output", output.to_str().unwrap()]);
        f.ok(&["language", language]);
        f.ok(&["sync"]);
        for host in [&first_host, &second_host] {
            assert_eq!(
                targets(host),
                vec![
                    output.join("index.md").canonicalize().unwrap(),
                    f.0.join(".devmeld/context.json").canonicalize().unwrap()
                ]
            );
            let index = &targets(host)[0];
            let page = &targets(index)[0];
            assert_eq!(targets(page)[0], source.canonicalize().unwrap());
            assert_eq!(
                fs::read(&targets(page)[0]).unwrap(),
                b"HTTP / SSH original answer"
            );
        }
        assert!(
            fs::read_to_string(&second_host)
                .unwrap()
                .ends_with("Author instructions\r\n")
        );
        assert!(
            fs::read_to_string(if language == "en" {
                &second_host
            } else {
                &first_host
            })
            .unwrap()
            .contains("file:///")
        );
        assert!(f.prepare(&["sync"]).unwrap().is_empty());
    }
    f.ok(&["entry", "remove", second_host.to_str().unwrap()]);
    f.ok(&["sync"]);
    assert_eq!(fs::read(&second_host).unwrap(), b"Author instructions\r\n");
    assert!(targets(&first_host)[0].is_file());
}

#[test]
fn a_pending_operation_appearing_after_noop_preview_invalidates_it() {
    let f = Fixture::new();
    f.ok(&["init", "--instruction-entry", "AGENTS.md"]);
    f.ok(&["sync"]);
    let plan = f.prepare(&["sync"]).unwrap();
    assert!(plan.is_empty());
    fs::write(
        f.0.join(".devmeld/state/pending.json"),
        "external pending record",
    )
    .unwrap();
    assert!(plan.apply().is_err());
    assert_eq!(
        f.read(".devmeld/state/pending.json"),
        b"external pending record"
    );
}
