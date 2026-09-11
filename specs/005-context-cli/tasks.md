# Tasks: Context CLI and Resource Organization

**Input**: [Spec](spec.md), [Plan](plan.md), [CLI contract](contracts/cli.md), [model](data-model.md), [Engineering](../../docs/engineering.md).
**Convention**: Each behavior task owns small RED/GREEN/refactor cycles through its stated boundary. Completed history is retained. Described future syntax is not implemented merely because a document exists.

## Phase 1: Baseline

- [x] T001 Verify the current workspace using `cargo xtask check`, inspect `Cargo.toml`, `.gitignore`, existing command/ownership boundaries and preserve the pending `README.md` / `README.zh-CN.md` edits; record native evidence in `specs/005-context-cli/acceptance.md`.

## Phase 2: US1 - Discover current commands

**Goal**: Users can ask for useful help before selecting or creating a context.
**Independent acceptance**: US1 scenarios 1-3; root/group/operation help succeeds with no writes, unknown topics fail.
**Boundary**: Real executable, application-owned command discovery; no new public domain API.

- [x] T002 [US1] Implement root/group/operation help using vertical TDD in `crates/devmeld/tests/cli.rs`, `crates/devmeld/src/cli.rs` and `crates/devmeld/src/main.rs`; show current supported syntax and avoid context resolution (FR-001/SC-001); depends on T001.
- [x] T003 [US1] Cover explicit unavailable context, short help, unknown topics and no-write behavior with focused RED/GREEN cases in `crates/devmeld/tests/cli.rs` and `crates/devmeld/src/cli.rs`; preserve real mutation validation; depends on T002.
- [x] T004 [US1] Verify the implemented help slice through the real CLI and full native checks; update only actual help usage in `README.md` / `README.zh-CN.md` and record evidence/unbuilt boundaries in `specs/005-context-cli/acceptance.md`; depends on T003.

**Checkpoint**: US1 can be delivered before the rest of the CLI model changes. Do not advertise future flags in live help.

## Phase 3: US2 - Add and publish without mandatory init

**Goal**: Add a real source with an organization address, not a user-invented internal ID, and publish it.
**Independent acceptance**: US2 scenarios 1-4 plus the real source-to-navigation path.
**Boundary**: Resource Organization for identities/addresses; CLI/application for IO and bootstrap.

- [x] T005 [US2] Implement distinct logical addresses and stable resource identity through `crates/resources/src/organization.rs`, `crates/resources/tests/organization.rs`, `crates/devmeld/src/declarations.rs`, `crates/devmeld/src/lib.rs`, `crates/devmeld/src/cli.rs`, `crates/devmeld/src/render.rs` and `crates/devmeld/tests/cli.rs`; test add SOURCE/--as, missing parents, deterministic navigation and association preservation without moving source files (FR-004); define any persisted v0 evolution before applying it to fixtures; depends on T004.
- [x] T006 [US2] Implement explicit/nearest/current-directory context selection, cwd-relative operands and successful first-mutation bootstrap using vertical TDD in `crates/devmeld/src/cli.rs`, `crates/devmeld/src/lib.rs`, `crates/devmeld/src/declarations.rs`, `crates/devmeld/src/storage.rs` and `crates/devmeld/tests/cli.rs`; prove add-then-sync without init, explicit new location and no implicit project entry (FR-002/003); depends on T005.
- [x] T007 [US2] Add regression cases for nearest corrupt/inaccessible/incomplete context, invalid sources, pending recovery and dry-run bootstrap in `crates/devmeld/tests/cli.rs` with fixes in the owning application/storage files; verify no fallback/adoption/partial initialization and source preservation; depends on T006.

## Phase 4: US3 - Organize and describe

**Goal**: Inspect and move nested organization independently from physical files, with explicit descriptions/tags/fields.
**Independent acceptance**: US3 scenarios 1-3, including same leaf names, collisions and unchanged associations.

- [x] T008 [US3] Implement group/resource list/show and logical move plus bounded group removal using vertical TDD in `crates/resources/src/organization.rs`, `crates/resources/tests/organization.rs`, `crates/devmeld/src/cli.rs`, `crates/devmeld/src/inspection.rs`, `crates/devmeld/src/lib.rs`, `crates/devmeld/src/declarations.rs`, `crates/devmeld/src/render.rs` and `crates/devmeld/tests/cli.rs`; reject self-descendant moves and collisions without source deletion (FR-007); depends on T007.
- [x] T009 [US3] Implement optional overall descriptions, tag sets and extensible context fields with equivalent shortcut flags using vertical TDD in `crates/resources/src/organization.rs`, `crates/resources/tests/organization.rs`, `crates/devmeld/src/cli.rs`, `crates/devmeld/src/annotation_args.rs`, `crates/devmeld/src/lib.rs`, `crates/devmeld/src/declarations.rs`, `crates/devmeld/src/inspection.rs`, `crates/devmeld/src/render.rs`, `crates/devmeld/src/language.rs` and `crates/devmeld/tests/cli.rs`; distinguish source-file attributes from context annotations and preserve technical names (FR-005/011); depends on T008.

