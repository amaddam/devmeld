use std::io::{self, Write};

fn run() -> devmeld::Result<()> {
    let mut args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|arg| {
            arg.into_string()
                .map_err(|_| devmeld::error("command arguments must be Unicode"))
        })
        .collect::<devmeld::Result<_>>()?;
    if args.is_empty() || args == ["--help"] {
        println!(
            "devmeld --context PATH COMMAND [--apply]\n\nCommands:\n  init [--output PATH] [--entry PATH]\n  resource add ID --document PATH\n  resource add ID --description PATH [--schema PATH]\n  resource remove ID\n  access add RESOURCE TOOL\n  access remove RESOURCE TOOL\n  entry add PATH\n  entry remove PATH\n  output PATH\n  sync\n  recover\n\nPaths resolve from the selected existing context root.\nCommands preview by default; --apply prints the preview then asks you to type apply.\nGenerated files remain readable without DevMeld running. No tools are executed or installed."
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
