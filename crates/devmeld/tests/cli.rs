use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

#[test]
fn inheritance_edges_moves_and_local_override_removal_recompute_without_copying() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    f.apply(&["group", "add", "root", "--environment", "test"]);
    f.apply(&[
        "group",
        "add",
        "root/middle",
        "--inherit",
        "--attention",
        "curl",
    ]);
    f.apply(&["resource", "add", "notes.md", "--as", "root/middle/item"]);
    for propagate in [false, true] {
        for inherit in [false, true] {
            f.apply(&[
                "group",
                "update",
                "root/middle",
                if propagate {
                    "--propagate"
                } else {
                    "--no-propagate"
                },
            ]);
            f.apply(&[
                "resource",
                "update",
                "root/middle/item",
                if inherit { "--inherit" } else { "--no-inherit" },
            ]);
            f.apply(&["sync"]);
            let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
            assert_eq!(
                page.contains("environment: test (Origin: root)"),
                propagate && inherit,
                "{page}"
            );
            assert_eq!(
                page.contains("attention: curl (Origin: root/middle)"),
                propagate && inherit,
                "{page}"
            );
        }
    }
    f.apply(&["group", "update", "root/middle", "--no-inherit"]);
    let shown = String::from_utf8(f.run(&["resource", "show", "root/middle/item"]).stdout).unwrap();
    assert!(!shown.contains("environment: test"));
    assert!(shown.contains("attention: curl (Origin: root/middle)"));
    f.apply(&[
        "resource",
        "update",
        "root/middle/item",
        "--attention",
        "local",
    ]);
    f.apply(&[
        "resource",
        "update",
        "root/middle/item",
        "--remove-field",
        "attention",
    ]);
    f.apply(&["config", "set", "defaults.inherit", "true"]);
    f.apply(&["config", "set", "defaults.propagate", "false"]);
    f.apply(&["group", "move", "root", "archive/root"]);
    let shown = String::from_utf8(
        f.run(&["resource", "show", "archive/root/middle/item"])
            .stdout,
    )
    .unwrap();
    assert!(shown.contains("attention: curl (Origin: archive/root/middle)"));
    assert!(!shown.contains("environment: test"));
    // A new destination captures current defaults; the existing resource keeps its saved choice.
    f.apply(&[
        "resource",
        "move",
        "archive/root/middle/item",
        "destination/item",
    ]);
    f.apply(&["group", "update", "destination", "--attention", "ssh"]);
    let shown = String::from_utf8(f.run(&["resource", "show", "destination/item"]).stdout).unwrap();
    assert!(!shown.contains("attention: ssh"));
    assert!(shown.contains("Saved inheritance choices: inherit: true"));
    f.apply(&["group", "update", "destination", "--propagate"]);
    f.apply(&["sync"]);
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("attention: ssh (Origin: destination)"));
    assert!(page.contains("../../notes.md"));
    assert!(!page.contains("curl"));
    assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"source");
}

#[test]
fn inheritance_commands_reject_invalid_input_and_preserve_preview_noop_and_stale_guards() {
    let f = Fixture::new();
    for command in [
        vec!["config", "set", "defaults.inherit", "true"],
        vec!["group", "add", "g", "--inherit", "--inherit"],
        vec!["group", "add", "g", "--propagate", "--no-propagate"],
    ] {
        assert!(!f.confirm(&command).status.success(), "{command:?}");
        f.assert_empty();
    }
    f.apply(&[
        "group",
        "add",
        "g",
        "--field",
        "inherit=true",
        "--field",
        "propagate=false",
    ]);
    fs::write(f.0.join("source.md"), "source").unwrap();
    f.apply(&["resource", "add", "source.md", "--as", "g/r"]);
    let config_path = f.0.join(".devmeld/context.json");
    let original = fs::read(&config_path).unwrap();
    for command in [
        vec![
            "resource",
            "add",
            "source.md",
            "--as",
            "invalid",
            "--propagate",
        ],
        vec!["resource", "update", "g/r", "--no-propagate"],
        vec!["resource", "update", "g/r", "--inherit", "--no-inherit"],
        vec!["group", "update", "g", "--no-propagate", "--propagate"],
        vec!["config", "set", "defaults.inherit", "yes"],
        vec!["config", "set", "defaults.propagate", "1"],
        vec!["config", "set", "defaults.unknown", "true"],
    ] {
        assert!(!f.confirm(&command).status.success(), "{command:?}");
        assert_eq!(fs::read(&config_path).unwrap(), original);
    }
    for args in [
        vec!["config", "set", "defaults.inherit", "true", "--dry-run"],
        vec!["resource", "update", "g/r", "--inherit", "--dry-run"],
    ] {
        assert!(f.run(&args).status.success());
        assert_eq!(fs::read(&config_path).unwrap(), original);
    }
    let modified = fs::metadata(&config_path).unwrap().modified().unwrap();
    f.apply(&["group", "update", "g", "--no-inherit", "--propagate"]);
    f.apply(&["resource", "update", "g/r", "--no-inherit"]);
    f.apply(&["config", "set", "defaults.inherit", "false"]);
    assert_eq!(fs::read(&config_path).unwrap(), original);
    assert_eq!(
        fs::metadata(&config_path).unwrap().modified().unwrap(),
        modified
    );
    let prepare = |args: &[&str]| {
        devmeld::prepare(
            &f.0,
            &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        )
        .unwrap()
    };
    let noop = prepare(&["config", "set", "defaults.inherit", "false"]);
    assert!(noop.is_empty());
    let creation = prepare(&["resource", "add", "source.md", "--as", "new"]);
    f.apply(&["config", "set", "defaults.inherit", "true"]);
    assert!(noop.apply().unwrap_err().to_string().contains("stale"));
    assert!(creation.apply().unwrap_err().to_string().contains("stale"));
    assert!(!f.run(&["resource", "show", "new"]).status.success());
    f.apply(&["resource", "update", "g/r", "--inherit"]);
    let sync = prepare(&["sync"]);
    f.apply(&["group", "update", "g", "--environment", "changed"]);
    assert!(sync.apply().unwrap_err().to_string().contains("stale"));
    assert!(!f.0.join(".devmeld/output").exists());
    f.apply(&["sync"]);
    let index = f.0.join(".devmeld/output/index.md");
    let before = fs::read(&index).unwrap();
    let modified = fs::metadata(&index).unwrap().modified().unwrap();
    f.apply(&["config", "set", "defaults.propagate", "false"]);
    assert!(prepare(&["sync"]).is_empty());
    f.apply(&["sync"]);
    assert_eq!(fs::read(&index).unwrap(), before);
    assert_eq!(fs::metadata(&index).unwrap().modified().unwrap(), modified);
}

