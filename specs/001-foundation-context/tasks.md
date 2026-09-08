# Tasks: Domain Foundation — Rust Restart

**Input**: Design documents in specs/001-foundation-context/

**Created**: 2026-09-02

**Reset**: 2026-09-07

**Status**: T001–T036 completed for the committed baseline with recorded Windows/Linux evidence.
Review corrections in 64dfb74 passed Windows/Linux checks; subsequent type
refinements below pass native Windows checks and have not been rerun on WSL/Linux.
The 2026-09-08 rejection-receipt correction is tracked separately as T037–T038.
Maintainer acceptance remains pending.

**Prerequisites**: [Spec](spec.md), [Plan](plan.md), [Domain Model](data-model.md),
[Rust Design](rust-design.md), [Research](research.md), [Context Semantics](context-semantics.md),
[Tool Guidance](tool-use-semantics.md), [Acceptance Guide](quickstart.md),
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md) and current
[ADR-0003](../../docs/adr/0003-rust-runtime.md). Engineering owns the check policy.

Gate 3 is still deferred. It does not block these Capability-independent tasks.
No language-comparison experiment is required. Old Python checked boxes/test
counts and WSL samples do not complete any task in this reset list.

**Tests**: Required by FR-016. Write tests before their corresponding rules;
observe intended failures, then implement. Environment or linker failure is not
a domain red test. Compile-fail cases require valid controls and expected diagnostics.

## Format and Scope

- Task lines use TNNN identifiers, completion checkboxes, optional [P], and story labels.
- [P] permits work on independent files after prerequisites; it does not request
  automatic agent spawning.
- Paths are repository-relative. Crate entrypoints required for compilation are
  ordinary package setup, not permission for empty application/port layers.
- No product executable, public API contract, database, async runtime, provider SDK,
  Capability fields, Tool Guidance implementation or supporting-domain crate.
- Conditional application/ports require a named implemented need and added
  exact-path tasks before creation. No such component is currently mandatory.
- This document creates tasks, not code or authorization to start implementing
  during the documentation-only reset. Do not commit or push without a request.

## Phase 1: Setup

**Purpose**: Prepare the accepted native Rust toolchain and lightweight checks.

- [X] T001 Verify the existing native Windows stable toolchain, rustfmt, Clippy and MSVC linker without silently installing tools; create the tested operational pin in rust-toolchain.toml and record host/version/linker evidence and intended rust-version in specs/001-foundation-context/quickstart.md. Stop for missing installation authority, not for another language-selection experiment.
- [X] T002 Create root Cargo.toml as a virtual workspace (resolver 3), the four member Cargo.toml and src/lib.rs entries at crates/shared-kernel/, crates/catalog/, crates/local-context/ and crates/knowledge/; explicitly inherit workspace package/lint settings, use only the Plan's normal path edges, publish=false and no build scripts, generate Cargo.lock with Cargo, and keep .gitignore aligned with actual outputs. No root facade, binary, external dependency or optional architectural layer.

**Checkpoint**: Native compile/link and tools can run. Four library entries are
not completed domains; an empty test run is not acceptance. Every member uses
the inherited conservative policy.

## Phase 2: Foundational Values

**Purpose**: Share only genuinely identical identity semantics.

- [X] T003 Confirm the RepositoryId/ResourceId consumer inventory in specs/001-foundation-context/plan.md and input-validation examples in specs/001-foundation-context/rust-design.md; keep owner-local IDs, versions, paths, Scope and status/error types outside the shared kernel.
- [X] T004 Write equality, immutability, valid Unicode/invalid-input and kind-separation tests in crates/shared-kernel/tests/identity.rs; include compiler-oriented fixtures or doctest cases proving kind mismatch and protect the intended diagnostic with a valid control in the later architecture harness.
- [X] T005 Implement only the admitted private validated identity newtypes and owner-specific errors in crates/shared-kernel/src/identity.rs with selective exports in crates/shared-kernel/src/lib.rs; make T004 pass without a generic utility, entity or repository framework.

**Checkpoint**: Identity tests and compiler checks pass; no claimed cross-platform
execution or public wire format.

