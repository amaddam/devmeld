# Tasks: Project Instruction Entry Integration

**Input**: Accepted 004 Spec, Plan, model and shared-instructions contract.
**Authorization**: Maintainer authorized continued task generation, implementation
and self-verification on 2026-09-09. Acceptance and external-client authority remain separate.
**Convention**: Follow `docs/engineering.md#behavioral-development`; each behavior
task owns its RED/GREEN/refactor cycles. Paths are repository-relative.

## Phase 1: Baseline

- [x] T001 Verify current workspace/ignore rules in Cargo.toml and .gitignore (including example-specific exclusions), run cargo xtask check, preserve existing changes and record the starting evidence in specs/004-project-entry-integration/acceptance.md; blocks T002.

## Phase 2: US1 — Explicit instruction attachment (P1)

**Goal**: Register an explicit host, publish a small common-navigation entry,
and leave original instructions/sources untouched.
**Boundary**: CLI and `devmeld::prepare`/Plan; Context Publication owns write policy.
**Independent acceptance**: Fresh/existing hosts receive exactly the confirmed
insertion; ordinary reading reaches original sources without DevMeld running.
Real-client startup consumption is checked in T014, not inferred here.

- [x] T002 [US1] Establish the single v0 baseline and typed ownership effects in crates/devmeld/src/storage.rs and crates/devmeld/src/declarations.rs using whole-file workflow and unsupported-record tests in crates/devmeld/tests/workflow.rs; preserve ordinary publication, reject v1/unknown state before writes, and bind records to the context (FR-008/010/011, SC-007; depends on T001).
- [x] T003 [US1] Implement explicit instruction registration and previewed attachment through crates/devmeld/src/lib.rs, main.rs, render.rs, instructions.rs and crates/publication/src/lib.rs with vertical TDD in crates/devmeld/tests/instructions.rs; distinguish config from publication, absent from unowned hosts, and insertion from whole-file ownership (FR-001/002/003/004/005/008/012; depends on T002).
- [x] T004 [US1] Verify and complete byte-envelope behavior in crates/devmeld/src/instructions.rs and crates/devmeld/tests/instructions.rs for BOM, mixed LF/CRLF, no final newline, invalid encoding and reserved-marker examples; reject ambiguity without adoption or normalization (FR-002/008, SC-001/004; depends on T003).

## Phase 3: US2 — Maintain only the insertion (P1)

**Goal**: Preserve current author text across updates and detachment.
**Boundary**: Same publication path; no separate patch executor.
**Independent acceptance**: Edit outside the block, change generated content,
repeat sync and detach; compare outside bytes and remaining navigation.

- [x] T005 [US2] Implement outside-edit/atomic-save maintenance and unchanged publication using crates/devmeld/src/instructions.rs, storage.rs and crates/devmeld/tests/instructions.rs; match exact insertion evidence, not old host inode, and preserve no-op bytes/mtime (FR-007/011, SC-003/004; depends on T004).
- [x] T006 [US2] Implement insertion-only detachment and multi-host lifecycle in crates/devmeld/src/lib.rs, storage.rs and crates/devmeld/tests/instructions.rs; retain empty/BOM-only/newly created hosts, release only the claim, cover add/remove before sync and re-add before detach (FR-005/009/012, SC-001/002; depends on T005).
- [x] T007 [US2] Complete language/output relocation and local-link behavior in crates/devmeld/src/render.rs, language.rs and crates/devmeld/tests/instructions.rs; both en/zh-CN lead to the same resources, use existing relative/cross-drive link rules and preserve SSH/HTTP and author text (FR-003/004/006, SC-002; depends on T006).

## Phase 4: US3 — Refuse uncertain writes and recover (P1)

**Goal**: Shared ownership does not weaken operation/recovery guarantees.
**Boundary**: Captured Plan plus existing staged transaction and recovery.
**Independent acceptance**: Every conflict preserves external bytes; current
journals recover verified effects and never roll back prior successful config.

- [x] T008 [US3] Fix stale no-op acceptance and exercise stale host/source/config/receipt inputs in crates/devmeld/src/storage.rs and crates/devmeld/tests/instructions.rs; full basis checks precede writes and no-op return (FR-007/010, SC-004; depends on T007).
- [x] T009 [US3] Enforce copied/missing/edited evidence, ownership-mode conflicts and physical aliases in crates/devmeld/src/storage.rs, declarations.rs, instructions.rs and crates/devmeld/tests/instructions.rs; cover sources, config/state, desired/obsolete targets and another context without scanning the machine (FR-008/010/012, SC-004/007; depends on T008).
- [x] T010 [US3] Exercise and correct every current initialization/attachment/update/detachment and recovery mutation boundary in crates/devmeld/src/storage.rs tests; distinguish pre-journal remnants, uncommitted verified rollback and committed cleanup, including external edits and incomplete v0 initialization (FR-005/008/010, SC-005; depends on T009).

## Phase 5: Cross-cutting and final verification

- [x] T011 Verify std-only domain dependency and visibility controls through tools/xtask/src/boundaries.rs, Cargo.toml and affected crate manifests using cargo xtask boundaries and existing valid/forbidden probes; no new dependency or domain is authorized (FR-004/010; depends on T010).
- [x] T012 [P] Update README.md, README.zh-CN.md, examples/README.md and specs/004-project-entry-integration/quickstart.md to match implemented CLI and single-format behavior; retain old example state and distinguish published files from tested client discovery (FR-001/005/011/013; depends on T010).
- [x] T013 Run full native Windows and isolated Linux checks plus real Windows cross-drive workflows from crates/devmeld/tests/instructions.rs, workflow.rs and storage.rs; record commands, actual outcomes and unverified platforms in specs/004-project-entry-integration/acceptance.md (SC-001/002/003/004/005/007; depends on T011/T012).
- [x] T014 Run the authorized named fresh Agent Client fixture in specs/004-project-entry-integration/quickstart.md and record actual navigation/source reads in acceptance.md, without a DevMeld process or task-supplied context path; missing authorization/environment remains a gap (FR-013, SC-006; depends on T013).
- [x] T015 Review implementation against specs/004-project-entry-integration/spec.md, plan.md and tasks.md, reconcile evidence in acceptance.md and hand off remaining gaps without claiming Maintainer acceptance or pushing remotely (all requirements; depends on T013 and T014 outcome).

## Dependencies and parallel opportunities

T001 → US1 (T002–T004) → US2 (T005–T007) → US3 (T008–T010)
→ T011/T012 → T013 → T014 → T015.

US1/US2/US3 share implementation files; their complete tasks run sequentially.
There is no independent per-story parallel task to invent. T012 documentation
can run alongside read-only T011 boundary checks after T010; `[P]` does not
authorize delegation. Native and isolated Linux verification can use independent
build outputs; record their evidence separately.

## Delivery strategy and evidence

The minimum shippable feature includes all three P1 stories: attachment without
safe maintenance/recovery is not a complete MVP. During each task, finish one
failing case, implement coherently, run the affected suite and refactor green.
Record focused evidence under its task in acceptance.md, not a new document per
cycle. Preparatory refactors start green and need not manufacture a failure.
Historical tests inform behavior but do not pin old internal APIs or records.

Task completion is implementation evidence, not Maintainer acceptance. A missing
required client/platform check stays unchecked. No task authorizes migration of
old data, installation, paid calls without approval, commit by checkpoint alone,
remote push or expanding this feature.