#[test]
fn inherited_navigation_is_derived_localized_and_separate_from_source_attributes() {
    let f = Fixture::new();
    let source =
        br#"{"title":"HTTP","summary":"ssh via curl","attributes":{"environment":"source"}}"#;
    fs::write(f.0.join("service.json"), source).unwrap();
    f.apply(&[
        "group",
        "add",
        "team",
        "--description",
        "parent-only description",
        "--tag",
        "backend",
        "--environment",
        "test",
    ]);
    f.apply(&["group", "add", "team/db", "--inherit", "--tag", "backend"]);
    f.apply(&[
        "resource",
        "add",
        "service.json",
        "--as",
        "team/db/http",
        "--kind",
        "description",
        "--inherit",
        "--attention",
        "Use curl",
    ]);
    let shown = f.run(&["resource", "show", "team/db/http"]);
    let shown = String::from_utf8(shown.stdout).unwrap();
    assert!(
        shown.contains("environment: test (Origin: team)"),
        "{shown}"
    );
    assert!(shown.contains("backend (Origin: team, team/db)"), "{shown}");
    assert!(!shown.contains("parent-only description"));
    for (language, heading, origin) in [
        ("en", "Effective tags and fields", "Origin"),
        ("zh-CN", "生效标签和字段", "来源"),
    ] {
        f.apply(&["language", language]);
        f.apply(&["sync"]);
        for name in ["index.md", "r-resource-1.md"] {
            let text = fs::read_to_string(f.0.join(".devmeld/output").join(name)).unwrap();
            assert!(text.contains(heading), "{text}");
            assert!(
                text.contains(&format!("environment: test ({origin}: team)")),
                "{text}"
            );
            assert!(text.contains("Use curl"));
        }
        let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
        assert!(page.contains("environment: source"));
        assert!(!page.contains("parent-only description"));
    }
    f.apply(&["group", "update", "team", "--environment", "staging"]);
    f.apply(&["sync"]);
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("environment: staging (来源: team)"));
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/context.json")).unwrap()).unwrap();
    assert!(
        config["resources"][0]["annotations"]["fields"]
            .get("environment")
            .is_none()
    );
    assert!(config["resources"][0]["annotations"].get("tags").is_none());
    f.apply(&["resource", "update", "team/db/http", "--environment", ""]);
    f.apply(&["sync"]);
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("environment:  (来源: team/db/http)"));
    assert!(!page.contains("environment: staging"));
    assert_eq!(fs::read(f.0.join("service.json")).unwrap(), source);
}

#[test]
fn inheritance_defaults_and_explicit_choices_are_saved_only_at_creation() {
    let f = Fixture::new();
    fs::write(f.0.join("source.md"), "source").unwrap();
    f.apply(&["group", "add", "old"]);
    f.apply(&["config", "set", "defaults.inherit", "true"]);
    f.apply(&["config", "set", "defaults.propagate", "false"]);
    f.apply(&[
        "resource",
        "add",
        "source.md",
        "--as",
        "new/nested/resource",
        "--no-inherit",
    ]);
    f.apply(&["group", "add", "explicit", "--no-inherit", "--propagate"]);
    let read = || -> serde_json::Value {
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/context.json")).unwrap()).unwrap()
    };
    let before = read();
    let group = |name: &str| {
        before["groups"]
            .as_array()
            .unwrap()
            .iter()
            .find(|g| g["path"] == name)
            .unwrap()
    };
    assert_eq!(group("old")["inherit"], false);
    assert_eq!(group("old")["propagate"], true);
    assert_eq!(group("new/nested")["inherit"], true);
    assert_eq!(group("new/nested")["propagate"], false);
    assert_eq!(group("explicit")["inherit"], false);
    assert_eq!(group("explicit")["propagate"], true);
    assert_eq!(before["resources"][0]["inherit"], false);
    f.apply(&["config", "set", "defaults.inherit", "false"]);
    f.apply(&["config", "set", "defaults.propagate", "true"]);
    let after = read();
    assert_eq!(before["groups"], after["groups"]);
    assert_eq!(before["resources"], after["resources"]);
    f.apply(&["group", "update", "new/nested", "--tag", "local"]);
    f.apply(&["resource", "update", "new/nested/resource", "--inherit"]);
    let after = read();
    assert_eq!(after["resources"][0]["inherit"], true);
    let shown = f.run(&["group", "show", "new/nested"]);
    let shown = String::from_utf8(shown.stdout).unwrap();
    assert!(shown.contains("inherit: true"), "{shown}");
    assert!(shown.contains("propagate: false"), "{shown}");
}

#[test]
fn group_annotations_are_saved_inspected_and_published_as_context_information() {
    let f = Fixture::new();
    f.apply(&[
        "group",
        "add",
        "services/database",
        "--description",
        "数据库 HTTP / ssh",
        "--tag",
        "backend",
        "--tag",
        "backend",
        "--environment",
        "test",
        "--shared",
        "--field",
        "owner/team=平台",
        "--attention",
        "Use curl",
    ]);
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/context.json")).unwrap()).unwrap();
    assert_eq!(config["groups"][0]["path"], "services");
    assert_eq!(config["groups"][0]["inherit"], false);
    assert_eq!(config["groups"][0]["propagate"], true);
    assert_eq!(
        config["groups"][1]["annotations"]["fields"]["shared"],
        "true"
    );
    assert_eq!(
        config["groups"][1]["annotations"]["tags"],
        serde_json::json!(["backend"])
    );
    let shown = f.run(&["group", "show", "services/database"]);
    assert!(shown.status.success());
    let shown = String::from_utf8(shown.stdout).unwrap();
    for expected in [
        "Context annotations (local)",
        "数据库 HTTP / ssh",
        "backend",
        "environment",
        "test",
        "owner/team",
        "平台",
    ] {
        assert!(shown.contains(expected), "{shown}");
    }
    f.apply(&["sync"]);
    let index = fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap();
    assert!(index.contains("Context annotations (local)"), "{index}");
    assert!(index.contains("数据库 HTTP / ssh"));
    assert!(index.contains("environment: test"));
    assert!(index.contains("owner/team: 平台"));
    assert!(!f.0.join("services").exists());
}

struct Fixture(PathBuf);

#[test]
fn annotated_nodes_retain_metadata_and_associations_across_saved_moves() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    f.apply(&[
        "group",
        "add",
        "db/empty",
        "--description",
        "empty subgroup",
    ]);
    f.apply(&[
        "group",
        "update",
        "db",
        "--description",
        "database",
        "--shared",
    ]);
    f.apply(&[
        "resource",
        "add",
        "notes.md",
        "--as",
        "db/service",
        "--field",
        "version=01",
        "--field",
        "attention=",
    ]);
    f.apply(&[
        "resource",
        "add",
        "notes.md",
        "--as",
        "tools/curl",
        "--description",
        "HTTP guide",
    ]);
    f.apply(&["access", "add", "db/service", "tools/curl"]);
    f.apply(&["sync"]);
    let before_index = fs::read(f.0.join(".devmeld/output/index.md")).unwrap();
    f.apply(&["group", "move", "db", "archive/db"]);
    f.apply(&["resource", "move", "tools/curl", "archive/db/http"]);
    assert_eq!(
        fs::read(f.0.join(".devmeld/output/index.md")).unwrap(),
        before_index
    );
    for (kind, path, description) in [
        ("group", "archive/db", "database"),
        ("group", "archive/db/empty", "empty subgroup"),
        ("resource", "archive/db/http", "HTTP guide"),
    ] {
        let shown = f.run(&[kind, "show", path]);
        assert!(shown.status.success());
        assert!(
            String::from_utf8(shown.stdout)
                .unwrap()
                .contains(description)
        );
    }
    let config_path = f.0.join(".devmeld/context.json");
    let before = fs::read(&config_path).unwrap();
    let modified = fs::metadata(&config_path).unwrap().modified().unwrap();
    let shown = f.run(&["resource", "show", "archive/db/service"]);
    let shown = String::from_utf8(shown.stdout).unwrap();
    assert!(shown.contains("archive/db/http"));
    assert!(shown.contains("version: 01"));
    assert!(!shown.contains("shared: true"));
    assert_eq!(fs::read(&config_path).unwrap(), before);
    assert_eq!(
        fs::metadata(config_path).unwrap().modified().unwrap(),
        modified
    );
    f.apply(&["sync"]);
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("archive/db/http"));
    assert!(page.contains("version: 01"));
    f.apply(&["group", "remove", "archive/db/empty"]);
    f.apply(&["access", "remove", "archive/db/service", "archive/db/http"]);
    f.apply(&["resource", "remove", "archive/db/http"]);
    f.apply(&["sync"]);
    assert!(!f.0.join(".devmeld/output/r-resource-2.md").exists());
    assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"source");
}

