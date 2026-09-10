//! Fixture compatibility only: exercise the real terminal parser, then the typed application.
//! No string-based entrypoint is exported by the production library.
#![allow(dead_code)]
#[path = "../../src/cli/mod.rs"]
mod cli;
use std::path::Path;

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
