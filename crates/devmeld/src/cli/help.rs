//! Layered help: root discovery, object operations, then local arguments and examples.
pub(crate) fn help(args: &[String]) -> devmeld::Result<Option<String>> {
    let Some((last, topic)) = args.split_last() else {
        return Ok(Some(ROOT.into()));
    };
    if !matches!(last.as_str(), "--help" | "-h") {
        return Ok(None);
    }
    let topic = if topic.first().is_some_and(|arg| arg == "--context") {
        if topic.len() < 2 || topic[1].starts_with('-') {
            return Err(devmeld::error(
                "expected CONTEXT_DIR after --context; help does not require --context",
            ));
        }
        &topic[2..]
    } else {
        topic
    };
    if topic.is_empty() {
        return Ok(Some(ROOT.into()));
    }
    let topic: Vec<_> = topic.iter().map(String::as_str).collect();
    if let Some(group) = group_help(&topic) {
        return Ok(Some(group));
    }
    let (usage, description) = match topic.as_slice() {
        ["resource", "add"] => (
            "resource add <SOURCE_FILE> [OPTIONS]",
            "Register an existing source without copying or modifying it.",
        ),
        ["resource", "remove"] => (
            "resource remove <RESOURCE_PATH>",
            "Remove a resource registration, not its source file.",
        ),
        ["resource" | "group", "update"] => (
            if topic[0] == "resource" {
                "resource update <RESOURCE_PATH> [OPTIONS]"
            } else {
                "group update <GROUP_PATH> [OPTIONS]"
            },
            "Update local annotations or inheritance; keep unspecified values.",
        ),
        ["resource", "list"] => (
            "resource list [GROUP_PATH]",
            "List registered resource addresses (read-only).",
        ),
        ["resource", "show"] => (
            "resource show <RESOURCE_PATH>",
            "Inspect a resource registration and its inherited annotations (read-only).",
        ),
        ["resource", "move"] => (
            "resource move <FROM_RESOURCE_PATH> <TO_RESOURCE_PATH>",
            "Move a logical address; keep source files, identity and associations.",
        ),
        ["group", "add"] => (
            "group add <GROUP_PATH> [OPTIONS]",
            "Create a logical group and any missing parents.",
        ),
        ["group", "list"] => (
            "group list [GROUP_PATH]",
            "List registered logical groups (read-only).",
        ),
        ["group", "show"] => (
            "group show <GROUP_PATH>",
            "Inspect a group, its annotations and direct children (read-only).",
        ),
        ["group", "move"] => (
            "group move <FROM_GROUP_PATH> <TO_GROUP_PATH>",
            "Move a logical subtree without moving source files.",
        ),
        ["group", "remove"] => (
            "group remove <GROUP_PATH>",
            "Remove an empty logical group; never delete source files or directories.",
        ),
        ["config", "set"] => (
            "config set <KEY> <VALUE>",
            "Set generated language or creation defaults for future nodes.",
        ),
        ["access", "add"] => (
            "access add <RESOURCE_PATH> <TOOL_RESOURCE_PATH>",
            "Link access guidance; does not execute tools, install dependencies or grant permission.",
        ),
        ["access", "remove"] => (
            "access remove <RESOURCE_PATH> <TOOL_RESOURCE_PATH>",
            "Remove an access association; keep registrations and source files.",
        ),
        ["entry", "attach"] => (
            "entry attach <ENTRY_FILE>",
            "Register a managed insertion; sync preserves authored text and can create a missing host.",
        ),
        ["entry", "create"] => (
            "entry create <ENTRY_FILE>",
            "Register a generated Markdown entry; sync never overwrites unowned content.",
        ),
        ["entry", "remove"] => (
            "entry remove <ENTRY_FILE>",
            "Unregister an entry; sync removes owned content only, keeping shared host text.",
        ),
        ["init"] => (
            "init [CONTEXT_DIR] [OPTIONS]",
            "Initialize an empty context (optional); existing contexts are not adopted.",
        ),
        ["output"] => (
            "output <OUTPUT_DIR>",
            "Set the generated output directory; source files stay in place.",
        ),
        ["status"] => (
            "status",
            "Compare generated output without writing; client consumption remains unverified.",
        ),
        ["sync"] => (
            "sync",
            "Publish context files and entries without modifying sources.",
        ),
        ["recover"] => (
            "recover",
            "Recover an interrupted operation; not an undo for completed changes.",
        ),
        _ => {
            return Err(devmeld::error(format!(
                "unknown help topic '{}'; see devmeld --help",
                topic.join(" ")
            )));
        }
    };
    let read_only = matches!(
        topic.as_slice(),
        ["resource" | "group", "list" | "show"] | ["status"]
    );
    let confirm = matches!(topic.as_slice(), ["sync" | "recover"]);
    let usage = usage
        .lines()
        .map(|line| {
            let flags = if confirm {
                " [--dry-run | --yes]"
            } else if !read_only && !line.ends_with("[OPTIONS]") {
                " [--dry-run]"
            } else {
                ""
            };
            format!("  devmeld [--context <CONTEXT_DIR>] {line}{flags}")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut output = format!("{description}\n\nUsage:\n{usage}\n");
    let (arguments, example) = details(&topic);
    section(&mut output, "Arguments", arguments);
    section(&mut output, "Examples", example);
    let mut options = String::new();
    match topic.as_slice() {
        ["init"] => options.push_str("  --output <OUTPUT_DIR>          Generated directory (default .devmeld/output)\n  --entry <ENTRY_FILE>           Register a generated Markdown entry; repeatable\n  --instruction-entry <ENTRY_FILE>  Register an instruction insertion; repeatable\n  --language <en|zh-CN>           Generated wording (default en)\n"),
        ["resource", "add"] => options.push_str("  --as <RESOURCE_PATH>           Logical organization address; default source filename without extension\n  --kind <document|description>  Source representation (default document)\n  --schema <SCHEMA_FILE>         Local JSON Schema; requires --kind description\n"),
        _ => {}
    }
    if !read_only {
        options.push_str("  --dry-run                     Preview without writes; place last\n");
        if confirm {
            options.push_str("  --yes                         Confirm noninteractively; place last, not with --dry-run\n");
        }
    }
    if matches!(topic.as_slice(), ["resource" | "group", "add" | "update"]) {
        let mut annotations = ANNOTATIONS.to_owned();
        if topic[1] == "update" {
            annotations.push_str("  --clear-description           Remove local description\n  --remove-tag <TEXT>            Remove a local tag; repeatable\n  --remove-field <KEY>           Remove a local field; repeatable\n");
        }
        options.push_str(&annotations);
        let mut inheritance =
            String::from("  --inherit / --no-inherit       Receive/block parent tags and fields\n");
        if topic[0] == "group" {
            inheritance.push_str(
                "  --propagate / --no-propagate   Allow/block transmission to children\n",
            );
        }

        options.push_str(&inheritance);
    }
    section(&mut output, "Options", &options);
    section(&mut output, "Global options", GLOBAL_OPTIONS);
    if confirm {
        output.push_str("\nConfirmation required; --yes skips the prompt, not conflicts or stale-input checks.\n");
    } else if !read_only {
        output.push_str("\nSaves configuration only; run sync to publish.\n");
        if matches!(topic.as_slice(), ["resource" | "group", "add"]) {
            output.push_str("Omitted inheritance options use context defaults.\n");
        }
    }
    Ok(Some(output))
}

fn section(output: &mut String, name: &str, body: &str) {
    if !body.is_empty() {
        output.push_str(&format!("\n{name}:\n{}\n", body.trim_end()));
    }
}
fn group_help(topic: &[&str]) -> Option<String> {
    let [name] = topic else { return None };
    let (description, commands) = match *name {
        "resource" => (
            "Manage registered resources.",
            "  add       Register an existing source\n  update    Change local annotations or inheritance\n  list      List registered addresses\n  show      Inspect one registration\n  move      Change a logical address\n  remove    Unregister a resource, keeping its source",
        ),
        "group" => (
            "Organize resources into logical groups.",
            "  add       Create a logical group\n  update    Change local annotations or inheritance\n  list      List registered groups\n  show      Inspect a group and its direct children\n  move      Move a logical subtree\n  remove    Remove an empty group",
        ),
        "access" => (
            "Associate registered resources with access guidance.",
            "  add       Associate a resource with guidance\n  remove    Remove an association",
        ),
        "entry" => (
            "Manage project reading entries.",
            "  attach    Register an instruction-file insertion\n  create    Register an entirely generated Markdown entry\n  remove    Unregister an entry",
        ),
        "config" => (
            "Change settings in an existing context.",
            "  set       Set generated language or new-node inheritance defaults",
        ),
        _ => return None,
    };
    Some(format!(
        "{description}\n\nUsage:\n  devmeld [--context <CONTEXT_DIR>] {name} <COMMAND>\n\nCommands:\n{commands}\n\nGlobal options:\n{GLOBAL_OPTIONS}\nUse devmeld {name} <COMMAND> --help for details.\n"
    ))
}
fn details<'a>(topic: &[&str]) -> (&'a str, &'a str) {
    match topic {
        ["resource", "add"] => (
            "  <SOURCE_FILE>  Local file; source/schema paths use the invoking directory.",
            "  devmeld resource add notes.md --as knowledge/notes",
        ),
        ["resource", "list"] => (
            "  [GROUP_PATH]  Logical group scope; omit for all resources, otherwise include its subtree.",
            "  devmeld resource list",
        ),
        ["group", "list"] => (
            "  [GROUP_PATH]  Logical scope: descendants only, excluding this group; omit for all groups.",
            "  devmeld group list",
        ),
        ["resource", "show"] => (
            "  <RESOURCE_PATH>  Registered logical resource address, not a source file or internal ID.",
            "  devmeld resource show knowledge/notes",
        ),
        ["resource", "remove"] => (
            "  <RESOURCE_PATH>  Registered logical address; remove its access associations first.",
            "  devmeld resource remove knowledge/notes",
        ),
        ["resource", "update"] => (
            "  <RESOURCE_PATH>  Registered logical resource address.",
            "  devmeld resource update knowledge/notes --description \"Team notes\" --tag backend",
        ),
        ["group", "add"] => (
            "  <GROUP_PATH>  New logical group address; missing parent groups are created.",
            "  devmeld group add database/test --description \"Test databases\" --environment test",
        ),
        ["group", "update"] => (
            "  <GROUP_PATH>  Registered logical group address.",
            "  devmeld group update database --propagate --attention \"Read connection guidance\"",
        ),
        ["group", "show"] => (
            "  <GROUP_PATH>  Registered logical group address, not a source directory.",
            "  devmeld group show database",
        ),
        ["group", "remove"] => (
            "  <GROUP_PATH>  Registered empty group, not a directory to delete.",
            "  devmeld group remove database/unused",
        ),
        ["resource", "move"] => (
            "  <FROM_RESOURCE_PATH>  Existing logical address\n  <TO_RESOURCE_PATH>    Exact destination logical address, not a directory",
            "  devmeld resource move knowledge/notes archive/notes",
        ),
        ["group", "move"] => (
            "  <FROM_GROUP_PATH>  Existing logical group address\n  <TO_GROUP_PATH>    Exact destination logical address, not a directory",
            "  devmeld group move database archive/database",
        ),
        ["access", "add"] => (
            ACCESS_ARGUMENTS,
            "  devmeld access add database/test tools/query",
        ),
        ["access", "remove"] => (
            ACCESS_ARGUMENTS,
            "  devmeld access remove database/test tools/query",
        ),
        ["entry", "attach"] => (ENTRY_ARGUMENTS, "  devmeld entry attach AGENTS.md"),
        ["entry", "create"] => (ENTRY_ARGUMENTS, "  devmeld entry create CONTEXT.md"),
        ["entry", "remove"] => (ENTRY_ARGUMENTS, "  devmeld entry remove AGENTS.md"),
        ["output"] => (
            "  <OUTPUT_DIR>  Local generated-output directory; relative to the invoking directory.",
            "  devmeld output generated-context",
        ),
        ["init"] => (
            "  [CONTEXT_DIR]  Local directory; cannot combine with --context. Native paths use the invoking directory.",
            "  devmeld init . --language zh-CN",
        ),
        ["config", "set"] => (
            "  language <en|zh-CN>                  Generated wording; authored text is unchanged\n  defaults.inherit <true|false>        New-node parent receipt (initial false)\n  defaults.propagate <true|false>      New-group transmission (initial true)",
            "  devmeld config set language zh-CN",
        ),
        ["status"] => ("", ""),
        ["sync"] => ("", "  devmeld sync --dry-run"),
        ["recover"] => ("", "  devmeld recover --dry-run"),
        _ => unreachable!("only recognized leaf topics reach details"),
    }
}
const ACCESS_ARGUMENTS: &str = "  <RESOURCE_PATH>       Registered resource receiving guidance\n  <TOOL_RESOURCE_PATH>  Registered access-guidance resource, not an executable";
const ENTRY_ARGUMENTS: &str =
    "  <ENTRY_FILE>  Selected local file; relative to the invoking directory.";
const ANNOTATIONS: &str = "  --description <TEXT>           Overall context description\n  --tag <TEXT>                   Repeatable unique tag\n  --field <KEY=VALUE>            Repeatable descriptive field (split at first =)\n  --environment <VALUE>         Equivalent to --field environment=VALUE\n  --attention <TEXT>            Equivalent to --field attention=TEXT\n  --shared / --no-shared        Equivalent to --field shared=true / shared=false\n";
const GLOBAL_OPTIONS: &str = "  --context <CONTEXT_DIR>  Context directory (before command)\n  -h, --help              Show help";
const ROOT: &str = "DevMeld - organize resources and publish local context files.

Usage:
  devmeld [--context <CONTEXT_DIR>] <COMMAND>

Commands:
  resource   Manage registered resources
  group      Organize resources into logical groups
  access     Associate resources with access guidance
  entry      Manage project reading entries
  output     Set the generated output directory
  config     Change context settings
  status     Inspect configuration and publication status
  sync       Publish generated context files
  recover    Recover an interrupted managed operation
  init       Explicitly initialize a context (optional setup)

Global options:
  --context <CONTEXT_DIR>  Select a local context directory; place before the command
  -h, --help              Show help

Examples:
  devmeld resource add --help

Context: explicit --context, otherwise nearest ancestor .devmeld, otherwise current directory.
A valid first resource/group add can initialize here.
Relative native paths resolve from the invoking directory; logical addresses are not files.
Configuration changes save directly; sync/recover ask for confirmation when needed.

<VALUE> is required; [VALUE] is optional; a|b means choose one.
Replace placeholders with values; do not type the brackets.
Use devmeld <COMMAND> --help for details.
";