#[test]
fn annotation_help_explains_add_update_and_only_implemented_flags_without_context() {
    let f = Fixture::new();
    for kind in ["resource", "group"] {
        for operation in ["add", "update"] {
            let result = f.run(&[kind, operation, "--help"]);
            assert!(
                result.status.success(),
                "{kind} {operation}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            let help = String::from_utf8(result.stdout).unwrap();
            for flag in [
                "--description",
                "--tag",
                "--field",
                "--environment",
                "--attention",
                "--shared",
                "--no-shared",
            ] {
                assert!(help.contains(flag), "{flag}: {help}");
            }
            if operation == "update" {
                for flag in ["--clear-description", "--remove-tag", "--remove-field"] {
                    assert!(help.contains(flag));
                }
            }
            assert!(help.contains("--inherit"));
            assert!(help.contains("--no-inherit"));
            assert_eq!(help.contains("--propagate"), kind == "group");
            assert!(!help.contains("--yes"));
        }
    }
    f.assert_empty();
}

#[test]
fn annotation_shortcuts_and_fields_are_equivalent_and_conflicting_edits_never_write() {
    let first = Fixture::new();
    let second = Fixture::new();
    first.apply(&[
        "group",
        "add",
        "db",
        "--shared",
        "--environment",
        "test",
        "--attention",
        "ssh",
    ]);
    second.apply(&[
        "group",
        "add",
        "db",
        "--field",
        "shared=true",
        "--field",
        "environment=test",
        "--field",
        "attention=ssh",
    ]);
    assert_eq!(
        fs::read(first.0.join(".devmeld/context.json")).unwrap(),
        fs::read(second.0.join(".devmeld/context.json")).unwrap()
    );
    let invalid = [
        vec!["--environment", "test", "--field", "environment=test"],
        vec!["--shared", "--no-shared"],
        vec!["--shared", "--field", "shared=true"],
        vec!["--field", "bad"],
        vec!["--field", "=value"],
        vec!["--tag", ""],
        vec!["--description", "  "],
        vec!["--description", "\u{1b}[2J"],
        vec!["--description", "x", "--clear-description"],
        vec!["--tag", "backend", "--remove-tag", "backend"],
        vec!["--field", "owner=me", "--remove-field", "owner"],
        vec!["--environment"],
        vec!["--inherit", "--no-inherit"],
    ];
    let before = fs::read(first.0.join(".devmeld/context.json")).unwrap();
    for options in invalid {
        let fresh = Fixture::new();
        let mut create = vec!["group", "add", "db"];
        create.extend(&options);
        let result = fresh.confirm(&create);
        assert!(!result.status.success(), "{options:?}");
        fresh.assert_empty();
        let mut update = vec!["group", "update", "db"];
        update.extend(&options);
        assert!(!first.confirm(&update).status.success(), "{options:?}");
        assert_eq!(
            fs::read(first.0.join(".devmeld/context.json")).unwrap(),
            before
        );
    }
    for args in [
        vec!["group", "update", "missing", "--tag", "x"],
        vec!["resource", "update", "db", "--tag", "x"],
        vec!["group", "update", "db", "--schema", "missing.json"],
        vec!["group", "update", "db"],
    ] {
        assert!(!first.confirm(&args).status.success(), "{args:?}");
        assert_eq!(
            fs::read(first.0.join(".devmeld/context.json")).unwrap(),
            before
        );
    }
    let preview = first.run(&["group", "update", "db", "--tag", "preview", "--dry-run"]);
    assert!(preview.status.success());
    assert_eq!(
        fs::read(first.0.join(".devmeld/context.json")).unwrap(),
        before
    );
    let plan = devmeld::prepare(
        &first.0,
        &[
            "group".into(),
            "update".into(),
            "db".into(),
            "--tag".into(),
            "stale".into(),
        ],
    )
    .unwrap();
    first.apply(&["group", "update", "db", "--attention", "changed"]);
    let current = fs::read(first.0.join(".devmeld/context.json")).unwrap();
    assert!(plan.apply().unwrap_err().to_string().contains("stale"));
    assert_eq!(
        fs::read(first.0.join(".devmeld/context.json")).unwrap(),
        current
    );
}

#[test]
fn annotation_updates_preserve_unspecified_values_and_support_explicit_removal() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    f.apply(&[
        "group",
        "add",
        "db",
        "--description",
        "database",
        "--tag",
        "backend",
        "--tag",
        "old",
        "--environment",
        "test",
        "--attention",
        "care",
    ]);
    f.apply(&[
        "resource",
        "add",
        "notes.md",
        "--as",
        "db/notes",
        "--description",
        "notes",
        "--tag",
        "old",
        "--field",
        "owner=team",
        "--shared",
    ]);
    f.apply(&[
        "group",
        "update",
        "db",
        "--environment",
        "production",
        "--tag",
        "new",
        "--remove-tag",
        "old",
        "--remove-field",
        "attention",
    ]);
    let shown = String::from_utf8(f.run(&["group", "show", "db"]).stdout).unwrap();
    assert!(shown.contains("Description: database"));
    assert!(shown.contains("Tags: backend, new"));
    assert!(shown.contains("environment: production"));
    assert!(!shown.contains("attention:"));
    f.apply(&[
        "resource",
        "update",
        "db/notes",
        "--clear-description",
        "--remove-tag",
        "old",
        "--no-shared",
    ]);
    let shown = String::from_utf8(f.run(&["resource", "show", "db/notes"]).stdout).unwrap();
    assert!(shown.contains("owner: team"));
    assert!(shown.contains("shared: false"));
    assert!(!shown.contains("Description:"));
    assert!(!shown.contains("Tags:"));
    let path = f.0.join(".devmeld/context.json");
    let before = fs::read(&path).unwrap();
    let modified = fs::metadata(&path).unwrap().modified().unwrap();
    f.apply(&["resource", "update", "db/notes", "--no-shared"]);
    assert_eq!(fs::read(&path).unwrap(), before);
    assert_eq!(fs::metadata(path).unwrap().modified().unwrap(), modified);
    f.apply(&[
        "resource",
        "update",
        "db/notes",
        "--remove-field",
        "owner",
        "--remove-field",
        "shared",
    ]);
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/context.json")).unwrap()).unwrap();
    assert!(config["resources"][0].get("annotations").is_none());
    assert_eq!(config["resources"][0]["id"], "resource-1");
    assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"source");
}

