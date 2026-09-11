# Local file and command contract (003)

Current representation note (2026-09-10): the original JSON and command examples
below are historical. The active [005 contract](../../005-context-cli/contracts/cli.md#current-persisted-representation-2026-09-10)
uses TOML for managed configuration, ownership and recovery, with v0 markers.
The [readable-page amendment](../../005-context-cli/contracts/cli.md#readable-page-paths-2026-09-10)
also supersedes the historical `r-<id>.md` layout below with logical-address cards.
The [005 reading view](../../005-context-cli/contracts/cli.md#reading-view-2026-09-10-t022)
supersedes repeated card notices, default summaries and configuration links below;
source/access links and managed-write protections remain applicable.
Authored JSON descriptions/schema and the safety/link rules still apply.

Feature-local version 1, not a ratified permanent public machine API. Unknown
control fields, unsupported versions and invalid semantics fail; no migration.

## Human entrypoint

```text
devmeld --context PATH init [--output PATH] [--entry PATH] [--language en|zh-CN]
devmeld --context PATH resource add ID --document PATH
devmeld --context PATH resource add ID --description PATH [--schema PATH]
devmeld --context PATH resource remove ID
devmeld --context PATH access add RESOURCE TOOL
devmeld --context PATH access remove RESOURCE TOOL
devmeld --context PATH entry add PATH
devmeld --context PATH entry remove PATH
devmeld --context PATH output PATH
devmeld --context PATH language en|zh-CN
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
`publication: {directory, entries, language?}`. Entries: `{kind: "file", path}`.
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

### Output language

`publication.language` accepts exactly `en` or `zh-CN`. Absence means `en`;
English is omitted on serialization to preserve existing version-1 configuration
bytes and no-op behavior. This is an additive version-1 option: existing files
need no migration; older binaries reject the new Chinese field rather than
silently ignoring its meaning. Null, unknown codes and duplicate language fields
or init options are errors, not automatic fallbacks.

Set it with `init --language zh-CN` or `language zh-CN`, using the usual preview
and confirmation. The latter changes configuration only; a subsequent confirmed
`sync` updates all generated files together under existing recovery rules.
There is one selected language, not two parallel trees or per-entry languages.
Switching does not change target filenames, link destinations or resource sets.

Only fixed generated headings, maintenance/freshness instructions, source labels,
direct-document summaries and access-guidance prose are localized. Chinese text
uses formal technical wording. Authored titles, summaries, attribute keys/values,
reference labels, commands, paths, IDs and technical names such as ssh, http,
curl and JSON remain unchanged apart from existing Markdown escaping. A source
already written in English remains English in Chinese output, and vice versa.
No translation engine, locale detection, Agent response-language instruction,
CLI/help translation or additional dependency is introduced.

### Local path and link representation

Input paths are native filesystem paths, relative to the bases defined above or
absolute on this machine. Windows drive-relative forms such as `D:notes.md` are
ambiguous and rejected, as are rooted paths without a drive (`\notes.md`);
use `D:/knowledge/notes.md`. URLs, UNC and device paths
are not indexing inputs. Store remote addresses as description attributes, not
as file references; DevMeld does not fetch them or traverse remote indexes.
Local disks are an operating assumption, not an OS-level check of hidden mounts.

Generated relative links are based on the containing Markdown file, never the
reader's working directory. Same-root destinations retain relative links.
Different Windows disk roots use `file:///D:/knowledge/notes.md`, with no remote
host. UTF-8 path segments are percent-encoded (including spaces, `#`, `%` and
parentheses); filesystem paths are not already-encoded URLs. Windows canonical
disk prefixes are rendered as normal drive paths, not `\\?\` URI authorities.

Each file-URI link includes a readable absolute local path in the selected output
language. A reader can use that path when its Markdown viewer blocks file links;
this does not bypass access controls or promise universal clickable links.
No source copy, symlink, DevMeld resolver process or cross-machine root mapping
is created. Relative links require preserved directory relationships; absolute
links require the same machine paths. Moving files requires configuration/source
reference maintenance and a new sync, not hand-editing generated links.

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

The [005 compact receipt](../../005-context-cli/contracts/cli.md#compact-ownership-evidence-2026-09-11-t026)
stores permanent whole-file fingerprints/identities rather than bodies. Full
before/after images remain temporary recovery evidence. The preceding TOML body
receipt can be compacted via an explicitly previewed/confirmed receipt-only sync;
that maintenance change does not regenerate otherwise unchanged output.

Restore bytes/existence, not all ACLs, timestamps or extended attributes. Required
native filesystem operations may fail explicitly; no delete-and-retry fallback.
No cross-file atomic visibility, hostile-editor isolation or power-loss guarantee.
Read at most 8 MiB per resource/output file. Internal records and captured
before/after data are bounded at 128 MiB; oversized operations fail. Records
encode managed UTF-8 text as text, not JSON byte arrays. Initial pending journals
are published only after their complete contents are staged. Commit staging
identity is recorded so a partial commit record remains safely removable.

Failure or interruption during pre-journal preparation may leave unique staging files/empty
directories without changing targets; no automatic orphan adoption or recursive
cleanup. Malformed/external recovery data is reported, not guessed away.
Cross-drive targets use sibling staging on their own volumes; no cross-drive
rename or hard link is required. No recursive source discovery or directory deletion.
