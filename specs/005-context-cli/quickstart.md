# 005 Validation Guide

This guide covers the implemented 005 command journey. Use disposable contexts and the host's built executable; final Maintainer acceptance is recorded separately.

## First slice: command discovery

Build using the existing toolchain and lockfile:

```text
cargo build --locked --offline -p devmeld
cargo test --locked --offline -p devmeld --test cli
cargo xtask check
```

From an empty temporary directory, invoke the built executable with root help, `resource --help`, `resource add --help`, `entry attach --help` and `entry create --help` and `sync --help`. Use its absolute executable path on the host; this guide does not install or alter PATH. The selected directory must remain empty.

Repeat help with `--context` naming a nonexistent directory. Confirm it remains nonexistent. An unknown group/topic must fail without writing or falling back to unrelated successful help.

For the T017 layout, root and group help list direct children without their full
option inventories. `group list --help` must explain only its logical group
scope, examples and applicable options, without tool/schema/inheritance text.
`resource add --help` must still explain source paths, logical addresses, schema
and its supported annotation/inheritance choices. `-h` remains equivalent.

## Non-CLI application boundary (T016)

Run `cargo test --locked --offline -p devmeld --test application`. These callers
use `ContextLocation`, `Mutation`, `Query`, structured `Inspection` and
`PlanPreview`, never CLI arguments or rendered report parsing. Verify registration
then publication, inheritance origins after a logical move, missing-source
registration inspection, explicit native input bases, entry attach/detach and
stale/no-op refusal. Dropping a prepared plan must not write anything. Existing
CLI and interruption tests still exercise the same apply/recovery path.

This is a reusable in-process application API, not a delivered GUI or a frozen
machine wire protocol. CLI confirmation and output formatting remain in the binary.

## Implemented registration slice (T005-T007)

From a disposable project directory, use an existing document:

```text
devmeld resource add <SOURCE_FILE> --as knowledge/notes --dry-run
devmeld resource add <SOURCE_FILE> --as knowledge/notes
devmeld sync
```

Registration saves directly. For `sync`, inspect the preview and answer `y` once; use `sync --yes` in noninteractive checks. A command-ending `--dry-run` never writes. The dry-run must not create `.devmeld`. Successful add creates only configuration/state; sync publishes `.devmeld/output/index.md`, without editing project instructions.

Repeat from a descendant directory with a relative source, then with `--context` naming a new separate location. Verify source/schema/entry/output operands use cwd, logical addresses remain independent and read errors do not fall through to another context. `crates/devmeld/tests/cli.rs` exercises real executable cases, including Windows file-sharing denial and stale bootstrap.

## Implemented organization slice (T008)

The group/resource list/show/move journey in both READMEs is implemented. Run it from a disposable context with `knowledge/notes`; validate exact destination behavior, empty-group-only removal and stable source/identity/association evidence. The CLI tests include separate Windows cross-drive coverage for logical moves.

## Readable page amendment (T021)

### Hierarchical group documents (T024)

Register `code`, `code/http`, an empty group, `code/backend` and
`code/http/client`, with distinct descriptions and inherited fields. Sync in
both languages. Follow index → `resources/code/code.md` → `http/http.md` →
`client.md` → original source. Each group shows its own information and only
direct children; parent/index links resolve. No child metadata is expanded in
the total index. Verify implicit, empty, Unicode and escaped group paths too.

Move/remove groups and sync: old unchanged owned group pages are withdrawn,
sources and IDs are preserved, and new links resolve. A group/card collision
such as `code/code`, unowned destination or externally edited group page must
block writes. Storage fault injection covers create/update/move/remove through
the commit boundary. Replay both Shop README recipes and verify the generated
group documents independently of CLI show. Repeated sync must remain a no-op.

### Resource card addresses

After registering `knowledge/notes`, sync publishes
`.devmeld/output/resources/knowledge/notes.md`. Move it to `reference/notes`:
the save retains the old publication, then sync creates `resources/reference/notes.md`,
updates generated links and removes only the old unchanged owned card. The source,
internal ID and access associations remain unchanged; a further sync is a no-op.
Repeat with a group subtree and incoming access association. External bookmarks
are not maintained; use the index entry. Empty output containers may remain.

The real executable tests also cover equal leaf names in different groups,
Unicode/spaces, reserved device names, literal percent signs and trailing dots.
Case-folded spelling and file/directory collisions must fail before publication.
Storage tests use genuine former-layout ownership receipts to verify replacement,
external edit/unowned destination refusal and rollback at each mutation boundary.

## Implemented local annotation slice (T009)

T022 reading view: cards show a short ownership comment, title, supplied
description/summary and source link, without placeholder prose, configuration
links or saved inheritance controls. Index/entries carry the full maintenance
rules. Verify both languages and a document without any description; no empty
metadata blocks should appear. CLI show must retain its maintenance detail.