## Phase 3: User Story 1 — Establish the Complete Domain Map (P1)

**Goal**: Preserve and review the existing language/ownership design before code
rules are implemented, not redesign Product around Rust.

**Independent Test**: Document review locates all five owners, invariants,
lifecycle classes, provisional terms and three-domain code boundary.

- [X] T006 [US1] Cross-check specs/001-foundation-context/data-model.md, rust-design.md and tool-use-semantics.md against Product and Spec; record reviewed revisions in specs/001-foundation-context/quickstart.md, including complete core field coverage and supporting-domain design-only status.
- [X] T007 [US1] Review ADR-0001/0003, the scoped Capability Gate 3, accepted Tool Guidance selection/change authority, deferred protocols, FR-018 extension rule and demand-created layers against specs/001-foundation-context/plan.md; record implementation-start review in quickstart.md without treating specification checklist boxes as executable acceptance.

**Checkpoint**: First reviewable design increment. Not a user-visible MVP and
not sufficient for completed foundation acceptance.

## Phase 4: User Story 2 — Implement Only the Core Domain Foundation (P2)

**Goal**: Implement the complete approved core subset using pure rules and facts.

**Independent Test**: Each core's Cargo suite runs without Git/Vault/database/
network/client/production IO. No production application service is necessary.

### Tests Before Implementation

- [X] T008 [P] [US2] Write portable-reference and Repository/Resource registration tests in crates/catalog/tests/registrations.rs: identity, display-name/alias changes, canonical-key collisions, source/type/locator/eligibility and complete supported relative/remote locator boundaries, including Windows/POSIX and encoded traversal/control/credential failures without IO.
- [X] T009 [P] [US2] Write Workspace/Profile tests in crates/catalog/tests/workspace_profiles.rs: exactly one primary Vault, distinct sources, supported schema version, names, known same-Workspace eligible Repository/Resource references, cross-registration alias collisions, checked updates and blocked referenced removal, with Capability placeholders absent.
- [X] T010 [P] [US2] Write binding/observation/raw-task tests in crates/local-context/tests/state_validation.rs: registry revision and local-only transitions, full branch/commit/working-tree/time/source facts, explicit availability/freshness, working area, immutable snapshots and rejected-selection explanations for conflicting/unknown/wrong-Repository/ineligible input.
- [X] T011 [P] [US2] Write resolution tests in crates/local-context/tests/resolution.rs: four ordered bases, exactly three valid-input outcomes, Repository and considered observations/basis, invalid explicit no-fallback, duplicate/contradictory snapshot rejection, candidate-order independence and changed-observation revalidation.
- [X] T012 [P] [US2] Write source/Evidence/Relation tests in crates/knowledge/tests/provenance_relations.rs: source revisions, locator/range or derivation, optional content hash, required support, independent review/validity, typed direction and unavailable target explanations.
- [X] T013 [P] [US2] Write Scope tests in crates/knowledge/tests/scope.rs: all designed dimensions, exact match/mismatch/unknown, missing as unspecified, mismatch precedence, unsupported interval semantics and recomputation without source/status mutation.
- [X] T014 [P] [US2] Write Resource/Relation result tests in crates/knowledge/tests/context_result.rs: identity/source/Evidence, declared Scope and per-dimension match explanations, independent statuses, applicable consistent Checkout basis, repository-only queries without invented Checkout requirements and relevance unable to hide mismatches.

### Domain Implementation

