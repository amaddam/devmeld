//! Check declared Cargo edges, including inactive target/build/dev dependencies.
use crate::{Result, cargo};
use serde_json::Value;
use std::path::Path;

pub(super) fn check(root: &Path) -> Result<()> {
    let result = cargo(
        root,
        &[
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
            "--locked",
        ],
    )
    .output()?;
    if !result.status.success() {
        return Err(format!(
            "Cargo metadata failed: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }
    validate(&serde_json::from_slice(&result.stdout)?)?;
    println!("PASS domain dependencies: resources and publication use std only");
    Ok(())
}
fn validate(metadata: &Value) -> Result<()> {
    let packages = metadata["packages"]
        .as_array()
        .ok_or("metadata packages missing")?;
    let members = metadata["workspace_members"]
        .as_array()
        .ok_or("metadata workspace_members missing")?;
    for name in ["devmeld-resources", "devmeld-publication"] {
        let package = packages
            .iter()
            .find(|p| p["name"] == name && members.contains(&p["id"]))
            .ok_or_else(|| format!("reviewed domain package missing: {name}"))?;
        let dependencies = package["dependencies"]
            .as_array()
            .ok_or("metadata dependencies missing")?;
        if !dependencies.is_empty() {
            return Err(format!(
                "{name} must remain std-only; declared dependency: {}",
                dependencies[0]["name"]
            )
            .into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "devmeld-boundaries-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn write(&self, path: &str, text: &str) {
            let path = self.0.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn real_metadata_checks_inactive_renamed_optional_build_and_dev_edges() {
        let f = Fixture::new();
        f.write(
            "Cargo.toml",
            "[workspace]\nmembers = ['resources', 'publication', 'adapter']\nresolver = '3'\n",
        );
        for (folder, name) in [
            ("resources", "devmeld-resources"),
            ("publication", "devmeld-publication"),
            ("adapter", "adapter"),
        ] {
            f.write(
                &format!("{folder}/Cargo.toml"),
                &format!("[package]\nname = '{name}'\nversion = '0.1.0'\nedition = '2024'\n"),
            );
            f.write(&format!("{folder}/src/lib.rs"), "pub struct Control;\n");
        }
        assert!(
            cargo(&f.0, &["generate-lockfile", "--offline"])
                .output()
                .unwrap()
                .status
                .success()
        );
        check(&f.0).unwrap();
        for section in [
            "dependencies",
            "build-dependencies",
            "dev-dependencies",
            "target.'cfg(target_os = \"none\")'.dependencies",
        ] {
            let optional = if section == "dependencies" {
                ", optional = true"
            } else {
                ""
            };
            f.write("resources/Cargo.toml", &format!("[package]\nname = 'devmeld-resources'\nversion = '0.1.0'\nedition = '2024'\n[{section}]\nrenamed = {{package = 'adapter', path = '../adapter'{optional}}}\n"));
            assert!(
                cargo(&f.0, &["generate-lockfile", "--offline"])
                    .output()
                    .unwrap()
                    .status
                    .success()
            );
            assert!(check(&f.0).unwrap_err().to_string().contains("std-only"));
        }
    }

    #[test]
    fn resource_identity_has_a_valid_constructor_but_no_public_unchecked_field() {
        let f = Fixture::new();
        let domain = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/resources")
            .canonicalize()
            .unwrap();
        let raw = domain.to_string_lossy();
        let path = raw.strip_prefix(r"\\?\").unwrap_or(&raw).replace('\\', "/");
        f.write("Cargo.toml", &format!("[package]\nname = 'visibility-probe'\nversion = '0.1.0'\nedition = '2024'\n[dependencies]\ndevmeld-resources = {{path = '{path}'}}\n"));
        f.write("src/main.rs", "fn main() { let id = devmeld_resources::ResourceId::new(\"valid\").unwrap(); assert_eq!(id.as_str(), \"valid\"); }\n");
        let control = cargo(&f.0, &["check", "--offline"]).output().unwrap();
        assert!(
            control.status.success(),
            "{}",
            String::from_utf8_lossy(&control.stderr)
        );
        f.write(
            "src/main.rs",
            "fn main() { let _ = devmeld_resources::ResourceId(String::from(\"unchecked\")); }\n",
        );
        let forbidden = cargo(&f.0, &["check", "--offline"]).output().unwrap();
        let stderr = String::from_utf8_lossy(&forbidden.stderr);
        assert!(
            !forbidden.status.success() && stderr.contains("E0603") && stderr.contains("private"),
            "{stderr}"
        );
    }
}
