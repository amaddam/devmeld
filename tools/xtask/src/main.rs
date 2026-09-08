//! Cross-platform developer checks. This is not a product entrypoint.

use std::path::{Path, PathBuf};
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn cargo(root: &Path, arguments: &[&str]) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(root).args(arguments);
    command
}

fn run() -> Result<()> {
    let mut arguments = std::env::args_os().skip(1);
    let operation = arguments.next().unwrap_or_else(|| "help".into());
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    if let Some(flag) = arguments.next() {
        if flag != "--root" {
            return Err("expected --root <workspace>".into());
        }
        root = arguments
            .next()
            .ok_or("--root needs a workspace path")?
            .into();
    }
    if arguments.next().is_some() {
        return Err("unexpected extra argument".into());
    }
    let root = root.canonicalize()?;
    match operation.to_str() {
        Some("check") => {
            for arguments in [
                vec!["fmt", "--all", "--", "--check"],
                vec![
                    "check",
                    "--workspace",
                    "--all-targets",
                    "--locked",
                    "--offline",
                ],
                vec![
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--locked",
                    "--offline",
                ],
                vec!["test", "--workspace", "--locked", "--offline"],
            ] {
                println!("RUN cargo {}", arguments.join(" "));
                let status = cargo(&root, &arguments).status()?;
                if !status.success() {
                    return Err(format!("Cargo gate failed: {status}").into());
                }
            }
            Ok(())
        }
        Some("help" | "--help" | "-h") => {
            println!(
                "cargo xtask <check> [--root <workspace>]\nRuns fmt, check, clippy and test with the existing Rust toolchain, without a shell or Python. No domain architecture gate is configured during the design reset."
            );
            Ok(())
        }
        _ => Err("unknown xtask operation; run cargo xtask help".into()),
    }
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
