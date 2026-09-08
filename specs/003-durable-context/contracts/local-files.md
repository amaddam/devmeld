# Local file and command contract (003)

Feature-local version 1, not a ratified permanent public machine API. Unknown
control fields, unsupported versions and invalid semantics fail; no migration.

## Human entrypoint

```text
devmeld --context PATH init [--output PATH] [--entry PATH]
devmeld --context PATH resource add ID --document PATH
devmeld --context PATH resource add ID --description PATH [--schema PATH]
devmeld --context PATH resource remove ID
devmeld --context PATH access add RESOURCE TOOL
devmeld --context PATH access remove RESOURCE TOOL
devmeld --context PATH entry add PATH
devmeld --context PATH entry remove PATH
devmeld --context PATH output PATH
devmeld --context PATH sync
devmeld --context PATH recover
```

Mutations preview read-only by default. Add `--apply`, inspect the printed
before/after content and type `apply`. EOF/other input cancels without applying.
Recovery is also confirmed. Exit 0 is successful preview/no-op/application;
errors, cancellation, conflicts and pending recovery exit nonzero.

Context root must exist. Command and configuration paths resolve from that root;
no directory discovery. `init` defaults to output `.devmeld/output` and no entry.

## JSON

`.devmeld/context.json`: `format_version: 1`, `resources`, `access` and
`publication: {directory, entries}`. Entries: `{kind: "file", path}`.
Registration: `{id, document}` OR `{id, description, attributes_schema?}`.
Access: `{resource, tool}`. IDs match `[a-z0-9][a-z0-9_-]*`, at most 64 bytes.
Pages are `r-<id>.md` to avoid reserved platform basenames. Paths/titles allow
Unicode and spaces.

Authored description: `{title, summary, attributes?, references?}`. Attributes
map strings to strings; ordered references contain `{label,path}` resolved from
the description's parent. Reject empty titles/summaries/labels and unknown
envelope fields. A direct document uses its ID as title and links its original.

Optional local schema validates attributes only. Draft 2020-12 is the sole dialect
(default if omitted). Only fragment refs in the supplied schema; no external-file
or network retrieval. Unknown required vocabularies/unsupported dialects fail.
`format` is an annotation. No code or prompts are evaluated.

## Output

Generate `index.md`, resource pages and chosen ordinary entry files. Sort IDs,
attributes, associations and targets; preserve authored reference order. Escape
Markdown text and encode file links. No run timestamps or file IDs in knowledge.
Link original sources and managed configuration. Access guidance retains Product's
explicit / existing-project / verified-local / proposed-change order without
claiming that a linked tool has passed these checks.

Entries are whole owned files, not AGENTS.md patches; existing unowned files fail.
Users point Agents at them explicitly. Entry removal unpublishes the owned entry
on next sync. `output PATH` selects a new output location; sync removes previous
owned outputs but never sources. All entries share the complete index.

## Safe change and recovery

Preview captures input/config/schema/state/target bytes and target identities.
Apply checks that basis again under a permanent cooperative lock. Reject sources
aliasing targets and redirected write components, as well as overlap between
entry/output and config/state. Do not adopt unowned or externally edited files.

Local `.devmeld/state` stores ownership and journal records. Persist before/after
intent and stage identity before writes. Include ownership changes in the same
operation. No-op does not rewrite. Configuration commits survive failed later
sync. Pending operations block new mutations; recovery reverses only verified
effects, preserves external conflicts and is restartable.

Restore bytes/existence, not all ACLs, timestamps or extended attributes. Required
native filesystem operations may fail explicitly; no delete-and-retry fallback.
No cross-file atomic visibility, hostile-editor isolation or power-loss guarantee.
Read at most 8 MiB per file and capture at most 128 MiB per operation. No recursive
source discovery or directory deletion.
