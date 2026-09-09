# CLI Contract and Delivery Tracking

Status: target interaction for 005. Only completed tasks and live help establish current support. T001-T011 are implemented: source-first add/addresses, nearest context, no-init first add, cwd-relative native operands, `--dry-run`, group add/remove, group/resource list/show/move, local annotation add/update, configurable inheritance defaults and saved choices with derived origins. Confirmation still uses `--apply` / `apply`; simplified saves, status and entry attach/create below remain unbuilt.

## Help (first deliverable)

- `devmeld --help`, `devmeld resource --help`, `devmeld resource add --help`.
- Equivalent help for implemented `init`, `resource remove`, `access add/remove`, `entry add/remove`, `output`, `language`, `sync` and `recover`.
- `-h` is equivalent to `--help` for these requests.
- A syntactically complete `--context PATH` selector may precede the help path; the path is never resolved or opened for help.
- Root/group help lists actual children; operation help documents current operands/options, path interpretation and write behavior. No future `group`/`--as`/`--inherit` syntax is shown until implemented.
- An unknown topic is an error, not successful unrelated help. No-argument invocation retains root help. Help is terminal-only output and creates no context/output/entry files.

## Target command vocabulary (later slices)

```text
devmeld [--context PATH] COMMAND
resource add SOURCE [--as ORGANIZATION_PATH] [--kind document|description] [--schema PATH] [ANNOTATIONS] [INHERITANCE]
resource update ORGANIZATION_PATH [ANNOTATIONS] [INHERITANCE]
resource list [GROUP_PATH]
resource show ORGANIZATION_PATH
resource move FROM TO
resource remove ORGANIZATION_PATH
group add PATH [ANNOTATIONS] [INHERITANCE]
group update PATH [ANNOTATIONS] [INHERITANCE]
group list [PATH]
group show PATH
group move FROM TO
group remove PATH
config set language en|zh-CN
config set defaults.inherit true|false
config set defaults.propagate true|false
entry attach PATH
entry create PATH
entry remove PATH
status
sync [--dry-run | --yes]
recover [--dry-run | --yes]
init [PATH]
```

All path-taking operations use native local paths, except organization addresses which use logical `/` segments. Default `--as` is derived from the source filename only when unambiguous and valid; collisions require an explicit address, not invented suffixes. IDs are internal.

`--kind description` explicitly selects an authored resource-description source; `--description TEXT` is an annotation, avoiding the old overloaded flag. Tags use repeatable `--tag TEXT`. Named fields use `--field KEY=VALUE`; common shortcuts include `--environment VALUE`, `--attention TEXT`, `--shared`/`--no-shared`. Duplicate assignments to the same field in one command are diagnosed, not resolved by argument order.

`--inherit`/`--no-inherit` select receipt of parent information. On groups, `--propagate`/`--no-propagate` select downward transmission. Opposite flags together are errors. Updating without either flag preserves the existing choice; creating without either uses saved context defaults.

## Selection and mutation

- Explicit context selection precedes nearest-ancestor selection. Do not parse Markdown entry contents as a context-selection registry.
- First use without a context defaults to the invoking directory; only creating operations may initialize it. An explicit new location is permitted, but validation/dry-run must not create it.
- A nearest `.devmeld` marker with corrupt/inaccessible/incomplete data blocks fallback. No machine scan or implicit Git-root inference.
- Preserve the existing explicit-init recovery route: after rollback leaves no configuration or owned surfaces, `--context PATH init` may initialize that exact location without adopting any other files. An inferred marker is never implicitly reinitialized, and missing configuration with remaining ownership claims still blocks init.
- Command-line relative source/schema/entry/output paths use invocation cwd. Persist normalized references without moving source files.
- Scoped configuration/registration operations save requested changes and print context plus pending-publication information; `--dry-run` previews with no filesystem mutations.
- Sync/recovery preview and ask one `y/N` confirmation; `--yes` skips prompting only. Without interactive input or explicit confirmation, fail rather than hang. No-op still rechecks state and does not prompt/rewrite.
- Entry registration is distinct from publication. No setup command edits `AGENTS.md` implicitly. Status cannot claim a client actually loaded the entry.

## Organization operations (T008)