#[test]
fn resource_annotations_keep_source_attributes_separate_in_both_output_languages() {
    let f = Fixture::new();
    f.apply(&["group", "add", "database", "--tag", "parent-only"]);
    let child = f.0.join("child");
    fs::create_dir(&child).unwrap();
    let invocation = Fixture(child);
    let source = invocation.0.join("service.json");
    let bytes = br#"{"title":"HTTP service","summary":"ssh curl source","attributes":{"environment":"production"}}"#;
    fs::write(&source, bytes).unwrap();
    fs::write(
        invocation.0.join("schema.json"),
        r#"{"type":"object","required":["environment"]}"#,
    )
    .unwrap();
    let modified = fs::metadata(&source).unwrap().modified().unwrap();
    let result = invocation.confirm(&[
        "resource",
        "add",
        "service.json",
        "--kind",
        "description",
        "--shared",
        "--no-inherit",
        "--schema",
        "schema.json",
        "--as",
        "database/service",
        "--description",
        "本地使用说明",
        "--tag",
        "backend",
        "--environment",
        "test",
        "--field",
        "attention=[ssh] <http>",
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let show = f.run(&["resource", "show", "database/service"]);
    assert!(show.status.success());
    let shown = String::from_utf8(show.stdout).unwrap();
    assert!(shown.contains("Context annotations (local)"), "{shown}");
    assert!(shown.contains("environment: test"));
    assert!(!shown.contains("parent-only"));
    for (language, heading, attributes) in [
        (
            "en",
            "Context annotations (local)",
            "Source-declared attributes",
        ),
        ("zh-CN", "上下文标注（本级）", "源文件声明的属性"),
    ] {
        f.apply(&["language", language]);
        f.apply(&["sync"]);
        let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-1.md")).unwrap();
        for expected in [
            heading,
            attributes,
            "environment: production",
            "environment: test",
            "本地使用说明",
            "\\[ssh\\] &lt;http&gt;",
            "../../child/service.json",
        ] {
            assert!(page.contains(expected), "{expected}: {page}");
        }
        assert!(!page.contains("parent-only"));
        let index = fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap();
        assert!(index.contains("本地使用说明"), "{index}");
    }
    assert_eq!(fs::read(&source).unwrap(), bytes);
    assert_eq!(fs::metadata(source).unwrap().modified().unwrap(), modified);
    let page = f.0.join(".devmeld/output/r-resource-1.md");
    let before = fs::read(&page).unwrap();
    let modified = fs::metadata(&page).unwrap().modified().unwrap();
    f.apply(&["sync"]);
    assert_eq!(fs::read(&page).unwrap(), before);
    assert_eq!(fs::metadata(page).unwrap().modified().unwrap(), modified);
}

impl Fixture {
    fn new() -> Self {
        Self::new_in(&std::env::temp_dir())
    }

    fn new_in(parent: &std::path::Path) -> Self {
        let path = parent.join(format!(
            "devmeld-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_devmeld"))
            .current_dir(&self.0)
            .args(args)
            .stdin(Stdio::null())
            .output()
            .unwrap()
    }

    fn assert_empty(&self) {
        assert_eq!(fs::read_dir(&self.0).unwrap().count(), 0);
    }

    fn apply(&self, args: &[&str]) -> Output {
        let mut selected = vec!["--context", self.0.to_str().unwrap()];
        selected.extend_from_slice(args);
        let output = self.confirm(&selected);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    fn confirm(&self, args: &[&str]) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_devmeld"))
            .current_dir(&self.0)
            .args(args)
            .arg("--apply")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        if let Err(error) = child.stdin.take().unwrap().write_all(b"apply\n") {
            assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
        }
        child.wait_with_output().unwrap()
    }
}

#[cfg(windows)]
#[test]
#[ignore = "requires DEVMELD_TEST_OTHER_ROOT on a second local Windows drive"]
fn cross_drive_logical_moves_preserve_native_sources_and_link_destinations() {
    let f = Fixture::new();
    let other_root = PathBuf::from(
        std::env::var_os("DEVMELD_TEST_OTHER_ROOT")
            .expect("set the second local-drive test directory"),
    );
    let other = Fixture::new_in(&other_root);
    assert_ne!(f.0.components().next(), other.0.components().next());
    let source = other.0.join("团队 notes.md");
    fs::write(&source, "ssh http original source").unwrap();
    let source_time = fs::metadata(&source).unwrap().modified().unwrap();
    f.apply(&["group", "add", "knowledge", "--attention", "Use curl"]);
    f.apply(&[
        "resource",
        "add",
        source.to_str().unwrap(),
        "--as",
        "knowledge/团队 notes",
        "--description",
        "跨盘 HTTP 资料",
        "--tag",
        "ssh",
        "--inherit",
    ]);
    f.apply(&["sync"]);
    let page = f.0.join(".devmeld/output/r-resource-1.md");
    let before = fs::read_to_string(&page).unwrap();
    assert!(before.contains("attention: Use curl (Origin: knowledge)"));
    let link = before
        .split("file:///")
        .nth(1)
        .unwrap()
        .split(')')
        .next()
        .unwrap()
        .to_owned();
    assert!(link.contains("%E5%9B%A2%E9%98%9F%20notes.md"), "{before}");
    f.apply(&["group", "move", "knowledge", "archive/knowledge"]);
    f.apply(&[
        "resource",
        "move",
        "archive/knowledge/团队 notes",
        "archive/team-notes",
    ]);
    f.apply(&["group", "remove", "archive/knowledge"]);
    f.apply(&["group", "update", "archive", "--attention", "Use ssh"]);
    f.apply(&[
        "resource",
        "update",
        "archive/team-notes",
        "--environment",
        "test",
    ]);
    f.apply(&["sync"]);
    let after = fs::read_to_string(&page).unwrap();
    assert!(after.contains(&format!("file:///{link}")), "{after}");
    assert!(after.contains("跨盘 HTTP 资料"));
    assert!(after.contains("environment: test"));
    assert!(after.contains("attention: Use ssh (Origin: archive)"));
    assert!(!after.contains("Use curl"));
    let shown = f.run(&["resource", "show", "archive/team-notes"]);
    assert!(shown.status.success());
    assert!(String::from_utf8_lossy(&shown.stdout).contains("resource-1"));
    assert_eq!(fs::read(&source).unwrap(), b"ssh http original source");
    assert_eq!(
        fs::metadata(&source).unwrap().modified().unwrap(),
        source_time
    );
    assert_eq!(fs::read_dir(&other.0).unwrap().count(), 1);
}

#[test]
fn resource_show_reports_registration_without_reading_or_rewriting_source() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "authored knowledge").unwrap();
    f.apply(&["resource", "add", "notes.md", "--as", "knowledge/notes"]);
    let config_path = f.0.join(".devmeld/context.json");
    let config = fs::read(&config_path).unwrap();
    let modified = fs::metadata(&config_path).unwrap().modified().unwrap();
    // Inspecting a registration must not require the source to be currently available.
    fs::remove_file(f.0.join("notes.md")).unwrap();
    let shown = f.run(&["resource", "show", "knowledge/notes"]);
    assert!(
        shown.status.success(),
        "{}",
        String::from_utf8_lossy(&shown.stderr)
    );
    let report = String::from_utf8(shown.stdout).unwrap();
    for expected in [
        "Resource: knowledge/notes",
        "Identity: resource-1",
        "Source kind: document",
        "notes.md",
        "Registration only",
    ] {
        assert!(report.contains(expected), "{report}");
    }
    assert!(!report.contains("Type apply"), "{report}");
    assert_eq!(fs::read(&config_path).unwrap(), config);
    assert_eq!(
        fs::metadata(&config_path).unwrap().modified().unwrap(),
        modified
    );
    assert!(!f.0.join(".devmeld/output").exists());
    assert!(!f.0.join("notes.md").exists());
}

#[test]
fn organization_lists_use_logical_subtrees_and_show_direct_group_children() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    for address in [
        "database/test/orders",
        "database/prod/orders",
        "database2/other",
    ] {
        f.apply(&["resource", "add", "notes.md", "--as", address]);
    }
    let before = fs::read(f.0.join(".devmeld/context.json")).unwrap();
    let cases = [
        (
            vec!["resource", "list"],
            vec![
                "database/prod/orders",
                "database/test/orders",
                "database2/other",
            ],
            vec![],
        ),
        (
            vec!["resource", "list", "database"],
            vec!["database/prod/orders", "database/test/orders"],
            vec!["database2/other"],
        ),
        (
            vec!["group", "list", "database"],
            vec!["database/prod", "database/test"],
            vec!["database2", "orders"],
        ),
        (
            vec!["group", "show", "database"],
            vec!["Group: database", "database/prod", "database/test"],
            vec!["orders", "database2"],
        ),
        (
            vec!["group", "show", "database/test"],
            vec!["Group: database/test", "database/test/orders"],
            vec!["database/prod"],
        ),
    ];
    for (args, present, absent) in cases {
        let result = f.run(&args);
        assert!(
            result.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let report = String::from_utf8(result.stdout).unwrap();
        for value in present {
            assert!(report.contains(value), "{report}");
        }
        for value in absent {
            assert!(!report.contains(value), "{report}");
        }
        assert!(!report.contains("changed target"), "{report}");
    }
    for args in [
        vec!["resource", "list", "missing"],
        vec!["group", "show", "database/test/orders"],
        vec!["resource", "show", "database"],
        vec!["group", "list", "../database"],
    ] {
        assert!(!f.run(&args).status.success(), "{args:?}");
    }
    assert_eq!(fs::read(f.0.join(".devmeld/context.json")).unwrap(), before);
    assert!(!f.0.join(".devmeld/output").exists());
}

