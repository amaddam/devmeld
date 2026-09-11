# CLI Contract and Delivery Tracking

Status: implemented command contract for 005. Scoped saves, status, config language, entry attach/create/remove and one sync/recover confirmation complete the earlier organization/inheritance slices. Platform evidence and pending Maintainer acceptance are recorded in acceptance.md.

## Help (first deliverable)

### Current persisted representation (2026-09-10)

DevMeld owns `.devmeld/context.toml`, `.devmeld/state/owned.toml` and, while an
operation is pending, `.devmeld/state/pending.toml`. All retain `format_version = 0`.
This replaces the JSON representation described by earlier design/acceptance
records; it does not change command syntax, ownership or source formats.
Authored description resources and optional schemas remain JSON. Use maintenance
commands to edit the TOML configuration, not a text editor that bypasses receipts.

For example, a generated resource registration contains:

```toml
[[resources]]
id = "resource-1"
path = "knowledge/notes"
document = "knowledge/notes.md"
inherit = false
```

Ordinary in-context Windows references use `/`. Canonical absolute Windows paths
may retain `\\?\` where removing it could change filesystem semantics; TOML literal
strings avoid JSON backslash escaping. Multiline evidence is encoded by the TOML
serializer and must round-trip exact bytes, including CRLF and quote delimiters.
No hand-written parser or blanket slash/newline substitution is permitted.

Any legacy `context.json`, `state/owned.json` or `state/pending.json` blocks normal
and recovery operations, even beside TOML records. Files remain untouched; use
the previous build for old contexts or choose a separate fresh context. The
absence of legacy records is rechecked when applying a preview. Renaming files
does not migrate their syntax, owned targets, byte evidence or generated links.

### Compact ownership evidence (2026-09-11, T026)

`context.toml` owns configuration/registrations; `state/owned.toml` owns the
management receipt. They remain separate. A whole-file claim stores its target,
`kind = "whole_file"`, and `observed = { sha256, identity }`, where `sha256` is
the lowercase 64-digit SHA-256 of exact published bytes and `identity` is the
physical file identity. It no longer stores the file body. No timestamp or source
content registry is added. Fingerprints are generated automatically, not manually
maintained values, and do not confer authority on unowned/replaced files.

An instruction-entry claim still stores only its exact small insertion, including
markers/separators, never the whole author's host. Temporary pending journals and
staging/backups retain complete before/after data for recovery; these are cleaned
up after successful completion. The permanent receipt is not a disposable cache.

The preceding full-body TOML claim (`observed = { bytes, identity }`) remains
readable. Its fingerprint is derived from the recorded bytes, never current disk
content. Ambiguous/mixed fields and malformed digests are rejected. Reads/status,
no-op saves, dry-run and cancellation leave these records intact. Actual writes
emit compact receipts. If sync has no output changes but has full-body claims,
its preview shows a receipt-only target; normal confirmation and the same journal,
stale-input checks and recovery rules apply. Config, sources, output and host are
not rewritten for that compaction. A subsequent unchanged sync is a no-op.
Status continues to describe publication, not receipt encoding maintenance.

This is an authorized unreleased design refinement, not a version bump: markers
remain v0. Older builds reject the new claim shape. Existing full-image TOML
pending journals retain their recovery path; the JSON rejection above is unchanged.


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
- Group move readdresses only that logical subtree, including empty subgroups. Resource IDs, source/schema references and incoming/outgoing associations remain stable. Empty old logical parents remain until explicitly removed. Generated page paths, navigation and association labels follow the new addresses on sync, not when the configuration move is saved.

### Readable page paths (2026-09-10)

- Publish resource `services/shop` at `<output>/resources/services/shop.md`.
  Use every logical segment, append `.md` to the leaf (do not replace an existing
  extension), and keep the root navigation at `<output>/index.md`.
- This adapter mapping does not change logical names, IDs, source paths or
  access associations. Ordinary Unicode, spaces and technical names stay readable.
  Percent-escape `%`, Windows-forbidden filename characters and a trailing dot;
  escape the initial character of a Windows reserved device basename, on all hosts.
  Markdown links separately URI-encode the resulting native paths.
- Reject case-folded target/ancestor spelling collisions and file/directory
  conflicts across the planned page set before writes. Do not merge, overwrite,
  silently lowercase names or invent numbered suffixes.
- Sync computes all page destinations once and uses them for navigation and
  access links. Withdraw previous unchanged owned page files through the normal
  transaction, including former `r-<id>.md` files. Never delete unknown files or
  edited old pages. Configuration, IDs and original sources remain untouched.
- Old directory containers may remain empty; directory ownership and recursive
  cleanup are not introduced. External bookmarks to moved pages are not repaired;
  entry/index paths stay stable unless the user explicitly changes their locations.

### Reading view (2026-09-10, T022)

Cards keep a short localized HTML comment identifying DevMeld ownership, title,
authored summary/registration description if present, original source and access
links, source attributes and effective context information. No fallback summary,
managed-configuration link, inheritance controls or repeated maintenance prose.
Descriptions are paragraphs; use a context-notes heading only when an authored
summary also exists. Keep source attributes and context fields in separate
sections when both exist. Generated labels use natural EN/zh-CN wording;
authored text, field keys, values and technical names are not translated.

The index uses the registration description (otherwise authored summary) for
navigation, not a placeholder. Groups show their own descriptions. Publish tags
and fields once from the domain's effective view, without inheritance-origin
labels (2026-09-11 refinement). CLI show retains all contributing origins.
Omit empty sections. Full maintenance/source-ownership rules appear in
the index and configured entries; none of these reading surfaces links managed
configuration. CLI show remains the separate detailed maintenance view.

Serialize structured Markdown with `pulldown-cmark-to-cmark`, not handwritten
escaping. Field keys and technical literals use code spans; ordinary prose is
literal text with any necessary syntax protection handled by the serializer.
The Markdown parser distinguishes ordinary prose from syntax-like values; show
the latter as literal code spans instead of maintaining custom escape rules.
Do not interpret supplied text as Markdown or HTML. File-link URI encoding is
separate and unchanged. Formatting changes use ordinary owned sync/recovery,
not a configuration rewrite or format version bump.

### Group documents (2026-09-10, T024)

This refines T022's reading view: group metadata lives in each generated group
document, not expanded in the total index. For logical group `code/http`, append
its leaf again as a filename: `<output>/resources/code/http/http.md`. Encode every
segment using the same portable rules as resource cards, then encode Markdown
link destinations separately. Include empty and implicitly created groups.

The total index lists direct top-level groups and ungrouped resources. Every
group document has a short generated-file comment, full logical group title,
its own description, effective tags/fields without inheritance-origin labels, a
parent/index link and sorted direct-child group/resource links with descriptions.
Lists contain summaries, not child metadata or all descendants. Omit empty lists.
Cards retain their source/access links and current reading view. CLI show remains
the detailed configuration view; registration commands do not publish group files.

Compute group and resource destinations together. Reject case-folded and
file/directory collisions, including `code/code` versus group `code`, before
publication writes. Naming is not enforced by rewriting logical addresses or
assigning alternate filenames. Normal confirmed sync handles first creation,
updates, output relocation, group moves/removal and unchanged-owned withdrawal;
external edits/unowned targets/stale plans block it. Reuse current recovery.
No new configuration fields, group-as-resource entities or format labels.

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
- Show identifies local context annotations; publication uses the reading view above and keeps them distinct from source-declared attributes. Supplied names/text are not translated or interpreted as Markdown instructions. Only generated labels switch between English and Simplified Chinese. List remains an address listing, not a metadata query engine.
- Updates save configuration directly; `--dry-run` previews without writing. Sync publishes changes. Inheritance extends local annotations as described below.

## Inheritance (T010-T011)

- Group/resource add and update support `--inherit`/`--no-inherit`; only groups support `--propagate`/`--no-propagate`. Duplicate or opposite flags fail without writes. Descriptive fields named `inherit` or `propagate` are independent text, never controls.
- `config set defaults.inherit <true|false>` and `config set defaults.propagate <true|false>` require an existing valid owned context. They alter one creation default, without changing existing nodes or effective output. Initial defaults are false/true; no boolean coercion from `yes`, `1` or other values.
- Explicit add choices apply to the target only. Implicit parents use the context defaults. Each new node persists its resolved choice; omitted update flags preserve it. Moves retain choices but derive inherited meaning from the new ancestry.
- Inheritance requires the immediate parent to propagate and the child to inherit. A broken edge cuts all information crossing it, including distant ancestors. Tags union with all contributing origins; the closest local field overrides inherited value/origin, including identical or empty text. Removing a local override may reveal its inherited value; there is no per-value suppression mechanism.
- Show displays creation defaults separately from saved node choices, local annotations and effective tags/fields with origins (or an explicit empty result). Publication shows only the final reading view above, without configuration controls or empty-result diagnostics. Fixed output labels support en/zh-CN; supplied text/technical terms stay unchanged. List remains a logical-address listing.
- Overall descriptions, identities, source locations and permissions never inherit. No derived values are persisted into child declarations or source attributes. Defaults-only changes do not change generated content. Parent annotation/choice changes and moves require sync to update publication; all existing preview/stale/ownership checks still apply.

## Publication interaction and status (T012-T013)

- One optional terminal `--dry-run` applies to mutations; no directories, locks, journals or targets are created. One terminal `--yes` is valid only for exact sync/recover commands. Mixed/duplicate flags fail before context IO. Removed `--apply`, `language VALUE` and `entry add` are not compatibility aliases.
- A sync/recover with changes flushes the preview before prompting once. Both input and output must be terminals unless `--yes` was supplied. Accept `y` or `yes` case-insensitively; blank, n or EOF cancel. A no-op still calls the existing apply/recheck path without prompting.
- Optional `init <CONTEXT_DIR>` is an explicit cwd-relative native context selector, mutually exclusive with `--context`. Init flags for output/language/explicit entries remain supported. No init or add implicitly chooses an instruction host.
- `entry attach <ENTRY_FILE>` registers an insertion in a selected UTF-8 host (sync may create an absent selected host); `entry create <ENTRY_FILE>` registers a wholly generated file. Both save registration only. Existing ownership modes cannot be silently switched. Remove plus sync withdraws only the owned content.
- `status` reads current declarations/sources and shares publication preparation with sync; it rechecks inputs without applying changes. It reports saved configuration, pending/up-to-date generated content, configured/unpublished or published entries and pending targets. Missing inputs, conflicts or pending recovery block verification without repairing anything.
- Status compares currently expected generated bytes with owned files, not a historical source-freshness receipt. Authored document body changes that leave generated references unchanged need not mark publication pending. Client consumption always remains unverified; no persistent status registry, timestamp or new data format is introduced.
