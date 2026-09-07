use crate::{Result, architecture, cargo, successful};
use serde_json::Value;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct Fixtures {
    root: PathBuf,
    parent: PathBuf,
    source: PathBuf,
}
impl Fixtures {
    fn new(source: &Path) -> Result<Self> {
        architecture::reject_redirects(&source.join("crates"))?;
        architecture::reject_redirects(&source.join("tools"))?;
        let parent = std::env::temp_dir().canonicalize()?;
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        // Every probe exercises spaces and Unicode in real process arguments.
        let root = parent.join(format!(
            "devmeld architecture-{}-{nonce}-检查",
            std::process::id()
        ));
        fs::create_dir(&root)?;
        Ok(Self {
            root,
            parent,
            source: source.to_owned(),
        })
    }
    fn workspace(&self, name: &str) -> Result<PathBuf> {
        let path = self.root.join(name);
        if path.parent() != Some(self.root.as_path()) {
            return Err("fixture name must be one component".into());
        }
        fs::create_dir(&path)?;
        for file in ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml"] {
            fs::copy(self.source.join(file), path.join(file))?;
        }
        for directory in ["crates", "tools"] {
            copy_tree(&self.source.join(directory), &path.join(directory))?;
        }
        Ok(path)
    }
}
impl Drop for Fixtures {
    fn drop(&mut self) {
        let cleanup = || -> Result<()> {
            let resolved = self.root.canonicalize()?;
            if self.root.parent() != Some(self.parent.as_path())
                || resolved.parent() != Some(self.parent.as_path())
                || resolved.file_name() != self.root.file_name()
            {
                return Err("refusing cleanup outside the owned temporary directory".into());
            }
            fs::remove_dir_all(&self.root)?;
            Ok(())
        };
        if let Err(error) = cleanup() {
            eprintln!(
                "Fixture cleanup failed, retained {}: {error}",
                self.root.display()
            );
        }
    }
}
fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
fn write(root: &Path, relative: &str, content: &str) -> Result<()> {
    let relative = Path::new(relative);
    if relative
        .components()
        .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err("refusing fixture write outside its root".into());
    }
    let path = root.join(relative);
    fs::create_dir_all(path.parent().ok_or("fixture file needs parent")?)?;
    fs::write(path, content)?;
    Ok(())
}
fn append(root: &Path, relative: &str, content: &str) -> Result<()> {
    write(
        root,
        relative,
        &format!("{}\n{content}\n", fs::read_to_string(root.join(relative))?),
    )
}
fn replace(root: &Path, relative: &str, old: &str, new: &str) -> Result<()> {
    let content = fs::read_to_string(root.join(relative))?;
    if !content.contains(old) {
        return Err(format!("fixture mutation precondition missing: {old}").into());
    }
    write(root, relative, &content.replacen(old, new, 1))
}
fn edge(root: &Path, owner: &str, table: &str, declaration: &str) -> Result<()> {
    let manifest = format!("crates/{owner}/Cargo.toml");
    let content = fs::read_to_string(root.join(&manifest))?;
    let header = format!("[{table}]");
    if content.contains(&header) {
        replace(
            root,
            &manifest,
            &header,
            &format!("{header}\n{declaration}"),
        )
    } else {
        append(root, &manifest, &format!("{header}\n{declaration}"))
    }
}
fn fixture_cargo(root: &Path, arguments: &[&str]) -> Result<std::process::Output> {
    Ok(cargo(root, arguments)
        .arg("--manifest-path")
        .arg(root.join("Cargo.toml"))
        .arg("--offline")
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()?)
}
fn lock(root: &Path) -> Result<()> {
    successful(
        fixture_cargo(root, &["generate-lockfile"])?,
        "fixture lock generation (not a passing rejection)",
    )?;
    Ok(())
}
fn intended_diagnostic(bytes: &[u8], code: &str, text: &str) -> Result<bool> {
    for line in bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
    {
        let message: Value = serde_json::from_slice(line)?;
        if message["reason"] == "compiler-message"
            && message["message"]["level"] == "error"
            && message["message"]["code"]["code"] == code
            && message["message"]["message"]
                .as_str()
                .is_some_and(|m| m.contains(text))
        {
            return Ok(true);
        }
    }
    Ok(false)
}
struct Harness {
    fixtures: Fixtures,
    passed: usize,
}
impl Harness {
    fn pass(&mut self, name: &str, reason: &str) {
        self.passed += 1;
        println!("PASS {name} ({reason})");
    }
    fn graph(
        &mut self,
        name: &str,
        mutation: impl FnOnce(&Path) -> Result<()>,
        expected: &str,
    ) -> Result<()> {
        let path = self.fixtures.workspace(name)?;
        mutation(&path)?;
        lock(&path)?;
        // Execute this compiled real checker with an explicit root, not a
        // second implementation or shell wrapper. Copied source stays intact.
        let output = Command::new(std::env::current_exe()?)
            .arg("architecture")
            .arg("--root")
            .arg(&path)
            .current_dir(&path)
            .env("CARGO_TARGET_DIR", path.join("target"))
            .output()?;
        let text = format!(
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let success = expected == "PASS";
        if output.status.success() != success || !text.contains(expected) {
            return Err(format!("{name}: missing intended {expected}: {text}").into());
        }
        self.pass(name, expected);
        Ok(())
    }
    fn failure<T>(&mut self, name: &str, result: Result<T>, expected: &str) -> Result<()> {
        let Err(error) = result else {
            return Err(format!("{name}: unexpectedly succeeded").into());
        };
        if !error.to_string().contains(expected) {
            return Err(format!("{name}: wrong failure, expected {expected}: {error}").into());
        }
        self.pass(name, expected);
        Ok(())
    }
    fn consumer(
        &mut self,
        path: &Path,
        name: &str,
        code: &str,
        expected: Option<(&str, &str)>,
    ) -> Result<()> {
        write(path, "src/lib.rs", code)?;
        let output = fixture_cargo(
            path,
            &["check", "--lib", "--locked", "--message-format=json"],
        )?;
        if let Some((diagnostic, text)) = expected {
            if output.status.success() || !intended_diagnostic(&output.stdout, diagnostic, text)? {
                return Err(format!(
                    "{name}: missing intended {diagnostic}\n{}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                )
                .into());
            }
            self.pass(name, diagnostic);
        } else {
            successful(output, "valid consumer control")?;
            self.pass(name, "PASS");
        }
        Ok(())
    }
}

pub fn run(root: &Path) -> Result<()> {
    let actual = architecture::metadata(root)?;
    architecture::validate(root, &actual)?;
    let mut h = Harness {
        fixtures: Fixtures::new(root)?,
        passed: 0,
    };
    h.graph("real-graph-with-allowed-dev-edges", |_| Ok(()), "PASS")?;
    for (name, owner, table, declaration) in [
        (
            "core-normal",
            "catalog",
            "dependencies",
            "devmeld-local-context = { path = \"../local-context\" }",
        ),
        (
            "kernel-outward",
            "shared-kernel",
            "dev-dependencies",
            "devmeld-catalog = { path = \"../catalog\" }",
        ),
        (
            "build-edge",
            "catalog",
            "build-dependencies",
            "devmeld-local-context = { path = \"../local-context\" }",
        ),
        (
            "optional-edge",
            "catalog",
            "dependencies",
            "devmeld-local-context = { path = \"../local-context\", optional = true }",
        ),
        (
            "inactive-target-edge",
            "catalog",
            "target.'cfg(target_os = \"none\")'.dependencies",
            "devmeld-local-context = { path = \"../local-context\" }",
        ),
        (
            "renamed-edge",
            "catalog",
            "dependencies",
            "hidden = { package = \"devmeld-local-context\", path = \"../local-context\" }",
        ),
        (
            "core-to-tool",
            "catalog",
            "dependencies",
            "xtask = { path = \"../../tools/xtask\" }",
        ),
        (
            "tool-library-leak",
            "catalog",
            "dependencies",
            "serde_json = \"1.0\"",
        ),
    ] {
        h.graph(name, |p| edge(p, owner, table, declaration), "ARCH_EDGE")?;
    }
    for (name, declaration) in [
        (
            "additional-target-kernel",
            "devmeld-shared-kernel = { path = \"../shared-kernel\" }",
        ),
        (
            "additional-optional-target-kernel",
            "devmeld-shared-kernel = { path = \"../shared-kernel\", optional = true }",
        ),
    ] {
        h.graph(
            name,
            |p| {
                edge(
                    p,
                    "catalog",
                    "target.'cfg(target_os = \"none\")'.dependencies",
                    declaration,
                )
            },
            "ARCH_EDGE",
        )?;
    }
    h.graph(
        "renamed-kernel",
        |p| {
            replace(
                p,
                "crates/catalog/Cargo.toml",
                "devmeld-shared-kernel = { path = \"../shared-kernel\" }",
                "hidden = { package = \"devmeld-shared-kernel\", path = \"../shared-kernel\" }",
            )
        },
        "ARCH_EDGE",
    )?;
    for role in ["adapter", "entrypoint"] {
        h.graph(&format!("core-to-{role}"), |p| {
            replace(p, "Cargo.toml", "[workspace]", &format!("[workspace]\nexclude = [\"fixtures/{role}\"]"))?;
            write(p, &format!("fixtures/{role}/Cargo.toml"), &format!("[package]\nname = \"fixture-{role}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"))?;
            write(p, &format!("fixtures/{role}/src/lib.rs"), "pub fn fixture() {}")?;
            edge(p, "catalog", "dependencies", &format!("fixture-{role} = {{ path = \"../../fixtures/{role}\" }}"))
        }, "ARCH_EDGE")?;
    }
    for (name, path, code, expected) in [
        (
            "build-script",
            "crates/catalog/build.rs",
            "fn main() {}",
            "ARCH_TARGET",
        ),
        (
            "production-binary",
            "crates/catalog/src/main.rs",
            "fn main() {}",
            "ARCH_TARGET",
        ),
        (
            "supporting-domain-placeholder",
            "crates/capability-integration/src/lib.rs",
            "",
            "ARCH_SCOPE",
        ),
        (
            "speculative-application-layer",
            "crates/catalog/src/application/mod.rs",
            "",
            "ARCH_SCOPE",
        ),
        (
            "out-of-tree-source",
            "crates/catalog/hidden.rs",
            "",
            "ARCH_SCOPE",
        ),
        ("root-facade", "src/lib.rs", "", "ARCH_SCOPE"),
    ] {
        h.graph(name, |p| write(p, path, code), expected)?;
    }
    h.graph(
        "extra-member",
        |p| {
            replace(
                p,
                "Cargo.toml",
                "members = [",
                "members = [\"crates/extra\", ",
            )?;
            write(
                p,
                "crates/extra/Cargo.toml",
                "[package]\nname = \"extra\"\nversion = \"0.1.0\"\n",
            )?;
            write(p, "crates/extra/src/lib.rs", "")
        },
        "ARCH_MEMBERS",
    )?;
    h.graph(
        "tool-to-core",
        |p| {
            append(
                p,
                "tools/xtask/Cargo.toml",
                "[dev-dependencies]\ndevmeld-catalog = { path = \"../../crates/catalog\" }",
            )
        },
        "ARCH_EDGE",
    )?;
    h.failure(
        "malformed-json",
        architecture::parse(b"{bad"),
        "ARCH_METADATA_JSON",
    )?;
    h.failure(
        "failed-native-command",
        architecture::metadata(&h.fixtures.root),
        "ARCH_METADATA_COMMAND",
    )?;
    for (name, expected) in [
        ("unknown-kind", "ARCH_KIND"),
        ("external-path", "ARCH_PATH"),
        ("unknown-identity", "ARCH_MEMBERS"),
        ("missing-schema", "ARCH_SCHEMA"),
        ("malformed-optional", "ARCH_SCHEMA"),
    ] {
        let mut copy = actual.clone();
        let catalog = copy["packages"]
            .as_array_mut()
            .ok_or("package array")?
            .iter_mut()
            .find(|p| p["name"] == "devmeld-catalog")
            .ok_or("catalog metadata")?;
        match name {
            "unknown-kind" => catalog["dependencies"][0]["kind"] = "future".into(),
            "external-path" => {
                catalog["dependencies"][0]["path"] =
                    h.fixtures.root.to_string_lossy().as_ref().into()
            }
            "unknown-identity" => catalog["id"] = "unknown opaque identity".into(),
            "malformed-optional" => catalog["dependencies"][0]["optional"] = "false".into(),
            "missing-schema" => {
                copy.as_object_mut()
                    .ok_or("metadata object")?
                    .remove("packages");
            }
            _ => unreachable!(),
        }
        h.failure(name, architecture::validate(root, &copy), expected)?;
    }

    let workspace = h.fixtures.workspace("compiler")?;
    let consumer = workspace.join("consumer");
    write(
        &consumer,
        "Cargo.toml",
        r#"[workspace]
[package]
name = "boundary-consumer"
version = "0.1.0"
edition = "2024"
[dependencies]
devmeld-shared-kernel = { path = "../crates/shared-kernel" }
devmeld-catalog = { path = "../crates/catalog" }
devmeld-local-context = { path = "../crates/local-context" }
devmeld-knowledge = { path = "../crates/knowledge" }
"#,
    )?;
    write(&consumer, "src/lib.rs", "")?;
    lock(&consumer)?;
    h.consumer(&consumer, "valid-public-consumer", r#"
use devmeld_shared_kernel::{RepositoryId, ResourceId};
use devmeld_local_context::{TaskContext, ValidatedTaskContext, Resolution};
pub fn identities() { let _ = RepositoryId::new("repo").unwrap(); let _ = ResourceId::new("res").unwrap(); }
pub fn context() -> ValidatedTaskContext { TaskContext::default().validate(vec![], vec![], vec![], vec![]).unwrap() }
pub fn catalog() { let _ = devmeld_catalog::WorkspaceId::new("ws").unwrap(); }
pub fn knowledge() { let _ = devmeld_knowledge::Scope::new(Default::default()).unwrap(); }
pub fn outcomes(value: Resolution) { match value { Resolution::Resolved(_) => (), Resolution::Ambiguous(_) => (), Resolution::Unavailable(_) => () } }
"#, None)?;
    for (name, code, diagnostic, text) in [
        (
            "foreign-private-module",
            "use devmeld_catalog::workspace::Workspace;",
            "E0603",
            "private",
        ),
        (
            "private-identity-field",
            "pub fn alter(mut id: devmeld_shared_kernel::RepositoryId) { id.0 = String::new(); }",
            "E0616",
            "private",
        ),
        (
            "identity-kind-mismatch",
            "pub fn wrong(id: devmeld_shared_kernel::ResourceId) -> devmeld_shared_kernel::RepositoryId { id }",
            "E0308",
            "mismatched",
        ),
        (
            "fabricated-context",
            "pub fn fabricate() { let _ = devmeld_local_context::ValidatedTaskContext::default(); }",
            "E0599",
            "default",
        ),
        (
            "raw-context",
            "fn needs(_: &devmeld_local_context::ValidatedTaskContext) {} pub fn raw() { needs(&devmeld_local_context::TaskContext::default()); }",
            "E0308",
            "mismatched",
        ),
        (
            "mutable-snapshot-escape",
            "pub fn alter(ctx: &mut devmeld_local_context::ValidatedTaskContext) { ctx.observations().clear(); }",
            "E0596",
            "mutable",
        ),
        (
            "second-snapshot-argument",
            "pub fn replace(ctx: &devmeld_local_context::ValidatedTaskContext, repo: &devmeld_shared_kernel::RepositoryId) { let _ = ctx.resolve(repo, &[]); }",
            "E0061",
            "argument",
        ),
        (
            "missing-resolution-variant",
            "pub fn missing(r: devmeld_local_context::Resolution) { match r { devmeld_local_context::Resolution::Resolved(_) => () } }",
            "E0004",
            "non-exhaustive",
        ),
    ] {
        h.consumer(&consumer, name, code, Some((diagnostic, text)))?;
    }
    for member in architecture::CORES {
        let path = h.fixtures.workspace(&format!("lint-{member}"))?;
        let package = format!("devmeld-{member}");
        successful(
            fixture_cargo(
                &path,
                &["check", "--lib", "--locked", "--package", &package],
            )?,
            "unsafe probe control",
        )?;
        append(
            &path,
            &format!("crates/{member}/src/lib.rs"),
            "pub fn unsafe_policy_probe() { unsafe {} }",
        )?;
        let output = fixture_cargo(
            &path,
            &[
                "check",
                "--lib",
                "--locked",
                "--package",
                &package,
                "--message-format=json",
            ],
        )?;
        if output.status.success() || !intended_diagnostic(&output.stdout, "unsafe_code", "unsafe")?
        {
            return Err(format!("{member} failed without intended unsafe diagnostic").into());
        }
        h.pass(&format!("unsafe-{member}"), "unsafe_code");
    }
    // Guard the guard: path traversal is a fixture error, never a write.
    h.failure(
        "fixture-write-escape",
        write(&h.fixtures.root, "../escape", "never written"),
        "refusing fixture write",
    )?;
    println!(
        "Architecture verification: {} probes passed; every fixture path includes spaces and Unicode. Semantic purity still requires review.",
        h.passed
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arbitrary_failed_build_is_not_a_compiler_boundary_pass() {
        assert!(
            !intended_diagnostic(
                br#"{"reason":"build-finished","success":false}"#,
                "E0603",
                "private"
            )
            .unwrap()
        );
        assert!(intended_diagnostic(b"not JSON", "E0603", "private").is_err());
    }
    #[test]
    fn diagnostic_code_and_text_must_both_match() {
        let message = br#"{"reason":"compiler-message","message":{"level":"error","code":{"code":"E0603"},"message":"module is private"}}"#;
        assert!(intended_diagnostic(message, "E0603", "private").unwrap());
        assert!(!intended_diagnostic(message, "E0308", "private").unwrap());
        assert!(!intended_diagnostic(message, "E0603", "mismatched").unwrap());
    }
}