#[test]
fn logical_resource_move_retains_source_identity_associations_and_published_page_location() {
    let f = Fixture::new();
    fs::write(f.0.join("source.md"), "untouched source").unwrap();
    f.apply(&[
        "resource",
        "add",
        "source.md",
        "--as",
        "database/test/orders",
    ]);
    f.apply(&["resource", "add", "source.md", "--as", "tools/query"]);
    f.apply(&["access", "add", "database/test/orders", "tools/query"]);
    f.apply(&["sync"]);
    let original: serde_json::Value =
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/context.json")).unwrap()).unwrap();
    let page = f.0.join(".devmeld/output/r-resource-1.md");
    let page_before = fs::read(&page).unwrap();
    f.apply(&["resource", "move", "database/test/orders", "archive/orders"]);
    let moved: serde_json::Value =
        serde_json::from_slice(&fs::read(f.0.join(".devmeld/context.json")).unwrap()).unwrap();
    assert_eq!(moved["resources"][0]["id"], original["resources"][0]["id"]);
    assert_eq!(
        moved["resources"][0]["document"],
        original["resources"][0]["document"]
    );
    assert_eq!(moved["resources"][0]["path"], "archive/orders");
    assert_eq!(moved["access"], original["access"]);
    assert_eq!(moved["next_resource_id"], original["next_resource_id"]);
    assert_eq!(fs::read(&page).unwrap(), page_before);
    f.apply(&["sync"]);
    let index = fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap();
    assert!(index.contains("archive/orders"), "{index}");
    assert!(!index.contains("database/test/orders"), "{index}");
    assert!(page.exists());
    assert!(
        fs::read_to_string(&page)
            .unwrap()
            .contains("r-resource-2.md")
    );
    let shown = f.run(&["resource", "show", "archive/orders"]);
    assert!(shown.status.success());
    assert!(String::from_utf8_lossy(&shown.stdout).contains("tools/query"));
    assert_eq!(
        fs::read(f.0.join("source.md")).unwrap(),
        b"untouched source"
    );
    assert!(
        !f.run(&["resource", "show", "database/test/orders"])
            .status
            .success()
    );
}

#[test]
fn groups_can_start_a_context_and_only_empty_groups_can_be_removed() {
    let f = Fixture::new();
    let preview = f.run(&["group", "add", "database/test", "--dry-run"]);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    f.assert_empty();
    f.apply(&["group", "add", "database/test"]);
    assert!(!f.0.join("database").exists());
    f.apply(&["sync"]);
    let index = fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap();
    assert!(index.contains("database/test"), "{index}");
    let before = fs::read(f.0.join(".devmeld/context.json")).unwrap();
    assert!(!f.confirm(&["group", "remove", "database"]).status.success());
    assert_eq!(fs::read(f.0.join(".devmeld/context.json")).unwrap(), before);
    f.apply(&["group", "remove", "database/test"]);
    f.apply(&["group", "remove", "database"]);
    assert_eq!(
        fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap(),
        index
    );
    f.apply(&["sync"]);
    let index = fs::read_to_string(f.0.join(".devmeld/output/index.md")).unwrap();
    assert!(!index.contains("database"), "{index}");
}

#[test]
fn group_move_updates_descendant_navigation_and_incoming_associations_only_after_sync() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    for address in ["database/test/orders", "tools/query", "database2/orders"] {
        f.apply(&["resource", "add", "notes.md", "--as", address]);
    }
    f.apply(&["group", "add", "database/empty"]);
    f.apply(&["access", "add", "tools/query", "database/test/orders"]);
    f.apply(&["sync"]);
    let config_path = f.0.join(".devmeld/context.json");
    let original: serde_json::Value =
        serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    let index_before = fs::read(f.0.join(".devmeld/output/index.md")).unwrap();
    f.apply(&["group", "move", "database", "archive/database"]);
    let moved: serde_json::Value =
        serde_json::from_slice(&fs::read(&config_path).unwrap()).unwrap();
    assert_eq!(
        moved["resources"][0]["path"],
        "archive/database/test/orders"
    );
    assert_eq!(moved["resources"][2], original["resources"][2]);
    assert_eq!(moved["access"], original["access"]);
    assert_eq!(moved["next_resource_id"], original["next_resource_id"]);
    assert_eq!(
        fs::read(f.0.join(".devmeld/output/index.md")).unwrap(),
        index_before
    );
    let shown = f.run(&["resource", "show", "tools/query"]);
    assert!(String::from_utf8_lossy(&shown.stdout).contains("archive/database/test/orders"));
    f.apply(&["sync"]);
    let page = fs::read_to_string(f.0.join(".devmeld/output/r-resource-2.md")).unwrap();
    assert!(page.contains("archive/database/test/orders"), "{page}");
    assert!(page.contains("r-resource-1.md"), "{page}");
    let groups = f.run(&["group", "list", "archive"]);
    assert!(String::from_utf8_lossy(&groups.stdout).contains("archive/database/empty"));
    assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"source");
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn resource_add_help_needs_no_context_and_creates_nothing() {
    let fixture = Fixture::new();
    let output = fixture.run(&["resource", "add", "--help"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("resource add SOURCE [--as PATH]"), "{text}");
    assert!(
        text.contains("--kind document|description] [--schema PATH]"),
        "{text}"
    );
    assert!(text.contains("sync"), "{text}");
    assert!(text.contains("logical organization address"), "{text}");
    fixture.assert_empty();
}

#[test]
fn command_groups_and_operations_have_specific_current_help() {
    let fixture = Fixture::new();
    let cases: &[(&[&str], &[&str])] = &[
        (&["resource"], &["resource add", "resource remove"]),
        (&["resource", "remove"], &["resource remove PATH", "source"]),
        (&["entry"], &["entry add", "entry remove"]),
        (&["entry", "add"], &["--kind file|instructions", "sync"]),
        (&["entry", "remove"], &["entry remove PATH", "sync"]),
        (&["access"], &["access add", "access remove"]),
        (&["access", "add"], &["access add RESOURCE TOOL", "install"]),
        (
            &["access", "remove"],
            &["access remove RESOURCE TOOL", "source"],
        ),
        (
            &["init"],
            &["--instruction-entry PATH", "--language en|zh-CN"],
        ),
        (&["output"], &["output PATH", "sync"]),
        (&["language"], &["language en|zh-CN", "authored"]),
        (
            &["config"],
            &["defaults.inherit true|false", "existing context"],
        ),
        (
            &["config", "set"],
            &["defaults.propagate true|false", "future nodes"],
        ),
        (&["sync"], &["sync [--apply | --dry-run]", "source"]),
        (&["recover"], &["recover [--apply | --dry-run]", "recovery"]),
    ];
    for (topic, expected) in cases {
        let mut args = topic.to_vec();
        args.push("--help");
        let output = fixture.run(&args);
        assert!(
            output.status.success(),
            "{topic:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        for needle in *expected {
            assert!(text.contains(needle), "{topic:?}: missing {needle}: {text}");
        }
        assert!(!text.contains("--inherit"), "unimplemented option: {text}");
        fixture.assert_empty();
    }
}

#[test]
fn help_ignores_unavailable_context_and_supports_short_form() {
    let fixture = Fixture::new();
    let missing = fixture.0.join("not initialized");
    for topic in [vec![], vec!["resource"], vec!["resource", "add"]] {
        for flag in ["--help", "-h"] {
            let mut args = vec!["--context", missing.to_str().unwrap()];
            args.extend_from_slice(&topic);
            args.push(flag);
            let output = fixture.run(&args);
            assert!(
                output.status.success(),
                "{args:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(!output.stdout.is_empty());
            assert!(output.stderr.is_empty());
            fixture.assert_empty();
        }
    }
}

#[test]
fn unknown_help_topics_fail_before_context_access_without_writing() {
    let fixture = Fixture::new();
    let missing = fixture.0.join("missing");
    for topic in [
        vec!["unknown"],
        vec!["resource", "dance"],
        vec!["group", "dance"],
    ] {
        let mut args = vec!["--context", missing.to_str().unwrap()];
        args.extend_from_slice(&topic);
        args.push("--help");
        let output = fixture.run(&args);
        assert!(!output.status.success());
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.contains("unknown help topic"), "{error}");
        assert!(error.contains("--help"), "{error}");
        assert!(output.stdout.is_empty());
        fixture.assert_empty();
    }
}

#[test]
fn organization_help_describes_real_operations_and_read_only_queries() {
    let f = Fixture::new();
    for (topic, expected) in [
        (vec!["group"], "group add PATH"),
        (vec!["group", "add"], "group add PATH"),
        (vec!["group", "remove"], "empty"),
        (vec!["group", "move"], "group move FROM TO"),
        (vec!["resource", "move"], "resource move FROM TO"),
    ] {
        let mut args = topic;
        args.push("--help");
        let output = f.run(&args);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains(expected));
    }
    for noun in ["resource", "group"] {
        for action in ["list", "show"] {
            let output = f.run(&[noun, action, "--help"]);
            assert!(output.status.success());
            let help = String::from_utf8_lossy(&output.stdout);
            assert!(help.contains("read-only"), "{help}");
            assert!(!help.contains("[--apply"), "{help}");
        }
    }
    f.assert_empty();
}

