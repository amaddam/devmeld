# 005 Validation Guide

Run only commands marked implemented by the current `tasks.md`; the full target vocabulary in contracts is not a current usage manual.

## First slice: command discovery

Build using the existing toolchain and lockfile:

```text
cargo build --locked --offline -p devmeld
cargo test --locked --offline -p devmeld --test cli
cargo xtask check
```

From an empty temporary directory, invoke the built executable with root help, `resource --help`, `resource add --help`, `entry add --help` and `sync --help`. Use its absolute executable path on the host; this guide does not install or alter PATH. The selected directory must remain empty.

Repeat help with `--context` naming a nonexistent directory. Confirm it remains nonexistent. An unknown group/topic must fail without writing or falling back to unrelated successful help.

## Implemented registration slice (T005-T007)

From a disposable project directory, use an existing document:

```text
devmeld resource add SOURCE --as knowledge/notes --dry-run
devmeld resource add SOURCE --as knowledge/notes --apply
devmeld sync --apply
```

For each `--apply`, inspect the preview and type `apply`. This interim confirmation syntax remains until T012; `--yes` and automatic configuration saves are not implemented. The dry-run must not create `.devmeld`. Successful add creates only configuration/state; sync publishes `.devmeld/output/index.md`, without editing project instructions.

Repeat from a descendant directory with a relative source, then with `--context` naming a new separate location. Verify source/schema/entry/output operands use cwd, logical addresses remain independent and read errors do not fall through to another context. `crates/devmeld/tests/cli.rs` exercises real executable cases, including Windows file-sharing denial and stale bootstrap.

## Implemented organization slice (T008)

The group/resource list/show/move journey in both READMEs is implemented. Run it from a disposable context with `knowledge/notes`; validate exact destination behavior, empty-group-only removal and stable source/identity/association evidence. The CLI tests include separate Windows cross-drive coverage for logical moves.

## Implemented local annotation slice (T009)

Use the description/tag/field examples in both READMEs on a disposable `knowledge/notes` registration before its logical move. Verify group/resource show, explicit update/removal and English/Chinese publication. `--environment test` and `--field environment=test` must produce identical configuration; repeated field assignments (including a shortcut plus the same field) must fail without writes. Local annotations remain owned by their node; inheritance is tested separately below.

Inspect a resource description whose source declares `environment=production` while registration says `environment=test`: generated output must label the source-declared attributes separately from local context annotations, and source bytes/timestamps must remain unchanged. Verify same-value update/no-op sync does not rewrite records, preview is read-only, a changed registration invalidates an older preview, and logical/cross-drive moves retain annotations and existing associations. Current confirmation still uses `--apply` and the word `apply`.

## Implemented inheritance slice (T010-T011)

Use each README's inheritance examples before moving `knowledge/notes`. Verify origins in resource show and both output languages; author text, source attributes and source links must stay independent. Test all four parent-propagate/child-inherit pairs, three-level breaks, duplicate tag origins, closest-field override and removing a local override.

Change both defaults, create explicit and implicit groups/resources, then change defaults back: only new nodes capture new choices. Updates and moves retain saved choices; moved ancestry changes derived origins after sync. A defaults-only change leaves existing node records and generated output unchanged, and the next sync is a no-op.

The owned legacy fixture in `storage.rs` checks plain group strings and missing choices retain fixed false/true meaning even under changed context defaults. Reads, previews, no-op updates and sync must not normalize configuration. Additional real CLI cases cover invalid/opposite flags, descriptive fields not acting as switches, cwd-relative schema after switches, stale previews, and unchanged sources. T012-T015 remain unbuilt.

## Remaining story checks

Preview/cancel/confirm/no-op publication under the simplified interaction; preserve existing transaction/recovery and exact instruction-file boundaries. No paid Agent session is needed for CLI changes.

Record Windows and Linux execution separately in `acceptance.md`. macOS, other clients and unexecuted cross-drive cases remain unverified. Do not claim the Feature accepted while tasks remain unbuilt.
