use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        Self::new_in(&std::env::temp_dir())
    }
    fn new_in(parent: &std::path::Path) -> Self {
        let path = parent.join(format!(
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
            .current_dir(&self.0)
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
            if let Err(error) = child.stdin.take().unwrap().write_all(b"apply\n") {
                // A no-op or rejected command can exit without requesting confirmation.
                // Still collect and assert its actual exit status/output below.
                assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
            }
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
fn new_contexts_use_one_current_ownership_format() {
    let f = Fixture::new();
    f.ok(&["init", "--entry", "CONTEXT.md"]);
    f.ok(&["sync"]);
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/context.json")).unwrap()).unwrap();
    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/state/owned.json")).unwrap()).unwrap();
    assert_eq!(config["format_version"], 0);
    assert_eq!(receipt["format_version"], 0);
    assert_eq!(
        receipt["context_root"],
        f.0.canonicalize().unwrap().to_str().unwrap()
    );
    let surfaces = receipt["surfaces"].as_object().unwrap();
    assert_eq!(surfaces.len(), 3);
    assert!(surfaces.values().all(|claim| claim["kind"] == "whole_file"));
    assert!(f.ok(&["sync"]).status.success());
}

#[test]
fn unsupported_records_are_rejected_even_by_recover_without_creating_state() {
    fn tree(root: &std::path::Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
        let mut files = std::collections::BTreeMap::new();
        for entry in fs::read_dir(root).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                files.extend(tree(&path));
            } else {
                files.insert(path.clone(), fs::read(path).unwrap());
            }
        }
        files
    }
    for record in ["context.json", "state/owned.json", "state/pending.json"] {
        for version in [1, 999] {
            let f = Fixture::new();
            let path = f.0.join(".devmeld").join(record);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, format!("{{\"format_version\":{version}}}")).unwrap();
            let before = tree(&f.0);
            for args in [
                vec!["init"],
                vec!["sync"],
                vec!["recover"],
                vec!["language", "en"],
            ] {
                let result = f.run(&args, true);
                assert!(
                    !result.status.success(),
                    "accepted {record} v{version}: {args:?}"
                );
                assert_eq!(tree(&f.0), before);
            }
        }
    }
}