- [X] T015 [P] [US2] Implement portable references and Repository/Resource registrations in crates/catalog/src/references.rs and registrations.rs, exporting selected APIs from crates/catalog/src/lib.rs; satisfy T008 with validated private state and no filesystem/network discovery.
- [X] T016 [US2] Implement complete Workspace consistency and the Capability-independent Profile subset in crates/catalog/src/workspace.rs and profiles.rs after T015; satisfy T009 with schema version, names, same-Workspace references and immutable checked transitions, not cascade deletion or pass-through services.
- [X] T017 [P] [US2] Implement Local Binding Registry and full immutable Checkout observations in crates/local-context/src/bindings.rs and observations.rs, with selective lib.rs exports; satisfy T010's state rules without Git execution, hidden clocks, client integration or portable path leakage.
- [X] T018 [US2] Implement raw task validation in crates/local-context/src/task_context.rs after T017; retain rejected selections/reasons and produce a private ValidatedTaskContext owning the checked catalog/task/observation snapshot, with no unchecked construction or mutable escape.
- [X] T019 [US2] Implement resolution in crates/local-context/src/resolution.rs after T018; satisfy T011 using only the checked snapshot, typed Resolved/Ambiguous/Unavailable, explicit precedence, complete evidence and no separate unvalidated observation argument.
- [X] T020 [P] [US2] Implement source/resource/Evidence facts and directed Relations in crates/knowledge/src/provenance.rs and relations.rs, selectively exporting via lib.rs; satisfy T012 without a Knowledge base entity or foreign SDK/storage objects.
- [X] T021 [US2] Implement Scope and query-time matching in crates/knowledge/src/scope.rs after T020; satisfy T013, retaining unknown intervals and separate explanations without inventing a grammar or mutating facts.
- [X] T022 [US2] Implement Resource/Relation result evaluation and consumer-owned Checkout facts in crates/knowledge/src/context_result.rs after T021; satisfy T014 without a normal dependency on Catalog/Local Context or an invented query protocol.
- [X] T023 [US2] Review the three implementations for actual coordination/external-fact needs and record the result in specs/001-foundation-context/plan.md; if pure inputs suffice, keep application/ports absent. If not, name the in-scope core rule and add exact-path implementation/test tasks before creating any optional component.
- [X] T024 [US2] Add only the Plan's Catalog/Local Context dev-dependencies to crates/knowledge/Cargo.toml and refresh Cargo.lock through Cargo; implement crates/knowledge/tests/core_fact_boundaries.rs using two independent equivalent fact builders, explicit consumer-owned mapping and no foreign mutation. Add two-substitute tests only if T023 justified a real port; do not create a production facade, demo or extra harness crate.

**Checkpoint**: All core suites and cross-core fact tests pass; complete Spec/
Domain Model coverage is checked against Rust Design, not the partial WSL sample.

## Phase 5: User Story 3 — Prove Core Safety and Dependency Rules (P3)

**Goal**: Add the smallest effective enforcement over the real implemented graph.

**Independent Test**: Real gate rejects controlled violations and accepts valid
inward/test-only boundaries. No violation is seeded in the actual checkout.

- [X] T025 [P] [US3] Add adversarial regressions in crates/local-context/tests/safety.rs and crates/knowledge/tests/context_result.rs for stale/replaced snapshots, invalid-selection no-fallback, candidate reordering, independent status conflicts and query-derived/source separation; confirm protected state cannot be mutated through the public APIs.
- [X] T026 [P] [US3] Implement the cargo metadata JSON gate (now tools/xtask/src/architecture.rs, migrated at T034) after real cores exist; enforce the Plan member/path and normal/dev/build allowlist across optional/target-specific/renamed declarations and initial no-build-script/no-extra-target scope. Fail on command/JSON/unknown-kind errors; do not rely on the absent --no-deps resolve graph.
- [X] T027 [US3] Implement the regression harness (now tools/xtask/src/probes.rs, migrated at T034) using isolated copies of real manifests/source, the actual checker and fixture-local locks; prove valid graph and allowed dev edges pass, while core-to-core, kernel-outward, adapter/entrypoint and build/optional/target/renamed violations fail for intended reasons. Compile valid consumer controls before private-module/field, identity-kind, fabricated/raw-context and non-exhaustive-match probes; confirm specific diagnostics, not arbitrary failure, and effective inherited unsafe policy in each core member (Cargo metadata omits lint settings).
- [X] T028 [US3] Complete bounded-scope assertions in the harness (now tools/xtask/src/probes.rs) and human source review: only four planned core libraries, no Capability placeholders, hidden source includes, speculative layers or production IO; no public mutable escape/re-export bypass. The T033 developer tool is not a product domain. Record any automated-check limitations in specs/001-foundation-context/quickstart.md rather than claiming Cargo proves semantic purity.

**Checkpoint**: Real graph and compiler boundary evidence exists. Metadata checks
and human ownership review are complementary, not interchangeable.