## Phase 5: US4 - Explicit inheritance

**Goal**: Configurable creation defaults and inspectable computed inheritance.
**Independent acceptance**: Four parent/child combinations, local override, three-level break and default/parent changes.

- [x] T010 [US4] Implement saved inherit/propagate choices and pure effective metadata with provenance in `crates/resources/src/organization.rs` and `crates/resources/tests/organization.rs`; use one failing case at a time for enabled/broken edges, tags and same-name fields, never inherit identity/source/overall description (FR-006); depends on T009.
- [x] T011 [US4] Implement `config set defaults.inherit/defaults.propagate`, explicit overrides and synchronized origin presentation in `crates/devmeld/src/cli.rs`, `crates/devmeld/src/lib.rs`, `crates/devmeld/src/declarations.rs`, `crates/devmeld/src/render.rs`, `crates/devmeld/src/language.rs` and `crates/devmeld/tests/cli.rs`; prove defaults do not mutate existing choices and inherited values are not copied into declarations; depends on T010.

## Phase 6: US5 - Save, inspect, publish

**Goal**: Clear pending/published states and one publication confirmation, retaining managed boundaries.
**Independent acceptance**: US5 scenarios 1-5 through the real CLI, including an existing instruction file.

- [x] T012 [US5] Replace old apply-word interaction with scoped saves, dry-run, sync/recover confirmation, explicit noninteractive confirmation, status, config language and entry attach/create/remove in `crates/devmeld/src/main.rs`, `crates/devmeld/src/cli.rs`, `crates/devmeld/src/lib.rs`, `crates/devmeld/tests/cli.rs` and affected existing tests; preserve previews and report configured versus published, never claim automatic client consumption (FR-008); depends on T011.
- [x] T013 [US5] Verify stale/no-op/cancel/conflict/recovery and exact host preservation under the new CLI in `crates/devmeld/tests/cli.rs`, `crates/devmeld/tests/instructions.rs` and `crates/devmeld/src/storage.rs`; retain existing protection assertions rather than weakening them for new flags (FR-009); depends on T012.

## Phase 7: Full delivery and acceptance

- [x] T014 Update `README.md`, `README.zh-CN.md` and `specs/005-context-cli/quickstart.md` to the verified complete journey, remove superseded human syntax and test every executable example using disposable fixtures; depends on T013.
- [x] T015 Run `cargo xtask check` natively and in isolated Linux, run actual Windows cross-drive cases, review source/domain/ownership boundaries and record results plus unverified macOS/client cases in `specs/005-context-cli/acceptance.md`; report pending Maintainer acceptance separately; depends on T014.

## Dependencies and Execution Order

### Adapter refinement (authorized 2026-09-10)

- [x] T016 Refactor CLI/application coupling on the green baseline: typed mutation/query inputs, structured inspection and prepared-change previews, binary-owned parsing/rendering, no production argv entrypoints; verify a non-CLI registration/query/publication caller plus existing ownership/stale/recovery regressions. Files: `crates/devmeld/src/`, `crates/devmeld/tests/application.rs` and affected fixture helpers. Preserve formats, commands and prior assertions.
- [x] T017 Implement the selected root/group/operation help hierarchy and contextual options/examples in `crates/devmeld/src/cli/`, with real executable regression cases and corresponding README/contract updates; depends on T016. No new command, alias or parser dependency.
- [x] T018 Verify native Windows, isolated Linux and actual cross-drive behavior; inspect application/domain/adapter boundaries, rebuild the release executable and record accurate evidence and remaining platform limits in `acceptance.md`; update design status only after completion; depends on T017. No commit or push is implied.

T001 -> US1 (T002-T004) -> US2 (T005-T007) -> US3 (T008-T009) -> US4 (T010-T011) -> US5 (T012-T013) -> T014-T015.
The address model first appears inside its real add/publication slice, not as an empty shared infrastructure phase. Run affected checks within each task, not only at T015.

## Help as a quick reference (2026-09-10)