#[test]
fn organization_rejections_previews_and_noops_never_rewrite_registration() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    for address in ["database/test/orders", "tools/query"] {
        f.apply(&["resource", "add", "notes.md", "--as", address]);
    }
    f.apply(&["access", "add", "database/test/orders", "tools/query"]);
    let config_path = f.0.join(".devmeld/context.json");
    let before = fs::read(&config_path).unwrap();
    let modified = fs::metadata(&config_path).unwrap().modified().unwrap();
    for args in [
        vec!["resource", "move", "database/test/orders", "tools/query"],
        vec!["resource", "move", "database/test/orders", "tools"],
        vec!["resource", "move", "missing", "new/path"],
        vec!["group", "move", "database", "database/test/nested"],
        vec!["group", "move", "database", "tools"],
        vec!["group", "move", "database", "tools/query/child"],
        vec!["group", "remove", "database/test"],
        vec!["group", "remove", "database", "--recursive"],
        vec!["group", "add", "tools/query/child"],
        vec!["resource", "remove", "tools/query"],
    ] {
        assert!(!f.confirm(&args).status.success(), "{args:?}");
        assert_eq!(fs::read(&config_path).unwrap(), before, "{args:?}");
    }
    for args in [
        vec![
            "resource",
            "move",
            "database/test/orders",
            "new/orders",
            "--dry-run",
        ],
        vec!["group", "move", "database", "new/database", "--dry-run"],
        vec![
            "resource",
            "move",
            "database/test/orders",
            "database/test/orders",
            "--apply",
        ],
        vec!["group", "move", "database", "database", "--apply"],
    ] {
        let result = f.run(&args);
        assert!(
            result.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(!String::from_utf8_lossy(&result.stdout).contains("Type apply"));
    }
    assert_eq!(fs::read(&config_path).unwrap(), before);
    assert_eq!(
        fs::metadata(&config_path).unwrap().modified().unwrap(),
        modified
    );
    assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"source");
    assert!(!f.0.join(".devmeld/output").exists());

    let stale = devmeld::prepare_in(
        &f.0,
        None,
        &[
            "group".into(),
            "move".into(),
            "database".into(),
            "archive/database".into(),
        ],
    )
    .unwrap();
    f.apply(&["group", "add", "fresh"]);
    let latest = fs::read(&config_path).unwrap();
    assert!(
        stale
            .apply()
            .unwrap_err()
            .to_string()
            .contains("stale preview")
    );
    assert_eq!(fs::read(&config_path).unwrap(), latest);
}

#[test]
fn organization_queries_never_bootstrap_or_bypass_existing_ownership() {
    let f = Fixture::new();
    for args in [
        vec!["resource", "list"],
        vec!["group", "list"],
        vec!["group", "show", "missing"],
        vec!["resource", "show", "missing"],
    ] {
        assert!(!f.run(&args).status.success());
        f.assert_empty();
        let mut explicit = vec!["--context", "absent/context"];
        explicit.extend(args);
        assert!(!f.run(&explicit).status.success());
        f.assert_empty();
    }
    f.apply(&["group", "add", "database"]);
    let config_path = f.0.join(".devmeld/context.json");
    let before = fs::read(&config_path).unwrap();
    let refused = f.run(&["group", "list", "--apply"]);
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("read-only"));
    assert_eq!(fs::read(&config_path).unwrap(), before);
    assert!(f.run(&["group", "list", "--dry-run"]).status.success());
    fs::remove_file(f.0.join(".devmeld/state/owned.json")).unwrap();
    assert!(!f.run(&["group", "list"]).status.success());
    assert!(
        !f.confirm(&["group", "move", "database", "archive"])
            .status
            .success()
    );
    assert_eq!(fs::read(&config_path).unwrap(), before);
    assert!(!f.0.join(".devmeld/state/owned.json").exists());
}

#[test]
fn root_help_forms_are_identical_and_malformed_selectors_do_not_write() {
    let fixture = Fixture::new();
    let root = fixture.run(&[]);
    assert!(root.status.success());
    for args in [&["--help"][..], &["-h"][..]] {
        let output = fixture.run(args);
        assert!(output.status.success());
        assert_eq!(output.stdout, root.stdout);
    }
    for args in [
        &["--context", "--help"][..],
        &["--context", "--apply", "resource", "--help"][..],
    ] {
        let output = fixture.run(args);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("expected PATH"));
    }
    fixture.assert_empty();
}

#[test]
fn help_does_not_load_or_repair_corrupt_context_records() {
    let fixture = Fixture::new();
    let directory = fixture.0.join(".devmeld");
    fs::create_dir(&directory).unwrap();
    let path = directory.join("context.json");
    fs::write(&path, b"authored invalid configuration").unwrap();
    let modified = fs::metadata(&path).unwrap().modified().unwrap();
    let output = fixture.run(&[
        "--context",
        fixture.0.to_str().unwrap(),
        "resource",
        "add",
        "--help",
    ]);
    assert!(output.status.success());
    assert_eq!(fs::read(&path).unwrap(), b"authored invalid configuration");
    assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), modified);
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);
}

