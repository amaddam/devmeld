# Contract: Shared Project Instruction Entries

**Status**: 004 implementation baseline authorized on 2026-09-09; [acceptance remains separate](../plan.md#review-gates).
Retains the applicable behavior of [003 local files](../../003-durable-context/contracts/local-files.md),
but replaces its maintenance-format baseline as specified below. No public
machine API or Agent-specific runtime is added.

## 1. Explicit commands and configuration

```text
devmeld --context PATH init [--output PATH] [--entry PATH] [--instruction-entry PATH] [--language en|zh-CN]
devmeld --context PATH entry add PATH [--kind file|instructions]
devmeld --context PATH entry remove PATH
devmeld --context PATH sync
devmeld --context PATH recover
```

`--entry` and default `entry add` retain whole-file semantics. Only
`--instruction-entry` or `--kind instructions` selects a shared host. Multiple
different paths may be registered; duplicate/resolved-alias targets and
conflicting kinds are rejected, not converted. Removal resolves the existing
registration by path and does not need a kind flag. There is no automatic search
for AGENTS.md, override files, client settings or project roots.

All commands keep 003's read-only preview default and `--apply` plus literal
`apply` confirmation. Configuration commands change declarations, not host
content; actual attachment/update/detachment happens on a separately confirmed
sync. Show this distinction in preview; attachment is not a format upgrade. Publication
preview identifies the host, the insertion being created/replaced/removed, and
whether the file itself must be created. Cancellation writes nothing.

All initialization uses `format_version: 0`, with or without instruction entries.
Config retains the resource/publication fields, with entries such as:

```json
{
  "format_version":0,
  "resources": [],
  "access": [],
  "publication": {
    "directory": ".devmeld/output",
    "entries": [
      {"kind": "file", "path": "../project/CONTEXT.md"},
      {"kind": "instructions", "path": "../project/AGENTS.md"}
    ]
  }
}
```

Existing language default/serialization, resource and access semantics remain.
The only supported config format is v0; it permits `file` and `instructions`.
Unsupported versions (including v1), kinds and fields fail explicitly. Managed
config is changed through DevMeld, not hand edited to bypass version/ownership checks.

## 2. Supported host and exact insertion boundary

The host is an ordinary local UTF-8 file, with an optional initial UTF-8 BOM.
Reject invalid UTF-8, NUL and UTF-16/32; no lossy decoding or transcoding.
This is a plain Markdown/text instruction entry, not a syntax-aware editor for
front-matter-dependent or other structured instruction formats. The selected
reader must support this text arrangement. Preview never promises compatibility
with every file that happens to have a Markdown extension.

Reserve the raw byte prefix `<!-- devmeld:entry:`. Use exactly:

```text
<!-- devmeld:entry:v0:begin -->
...generated navigation and maintenance guidance...
<!-- devmeld:entry:v0:end -->

```

The marker version describes this first insertion envelope, independently of the
config/receipt version; `entry:v0` does not imply support for v1 maintenance records.
The illustration's body is a placeholder, not literal
output. There is one line ending after the begin marker, generated body lines,
one line ending immediately before the end marker, and exactly two line endings
after the end marker. All these bytes belong to the insertion.

On first attachment:

1. Require no existing ownership claim at that target and no reserved prefix
   anywhere in the host, including raw examples in code fences or comments.
2. Select CRLF if the host's first LF is preceded by CR, otherwise LF. With no
   LF, use LF. Do not modify any host line ending, including mixed endings.
3. Insert at byte zero, after the initial BOM if present. Preserve the old host
   bytes as the unchanged suffix; do not add a newline to the author's last line.
   A new file uses UTF-8 without BOM and LF.

For subsequent maintenance require the recorded insertion to occur exactly
once and contain the only two reserved-prefix occurrences in the host, in the
recorded begin/end order. Compare the entire insertion, including separators,
not only its body or an old offset. Missing, modified, duplicate, nested,
truncated or unknown-version markers are conflicts. Raw examples are not adopted
or silently ignored; use `&lt;!-- devmeld:entry:...` in authored examples instead.

With current bytes `prefix + recorded_insertion + suffix`, update only the
insertion or detach it as `prefix + suffix`. Preserve current outside bytes,
even if they differ from the previous publication. Do not relocate a uniquely
matching insertion just because outside edits shifted it. Retain its recorded
newline style on update. Outside edits may affect the client's interpretation;
this byte contract does not validate all Markdown or instruction precedence.

No-op leaves host bytes and modification time unchanged. Detach leaves the host
present, even if empty or BOM-only, and releases only the insertion claim.
Do not invent a replacement entry when owned content is absent.

## 3. Generated meaning and links

Generate one small section titled `Project context` / `项目上下文`, containing:

- A link to this context's common published `index.md`, with guidance to follow
  the navigation and read resources relevant to the task.
- A statement that these files are readable without running DevMeld.
- A distinction between authored sources, edited according to their owners'
  rules, and this generated insertion/navigation, maintained through DevMeld.
- Originally a configuration maintenance link was included. The accepted
  [005 reading view](../../005-context-cli/contracts/cli.md#reading-view-2026-09-10-t022)
  removes it; configuration inspection remains a management operation, not part
  of the Agent's reading path. The ownership guidance and insertion envelope remain.

Do not copy resource lists, facts or tool choices into the entry. Do not grant
installation/execution authority or override higher-priority instructions.
English and Chinese change generated labels, not IDs, paths, commands or
technical names such as SSH/HTTP. Surrounding text is never translated.

Reuse 003's escaped Markdown links: relative destinations on the same local
root; empty-authority `file:///D:/...` plus a readable native-path fallback for
another Windows drive. Links are calculated from the host's location, not the
context root. No UNC/remote index or source copying is introduced. Render from
current config and publish the referenced navigation in the same planned sync;
a broken publication cannot be reported as successful attachment.

## 4. One current maintenance baseline

This is an unreleased development design, not a versioned public protocol.
`format_version: 0` and the `entry:v0` envelope are internal draft discriminators;
they do not declare a product release or a stable v0 compatibility line.
Do not bump these merely because the design changes. Validate the actual current
record shape and evidence; incompatible development records remain untouched.
Generated index/resource content does not receive a release/version heading.

V0 ownership state now uses `.devmeld/state/owned.toml` (the 2026-09-10
[representation amendment](../../005-context-cli/contracts/cli.md#current-persisted-representation-2026-09-10)
replaces JSON; earlier encoding examples are historical). The accepted
[compact-receipt amendment](../../005-context-cli/contracts/cli.md#compact-ownership-evidence-2026-09-11-t026)
replaces permanent whole-file bodies with fingerprints, without changing exact
instruction insertions or temporary full-image recovery. Its typed target map is
illustrated below in JSON notation (the persisted file is TOML):

```json
{
  "format_version":0,
  "context_root": "<canonical context root>",
  "surfaces": {
    "<canonical whole-file target>": {
      "kind": "whole_file",
      "observed": {"sha256": "<64 lowercase hex digits>", "identity": "<physical file identity>"}
    },
    "<canonical instruction target>": {
      "kind": "instruction_entry",
      "insertion": "<exact published insertion, including separators>"
    }
  }
}
```

Angle-bracket values above denote schema examples, not actual paths or content.
The config itself remains a whole-file claim. No target can have both kinds.
Record insertion/recovery text as TOML strings, not byte-number arrays. Reject unknown fields,
invalid envelope evidence, duplicate or normalized-alias target keys, wrong
context provenance and unknown versions. Canonical root comparison uses the
same platform path semantics as target resolution.

The root/target binding rejects copying a receipt into a different local context;
it is not cryptographic authority or a new Product identity. Do not accept a
manually copied block by matching its text. Moving/rebinding a context's managed
state or deliberately forged records is not a supported automatic migration.

Config, receipt and journal all use v0 in the new implementation. Whole-file-only,
insertion-only and mixed publication use the same model, serializers and
transaction rules. Adding/removing an entry never changes the format. There is
no v1 model reader/writer, conditional upgrade, downgrade or conversion command.
Reading a version envelope to report unsupported input is not a legacy adapter.

V0 journals retain the established operation evidence: `committed`, `steps`,
manifest/commit-file identities and each step's complete before/after observations
and sibling paths. They use `format_version: 0` and `context_root`, validated
independently before following any recorded target or staging path for recovery.
An unsupported journal is not executable merely because config is current.

Unsupported config/receipt/pending records must fail before maintenance writes,
including creation of locks, staging, backups or rewritten records. Malformed,
missing or mixed records are not a fresh-context shortcut. Normal operations
require a valid current baseline, except explicit initialization of fresh state.
Recovery instead uses a valid current journal as its operation authority: an
interrupted v0 initialization may legitimately lack config/receipt, so requiring
a complete normal baseline first must not make current recovery impossible.

### Existing development data

T026 permits a narrow conversion of the preceding full-body TOML receipt during
normal confirmed sync or actual saves, as specified in 005. It does not migrate
JSON records or unsupported journals. The original 004 restriction below applies
to those unsupported formats, not this accepted TOML refinement.

There is no automatic migration or reset in 004. Leave unsupported records and
their associated files intact. Existing generated Markdown remains readable;
dropping maintenance compatibility does not delete or rewrite its content.

An operator can explicitly prepare a separate fresh context with unused state,
output and ordinary-entry paths, and re-register the same author-owned sources.
Select non-overlapping maintenance targets; a new context root does not authorize
adoption of another context's output or bypass a shared host's marker conflict.
Do not rename a format field, copy a receipt or delete `.devmeld` to bypass this.
No whole-machine ownership scan is introduced.

If an old pending operation exists, preserve its records and targets. The program
may point to its matching historical implementation for a separate authorized
recovery, but does not invoke it, bundle it or decode its operation automatically.
When that path is unavailable, report the unresolved state. A concrete need for
in-place migration/recovery requires a separately scoped decision, not a generic
migration framework or silent cleanup added to this feature.

## 5. Operation and recovery rules

Resolve and capture only selected inputs/targets, with 003's path, size and
cooperative-writer restrictions. Check path overlap and observed physical aliases
against sources, config/state, generated files and other entry targets, including
obsolete targets being withdrawn. Do not treat two hard-link paths as permission
to maintain two independent surfaces.

Before apply, recheck the complete captured basis: bytes, presence and physical
identity, including shared hosts and maintenance records. Recheck no-op plans
too. Pre-preview outside changes are valid; post-preview changes require a new
preview. Do not require a shared host to keep its inode across independent edits
between publications.

Stage complete merged host images with explicit ownership effects, then use the
existing journal and per-context cooperative lock. Each step still records a
full before/after image. A whole-file replacement is a physical operation, not
permission to record the shared host as wholly owned.

Retain the existing three recovery phases:

- Before journal publication, failed staging leaves targets unchanged. Unique
  staging files/empty directories may remain; report them without automatic
  orphan adoption or recursive cleanup.
- With an uncommitted journal, reverse only verified operation effects. External
  changes to any part of a shared host can make rollback uncertain; stop without
  overwriting them or clearing unresolved recovery state. An interrupted creation
  may restore prior absence, unlike normal detachment of a committed entry.
- With a committed journal, perform verified staging/journal cleanup only, never
  target rollback. Later author edits do not block cleanup merely because target
  bytes differ; altered cleanup artifacts can still produce an explicit conflict.

Previously committed configuration is not undone by recovery of a later publication.

Missing ownership, edited insertion, incompatible ownership mode, overlap,
unsupported format, stale basis and unresolved recovery must produce nonzero
outcomes with the affected path and reason. Do not suggest an implicit force,
adoption or host deletion. No cross-file atomicity, power-loss guarantee, hostile
writer isolation or automatic three-way merge is added to 003.