Use the description/tag/field examples in both READMEs on a disposable `knowledge/notes` registration before its logical move. Verify group/resource show, explicit update/removal and English/Chinese publication. `--environment test` and `--field environment=test` must produce identical configuration; repeated field assignments (including a shortcut plus the same field) must fail without writes. Local annotations remain owned by their node; inheritance is tested separately below.

Inspect a resource description whose source declares `environment=production` while registration says `environment=test`: generated output must label the source-declared attributes separately from local context annotations, and source bytes/timestamps must remain unchanged. Verify same-value update/no-op sync does not rewrite records, preview is read-only, a changed registration invalidates an older preview, and logical/cross-drive moves retain annotations and existing associations.

## Implemented inheritance slice (T010-T011)

Use each README's inheritance examples before moving `knowledge/notes`. Verify origins in resource show and effective values in both publication languages; author text, source attributes and source links must stay independent. Test all four parent-propagate/child-inherit pairs, three-level breaks, duplicate tag origins, closest-field override and removing a local override.

Change both defaults, create explicit and implicit groups/resources, then change defaults back: only new nodes capture new choices. Updates and moves retain saved choices; moved ancestry changes derived origins after sync. A defaults-only change leaves existing node records and generated output unchanged, and the next sync is a no-op.

The owned legacy fixture in `storage.rs` checks plain group strings and missing choices retain fixed false/true meaning even under changed context defaults. Reads, previews, no-op updates and sync must not normalize configuration. Additional real CLI cases cover invalid/opposite flags, descriptive fields not acting as switches, cwd-relative schema after switches, stale previews, and unchanged sources.

For T025's refinement of T022, verify a tag contributed by both parent and child
appears once in the card, without origin labels; inherited fields and local
overrides display their final effective values. Detailed origins and controls
remain in CLI show. Keys such as `use_when` use code spans without backslash
escapes; authored syntax-like text remains literal, not HTML or Markdown commands.
A source attribute and a context field with the
same key must remain under distinct sections, even when their values conflict.

## Save, inspect and publish (T012-T013)

T026 ownership compaction: a fresh context's `state/owned.toml` must contain
SHA-256/physical identity evidence, not copies of config or generated whole files.
Shared hosts retain only their insertion. On a disposable preceding full-body
TOML receipt, reads/status/no-op saves/preview must preserve bytes and timestamps.
An otherwise unchanged sync previews only `owned.toml` and requires confirmation;
all config/source/output/host bytes and timestamps stay unchanged. Repeat sync is
a no-op. Verify same-length edits and identical-content physical replacements
still conflict, malformed/mixed fingerprint shapes fail, and receipt-only
interruption/rollback/commit cleanup retain exact data and stale-preview checks.
The storage suite also checks a 3 MiB whole file leaves a receipt below 1 KiB.

Starting with the disposable `knowledge/notes` registration above and an authored UTF-8 `AGENTS.md`:

```text
devmeld entry attach AGENTS.md
devmeld entry create CONTEXT.md
devmeld config set language zh-CN
devmeld status
devmeld sync --dry-run
devmeld sync
devmeld status
devmeld entry remove AGENTS.md
devmeld sync
```

Entry/config saves must leave generated files and authored instruction bytes unchanged until sync. Initial status reports pending publication and unpublished entries; after sync it reports matching generated content and published entries. Detach plus sync restores the host's authored bytes, including BOM, mixed line endings and missing final newline. Ordinary entries remain independent; no implicit Agent discovery is claimed.

Status reuses the read-only publication preparation and rechecks its inputs. It reads current sources, unlike registration list/show, and reports blocked/unverified for missing inputs, conflicts or pending recovery. It compares generated bytes, not historical source freshness: editing a linked document body can leave the generated link unchanged. No new status counter, source hash registry or persisted snapshot is created.

Use a real terminal to test cancel (blank/n), accept (y) and stale confirmation: pause at the prompt, change configuration using another DevMeld invocation, then confirm; it must reject the stale preview without publishing. Noninteractive sync/recover without `--yes` must fail if changes exist; no-op must recheck and succeed without a prompt. `--yes` cannot override modified managed files. Duplicate/mixed confirmation flags and removed `--apply`, `language VALUE`, `entry add` syntax must fail.

Optional `init <CONTEXT_DIR> --dry-run` leaves the selected new path absent; `init <CONTEXT_DIR>` saves only an empty context there. Combining it with `--context` is rejected. Existing contexts/claims are not adopted.

Recovery uses the same confirmation interaction. Preview with `recover --dry-run`; confirm once with `recover` or explicitly use `recover --yes`. Existing storage fault-injection tests exercise each mutation/commit/rollback boundary. Pending operations block ordinary saves/status. No paid Agent session is needed for these CLI changes.

Record native Windows, isolated Linux, real C:/D: execution and README/example replay separately in `acceptance.md`. macOS and additional Agent Clients remain unverified. Completed implementation and tests do not themselves approve Maintainer acceptance.