## Phase 6: Polish and Acceptance

- [X] T029 Review full FR/SC coverage against specs/001-foundation-context/spec.md, data-model.md, rust-design.md and context-semantics.md; record semantic findings, justified optional layers and resolved gaps in quickstart.md without changing Product meaning or crediting old runtime results.
- [X] T030 Run the complete Engineering Cargo/fmt/Clippy/test and architecture sequence, plus each core suite independently; record commands, exits, toolchain/linker, native platform, test/negative-probe counts and unverified targets in specs/001-foundation-context/quickstart.md. Include doctests and never substitute a WSL result for Windows.
- [X] T031 Recheck docs/product.md, docs/engineering.md, docs/adr/0001-domain-oriented-modular-monolith.md, docs/adr/0003-rust-runtime.md and all specs/001-foundation-context/ links/statuses; correct only owning-artifact discrepancies, retain Gate 3 and supporting-domain exclusions, and do not claim a user-visible vertical slice.
- [X] T032 Submit the exact code/Spec/Plan/ADR revisions, evidence and remaining limitations using specs/001-foundation-context/quickstart.md Acceptance Record; leave Maintainer ACCEPT/REVISE to the reviewer, and do not commit or push without explicit instruction.

## Cross-platform Tooling Correction (2026-09-07)

The Maintainer approved Rust-only project verification via a developer-only
`tools/xtask` package and its `serde_json` dependency, and removal of redundant
Spec Kit PowerShell scripts. This does not change domain behavior or Gate 3.

