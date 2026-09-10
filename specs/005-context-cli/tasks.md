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

T001 -> US1 (T002-T004) -> US2 (T005-T007) -> US3 (T008-T009) -> US4 (T010-T011) -> US5 (T012-T013) -> T014-T015.
The address model first appears inside its real add/publication slice, not as an empty shared infrastructure phase. Run affected checks within each task, not only at T015.

## Parallel Opportunities

None for implementation: adjacent slices change the same CLI/declaration/rendering files. Windows/Linux verification may run independently on isolated copies once the relevant slice is frozen. No task marker authorizes delegation.

## Delivery Strategy

Deliver US1 first and explicitly report the rest unbuilt. Then complete the registration-to-publication journey before metadata and inheritance. Each story has observable acceptance; completed tasks do not imply full Feature acceptance. No automatic deployment, commit or push.