#[test]
fn source_first_registration_publishes_organized_addresses_and_stable_associations() {
    let fixture = Fixture::new();
    fs::write(fixture.0.join("first.md"), "first source").unwrap();
    fs::write(fixture.0.join("second.md"), "second source").unwrap();
    fixture.apply(&["init"]);
    fixture.apply(&[
        "resource",
        "add",
        "first.md",
        "--as",
        "database/test/orders",
    ]);
    fixture.apply(&[
        "resource",
        "add",
        "second.md",
        "--as",
        "database/production/orders",
    ]);
    fixture.apply(&[
        "access",
        "add",
        "database/test/orders",
        "database/production/orders",
    ]);
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(fixture.0.join(".devmeld/context.json")).unwrap())
            .unwrap();
    assert_eq!(config["resources"][0]["id"], "resource-1");
    assert_eq!(config["resources"][0]["path"], "database/test/orders");
    assert_eq!(config["access"][0]["resource"], "resource-1");
    assert_eq!(config["access"][0]["tool"], "resource-2");
    assert_eq!(
        config["groups"]
            .as_array()
            .unwrap()
            .iter()
            .map(|group| group["path"].clone())
            .collect::<serde_json::Value>(),
        serde_json::json!(["database", "database/production", "database/test"])
    );
    assert!(!fixture.0.join(".devmeld/output").exists());
    fixture.apply(&["sync"]);
    let index = fs::read_to_string(fixture.0.join(".devmeld/output/index.md")).unwrap();
    assert!(index.contains("database/test/orders"), "{index}");
    assert!(index.contains("database/production/orders"), "{index}");
    let page = fs::read_to_string(fixture.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("../../first.md"), "{page}");
    assert!(page.contains("r-resource-2.md"), "{page}");
    assert!(page.contains("database/production/orders"), "{page}");
    assert_eq!(
        fs::read(fixture.0.join("first.md")).unwrap(),
        b"first source"
    );
    let before = fs::metadata(fixture.0.join(".devmeld/output/index.md"))
        .unwrap()
        .modified()
        .unwrap();
    fixture.apply(&["sync"]);
    assert_eq!(
        fs::metadata(fixture.0.join(".devmeld/output/index.md"))
            .unwrap()
            .modified()
            .unwrap(),
        before
    );
}

#[test]
fn default_addresses_and_failed_adds_do_not_recycle_or_partially_register() {
    let f = Fixture::new();
    fs::write(f.0.join("团队 notes.md"), "ssh http").unwrap();
    f.apply(&["init"]);
    f.apply(&["resource", "add", "团队 notes.md"]);
    let config_path = f.0.join(".devmeld/context.json");
    let before = fs::read(&config_path).unwrap();
    let config: serde_json::Value = serde_json::from_slice(&before).unwrap();
    assert_eq!(config["resources"][0]["path"], "团队 notes");
    for args in [
        vec!["resource", "add", "团队 notes.md"],
        vec![
            "resource",
            "add",
            "团队 notes.md",
            "--as",
            "团队 notes/child",
        ],
        vec![
            "resource",
            "add",
            "missing.md",
            "--as",
            "new/group/resource",
        ],
        vec![
            "resource",
            "add",
            "团队 notes.md",
            "--schema",
            "missing.json",
        ],
        vec![
            "resource",
            "add",
            "团队 notes.md",
            "--as",
            "one",
            "--as",
            "two",
        ],
        vec!["resource", "add", "团队 notes.md", "--as", "../escape"],
    ] {
        let mut command = vec!["--context", f.0.to_str().unwrap()];
        command.extend(args);
        let output = f.run(&command);
        assert!(!output.status.success(), "{command:?}");
        assert_eq!(fs::read(&config_path).unwrap(), before);
        assert!(!f.0.join(".devmeld/output").exists());
    }
    f.apply(&["resource", "remove", "团队 notes"]);
    f.apply(&[
        "resource",
        "add",
        "团队 notes.md",
        "--as",
        "knowledge/团队 notes",
    ]);
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(config_path).unwrap()).unwrap();
    assert_eq!(config["resources"][0]["id"], "resource-2");
    assert_eq!(config["next_resource_id"], 3);
    assert_eq!(fs::read(f.0.join("团队 notes.md")).unwrap(), b"ssh http");
}

#[test]
fn first_resource_add_and_sync_need_neither_init_nor_context_selector() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "durable source").unwrap();
    let add = f.confirm(&["resource", "add", "notes.md", "--as", "knowledge/notes"]);
    assert!(
        add.status.success(),
        "{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let report = String::from_utf8(add.stdout).unwrap();
    assert!(report.contains("Context:"), "{report}");
    assert!(report.contains("sync"), "{report}");
    assert!(f.0.join(".devmeld/context.json").exists());
    assert!(!f.0.join(".devmeld/output").exists());
    assert!(!f.0.join("AGENTS.md").exists());
    let sync = f.confirm(&["sync"]);
    assert!(
        sync.status.success(),
        "{}",
        String::from_utf8_lossy(&sync.stderr)
    );
    assert!(f.0.join(".devmeld/output/index.md").exists());
    assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"durable source");
    assert!(!f.0.join("AGENTS.md").exists());
}

#[test]
fn descendant_invocations_use_nearest_context_but_resolve_sources_from_cwd() {
    let outer = Fixture::new();
    outer.apply(&["init"]);
    let nested = outer.0.join("nested");
    fs::create_dir(&nested).unwrap();
    let context = Fixture(nested);
    context.apply(&["init"]);
    let child = context.0.join("src");
    fs::create_dir(&child).unwrap();
    let invocation = Fixture(child);
    fs::write(context.0.join("notes.md"), "wrong file at context root").unwrap();
    fs::write(invocation.0.join("notes.md"), "correct source from cwd").unwrap();
    let outer_before = fs::read(outer.0.join(".devmeld/context.json")).unwrap();
    let add = invocation.confirm(&["resource", "add", "notes.md", "--as", "knowledge/notes"]);
    assert!(
        add.status.success(),
        "{}",
        String::from_utf8_lossy(&add.stderr)
    );
    assert!(!invocation.0.join(".devmeld").exists());
    let sync = invocation.confirm(&["sync"]);
    assert!(
        sync.status.success(),
        "{}",
        String::from_utf8_lossy(&sync.stderr)
    );
    let page = fs::read_to_string(context.0.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("../../src/notes.md"), "{page}");
    assert_eq!(
        fs::read(outer.0.join(".devmeld/context.json")).unwrap(),
        outer_before
    );
    assert_eq!(
        fs::read(invocation.0.join("notes.md")).unwrap(),
        b"correct source from cwd"
    );
}

#[test]
fn explicit_new_context_takes_precedence_without_creating_it_during_preview() {
    let f = Fixture::new();
    f.apply(&["init"]);
    fs::write(f.0.join("source.md"), "outside selected context").unwrap();
    let existing = fs::read(f.0.join(".devmeld/context.json")).unwrap();
    let args = [
        "--context",
        "separate/context",
        "resource",
        "add",
        "source.md",
        "--as",
        "knowledge/source",
    ];
    let preview = f.run(&args);
    assert!(
        preview.status.success(),
        "{}",
        String::from_utf8_lossy(&preview.stderr)
    );
    assert!(!f.0.join("separate").exists());
    let add = f.confirm(&args);
    assert!(
        add.status.success(),
        "{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let root = f.0.join("separate/context");
    let config: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join(".devmeld/context.json")).unwrap()).unwrap();
    assert_eq!(config["publication"]["entries"], serde_json::json!([]));
    let source = std::path::Path::new(config["resources"][0]["document"].as_str().unwrap());
    assert_eq!(
        source.canonicalize().unwrap(),
        f.0.join("source.md").canonicalize().unwrap()
    );
    assert_eq!(
        fs::read(f.0.join(".devmeld/context.json")).unwrap(),
        existing
    );
    let sync = f.confirm(&["--context", "separate/context", "sync"]);
    assert!(
        sync.status.success(),
        "{}",
        String::from_utf8_lossy(&sync.stderr)
    );
    assert!(!f.0.join(".devmeld/output").exists());
    let page = fs::read_to_string(root.join(".devmeld/output/r-resource-1.md")).unwrap();
    assert!(page.contains("../../../../source.md"), "{page}");
}

#[test]
fn first_use_dry_run_does_not_create_even_the_requested_root() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    let result = f.run(&[
        "--context",
        "not-created/context",
        "resource",
        "add",
        "notes.md",
        "--dry-run",
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("context.json"));
    assert!(!f.0.join("not-created").exists());
    assert!(!f.0.join(".devmeld").exists());
    assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"source");
}