// Opt-in: ordinary test runs need not have a second disk. The explicit run must
// supply an existing local directory on a different drive, never silently skip.
#[cfg(windows)]
#[test]
#[ignore = "requires DEVMELD_TEST_OTHER_ROOT on a second local Windows drive"]
fn cross_drive_workflow_preserves_sources_and_follows_offline_links() {
    fn targets(path: &std::path::Path) -> Vec<PathBuf> {
        fs::read_to_string(path)
            .unwrap()
            .split("](")
            .skip(1)
            .map(|suffix| {
                let href = suffix.split(')').next().unwrap();
                let local = href.strip_prefix("file:///").unwrap_or(href);
                let mut decoded = Vec::new();
                let mut bytes = local.as_bytes().iter().copied();
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
    fn verify_targets(path: &std::path::Path, expected: &[&std::path::Path]) {
        assert_eq!(
            targets(path),
            expected
                .iter()
                .map(|p| p.canonicalize().unwrap())
                .collect::<Vec<_>>()
        );
    }
    let f = Fixture::new();
    let second = Fixture::new_in(&PathBuf::from(
        std::env::var_os("DEVMELD_TEST_OTHER_ROOT")
            .expect("set the second local-drive test directory"),
    ));
    assert_ne!(
        f.0.canonicalize().unwrap().components().next(),
        second.0.canonicalize().unwrap().components().next(),
        "test requires two distinct drive roots"
    );
    let source = second.0.join("知识 #100% (ssh).md");
    fs::write(&source, "# ssh / http\nOriginal; not a publication copy.\n").unwrap();
    let description = f.0.join("service.json");
    let authored = serde_json::json!({"title":"API", "summary":"http access", "attributes":{"endpoint":"https://remote.invalid/index.md"}, "references":[{"label":"团队说明", "path":source}]});
    fs::write(&description, serde_json::to_vec_pretty(&authored).unwrap()).unwrap();
    let config = f.0.join(".devmeld/context.json");
    let entry = second.0.join("project/入口.md");
    f.ok(&[
        "init",
        "--language",
        "zh-CN",
        "--entry",
        entry.to_str().unwrap(),
    ]);
    f.ok(&["resource", "add", source.to_str().unwrap(), "--as", "notes"]);
    f.ok(&[
        "resource",
        "add",
        "service.json",
        "--as",
        "service",
        "--kind",
        "description",
    ]);
    let original = fs::read(&source).unwrap();
    let original_description = fs::read(&description).unwrap();
    let preview = f.run(&["sync"], false);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert!(!entry.exists());
    assert!(String::from_utf8_lossy(&preview.stdout).contains("file:///"));
    f.ok(&["sync"]);
    let output = f.0.join(".devmeld/output");
    let index = output.join("index.md");
    verify_targets(&entry, &[&index]);
    verify_targets(
        &index,
        &[
            &output.join("r-resource-1.md"),
            &output.join("r-resource-2.md"),
            &config,
        ],
    );
    verify_targets(&output.join("r-resource-1.md"), &[&source, &config]);
    verify_targets(
        &output.join("r-resource-2.md"),
        &[&description, &config, &source],
    );
    let page = fs::read_to_string(output.join("r-resource-1.md")).unwrap();
    assert!(page.contains("%E7%9F%A5%E8%AF%86%20%23100%25%20%28ssh%29.md"));
    assert!(page.contains("本地路径:"));
    assert!(!page.contains(r"\\?\"));
    let before = fs::read(&entry).unwrap();
    let modified = fs::metadata(&entry).unwrap().modified().unwrap();
    assert!(String::from_utf8_lossy(&f.ok(&["sync"]).stdout).contains("0 changed target(s)"));
    assert_eq!(fs::read(&entry).unwrap(), before);
    assert_eq!(fs::metadata(&entry).unwrap().modified().unwrap(), modified);

    // Relocate output to the other drive; configuration and original description
    // now need file URIs, while the old remote-drive source becomes relative.
    let moved = second.0.join("published #100%");
    let first_entry = f.0.join("ENTRY.md");
    f.ok(&["entry", "add", "ENTRY.md"]);
    f.ok(&["output", moved.to_str().unwrap()]);
    f.ok(&["language", "en"]);
    f.ok(&["sync"]);
    assert!(!index.exists());
    verify_targets(&entry, &[&moved.join("index.md")]);
    verify_targets(&first_entry, &[&moved.join("index.md")]);
    verify_targets(
        &moved.join("index.md"),
        &[
            &moved.join("r-resource-1.md"),
            &moved.join("r-resource-2.md"),
            &config,
        ],
    );
    verify_targets(&moved.join("r-resource-1.md"), &[&source, &config]);
    verify_targets(
        &moved.join("r-resource-2.md"),
        &[&description, &config, &source],
    );
    assert!(
        fs::read_to_string(&first_entry)
            .unwrap()
            .contains("Local path:")
    );
    assert!(String::from_utf8_lossy(&f.ok(&["sync"]).stdout).contains("0 changed target(s)"));
    fs::write(&entry, "external edit").unwrap();
    assert!(!f.run(&["sync"], true).status.success());
    assert_eq!(fs::read_to_string(&entry).unwrap(), "external edit");
    assert_eq!(fs::read(&source).unwrap(), original);
    assert_eq!(fs::read(&description).unwrap(), original_description);
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
    f.ok(&["resource", "add", "团队 notes.md", "--as", "notes"]);
    assert!(!f.0.join(".devmeld/output/index.md").exists());
    f.ok(&["sync"]);
    let index = f.0.join(".devmeld/output/index.md");
    let first = fs::read(&index).unwrap();
    let time = fs::metadata(&index).unwrap().modified().unwrap();
    assert!(
        String::from_utf8(first.clone())
            .unwrap()
            .contains("r-resource-1.md")
    );
    assert!(
        fs::read_to_string(f.0.join("project/CONTEXT.md"))
            .unwrap()
            .contains("../.devmeld/output/index.md")
    );
    assert!(
        fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md"))
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
fn chinese_publication_localizes_generated_text_without_changing_document_or_links() {
    let f = Fixture::new();
    let source = "# ssh / http\nUse curl and JSON; 原始内容。\n";
    fs::write(f.0.join("ssh http.md"), source).unwrap();
    let init = [
        "init",
        "--language",
        "zh-CN",
        "--entry",
        "project/CONTEXT.md",
    ];
    let preview = f.run(&init, false);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert!(!f.0.join(".devmeld").exists());
    f.ok(&init);
    f.ok(&["resource", "add", "ssh http.md", "--as", "ssh-http"]);
    let preview = f.run(&["sync"], false);
    assert!(preview.status.success());
    assert!(String::from_utf8_lossy(&preview.stdout).contains("# 上下文"));
    assert!(!f.0.join(".devmeld/output").exists());
    f.ok(&["sync"]);
    let index = fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap();
    assert!(index.starts_with("# 上下文\n"));
    assert!(index.contains("最近一次成功同步"));
    assert!(index.contains("[ssh-http](r-resource-1.md) — 原始文档"));
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("[原始来源](../../ssh%20http.md)"));
    assert!(page.contains("[受管注册配置](../context.json)"));
    let entry = fs::read_to_string(f.0.join("project/CONTEXT.md")).unwrap();
    assert!(entry.starts_with("# 项目上下文\n"));
    assert!(entry.contains("[上下文索引](../.devmeld/output/index.md)"));
    for file in [
        ".devmeld/output/index.md",
        ".devmeld/output/r-resource-1.md",
        "project/CONTEXT.md",
    ] {
        f.assert_local_links(file);
        let path = f.0.join(file);
        let before = fs::read(&path).unwrap();
        let modified = fs::metadata(&path).unwrap().modified().unwrap();
        let repeat = f.ok(&["sync"]);
        assert!(String::from_utf8_lossy(&repeat.stdout).contains("0 changed target(s)"));
        assert_eq!(fs::read(&path).unwrap(), before);
        assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), modified);
    }
    assert_eq!(fs::read_to_string(f.0.join("ssh http.md")).unwrap(), source);
}

#[test]
fn language_changes_require_confirmation_and_sync_and_preserve_authored_terms() {
    let f = Fixture::new();
    let service = r#"{"title":"ssh / http API","summary":"Original document","attributes":{"protocol":"ssh","transport":"http","command":"curl --head http://local.invalid","多语言":"i18n","format":"JSON"},"references":[{"label":"ssh / http CLI","path":"ssh-http.md"}]}"#;
    let tool = r#"{"title":"curl","summary":"curl for http; ssh 使用独立授权。"}"#;
    fs::write(f.0.join("service.json"), service).unwrap();
    fs::write(f.0.join("tool.json"), tool).unwrap();
    fs::write(f.0.join("ssh-http.md"), "ssh / http instructions").unwrap();
    f.ok(&[
        "init",
        "--entry",
        "project/CONTEXT.md",
        "--entry",
        "other/ENTRY.md",
    ]);
    f.ok(&[
        "resource",
        "add",
        "service.json",
        "--as",
        "service",
        "--kind",
        "description",
    ]);
    f.ok(&[
        "resource",
        "add",
        "tool.json",
        "--as",
        "curl",
        "--kind",
        "description",
    ]);
    f.ok(&["access", "add", "service", "curl"]);
    f.ok(&["sync"]);
    let config_path = f.0.join(".devmeld/context.json");
    let english_config = fs::read(&config_path).unwrap();
    let old_config: serde_json::Value = serde_json::from_slice(&english_config).unwrap();
    assert!(old_config["publication"].get("language").is_none());
    let files = [
        ".devmeld/output/index.md",
        ".devmeld/output/r-resource-1.md",
        ".devmeld/output/r-resource-2.md",
        "project/CONTEXT.md",
        "other/ENTRY.md",
    ];
    let english: Vec<_> = files
        .iter()
        .map(|path| fs::read(f.0.join(path)).unwrap())
        .collect();
    let preview = f.run(&["language", "zh-CN"], false);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert!(String::from_utf8_lossy(&preview.stdout).contains("zh-CN"));
    assert_eq!(fs::read(&config_path).unwrap(), english_config);
    let cancelled = Command::new(env!("CARGO_BIN_EXE_devmeld"))
        .arg("--context")
        .arg(&f.0)
        .args(["language", "zh-CN", "--apply"])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!cancelled.status.success());
    assert!(String::from_utf8_lossy(&cancelled.stderr).contains("cancelled"));
    assert_eq!(fs::read(&config_path).unwrap(), english_config);
    f.ok(&["language", "zh-CN"]);
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    assert_eq!(config["publication"]["language"], "zh-CN");
    for (file, before) in files.iter().zip(&english) {
        assert_eq!(
            fs::read(f.0.join(file)).unwrap(),
            *before,
            "language must not implicitly sync"
        );
    }
    f.ok(&["sync"]);
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    for unchanged in [
        "# ssh / http API",
        "Original document",
        "protocol: ssh",
        "transport: http",
        "curl --head http://local.invalid",
        "多语言: i18n",
        "format: JSON",
        "[ssh / http CLI](../../ssh-http.md)",
    ] {
        assert!(
            page.contains(unchanged),
            "authored text changed: {unchanged}"
        );
    }
    assert!(page.contains("## 接入指引"));
    assert!(page.contains("[关联资源: curl](r-resource-2.md)"));
    assert!(page.contains("不代表排他性选择或已验证的可用性"));
    assert!(page.contains("不得静默替换"));
    assert!(page.contains("优先项目管理的方案"));
    assert!(page.contains("已验证可调用、兼容且获授权的本地或系统方案"));
    assert!(page.contains("选择工具不等于授权安装"));
    assert!(page.contains("执行超出当前任务授权的操作"));
    assert!(
        fs::read_to_string(f.0.join(".devmeld/output/r-resource-2.md"))
            .unwrap()
            .contains("curl for http; ssh 使用独立授权。")
    );
    for file in files {
        f.assert_local_links(file);
    }
    for entry in ["project/CONTEXT.md", "other/ENTRY.md"] {
        assert!(
            fs::read_to_string(f.0.join(entry))
                .unwrap()
                .starts_with("# 项目上下文\n")
        );
    }
    assert!(
        String::from_utf8_lossy(&f.ok(&["language", "zh-CN"]).stdout)
            .contains("0 changed target(s)")
    );
    f.ok(&["language", "en"]);
    f.ok(&["sync"]);
    assert_eq!(fs::read(&config_path).unwrap(), english_config);
    for (file, before) in files.iter().zip(&english) {
        assert_eq!(fs::read(f.0.join(file)).unwrap(), *before);
    }
    assert!(String::from_utf8_lossy(&f.ok(&["sync"]).stdout).contains("0 changed target(s)"));
    assert_eq!(
        fs::read_to_string(f.0.join("service.json")).unwrap(),
        service
    );
    assert_eq!(fs::read_to_string(f.0.join("tool.json")).unwrap(), tool);
    assert_eq!(
        fs::read_to_string(f.0.join("ssh-http.md")).unwrap(),
        "ssh / http instructions"
    );
}

#[test]
fn invalid_output_languages_are_rejected_without_mutating_context() {
    let empty = Fixture::new();
    for code in ["zh", "zh-cn", "en-US", "fr", "", "EN"] {
        let output = empty.run(&["init", "--language", code], false);
        assert!(!output.status.success(), "accepted {code:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains("expected en or zh-CN"));
        assert_eq!(fs::read_dir(&empty.0).unwrap().count(), 0);
    }
    for args in [
        vec!["init", "--language"],
        vec!["init", "--language", "en", "--language", "zh-CN"],
    ] {
        assert!(!empty.run(&args, false).status.success());
        assert_eq!(fs::read_dir(&empty.0).unwrap().count(), 0);
    }
    let f = Fixture::new();
    f.ok(&["init"]);
    f.ok(&["sync"]);
    let config_path = f.0.join(".devmeld/context.json");
    let before = fs::read(&config_path).unwrap();
    let index = fs::read(f.0.join(".devmeld/output/index.md")).unwrap();
    let receipt = fs::read(f.0.join(".devmeld/state/owned.json")).unwrap();
    for args in [
        vec!["language", "fr"],
        vec!["language"],
        vec!["language", "en", "zh-CN"],
        vec!["sync", "--language", "zh-CN"],
    ] {
        assert!(!f.run(&args, false).status.success());
        assert_eq!(fs::read(&config_path).unwrap(), before);
        assert_eq!(
            fs::read(f.0.join(".devmeld/output/index.md")).unwrap(),
            index
        );
    }
    for invalid in [
        "null",
        "42",
        "true",
        "[]",
        r#""zh""#,
        r#"{"en":null}"#,
        r#"{"zh-CN":null}"#,
    ] {
        let malformed = format!(
            r#"{{"format_version":0,"resources":[],"access":[],"publication":{{"directory":".devmeld/output","entries":[],"language":{invalid}}}}}"#
        );
        fs::write(&config_path, &malformed).unwrap();
        let output = f.run(&["sync"], false);
        assert!(
            !output.status.success(),
            "accepted configuration language {invalid}"
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("context.json"));
        assert_eq!(fs::read_to_string(&config_path).unwrap(), malformed);
        assert_eq!(
            fs::read(f.0.join(".devmeld/output/index.md")).unwrap(),
            index
        );
        assert_eq!(
            fs::read(f.0.join(".devmeld/state/owned.json")).unwrap(),
            receipt
        );
    }
    fs::write(&config_path, r#"{"format_version":0,"resources":[],"access":[],"publication":{"directory":".devmeld/output","entries":[],"language":"en","language":"zh-CN"}}"#).unwrap();
    let duplicate = f.run(&["sync"], false);
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr).contains("duplicate field"));
}

#[test]
fn language_defaults_ignore_host_locale_and_keep_deterministic_english_output_bytes() {
    for options in [
        vec!["init", "--entry", "project/CONTEXT.md"],
        vec!["init", "--language", "en", "--entry", "project/CONTEXT.md"],
    ] {
        let f = Fixture::new();
        let init = Command::new(env!("CARGO_BIN_EXE_devmeld"))
            .arg("--context")
            .arg(&f.0)
            .args(&options)
            .env("LANG", "zh_CN.UTF-8")
            .env("LC_ALL", "zh_CN.UTF-8")
            .env("LANGUAGE", "zh_CN")
            .output()
            .unwrap();
        assert!(init.status.success());
        assert!(!String::from_utf8_lossy(&init.stdout).contains("zh-CN"));
        f.ok(&options);
        fs::write(f.0.join("doc.md"), "ssh and http").unwrap();
        f.ok(&["resource", "add", "doc.md", "--as", "doc"]);
        let sync = Command::new(env!("CARGO_BIN_EXE_devmeld"))
            .arg("--context")
            .arg(&f.0)
            .arg("sync")
            .env("LANG", "zh_CN.UTF-8")
            .env("LC_ALL", "zh_CN.UTF-8")
            .env("LANGUAGE", "zh_CN")
            .output()
            .unwrap();
        assert!(sync.status.success());
        assert!(String::from_utf8_lossy(&sync.stdout).contains("# Context\n"));
        f.ok(&["sync"]);
        assert_eq!(
            fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap(),
            "# Context\n\nGenerated by DevMeld. Read these files without running DevMeld.\nOnly reflects the last successful synchronization. Do not edit generated files.\n\n- [doc](r-resource-1.md) — Original document\n  Saved inheritance choices: inherit: false\n  \n\n[Managed registration](../context.json)\n"
        );
        assert_eq!(
            fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap(),
            "# doc\n\nOriginal document\n\nGenerated by DevMeld; update the original source or use DevMeld to change registration.\n\n[Original source](../../doc.md)\n\n[Managed registration](../context.json)\n\n\nSaved inheritance choices: inherit: false\n\n"
        );
        assert_eq!(
            fs::read_to_string(f.0.join("project/CONTEXT.md")).unwrap(),
            "# Project context\n\nGenerated by DevMeld. Follow [context navigation](../.devmeld/output/index.md) and choose relevant resources.\nNo running DevMeld process is needed. Do not edit this generated entry.\n"
        );
        let config = fs::read(f.0.join(".devmeld/context.json")).unwrap();
        assert!(!String::from_utf8_lossy(&config).contains("language"));
        assert!(
            String::from_utf8_lossy(&f.ok(&["language", "en"]).stdout)
                .contains("0 changed target(s)")
        );
        assert_eq!(fs::read(f.0.join(".devmeld/context.json")).unwrap(), config);
        f.ok(&["language", "zh-CN"]);
        let sync = Command::new(env!("CARGO_BIN_EXE_devmeld"))
            .arg("--context")
            .arg(&f.0)
            .arg("sync")
            .env("LANG", "en_US.UTF-8")
            .env("LC_ALL", "en_US.UTF-8")
            .output()
            .unwrap();
        assert!(sync.status.success());
        assert!(String::from_utf8_lossy(&sync.stdout).contains("# 上下文\n"));
    }
}

#[test]
fn language_changes_reject_stale_preview_and_preserve_external_publication_edits() {
    let f = Fixture::new();
    f.ok(&["init", "--entry", "project/CONTEXT.md"]);
    f.ok(&["sync"]);
    let stale = devmeld::prepare(&f.0, &["language".into(), "zh-CN".into()]).unwrap();
    f.ok(&["entry", "add", "other/ENTRY.md"]);
    let config = fs::read(f.0.join(".devmeld/context.json")).unwrap();
    assert!(stale.apply().unwrap_err().to_string().contains("stale"));
    assert_eq!(fs::read(f.0.join(".devmeld/context.json")).unwrap(), config);
    let stale_sync = devmeld::prepare(&f.0, &["sync".into()]).unwrap();
    f.ok(&["language", "zh-CN"]);
    assert!(
        stale_sync
            .apply()
            .unwrap_err()
            .to_string()
            .contains("stale")
    );
    let chinese_config = fs::read(f.0.join(".devmeld/context.json")).unwrap();
    let old_entry = fs::read(f.0.join("project/CONTEXT.md")).unwrap();
    fs::write(f.0.join(".devmeld/output/index.md"), "external edit").unwrap();
    assert!(!f.run(&["sync"], false).status.success());
    assert_eq!(
        fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap(),
        "external edit"
    );
    assert_eq!(fs::read(f.0.join("project/CONTEXT.md")).unwrap(), old_entry);
    assert!(!f.0.join("other/ENTRY.md").exists());
    assert_eq!(
        fs::read(f.0.join(".devmeld/context.json")).unwrap(),
        chinese_config
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
        ".devmeld/output/index.md",
        "--as",
        "alias",
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
    f.ok(&["resource", "add", "doc.md", "--as", "doc"]);
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
    f.ok(&["resource", "add", "notes.md", "--as", "notes"]);
    f.ok(&["sync"]);
    f.ok(&["entry", "remove", "project/CONTEXT.md"]);
    f.ok(&["entry", "add", "other/CONTEXT.md"]);
    f.ok(&["output", "published"]);
    f.ok(&["sync"]);
    assert!(!f.0.join("project/CONTEXT.md").exists());
    assert!(!f.0.join(".devmeld/output/index.md").exists());
    assert!(f.0.join("published/r-resource-1.md").exists());
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
        "service.json",
        "--as",
        "service",
        "--kind",
        "description",
        "--schema",
        "schema.json",
    ]);
    f.ok(&["sync"]);
    let page = f.0.join(".devmeld/output/r-resource-1.md");
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
    assert!(String::from_utf8_lossy(&help.stdout).contains("--kind document|description"));
    assert!(String::from_utf8_lossy(&help.stdout).contains("--language en|zh-CN"));
    assert!(String::from_utf8_lossy(&help.stdout).contains("language en|zh-CN"));
    assert!(String::from_utf8_lossy(&help.stdout).contains("then sync"));
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
fn invalid_addresses_envelopes_versions_and_oversize_sources_are_explicit_errors() {
    let f = Fixture::new();
    f.ok(&["init"]);
    fs::write(f.0.join("doc.md"), "original").unwrap();
    assert!(
        !f.run(&["resource", "add", "doc.md", "--as", "../bad"], true)
            .status
            .success()
    );
    f.ok(&["resource", "add", "doc.md", "--as", "doc"]);
    assert!(
        !f.run(&["resource", "add", "doc.md", "--as", "doc"], true)
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
            &[
                "resource",
                "add",
                "bad.json",
                "--as",
                "bad",
                "--kind",
                "description"
            ],
            true
        )
        .status
        .success()
    );
    fs::write(f.0.join("large.md"), vec![b'x'; 8 * 1024 * 1024 + 1]).unwrap();
    assert!(
        !f.run(&["resource", "add", "large.md", "--as", "large"], true)
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
    f.ok(&["resource", "add", "notes.md", "--as", "notes"]);
    f.ok(&[
        "resource",
        "add",
        "service.json",
        "--as",
        "service",
        "--kind",
        "description",
    ]);
    f.ok(&[
        "resource",
        "add",
        "tool.json",
        "--as",
        "tool",
        "--kind",
        "description",
    ]);
    f.ok(&["access", "add", "service", "tool"]);
    f.ok(&["sync"]);
    for file in [
        "project/CONTEXT.md",
        ".devmeld/output/index.md",
        ".devmeld/output/r-resource-1.md",
        ".devmeld/output/r-resource-2.md",
        ".devmeld/output/r-resource-3.md",
    ] {
        f.assert_local_links(file);
    }
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-2.md")).unwrap();
    assert!(
        page.contains("r-resource-3.md")
            && page.contains("does not authorize")
            && page.contains("verified local/system")
    );
    let tool_page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-3.md")).unwrap();
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
    assert!(!f.0.join(".devmeld/output/r-resource-3.md").exists());
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
                "description.json",
                "--as",
                "custom",
                "--kind",
                "description",
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
        "description.json",
        "--as",
        "custom",
        "--kind",
        "description",
        "--schema",
        "schema.json",
    ]);
    f.ok(&["sync"]);
    assert!(
        fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md"))
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
    f.ok(&["resource", "add", "alias.md", "--as", "alias"]);
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
