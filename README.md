# DevMeld

English | [简体中文](README.zh-CN.md)

DevMeld is a local-first tool for organizing project context. It turns team-maintained
documents, resource descriptions and access guidance into a readable Markdown index.
An Agent follows a small project entry to find relevant sources, without keeping
DevMeld running.

Resource registration → Published index → Project entry → Agent reads original sources

**Development status:** unreleased v0, intended for local trials. Formats may change.

## What You Can Do

- Organize existing documents and structured service/tool descriptions without moving the originals.
- Link resources to access instructions and tool documentation; optionally validate description attributes with a local JSON Schema.
- Publish ordinary Markdown entries or maintain a small insertion in a selected project instruction file.
- Generate English or Simplified Chinese navigation, including links across local Windows drives.

DevMeld generates context from explicit inputs; it does not use AI to rewrite your
knowledge, answer questions, connect to services or execute the listed tools.

## Quick Start

### 1. Build from source

With Rust installed, run this from the repository root. The toolchain is specified
in [rust-toolchain.toml](rust-toolchain.toml); a native linker is also required.

```text
cargo build --release --locked -p devmeld
```

The executable is `target/release/devmeld` on Linux/macOS, or
`target/release/devmeld.exe` on Windows. The commands below call it `devmeld`:
use its actual path or add its directory to your PATH. This does not require
installing an Agent Client or running a background service.

### 2. Register a document and publish

Run from the project directory where you want to keep context. No separate `init`
is required. Choose an existing local document and replace `<absolute-document-path>` with its
full path, such as `C:/knowledge/notes.md` or `/home/me/knowledge/notes.md`.

```text
devmeld resource add "<absolute-document-path>" --as knowledge/notes
devmeld sync
```

Registration saves configuration directly. `sync` shows the publication preview and asks
once: `Apply these changes? [y/N]`. Enter `y` to publish, or press Enter to cancel.
Append `--dry-run` to preview without writing; scripts use `sync --yes` instead of an
interactive prompt. `--yes` never overrides conflicts or grants additional ownership.

Open `.devmeld/output/index.md`, then follow the resource page to the
original document. You can give that index file to an Agent with local read
access. DevMeld can exit after publication; reading does not call it again.

The command reports the selected context: explicit `--context PATH` wins;
otherwise it reuses the nearest ancestor `.devmeld`, or creates one in the current
directory on the first successful add. A corrupt or incomplete marker blocks fallback.
For a separate trial, pass `--context "<new-context-directory>"` to both commands;
the new location is created only when saving valid changes. Optional `devmeld init PATH`
creates an empty context at an explicit location; it is not required before adding resources.

Native source/schema/entry/output paths resolve from the invoking directory.
`knowledge/notes` is a logical address, not a source folder or an internal ID;
missing parent groups are created. Original documents stay in place and are not modified.

## Everyday Use

### Find a command

Help works without initializing or selecting a context and does not create files:

```text
devmeld --help
devmeld resource --help
devmeld resource add --help
devmeld entry attach --help
```

Use `-h` as a short form. Help describes the currently implemented commands.

### Inspect and reorganize resources

After registering `knowledge/notes` in the quick start:

```text
devmeld group list
devmeld group show knowledge
devmeld resource list knowledge
devmeld resource show knowledge/notes
devmeld group add database
devmeld resource move knowledge/notes database/notes
devmeld group move database reference/database
devmeld group remove knowledge
devmeld sync
```

`list`/`show` are read-only and need no confirmation. List with a group path includes
its descendants; group show displays direct children. Resource show reports registered
sources and access associations, not live availability or whether an Agent read them.
Move destinations are exact logical addresses: existing targets are rejected, not merged.
Moving never relocates source files or changes stable identities/associations. Empty old
groups remain until explicitly removed; nonempty group removal is rejected. Only sync
updates published navigation. Append `--dry-run` to preview a change without saving.

### Describe groups and resources

Descriptions, tags and named fields are separate context annotations. For example,
after the quick start (before moving `knowledge/notes`):

```text
devmeld group update knowledge --description "Team documentation" --tag backend --environment test --shared
devmeld resource update knowledge/notes --description "Project conventions" --field "attention=Check the source before changing configuration"
devmeld group show knowledge
devmeld resource show knowledge/notes
devmeld sync
```