#[test]
fn invalid_first_operations_leave_sources_and_storage_untouched() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    for args in [
        vec!["resource", "add", "missing.md"],
        vec!["resource", "add", "notes.md", "--as", "../invalid"],
        vec!["resource", "add", "notes.md", "--kind", "unknown"],
        vec!["resource", "add", "notes.md", "--kind", "description"],
        vec!["resource", "add", "https://example.invalid/source"],
        vec!["resource", "add", "notes.md", "--schema", "missing.json"],
        vec!["resource", "add", "notes.md", "--dry-run"], // conflicts with confirm's --apply
        vec!["resource", "remove", "missing"],
        vec!["sync"],
        vec!["unknown"],
    ] {
        let result = f.confirm(&args);
        assert!(!result.status.success(), "{args:?}");
        assert!(!f.0.join(".devmeld").exists(), "{args:?}");
        assert_eq!(fs::read(f.0.join("notes.md")).unwrap(), b"source");
        let mut explicit = vec!["--context", "not-created/context"];
        explicit.extend(args);
        assert!(!f.confirm(&explicit).status.success(), "{explicit:?}");
        assert!(!f.0.join("not-created").exists());
    }
}

#[test]
fn incomplete_corrupt_and_pending_nearest_contexts_block_fallback_and_adoption() {
    for case in [
        "empty",
        "file",
        "corrupt",
        "unowned",
        "missing-config",
        "pending",
    ] {
        let outer = Fixture::new();
        outer.apply(&["init"]);
        let selected = outer.0.join("selected");
        fs::create_dir(&selected).unwrap();
        let inner = Fixture(selected);
        let marker = inner.0.join(".devmeld");
        match case {
            "empty" => fs::create_dir(&marker).unwrap(),
            "file" => fs::write(&marker, "not a context directory").unwrap(),
            "unowned" => {
                fs::create_dir(&marker).unwrap();
                fs::copy(
                    outer.0.join(".devmeld/context.json"),
                    marker.join("context.json"),
                )
                .unwrap();
            }
            _ => {
                inner.apply(&["init"]);
                match case {
                    "corrupt" => fs::write(marker.join("context.json"), "invalid").unwrap(),
                    "missing-config" => fs::remove_file(marker.join("context.json")).unwrap(),
                    "pending" => {
                        fs::write(marker.join("state/pending.json"), "pending recovery").unwrap()
                    }
                    _ => unreachable!(),
                }
            }
        }
        let child = inner.0.join("child");
        fs::create_dir(&child).unwrap();
        let invocation = Fixture(child);
        fs::write(invocation.0.join("notes.md"), "source").unwrap();
        let outer_before = fs::read(outer.0.join(".devmeld/context.json")).unwrap();
        let config_before = fs::read(marker.join("context.json")).ok();
        let receipt_before = fs::read(marker.join("state/owned.json")).ok();
        for args in [
            vec!["resource", "add", "notes.md"],
            vec!["init"],
            vec!["sync"],
        ] {
            let result = invocation.confirm(&args);
            assert!(!result.status.success(), "{case}: {args:?}");
            assert!(!invocation.0.join(".devmeld").exists());
            assert_eq!(fs::read(marker.join("context.json")).ok(), config_before);
            assert_eq!(
                fs::read(marker.join("state/owned.json")).ok(),
                receipt_before
            );
            assert_eq!(
                fs::read(outer.0.join(".devmeld/context.json")).unwrap(),
                outer_before
            );
        }
        if case == "pending" {
            assert_eq!(
                fs::read(marker.join("state/pending.json")).unwrap(),
                b"pending recovery"
            );
        }
        assert_eq!(fs::read(invocation.0.join("notes.md")).unwrap(), b"source");
    }
}

#[test]
fn a_new_marker_after_bootstrap_preview_is_not_adopted() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "source").unwrap();
    let plan = devmeld::prepare_in(
        &f.0,
        None,
        &["resource".into(), "add".into(), "notes.md".into()],
    )
    .unwrap();
    fs::create_dir(f.0.join(".devmeld")).unwrap();
    fs::write(f.0.join(".devmeld/outside.txt"), "external state").unwrap();
    let result = plan.apply();
    assert!(
        result.is_err(),
        "bootstrap adopted a marker that appeared after preview"
    );
    assert!(!f.0.join(".devmeld/context.json").exists());
    assert!(!f.0.join(".devmeld/state").exists());
    assert_eq!(
        fs::read(f.0.join(".devmeld/outside.txt")).unwrap(),
        b"external state"
    );
}

#[test]
fn stale_first_source_aborts_before_creating_context_storage() {
    let f = Fixture::new();
    let source = f.0.join("notes.md");
    fs::write(&source, "before").unwrap();
    let plan = devmeld::prepare_in(
        &f.0,
        None,
        &["resource".into(), "add".into(), "notes.md".into()],
    )
    .unwrap();
    fs::write(&source, "after").unwrap();
    assert!(
        plan.apply()
            .unwrap_err()
            .to_string()
            .contains("stale preview")
    );
    assert!(!f.0.join(".devmeld").exists());
    assert_eq!(fs::read(source).unwrap(), b"after");
}

#[cfg(windows)]
#[test]
fn inaccessible_nearest_config_blocks_fallback() {
    use std::os::windows::fs::OpenOptionsExt;
    let f = Fixture::new();
    f.apply(&["init"]);
    let path = f.0.join(".devmeld/context.json");
    let before = fs::read(&path).unwrap();
    let blocked = fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&path)
        .unwrap();
    let child = f.0.join("child");
    fs::create_dir(&child).unwrap();
    let invocation = Fixture(child);
    fs::write(invocation.0.join("notes.md"), "source").unwrap();
    let result = invocation.confirm(&["resource", "add", "notes.md"]);
    assert!(!result.status.success());
    assert!(!invocation.0.join(".devmeld").exists());
    drop(blocked);
    assert_eq!(fs::read(path).unwrap(), before);
}

#[test]
fn schema_entry_and_output_operands_use_cwd_while_description_references_use_source_directory() {
    let f = Fixture::new();
    f.apply(&["init"]);
    let child = f.0.join("child");
    fs::create_dir(&child).unwrap();
    let invocation = Fixture(child);
    fs::write(invocation.0.join("service.json"), r#"{"title":"HTTP service","summary":"source facts","attributes":{"environment":"test"},"references":[{"label":"guide","path":"guide.md"}]}"#).unwrap();
    fs::write(
        invocation.0.join("schema.json"),
        r#"{"type":"object","required":["environment"]}"#,
    )
    .unwrap();
    fs::write(invocation.0.join("guide.md"), "authored guide").unwrap();
    fs::write(invocation.0.join("AGENTS.md"), "author rules").unwrap();
    let add = invocation.confirm(&[
        "resource",
        "add",
        "service.json",
        "--kind",
        "description",
        "--schema",
        "schema.json",
    ]);
    assert!(
        add.status.success(),
        "{}",
        String::from_utf8_lossy(&add.stderr)
    );
    for args in [
        vec!["entry", "add", "AGENTS.md", "--kind", "instructions"],
        vec!["output", "generated"],
        vec!["sync"],
    ] {
        let result = invocation.confirm(&args);
        assert!(
            result.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    assert!(!f.0.join("AGENTS.md").exists());
    assert!(!f.0.join("generated").exists());
    let page = fs::read_to_string(invocation.0.join("generated/r-resource-1.md")).unwrap();
    assert!(page.contains("../service.json"), "{page}");
    assert!(page.contains("../guide.md"), "{page}");
    assert!(
        fs::read_to_string(invocation.0.join("AGENTS.md"))
            .unwrap()
            .ends_with("author rules")
    );
    for args in [vec!["entry", "remove", "AGENTS.md"], vec!["sync"]] {
        let result = invocation.confirm(&args);
        assert!(
            result.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    assert_eq!(
        fs::read(invocation.0.join("AGENTS.md")).unwrap(),
        b"author rules"
    );
    assert_eq!(
        fs::read(invocation.0.join("guide.md")).unwrap(),
        b"authored guide"
    );
}
