//! Terminal adapter: syntax, help and presentation. Application use cases live in the library.
mod annotation_args;
mod help;
mod parse;
pub(crate) mod render;
pub(super) fn help(args: &[String]) -> devmeld::Result<Option<String>> {
    help::help(args)
}

pub(super) enum Operation {
    Mutate(devmeld::Mutation),
    Query(devmeld::Query),
}

pub(super) struct Invocation {
    pub context: Option<std::path::PathBuf>,
    pub request: Operation,
    pub dry_run: bool,
    pub yes: bool,
}
impl Invocation {
    pub fn parse(args: &[String]) -> devmeld::Result<Self> {
        let dry_run = args.last().is_some_and(|arg| arg == "--dry-run");
        let yes = args.last().is_some_and(|arg| arg == "--yes");
        let args = if dry_run || yes {
            &args[..args.len() - 1]
        } else {
            args
        };
        if args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--apply" | "--yes" | "--dry-run"))
        {
            return Err(devmeld::error(
                "--apply has been removed; use one terminal --dry-run, or --yes only for sync/recover; see --help",
            ));
        }
        let (context, command) = if args.first().is_some_and(|arg| arg == "--context") {
            if args.len() < 3 || args[1].starts_with('-') {
                return Err(devmeld::error(
                    "expected --context <CONTEXT_DIR> <COMMAND>; see --help",
                ));
            }
            (Some(std::path::PathBuf::from(&args[1])), &args[2..])
        } else {
            (None, args)
        };
        let (context, command) = if command.first().is_some_and(|arg| arg == "init")
            && command.get(1).is_some_and(|arg| !arg.starts_with('-'))
        {
            if context.is_some() {
                return Err(devmeld::error(
                    "select either --context <CONTEXT_DIR> or init <CONTEXT_DIR>, not both",
                ));
            }
            (
                Some(std::path::PathBuf::from(&command[1])),
                std::iter::once(command[0].clone())
                    .chain(command[2..].iter().cloned())
                    .collect(),
            )
        } else {
            (context, command.to_vec())
        };
        let invocation = Self {
            context,
            request: parse::operation(&command)?,
            dry_run,
            yes,
        };
        if yes && !invocation.needs_confirmation() {
            return Err(devmeld::error(
                "--yes is only valid for sync/recover; configuration commands save directly and queries are read-only",
            ));
        }
        Ok(invocation)
    }
    pub fn needs_confirmation(&self) -> bool {
        matches!(
            self.request,
            Operation::Mutate(devmeld::Mutation::Publish | devmeld::Mutation::Recover)
        )
    }
}
