use std::io::{self, BufRead, IsTerminal, Write};

mod cli;

fn run() -> devmeld::Result<()> {
    let args: Vec<String> = std::env::args_os()
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
    let invocation = cli::Invocation::parse(&args)?;
    let explicit = invocation.context;
    let command = &invocation.command;
    let cwd = std::env::current_dir()?;
    if let Some(report) = devmeld::inspect_in(&cwd, explicit, command)? {
        print!("{report}");
        return Ok(());
    }
    let plan = devmeld::prepare_in(&cwd, explicit, command)?;
    println!("Context: {}", plan.context_root().display());
    print!("{}", plan.preview());
    io::stdout().flush()?;
    if invocation.dry_run {
        return Ok(());
    }
    if invocation.needs_confirmation() && !plan.is_empty() && !invocation.yes {
        let input = io::stdin();
        let output = io::stdout();
        if !confirm(
            &mut input.lock(),
            &mut output.lock(),
            input.is_terminal() && output.is_terminal(),
        )? {
            return Err(devmeld::error("cancelled; no managed changes applied"));
        }
    }
    plan.apply()?;
    println!(
        "{}",
        if invocation.needs_confirmation() {
            "Completed; client consumption is not verified."
        } else {
            "Configuration saved; publication may be pending. Run status or sync to check generated content."
        }
    );
    Ok(())
}

fn confirm(
    input: &mut impl BufRead,
    output: &mut impl Write,
    interactive: bool,
) -> devmeld::Result<bool> {
    if !interactive {
        return Err(devmeld::error(
            "confirmation requires an interactive terminal; use --yes for noninteractive execution or --dry-run to preview",
        ));
    }
    write!(output, "Apply these changes? [y/N] ")?;
    output.flush()?;
    let mut reply = String::new();
    input.read_line(&mut reply)?;
    Ok(matches!(
        reply.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}

fn main() {
    if let Err(error) = run() {
        eprintln!("DevMeld: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn terminal_confirmation_is_one_opt_in_question_and_nonterminal_never_reads() {
        for (answer, expected) in [
            ("y\n", true),
            ("YES\n", true),
            ("\n", false),
            ("n\n", false),
            ("apply\n", false),
            ("", false),
        ] {
            let mut input = io::Cursor::new(answer);
            let mut output = Vec::new();
            assert_eq!(confirm(&mut input, &mut output, true).unwrap(), expected);
            assert_eq!(
                String::from_utf8(output).unwrap(),
                "Apply these changes? [y/N] "
            );
        }
        let mut input = io::Cursor::new("y\n");
        assert!(confirm(&mut input, &mut Vec::new(), false).is_err());
        assert_eq!(input.position(), 0);
    }
}