- Resource/group list without a scope lists all registered addresses in sorted order. With a group scope, list includes its descendants using complete logical segments; group list excludes the scope itself. Unknown/wrong-kind scopes are errors.
- Group show displays direct child groups and resources. Resource show displays stable identity, stored source/schema references and incoming/outgoing access associations by current address. These are read-only registration snapshots, not source availability, publication freshness or client-consumption checks. Queries never bootstrap, do not open source contents and reject `--apply`.
- Group add creates missing parents and may be the first operation in a new context. Group remove accepts empty groups only; no recursive option is supported and no source directories/files are deleted.
- Move uses an exact full destination, not filesystem `mv` inference. Missing destination parents are created; existing destinations, resource-as-parent collisions and self-descendant group moves fail without changes. Same-address move is a no-op but still validates existence and rechecks captured state on apply.
- Group move readdresses only that logical subtree, including empty subgroups. Resource IDs, source/schema references, incoming/outgoing associations and generated page filenames remain stable. Empty old parents remain until explicitly removed. New navigation and association labels appear on sync, not when the configuration move is saved.

## Compatibility boundary

The first help slice changes no persisted data or mutating commands. Later replacement of old human CLI syntax is intentional unreleased evolution, not a public machine API guarantee. Do not keep a second permanent parser translating new concepts into the old flat-ID model. Do not apply incompatible data changes to existing user/example contexts implicitly.

## Local annotations (T009)

- Resource/group add accepts `--description TEXT`, repeatable `--tag TEXT`, repeatable `--field KEY=VALUE`, and the documented shortcuts. `--kind description` still selects a source descriptor; `--description` is never a source filename.
- Resource/group update changes annotations or inheritance choices on an existing node of the requested kind. It requires at least one option and preserves omitted information; it does not change identity, logical address, source, schema or associations.
- Update adds unique tags and sets only named fields. `--clear-description`, `--remove-tag TEXT` and `--remove-field KEY` explicitly remove information. Removing an absent value is a no-op; removal flags are not valid for add. Setting and removing the same value, setting and clearing description, or duplicate field assignments (including shortcuts) fail without writes.
- All field values are text. `--shared` / `--no-shared` store the same `shared` strings as `--field shared=true` / `--field shared=false`. An empty field value is permitted; `--field` splits only at the first `=`. No value is interpreted as a permission, an environment action or an executable extension.
- Show, generated group navigation and resource navigation/pages identify these as local context annotations, separate from source-declared attributes. Supplied names/text are not translated or interpreted as Markdown instructions. Only generated labels switch between English and Simplified Chinese. List remains an address listing, not a metadata query engine.
- Updates use the existing preview/`--apply` mechanism until T012. Sync publishes changes. T010-T011 extend this local-annotation behavior with inheritance as described below.

## Inheritance (T010-T011)

- Group/resource add and update support `--inherit`/`--no-inherit`; only groups support `--propagate`/`--no-propagate`. Duplicate or opposite flags fail without writes. Descriptive fields named `inherit` or `propagate` are independent text, never controls.
- `config set defaults.inherit true|false` and `config set defaults.propagate true|false` require an existing valid owned context. They alter one creation default, without changing existing nodes or effective output. Initial defaults are false/true; no boolean coercion from `yes`, `1` or other values.
- Explicit add choices apply to the target only. Implicit parents use the context defaults. Each new node persists its resolved choice; omitted update flags preserve it. Moves retain choices but derive inherited meaning from the new ancestry.
- Inheritance requires the immediate parent to propagate and the child to inherit. A broken edge cuts all information crossing it, including distant ancestors. Tags union with all contributing origins; the closest local field overrides inherited value/origin, including identical or empty text. Removing a local override may reveal its inherited value; there is no per-value suppression mechanism.
- Show displays creation defaults separately from saved node choices. Show and generated navigation/pages retain local annotations and, when receipt is enabled, show effective tags/fields with origins (or an explicit empty result). Fixed output labels support en/zh-CN; supplied text/technical terms stay unchanged. List remains a logical-address listing.
- Overall descriptions, identities, source locations and permissions never inherit. No derived values are persisted into child declarations or source attributes. Defaults-only changes do not change generated content. Parent annotation/choice changes and moves require sync to update publication; all existing preview/stale/ownership checks still apply.
