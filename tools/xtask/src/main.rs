//! Developer-only verification. Never linked into a DevMeld core library.
mod architecture;
mod probes;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn cargo(root: &Path, arguments: &[&str]) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(root).args(arguments);
    command
}

fn successful(output: Output, purpose: &str) -> Result<Output> {
    if !output.status.success() {
        return Err(format!(
            "{purpose}: {}\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(output)
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
        Some("architecture") => architecture::check(&root),
        Some("test-architecture") => probes::run(&root),
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
            architecture::check(&root)?;
            probes::run(&root)
        }
        Some("help" | "--help" | "-h") => {
            println!(
                "cargo xtask <check|architecture|test-architecture> [--root <workspace>]\ncheck runs all quality gates; requires the pinned Rust toolchain and fetched locked dependencies, not a shell or Python."
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