These options also work on `group add` and `resource add`. Custom fields need no
new plugin: `--environment test` is equivalent to `--field environment=test`;
`--shared` and `--no-shared` mean `--field shared=true` and `--field shared=false`.
All field values are descriptive text, not permissions or execution settings.
`--tag shared` remains an independent tag.

Update preserves unspecified information. Use `--clear-description`,
`--remove-tag TAG`, or `--remove-field KEY` with update to remove it explicitly.
Tags are unique; duplicate field assignments in one command are errors.
Show and generated navigation distinguish context annotations from source-file
attributes. Sources are not rewritten; local declarations and inherited values remain distinct.

### Control inheritance

Both sides must opt in: the parent group allows transmission (`propagate`), and
the child receives it (`inherit`). Initially, groups transmit by default and
children do not inherit. After the quick start, before moving `knowledge/notes`:

```text
devmeld group update knowledge --tag backend --environment test --propagate
devmeld resource update knowledge/notes --inherit
devmeld resource show knowledge/notes
devmeld sync
```

Show and generated navigation identify local annotations and the origin of effective
inherited tags/fields. Tags combine without duplicates; the nearest local field wins.
Overall descriptions, identities, source paths and permissions never inherit.
`--no-propagate` on a group or `--no-inherit` on a child cuts that inheritance edge,
including more distant ancestors. Removing a local field override may reveal its inherited value.

Creation defaults are configurable in an existing context:

```text
devmeld config set defaults.inherit true
devmeld config set defaults.propagate false
devmeld group add tools --no-inherit --propagate
devmeld group show tools
```

These defaults affect new nodes only, including automatically created parents.
Explicit flags override the target node's creation defaults; updates preserve omitted
choices. Existing choices stay saved when defaults change or nodes move.
Changed parent annotations take effect in generated files on the next `sync`;
inherited values are never copied into the child's registration.

### Check publication state

```text
devmeld status
devmeld sync --dry-run
devmeld sync
```

`status` is read-only: it distinguishes saved configuration, pending/up-to-date
publication and configured/published entries. Missing inputs, conflicts or pending
recovery block verification. It never claims that an Agent loaded the context.
An unchanged sync still validates inputs and ownership but does not prompt or rewrite files.

### Update context

Edit your original documents or descriptions, then synchronize:

```text
devmeld sync
```

Use DevMeld commands to change registrations and entries; do not hand-edit managed
configuration or generated files. Unchanged synchronization does not rewrite output.

### Add an entry to a project

Select an ordinary UTF-8 instruction file, such as your project's `AGENTS.md`:

```text
devmeld entry attach "<absolute-project-path>/AGENTS.md"
devmeld sync
```

Only the generated insertion is maintained; surrounding authored text is preserved.
Whether a new Agent session discovers the file depends on that client's setup.
Alternatively, register an ordinary entry using `devmeld entry create CONTEXT.md`,
then sync. Initialization does not register or edit a project entry implicitly.

To detach, run `entry remove "<absolute-project-path>/AGENTS.md"`, then
`sync`, from the same context (or with an explicit `--context`). This removes only the insertion,
not the host file or original resources.

### Choose the generated language

```text
devmeld config set language zh-CN
devmeld sync
```

Use `en` to switch back. English is the default. Only generated explanatory text
changes; authored content, ssh/http/curl, commands, IDs and paths stay unchanged.
CLI help and diagnostics currently remain English.

## Current Limits

- Local-machine use only. Sources may be on different local drives; remote service
  addresses can be described, but are not fetched or indexed remotely.
- Synchronization is manual. There is no automatic project discovery, scheduler,
  tool installation or execution.
- `status` compares expected generated content with owned files using current inputs;
  it is not a historical source-freshness record or proof that an Agent read the entry.
- Older incompatible development records are preserved and rejected, not migrated.
  Use fresh context paths for a first trial. For an interrupted current operation,
  preview `devmeld --context PATH recover --dry-run`, then run `recover` for one
  confirmation (or `recover --yes` in a script).
- See the [current CLI verification record](specs/005-context-cli/acceptance.md)
  for platform results. macOS and additional Agent Client setups remain unverified.

## More Information

- [Examples](examples/README.md): service/tool descriptions, access associations,
  schema validation and cross-drive paths.
- [Product overview](docs/product.md): product concepts and ownership boundaries.
- [Contributing](CONTRIBUTING.md) and [engineering guide](docs/engineering.md):
  development practices. Run `cargo xtask check` for the project checks.
