//! Human-facing command discovery. Help must not load or create a context.

pub(super) fn help(args: &[String]) -> devmeld::Result<Option<String>> {
    let Some((last, topic)) = args.split_last() else {
        return Ok(Some(ROOT.into()));
    };
    if !matches!(last.as_str(), "--help" | "-h") {
        return Ok(None);
    }
    // A help selector is syntactic only: even a missing context is irrelevant.
    let topic = if topic.first().is_some_and(|arg| arg == "--context") {
        if topic.len() < 2 || topic[1].starts_with('-') {
            return Err(devmeld::error(
                "expected PATH after --context; help does not require --context",
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
    let (usage, description) = match topic.as_slice() {
        ["resource"] => (
            "resource add SOURCE [--as PATH] [--kind document|description] [--schema PATH] [ANNOTATIONS] [INHERITANCE]\nresource update PATH [ANNOTATIONS] [INHERITANCE]\nresource list [GROUP]\nresource show PATH\nresource move FROM TO\nresource remove PATH",
            "Manage registered sources by logical address.\nUse resource OPERATION --help for operation details. list/show are read-only; changes require sync to update generated navigation.",
        ),
        ["resource", "add"] => (
            "resource add SOURCE [--as PATH] [--kind document|description] [--schema PATH] [ANNOTATIONS] [INHERITANCE]",
            "Register an existing source; the default kind is document.\n--as chooses a logical organization address, such as database/test/orders; default is the source filename without its extension. Missing parent groups are created.\nThe address is separate from the internal stable identity and the physical source path.\n--schema validates description attributes against an optional local JSON Schema.\nThe source is not copied or modified. Registration changes configuration only; sync publishes navigation.",
        ),
        ["resource", "remove"] => (
            "resource remove PATH",
            "Remove a resource registration, not its source file.\nExisting access associations must be removed explicitly first.\nRun sync afterward to withdraw only the owned generated output.",
        ),
        ["resource" | "group", "update"] => (
            if topic[0] == "resource" {
                "resource update PATH [ANNOTATIONS] [INHERITANCE]"
            } else {
                "group update PATH [ANNOTATIONS] [INHERITANCE]"
            },
            "Update local context annotations and saved inheritance choices, preserving unspecified values. No source, identity or association changes.\n--tag adds to the tag set; --field replaces only the named field.\n--clear-description removes the overall description. --remove-tag TEXT and --remove-field KEY remove only local values (an absent local value is a no-op); removing a local override can reveal an inherited value.\nDo not set and remove the same value in one command. Run sync to publish the changes.",
        ),
        ["resource", "list"] => (
            "resource list [GROUP]",
            "List registered logical addresses in sorted order; optional GROUP includes its entire subtree.\nThis is read-only registration inspection, not a source scan or publication check.",
        ),
        ["resource", "show"] => (
            "resource show PATH",
            "Show a logical resource's stable identity, declared source/schema references, local annotations, saved inheritance choice, effective inherited origins and incoming/outgoing access associations.\nThis is read-only; missing source files do not prevent inspecting registration. Availability and client consumption are not inferred.",
        ),
        ["resource", "move"] => (
            "resource move FROM TO",
            "Change a resource's logical address to the exact destination TO, creating missing parent groups.\nExisting destinations or resource-as-parent collisions are rejected. Moving to the same address is a no-op.\nThe source, stable identity, access associations and generated page filename do not move. Run sync to update navigation.",
        ),
        ["group"] => (
            "group add PATH [ANNOTATIONS] [INHERITANCE]\ngroup update PATH [ANNOTATIONS] [INHERITANCE]\ngroup list [PATH]\ngroup show PATH\ngroup move FROM TO\ngroup remove PATH",
            "Organize logical groups independently of source folders.\nUse group OPERATION --help for operation details. list/show are read-only.",
        ),
        ["group", "add"] => (
            "group add PATH [ANNOTATIONS] [INHERITANCE]",
            "Create a logical group and any missing parent groups. A valid first add may initialize the context.\nThis does not create source directories or invent descriptions. Existing logical addresses are rejected. Run sync to publish.",
        ),
        ["group", "list"] => (
            "group list [PATH]",
            "List groups in sorted logical order; optional PATH lists all descendant groups, excluding PATH itself.\nThis is read-only registration inspection, not a filesystem directory listing.",
        ),
        ["group", "show"] => (
            "group show PATH",
            "Show a logical group's local annotations, saved inheritance choices, effective inherited origins, direct child groups and direct resource registrations.\nThis is read-only; use group list PATH or resource list PATH to see the full subtree.",
        ),
        ["group", "move"] => (
            "group move FROM TO",
            "Move a logical group and its entire subtree to the exact destination TO; missing destination parents are created.\nReject existing destinations and moves into the group's own descendants. No source files or stable identities move.\nEmpty old parents remain registered until explicitly removed. Run sync to update navigation.",
        ),
        ["group", "remove"] => (
            "group remove PATH",
            "Remove an empty logical group only. Nonempty groups are rejected; no recursive deletion is performed.\nNo source directory or file is deleted. Run sync to update navigation.",
        ),
        ["config"] | ["config", "set"] => (
            "config set defaults.inherit true|false\nconfig set defaults.propagate true|false",
            "Set creation defaults in an existing context. Initial defaults: inherit=false, propagate=true.\nChanges affect future nodes, including implicitly created parents, not existing saved choices or current generated meaning.\nUse resource/group show to inspect saved choices and effective origins. Use the language command for output language.",
        ),
        ["access"] => (
            "access add RESOURCE TOOL\n  access remove RESOURCE TOOL",
            "Manage explicit associations using registered logical resource addresses.\nUse access add --help or access remove --help for operation details.",
        ),
        ["access", "add"] => (
            "access add RESOURCE TOOL",
            "Associate two registered logical addresses for access guidance; the association stores stable identities.\nThis does not execute a tool, install dependencies or grant permission.\nRun sync to publish the guidance.",
        ),
        ["access", "remove"] => (
            "access remove RESOURCE TOOL",
            "Remove the specified access association without deleting either registration or source.\nRun sync to update published guidance.",
        ),
        ["entry"] => (
            "entry add PATH [--kind file|instructions]\n  entry remove PATH",
            "Manage explicitly selected project reading entries.\nUse entry add --help or entry remove --help for operation details.\nRegistration changes configuration only; sync updates the entry files.",
        ),
        ["entry", "add"] => (
            "entry add PATH [--kind file|instructions]",
            "Register a reading entry; the default kind is file.\nfile: an entirely generated Markdown entry at a selected path.\ninstructions: a managed insertion in a selected existing UTF-8 instruction file, preserving surrounding authored text.\nRun sync to publish. Registration alone does not edit the file or prove an Agent has read it.",
        ),
        ["entry", "remove"] => (
            "entry remove PATH",
            "Unregister a reading entry. Run sync to withdraw its owned content.\nFor instructions, only the managed insertion is removed; the host file and surrounding authored text remain.",
        ),
        ["init"] => (
            "init [--output PATH] [--entry PATH] [--instruction-entry PATH] [--language en|zh-CN]",
            "Optionally initialize configuration at the selected context location; a new directory is created only on confirmed application. Existing configuration or ownership claims block init.\nAn inferred existing marker is not reinitialized. After recovery leaves no configuration/claims, explicitly select --context PATH to retry init.\nA valid first resource add can initialize the context without init.\n--output chooses generated navigation (default .devmeld/output).\n--entry registers a generated Markdown entry; --instruction-entry registers a shared instruction-file insertion.\n--language selects generated wording (default en). Run sync to publish the configured outputs.",
        ),
        ["output"] => (
            "output PATH",
            "Change the configured output directory without moving source files.\nRun sync to publish at the new location and withdraw obsolete owned outputs.",
        ),
        ["language"] => (
            "language en|zh-CN",
            "Set the context's generated-output language; default is en.\nRun sync to publish the change. Only fixed generated wording is localized; authored content, technical names and commands remain unchanged.\nThis does not change the CLI language or prescribe the Agent's response language.",
        ),
        ["sync"] => (
            "sync",
            "Generate navigation and explicitly registered entries from current sources/configuration.\nExisting source files are not modified. Conflicts or stale inputs block application.\nUnchanged publication does not rewrite outputs. Published files remain readable without DevMeld running.",
        ),
        ["recover"] => (
            "recover",
            "Inspect recovery for an interrupted managed operation; --apply confirms the displayed recovery plan.\nPending uncommitted changes are rolled back where ownership evidence permits; committed operations are cleaned up.\nConflicting external changes are not overwritten. This is not a general undo command for completed operations.",
        ),
        _ => {
            return Err(devmeld::error(format!(
                "unknown help topic '{}'; see devmeld --help",
                topic.join(" ")
            )));
        }
    };
    let read_only = matches!(topic.as_slice(), ["resource" | "group", "list" | "show"]);
    let usage = usage
        .lines()
        .map(|line| {
            let line = line.trim();
            let query = ["resource list", "resource show", "group list", "group show"]
                .iter()
                .any(|prefix| line.starts_with(prefix));
            let flags = if query { "" } else { " [--apply | --dry-run]" };
            format!("  devmeld [--context PATH] {line}{flags}")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let notes = if read_only {
        "No confirmation or writes. --apply is rejected. An existing valid context is required; source files are not opened.\n"
    } else {
        WRITE_NOTES
    };
    let annotations = if matches!(topic.as_slice(), ["resource" | "group", "add" | "update"]) {
        ANNOTATIONS
    } else {
        ""
    };
    let inheritance = if annotations.is_empty() {
        String::new()
    } else {
        let propagation = if topic[0] == "group" {
            "  --propagate / --no-propagate   Allow/block transmission to children\n"
        } else {
            ""
        };
        format!(
            "INHERITANCE:\n  --inherit / --no-inherit       Receive/block parent tags and fields\n{propagation}Both parent transmission and child receipt must be enabled. Broken edges cut the ancestor chain.\nOnly tags and fields inherit: tags union with origins; local fields override same-name inherited fields.\nDescription, identity, source paths and permissions never inherit. Descriptive fields do not control these switches.\nOmitted choices use context defaults on creation and remain unchanged on update. Implicit parents use defaults, not target flags. Duplicate/opposite switches are rejected.\nConfigure future defaults with config set defaults.inherit/defaults.propagate true|false.\n\n"
        )
    };
    Ok(Some(format!(
        "Usage:\n{usage}\n\n{description}\n\n{annotations}{inheritance}{notes}"
    )))
}

const ANNOTATIONS: &str = "ANNOTATIONS:\n  --description TEXT       Overall context description, not a source-file path\n  --tag TEXT               Repeatable unique tag\n  --field KEY=VALUE        Repeatable descriptive text field (split at the first =)\n  --environment VALUE     Equivalent to --field environment=VALUE\n  --attention TEXT        Equivalent to --field attention=TEXT\n  --shared / --no-shared   Equivalent to --field shared=true / shared=false\nDuplicate field assignments (including shortcuts) are rejected. Empty field values are allowed.\nThese annotations are separate from source-file attributes and do not grant permissions.\nOnly the selected node is annotated; implicit parent groups receive no invented annotations.\n\n";

const WRITE_NOTES: &str = "Context: explicit --context PATH, otherwise nearest ancestor .devmeld, otherwise current directory.\nA valid first resource/group add can create the context; an invalid or incomplete existing marker blocks fallback.\nNative input paths resolve from the invoking directory; logical organization paths do not refer to files.\nChanging commands preview by default; --dry-run is explicitly read-only. --apply prints the preview then asks you to type apply. list/show are read-only and reject --apply.\nNo project entry is registered implicitly. No tools are executed or installed.\n";

const ROOT: &str = "devmeld [--context PATH] COMMAND [--apply | --dry-run]

Commands:
  init [--output PATH] [--entry PATH] [--language en|zh-CN]
  resource add SOURCE [--as PATH] [--kind document|description] [--schema PATH] [ANNOTATIONS] [INHERITANCE]
  resource update PATH [ANNOTATIONS] [INHERITANCE]
  resource list [GROUP]
  resource show PATH
  resource move FROM TO
  resource remove PATH
  group add PATH [ANNOTATIONS] [INHERITANCE]
  group update PATH [ANNOTATIONS] [INHERITANCE]
  group list [PATH]
  group show PATH
  group move FROM TO
  group remove PATH
  access add RESOURCE TOOL
  access remove RESOURCE TOOL
  entry add PATH
  entry remove PATH
  output PATH
  language en|zh-CN
  config set defaults.inherit true|false
  config set defaults.propagate true|false
  sync
  recover

Help (no context required):
  devmeld --help
  devmeld resource --help
  devmeld resource add --help
  devmeld entry add --help
  Use -h as a short form of --help.

Context: explicit --context PATH, otherwise nearest ancestor .devmeld, otherwise current directory.
A valid first resource/group add can create the context; invalid existing markers block fallback.
Native input paths resolve from the invoking directory; logical organization paths are separate.
Changing commands preview by default; --dry-run is explicitly read-only. --apply prints the preview then asks you to type apply.
list/show are read-only and reject --apply.
No project entry is registered implicitly.
Output language defaults to en; language changes configuration, then sync publishes it.
Only generated wording is localized; authored content, technical names and commands are unchanged.
Generated files remain readable without DevMeld running. No tools are executed or installed.
Shared entries: init also accepts --instruction-entry PATH; entry add accepts --kind file|instructions.
Entry commands change registration only; sync attaches, updates or detaches the insertion.
All new contexts use maintenance format v0. Incompatible older records are rejected without migration.
";
