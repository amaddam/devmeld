use std::io::{self, Write};

fn run() -> devmeld::Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args == ["--help"] {
        println!(
            "devmeld --context PATH init|resource|access|entry|output|sync|recover [--apply]\nCommands preview by default. --apply asks for explicit confirmation."
        );
        return Ok(());
    }
    let apply = if args.last().is_some_and(|a| a == "--apply") {
        args.pop();
        true
    } else {
        false
    };
    if args.len() < 3 || args[0] != "--context" {
        return Err(devmeld::error(
            "expected --context PATH COMMAND; see --help",
        ));
    }
    let plan = devmeld::prepare(std::path::Path::new(&args[1]), &args[2..])?;
    print!("{}", plan.preview());
    if apply && !plan.is_empty() {
        print!("Type apply to confirm: ");
        io::stdout().flush()?;
        let mut reply = String::new();
        io::stdin().read_line(&mut reply)?;
        if reply.trim() != "apply" {
            return Err(devmeld::error("cancelled; no managed changes applied"));
        }
        plan.apply()?;
        println!("Applied.");
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("DevMeld: {error}");
        std::process::exit(2);
    }
}
