use std::io::{self, Write};

mod cli;

fn run() -> devmeld::Result<()> {
    let mut args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|arg| {
            arg.into_string()
                .map_err(|_| devmeld::error("command arguments must be Unicode"))
        })
        .collect::<devmeld::Result<_>>()?;
    if let Some(help) = cli::help(&args)? {
        print!("{help}");
        return Ok(());
    }
    let apply = match args.last().map(String::as_str) {
        Some("--apply") => {
            args.pop();
            true
        }
        Some("--dry-run") => {
            args.pop();
            false
        }
        _ => false,
    };
    if args
        .iter()
        .any(|a| matches!(a.as_str(), "--apply" | "--dry-run"))
    {
        return Err(devmeld::error(
            "use one terminal --apply OR --dry-run option; see --help",
        ));
    }
    let (explicit, command) = if args.first().is_some_and(|a| a == "--context") {
        if args.len() < 3 || args[1].starts_with('-') {
            return Err(devmeld::error(
                "expected --context PATH COMMAND; see --help",
            ));
        }
        (Some(std::path::Path::new(&args[1])), &args[2..])
    } else {
        (None, args.as_slice())
    };
    let cwd = std::env::current_dir()?;
    if let Some(report) = devmeld::inspect_in(&cwd, explicit, command)? {
        if apply {
            return Err(devmeld::error("read-only commands do not accept --apply"));
        }
        print!("{report}");
        return Ok(());
    }
    let plan = devmeld::prepare_in(&cwd, explicit, command)?;
    println!("Context: {}", plan.context_root().display());
    print!("{}", plan.preview());
    if apply && plan.is_empty() {
        plan.apply()?;
    } else if apply {
        print!("Type apply to confirm: ");
        io::stdout().flush()?;
        let mut reply = String::new();
        io::stdin().read_line(&mut reply)?;
        if reply.trim() != "apply" {
            return Err(devmeld::error("cancelled; no managed changes applied"));
        }
        plan.apply()?;
        println!("Applied.");
        if !matches!(command, [name] if name == "sync" || name == "recover") {
            println!(
                "Configuration saved; publication may be pending. Run sync to inspect and publish."
            );
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("DevMeld: {error}");
        std::process::exit(2);
    }
}
