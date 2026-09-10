# CLI Contract and Delivery Tracking

Status: implemented command contract for 005. Scoped saves, status, config language, entry attach/create/remove and one sync/recover confirmation complete the earlier organization/inheritance slices. Platform evidence and pending Maintainer acceptance are recorded in acceptance.md.

## Help (first deliverable)

The 2026-09-10 presentation refinement follows the [CLI style guide](../../../docs/cli-style.md).
Precise argument names, direct-child summaries, per-command option sections
and removal of unrelated shared help are implemented. The vocabulary
below is a contract inventory, not a template for printing every command in root help.

Help is a quick command reference, not the planned web user manual. Use one-line
purpose/option summaries and at most one relevant example; retain precise path
roles, defaults, required combinations and immediate write risks. Detailed
inheritance/ownership explanations and workflows remain in documentation. Do not
advertise a documentation URL before the website has an actual approved address.

- `devmeld --help`, `devmeld resource --help`, `devmeld resource add --help`.
- Equivalent help for implemented `init`, `resource remove`, `access add/remove`, `entry attach/create/remove`, `output`, `config set`, `status`, `sync` and `recover`.
- `-h` is equivalent to `--help` for these requests.
- A syntactically complete `--context <CONTEXT_DIR>` selector may precede the help path; the path is never resolved or opened for help.
- Root/group help lists direct children and one-line descriptions, not nested option inventories. Operation help documents only its operands/options, examples, path interpretation and write behavior; shared notation is explained at the root. Only implemented syntax is shown.
- An unknown topic is an error, not successful unrelated help. No-argument invocation retains root help. Help is terminal-only output and creates no context/output/entry files.

## Command vocabulary

```text
devmeld [--context <CONTEXT_DIR>] <COMMAND>
resource add <SOURCE_FILE> [--as <RESOURCE_PATH>] [--kind <document|description>] [--schema <SCHEMA_FILE>] [ANNOTATIONS] [INHERITANCE]
resource update <RESOURCE_PATH> [ANNOTATIONS] [INHERITANCE]
resource list [GROUP_PATH]
resource show <RESOURCE_PATH>
resource move <FROM_RESOURCE_PATH> <TO_RESOURCE_PATH>
resource remove <RESOURCE_PATH>
group add <GROUP_PATH> [ANNOTATIONS] [INHERITANCE]
group update <GROUP_PATH> [ANNOTATIONS] [INHERITANCE]
group list [GROUP_PATH]
group show <GROUP_PATH>
group move <FROM_GROUP_PATH> <TO_GROUP_PATH>
group remove <GROUP_PATH>
config set language <en|zh-CN>
config set defaults.inherit <true|false>
config set defaults.propagate <true|false>
access add <RESOURCE_PATH> <TOOL_RESOURCE_PATH>
access remove <RESOURCE_PATH> <TOOL_RESOURCE_PATH>
entry attach <ENTRY_FILE>
entry create <ENTRY_FILE>
entry remove <ENTRY_FILE>
output <OUTPUT_DIR>
status
sync [--dry-run | --yes]
recover [--dry-run | --yes]
init [CONTEXT_DIR] [--output <OUTPUT_DIR>] [--entry <ENTRY_FILE>] [--instruction-entry <ENTRY_FILE>] [--language <en|zh-CN>]
```

Help notation is consistent across levels: `<VALUE>` is required; `[VALUE]` or `[--option <VALUE>]` is optional; `a|b` denotes alternatives. Do not type the brackets or placeholder names. `[ANNOTATIONS]` and `[INHERITANCE]` above classify optional flags for this contract inventory. Executable help uses `[OPTIONS]` where appropriate; annotation and inheritance flags appear in the same concise Options list, not as extra operands or tutorial sections.

- `CONTEXT_DIR` / `OUTPUT_DIR`: native local directories.
- `SOURCE_FILE` / `SCHEMA_FILE` / `ENTRY_FILE`: native local files.
- `RESOURCE_PATH` / `GROUP_PATH`: logical organization addresses, not filesystem paths or internal IDs.
- `TOOL_RESOURCE_PATH`: logical address of a registered access-guidance resource, not an executable path.
- `FROM_RESOURCE_PATH` / `TO_RESOURCE_PATH` and their group equivalents: original and exact destination logical addresses.