- [X] T033 Add tools/xtask/Cargo.toml, src/main.rs and .cargo/config.toml; include the developer tool in the one workspace/lockfile while retaining the four core libraries and their dependency policy. Document JSON parsing as a tool-only dependency and fetch only this approved dependency closure.
- [X] T034 Port the real metadata checker and negative harness into tools/xtask/src/architecture.rs and probes.rs with native Rust process/filesystem APIs, safe isolated fixtures, fail-closed diagnostics and no shell dependency; preserve the previous 35 probes and add platform/path/tool-isolation regressions before retiring scripts/*.ps1.
- [X] T035 Remove the six .specify/scripts/powershell/*.ps1 files and corresponding integration inventory entries; retain the Python configuration and entrypoints, remove dangling comments, verify safe Python commands without modifying Feature artifacts.
- [X] T036 Run the complete Cargo/xtask sequence on native Windows, exercise Linux where an existing compatible environment is available without changing experiments, record macOS and any other unverified targets honestly, update owning documentation and verification.sha256. Do not commit or push.

## Dependencies and Parallel Opportunities

- T001 -> T002 -> T003 -> T004 -> T005 establish setup and identities.
- T006 -> T007 complete the fresh design-start review; both write quickstart.md,
  so they are deliberately sequential.
- After T007, T008–T014 can run on their distinct test files.
- Catalog: T008/T009 -> T015 -> T016.
- Local Context: T010/T011 -> T017 -> T018 -> T019.
- Knowledge: T012/T013/T014 -> T020 -> T021 -> T022.
- These three code branches can proceed independently after their tests; within
  a crate, shared lib.rs edits stay with that branch.
- All three -> T023 -> T024. Only T024 changes the shared lock/dev dependency map.
- T025 and T026 may run independently after T024; T027 follows T026, then T028.
- T029–T032 follow all implementation and evidence work, including any exact-path
  tasks added for a genuine optional need.
- T033 -> T034 replace the developer check implementation; T035 independently
  simplifies Spec Kit. T036 verifies and records the combined correction.

## Requirement and Success-Criterion Coverage

| Requirements / outcomes | Tasks |
| --- | --- |
| FR-001–004, FR-009, FR-018, FR-022; SC-001–002, SC-011 | T006–T007, T029, T031 |
| FR-005–006, FR-019–021; SC-003, SC-009 | T002, T009, T016, T023, T028–T029, T032 |
| FR-007–008, FR-017; SC-008 | T003–T005, T023–T024, T027 |
| FR-010–011; SC-005 | T008–T010, T015–T017, T025 |
| FR-012–013; SC-004 | T010–T011, T017–T019, T025, T027 |
| FR-014–015; SC-006 | T012–T014, T020–T022, T025, T029 |
| FR-016; SC-007, SC-010 | T004, T008–T014, T024–T030 |

## Completion Boundary

Committed baseline: 36 total tasks: Setup 2; Foundational 3; US1 2; US2 17; US3 4; Polish 4;
cross-platform correction 4. 36 checked, 0 open.
The earlier native Code Integrity interruption was resolved
by the user; the full suite and all independent package suites were recorded as passing without
an execution workaround. The baseline complete quality sequence passed on native
Windows and Linux in WSL: 37 core behavior tests, three tool unit tests, one
compile-fail doctest and 40 controlled architecture/type/scope/path probes.
macOS remains unverified. Core implementation and deferred domain scope are unchanged.
T032 is the review submission in quickstart.md with verification.sha256, not an
automatic Maintainer ACCEPT. The original implementation agent reported no commit
or push during submission; that snapshot was subsequently committed as
4da191195b3edafefb84d6f2896f0e56fc74e400.
US1 is the first independently reviewable design checkpoint, not a product MVP.
Foundation completion requires all three stories and evidence, not just design
or an executable demo. Real integrations and user-visible vertical slices are
planned by later Features.

## Post-commit Review Corrections (2026-09-07)

- Implemented SelectionSource on RejectedSelection in local-context, with
  regressions for explicit-task errors, source-free snapshot errors and identical
  rejected Workspace/LocalDefault selections through all three resolution outcomes.
- Adopted the Plan's exact shared-kernel declaration rule and added three real
  manifest probes for an allowed destination with target/optional/rename variants.
- Preserved the original submission history, identified commit 4da1911, and
  refreshed LF-normalized fingerprints for the revised working tree.

The complete `cargo xtask check` passes on native Windows and Linux in WSL:
38 core behavior tests, three tool unit tests, one compile-fail doctest and
43 architecture probes. After the user updated the Windows environment,
the native rerun with Rust/Cargo 1.98.1 exited 0 and superseded the earlier
Cargo 1.80.1 manifest-parsing failure. These are fresh checks of the review
corrections. Quickstart records environment changes and commands.
CI and locator fuzz testing remain follow-ups. Verification did not include an
ACCEPT decision, commit or push. The user subsequently authorized the local
commit and reserved push for themselves; Maintainer acceptance remains pending.

## Type Modeling Refinement (2026-09-07)

Following the user's request to strengthen string-based business fields:

- Knowledge owns WorkingTreeState, retaining Clean/Dirty/Unknown and validated
  explanations through explicit cross-domain mapping and both result types.
- Catalog owns ProfileId; Local Context owns ObservationId. Constructors and
  map/removal/selection APIs use the corresponding type. Shared Kernel is unchanged.
- RejectionReason provides eight structured causes and preserves previous
  explanations through Display, including the duplicated ObservationId.
- SUPPORTED_SCHEMA_VERSION names the existing value 1. Open text fields and
  Resource type labels retain their existing accepted inputs.

Native Windows `cargo xtask check` exits 0: 43 core behavior tests, three tool
unit tests, one compile-fail doctest and 46 architecture probes. Added compile
probes reject mixed Profile/Workspace IDs, Repository/Observation IDs and string
working-tree states after a valid public consumer compiles. WSL/Linux and macOS
were not rerun for this refinement; earlier platform results apply to earlier
snapshots. The refinement was subsequently committed as 75aa138; that commit
does not grant Maintainer ACCEPT.

## Rejection Receipt Correction (2026-09-08)

- [X] T037 [US2] Preserve both sides of a rejected selection conflict in crates/local-context/src/task_context.rs, using a failing regression in crates/local-context/tests/safety.rs; verify explicit-task, Workspace and LocalDefault sources and exchanged input order without changing validation or resolution priority (FR-012–013).
- [X] T038 Update the committed type-refinement status in README.md and specs/001-foundation-context/quickstart.md, record the focused and full verification results, and refresh specs/001-foundation-context/verification.sha256 after T037. Keep historical evidence and Maintainer acceptance separate; do not commit or push.
