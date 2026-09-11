//! Fixture compatibility only: exercise the real terminal parser, then the typed application.
//! No string-based entrypoint is exported by the production library.
#![allow(dead_code)]
#[path = "../../src/cli/mod.rs"]
mod cli;
use std::path::Path;

/// Reproduce the former TOML receipt using genuine, unchanged fixture targets.
/// Test setup only; production must never derive old claims from current files.
pub fn full_body_receipt(root: &Path) -> Vec<u8> {
    let path = root.join(".devmeld/state/owned.toml");
    let mut receipt: toml::Value = toml::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    for (target, claim) in receipt["surfaces"].as_table_mut().unwrap() {
        if claim["kind"].as_str() == Some("whole_file") {
            let observed = claim["observed"].as_table_mut().unwrap();
            observed.remove("sha256");
            observed.insert(
                "bytes".into(),
                std::fs::read_to_string(target).unwrap().into(),
            );
        }
    }
    let bytes = toml::to_string_pretty(&receipt).unwrap().into_bytes();
    std::fs::write(path, &bytes).unwrap();
    bytes
}

pub fn prepare(root: &Path, args: &[String]) -> devmeld::Result<devmeld::Plan> {
    prepare_in(root, Some(root), args)
}
pub fn prepare_in(
    base: &Path,
    explicit: Option<&Path>,
    args: &[String],
) -> devmeld::Result<devmeld::Plan> {
    let invocation = cli::Invocation::parse(args)?;
    let location = devmeld::ContextLocation {
        base_directory: base.into(),
        directory: explicit.map(Path::to_path_buf).or(invocation.context),
    };
    match invocation.request {
        cli::Operation::Mutate(request) => devmeld::prepare(&location, request),
        cli::Operation::Query(_) => Err(devmeld::error("test expected a mutation")),
    }
}
pub fn inspect_in(
    base: &Path,
    explicit: Option<&Path>,
    args: &[String],
) -> devmeld::Result<Option<String>> {
    let invocation = cli::Invocation::parse(args)?;
    let location = devmeld::ContextLocation {
        base_directory: base.into(),
        directory: explicit.map(Path::to_path_buf).or(invocation.context),
    };
    match invocation.request {
        cli::Operation::Query(query) => {
            devmeld::inspect(&location, query).map(|value| Some(cli::render::inspection(&value)))
        }
        cli::Operation::Mutate(_) => Ok(None),
    }
}