Native relative paths resolve from the invoking directory. Logical addresses use `/` segments independently of physical source locations. This is a help-label clarification only: actual commands, required/optional operands, persisted data and mutation behavior do not change. Default `--as` is derived from the source filename only when unambiguous and valid; collisions require an explicit address, not invented suffixes. IDs are internal.

`--kind description` explicitly selects an authored resource-description source; `--description <TEXT>` is an annotation, avoiding the old overloaded flag. Tags use repeatable `--tag <TEXT>`. Named fields use `--field <KEY=VALUE>`; common shortcuts include `--environment <VALUE>`, `--attention <TEXT>`, `--shared`/`--no-shared`. Duplicate assignments to the same field in one command are diagnosed, not resolved by argument order.

`--inherit`/`--no-inherit` select receipt of parent information. On groups, `--propagate`/`--no-propagate` select downward transmission. Opposite flags together are errors. Updating without either flag preserves the existing choice; creating without either uses saved context defaults.

## Selection and mutation

- Explicit context selection precedes nearest-ancestor selection. Do not parse Markdown entry contents as a context-selection registry.
- First use without a context defaults to the invoking directory; only creating operations may initialize it. An explicit new location is permitted, but validation/dry-run must not create it.
- A nearest `.devmeld` marker with corrupt/inaccessible/incomplete data blocks fallback. No machine scan or implicit Git-root inference.
- Preserve the existing explicit-init recovery route: after rollback leaves no configuration or owned surfaces, `--context <CONTEXT_DIR> init` may initialize that exact location without adopting any other files. An inferred marker is never implicitly reinitialized, and missing configuration with remaining ownership claims still blocks init.
- Command-line relative source/schema/entry/output paths use invocation cwd. Persist normalized references without moving source files.
- Scoped configuration/registration operations save requested changes and print context plus pending-publication information; `--dry-run` previews with no filesystem mutations.
- Sync/recovery preview and ask one `y/N` confirmation; `--yes` skips prompting only. Without interactive input or explicit confirmation, fail rather than hang. No-op still rechecks state and does not prompt/rewrite.
- Entry registration is distinct from publication. No setup command edits `AGENTS.md` implicitly. Status cannot claim a client actually loaded the entry.

## Organization operations (T008)

- Resource/group list without a scope lists all registered addresses in sorted order. With a group scope, list includes its descendants using complete logical segments; group list excludes the scope itself. Unknown/wrong-kind scopes are errors.
- Group show displays direct child groups and resources. Resource show displays stable identity, stored source/schema references and incoming/outgoing access associations by current address. These are read-only registration snapshots, not source availability, publication freshness or client-consumption checks. Queries never bootstrap, do not open source contents and reject `--yes`. The removed `--apply` is rejected on all commands.
- Group add creates missing parents and may be the first operation in a new context. Group remove accepts empty groups only; no recursive option is supported and no source directories/files are deleted.
- Move uses an exact full destination, not filesystem `mv` inference. Missing destination parents are created; existing destinations, resource-as-parent collisions and self-descendant group moves fail without changes. Same-address move is a no-op but still validates existence and rechecks captured state on apply.
- Group move readdresses only that logical subtree, including empty subgroups. Resource IDs, source/schema references, incoming/outgoing associations and generated page filenames remain stable. Empty old parents remain until explicitly removed. New navigation and association labels appear on sync, not when the configuration move is saved.

## Compatibility boundary

The CLI is an adapter over typed Rust mutation/query requests, structured inspection
results and borrowed before/after previews. It is not the only possible application
entrypoint. All callers share preparation and apply/recheck/ownership/recovery;
terminal confirmation remains a CLI interaction. These in-process types are not
a frozen SDK or a new public machine wire protocol. No GUI is implemented by 005.

The first help slice changes no persisted data or mutating commands. Later replacement of old human CLI syntax is intentional unreleased evolution, not a public machine API guarantee. Do not keep a second permanent parser translating new concepts into the old flat-ID model. Do not apply incompatible data changes to existing user/example contexts implicitly.

## Local annotations (T009)