- [x] T019 Shorten root/group/operation help into a command reference: one-line purpose, precise operands, all supported options with brief defaults/constraints, at most one relevant example, and immediate write-risk notes. Keep tutorials and detailed semantics in documentation for the future website; do not invent a documentation URL. Change only help, corresponding tests/style/contract text and acceptance evidence; preserve the parser/application behavior and uncommitted T016-T018 work. Run focused help tests and native `cargo xtask check`, then rebuild/inspect release help; prior cross-platform evidence is not a fresh run for this text-only change.

## Managed record readability (2026-09-10)

- [x] T020 Replace managed configuration/receipt/journal JSON with readable TOML,
  preserving exact byte evidence, path semantics, deterministic no-ops and recovery;
  reject and preserve legacy/mixed records without automatic migration. Cover
  real CLI publication, Unicode/CRLF/quotes/backslashes, duplicate keys and a legacy
  writer appearing after preview; run native Windows and isolated Linux checks.

## Readable resource pages (2026-09-10)

- [x] T021 Replace ID-based card filenames with logical-address publication paths
  through a bounded path adapter and one shared destination map. Use vertical TDD
  for real publication and move/link behavior; preserve internal identities,
  associations and sources. Cover portable names/collisions, old owned file
  withdrawal, conflicts, no-ops and recovery; update affected contracts, fixtures,
  both READMEs and acceptance evidence, run Windows/isolated Linux checks and
  regenerate the local Shop context via confirmed sync. No commit or push implied.

## Reading-oriented publication (2026-09-10)

- [x] T022 Make generated Markdown reading-oriented: short card ownership
  comment, meaningful descriptions and original-source/access links, final
  effective context values without saved controls/config links/empty sections;
  centralize maintenance rules in index/entries, preserve detailed CLI show,
  source/context distinctions and ancestor origins. Use focused RED/GREEN cases,
  update publication assertions and owning docs, run Windows/Linux/cross-drive
  checks, rebuild release and regenerate Shop through normal confirmed sync.
  Preserve prior work; no commit/push or dependency/format changes.

## Rich Shop context example (2026-09-10)

- [x] T023 Expand Shop's group/resource annotations and document reproducible
  setup, publication and existing-context updates in both example READMEs.
  Demonstrate group-owned descriptions/tags/fields, explicit propagation,
  child inheritance and local overrides without changing domain rules. Replay
  the documented commands on clean Windows/Linux fixtures, refresh the existing
  Shop through DevMeld commands, verify source/data preservation and no-op sync,
  and record evidence. No production-code change, dependency, commit or push.

## Generated group documents (2026-09-10)

- [x] T024 Publish every group's own document and hierarchical direct-child
  navigation through one combined group/resource destination map. Use a focused
  failing real-CLI case, implement coherently, then cover naming conflicts,
  nested/empty groups, origins, moves/removal, protected writes and recovery.
  Update superseded presentation assertions and owning docs/READMEs, verify
  Windows/Linux/cross-drive behavior and refresh Shop via normal sync. Preserve
  source/configuration, resource paths and earlier work; no dependency/commit/push.

## Library-based readable Markdown (2026-09-11)

- [x] T025 Generate Markdown with `pulldown-cmark-to-cmark` in the application
  adapter; use code spans for field keys and technical literals instead of a
  handwritten blanket escape function. Remove inherited-origin annotations from
  reading pages, retaining effective values and detailed CLI inspection. Start
  with a real CLI RED, verify literal text/link semantics, both languages, no-op,
  protected writes and recovery on Windows/Linux and across drives. Update owning
  presentation docs and refresh Shop with normal sync, preserving config/sources.
  No domain, format-version, commit or push change.

## Compact ownership receipts (2026-09-11)

- [x] T026 Replace persistent whole-file bodies with SHA-256 fingerprints and
  physical identities; retain exact shared-entry insertions and temporary recovery
  images. Read former full-body TOML claims without adopting current file edits;
  compact only on actual saves or an explicitly previewed/confirmed sync. Preserve
  no-op/stale/conflict/recovery behavior, test Windows/Linux/cross-drive cases,
  and shrink Shop through normal sync without rewriting config, sources or output.
  No merge with context.toml, format-version bump, commit or push.

## Parallel Opportunities

None for implementation: adjacent slices change the same CLI/declaration/rendering files. Windows/Linux verification may run independently on isolated copies once the relevant slice is frozen. No task marker authorizes delegation.

## Delivery Strategy

Deliver US1 first and explicitly report the rest unbuilt. Then complete the registration-to-publication journey before metadata and inheritance. Each story has observable acceptance; completed tasks do not imply full Feature acceptance. No automatic deployment, commit or push.