- Resource/group add accepts `--description <TEXT>`, repeatable `--tag <TEXT>`, repeatable `--field <KEY=VALUE>`, and the documented shortcuts. `--kind description` still selects a source descriptor; `--description` is never a source filename.
- Resource/group update changes annotations or inheritance choices on an existing node of the requested kind. It requires at least one option and preserves omitted information; it does not change identity, logical address, source, schema or associations.
- Update adds unique tags and sets only named fields. `--clear-description`, `--remove-tag <TEXT>` and `--remove-field <KEY>` explicitly remove information. Removing an absent value is a no-op; removal flags are not valid for add. Setting and removing the same value, setting and clearing description, or duplicate field assignments (including shortcuts) fail without writes.
- All field values are text. `--shared` / `--no-shared` store the same `shared` strings as `--field shared=true` / `--field shared=false`. An empty field value is permitted; `--field` splits only at the first `=`. No value is interpreted as a permission, an environment action or an executable extension.
- Show, generated group navigation and resource navigation/pages identify these as local context annotations, separate from source-declared attributes. Supplied names/text are not translated or interpreted as Markdown instructions. Only generated labels switch between English and Simplified Chinese. List remains an address listing, not a metadata query engine.
- Updates save configuration directly; `--dry-run` previews without writing. Sync publishes changes. Inheritance extends local annotations as described below.

## Inheritance (T010-T011)

- Group/resource add and update support `--inherit`/`--no-inherit`; only groups support `--propagate`/`--no-propagate`. Duplicate or opposite flags fail without writes. Descriptive fields named `inherit` or `propagate` are independent text, never controls.
- `config set defaults.inherit <true|false>` and `config set defaults.propagate <true|false>` require an existing valid owned context. They alter one creation default, without changing existing nodes or effective output. Initial defaults are false/true; no boolean coercion from `yes`, `1` or other values.
- Explicit add choices apply to the target only. Implicit parents use the context defaults. Each new node persists its resolved choice; omitted update flags preserve it. Moves retain choices but derive inherited meaning from the new ancestry.
- Inheritance requires the immediate parent to propagate and the child to inherit. A broken edge cuts all information crossing it, including distant ancestors. Tags union with all contributing origins; the closest local field overrides inherited value/origin, including identical or empty text. Removing a local override may reveal its inherited value; there is no per-value suppression mechanism.
- Show displays creation defaults separately from saved node choices. Show and generated navigation/pages retain local annotations and, when receipt is enabled, show effective tags/fields with origins (or an explicit empty result). Fixed output labels support en/zh-CN; supplied text/technical terms stay unchanged. List remains a logical-address listing.
- Overall descriptions, identities, source locations and permissions never inherit. No derived values are persisted into child declarations or source attributes. Defaults-only changes do not change generated content. Parent annotation/choice changes and moves require sync to update publication; all existing preview/stale/ownership checks still apply.

## Publication interaction and status (T012-T013)

- One optional terminal `--dry-run` applies to mutations; no directories, locks, journals or targets are created. One terminal `--yes` is valid only for exact sync/recover commands. Mixed/duplicate flags fail before context IO. Removed `--apply`, `language VALUE` and `entry add` are not compatibility aliases.
- A sync/recover with changes flushes the preview before prompting once. Both input and output must be terminals unless `--yes` was supplied. Accept `y` or `yes` case-insensitively; blank, n or EOF cancel. A no-op still calls the existing apply/recheck path without prompting.
- Optional `init <CONTEXT_DIR>` is an explicit cwd-relative native context selector, mutually exclusive with `--context`. Init flags for output/language/explicit entries remain supported. No init or add implicitly chooses an instruction host.
- `entry attach <ENTRY_FILE>` registers an insertion in a selected UTF-8 host (sync may create an absent selected host); `entry create <ENTRY_FILE>` registers a wholly generated file. Both save registration only. Existing ownership modes cannot be silently switched. Remove plus sync withdraws only the owned content.
- `status` reads current declarations/sources and shares publication preparation with sync; it rechecks inputs without applying changes. It reports saved configuration, pending/up-to-date generated content, configured/unpublished or published entries and pending targets. Missing inputs, conflicts or pending recovery block verification without repairing anything.
- Status compares currently expected generated bytes with owned files, not a historical source-freshness receipt. Authored document body changes that leave generated references unchanged need not mark publication pending. Client consumption always remains unverified; no persistent status registry, timestamp or new data format is introduced.
