# Tasks: Domain Foundation

**Input**: Design documents in `specs/001-foundation-context/`  
**Created**: 2026-09-02  
**Status**: Proposed tool/dependency-guidance Product change pending Maintainer
review; no task has been executed

**Prerequisites**: [Spec](spec.md), [Plan](plan.md), [Domain Model](data-model.md),
[Research](research.md), [Context Semantics](context-semantics.md), and
[Tool/Dependency Guidance](tool-use-semantics.md), and
[Acceptance Guide](quickstart.md). Gate 1 and Gate 2 are accepted in
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md) and
[ADR-0002](../../docs/adr/0002-initial-python-runtime.md). The
[Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates)
owns the required checks. Gate 3 remains deferred and is not a blocker for these
Capability-independent tasks.

**Tests**: Required by FR-016 and the Spec acceptance scenarios. Write focused
tests before their implementation, observe the intended failure, and then make
them pass. Do not confuse a missing environment dependency with a tested domain
failure.

**Organization**: User-story phases follow the Spec. The existing domain design
and accepted ADRs are inputs, not work to rewrite. US1 tasks preserve review
traceability; US2 implements three core domains; US3 proves their safety and
dependency constraints. Completion still requires evidence, not only checked
boxes.

## Format and Path Conventions

- Task lines use `- [ ] TNNN [P?] [USn?] description`.
- `[P]` means the task can run alongside the explicitly identified peers after
  its prerequisites, with no shared-file edits. It is not an instruction to
  spawn agents automatically.
- Paths below are repository-relative. Module filenames guide this first
  implementation; they do not require a class, aggregate, or service per noun.
- Create directories with real implementation or test content only. Ordinary
  package initialization is allowed, but empty architectural layers are not.
- `application/` is conditional on real coordination outside domain objects;
  `ports/` and `tests/contract/` are conditional on a demonstrated external-fact
  need. None is currently scheduled as mandatory production code.
- Capability selections, Capability IDs/placeholders, supporting-domain code,
  public protocols, production adapters/entrypoints, persistence, and benchmark
  claims remain outside this task list.

## Phase 1: Setup

**Purpose**: Make the accepted Python and three-tool baseline runnable without
creating a production workflow or speculative layers.

- [ ] T001 Create `pyproject.toml` for the Python 3.14 baseline and `src/` test/import layout; declare only Ruff, mypy, and pytest as initial quality dependencies, apply the Engineering Guide settings, keep core typing strict and logic/complexity/size gates disabled, and add no production framework, CLI, or storage dependency.
- [ ] T002 Create the minimal Python package entry at `src/devmeld/__init__.py`, preserve and extend `.gitignore` only for actual local development outputs, and record interpreter/tool versions plus reproducible setup commands in `specs/001-foundation-context/quickstart.md`; do not pre-create domain sublayers or report a test pass before tests exist.

**Checkpoint**: The interpreter and the three quality tools can be invoked.
Import Linter is not installed/configured as a required gate at this stage.

## Phase 2: Foundational Values

**Purpose**: Establish only the identities actually shared by the three core
domains, not a universal model framework.

- [ ] T003 Record the first shared-kernel admission inventory in `specs/001-foundation-context/plan.md`: identify actual consumers of each shared stable ID; keep single-owner IDs and representation versions domain-local, and exclude Capability values, paths, status enums, generic entity bases, and unused correlation/version types.
- [ ] T004 Write identity equality, immutability, invalid-value, and kind-separation tests in `tests/unit/shared_kernel/test_identity.py` for the inventory from T003; check runtime or static type-boundary evidence as appropriate to the chosen representation without assuming a wire format.
- [ ] T005 Implement only the admitted identity values in `src/devmeld/shared_kernel/identity.py` and make T004 pass; do not introduce a Generic Entity, generic persistence interface, or shared validation/utilities layer.

**Checkpoint**: Shared identity tests and core static typing pass. Remaining
single-owner values are implemented with their owning domain, not added to the
kernel for convenience.

## Phase 3: User Story 1 — Establish the Complete Domain Map (P1)

**Goal**: Retain the reviewed five-domain design and bounded code scope without
restarting domain design. The design already exists; these tasks record its
relationship to the implementation about to begin.

**Independent Test**: A document review can locate every concept's owner,
lifecycle, negative constraint, and design-only/implemented classification
without running an adapter or inspecting a future workflow.

- [ ] T006 [P] [US1] Cross-check the existing ownership, lifecycle, and invariant inventory in `specs/001-foundation-context/data-model.md`, including the proposed tool/dependency guidance in `specs/001-foundation-context/tool-use-semantics.md`, against the Spec and accepted Product rules; record the reviewed document revisions in `specs/001-foundation-context/quickstart.md` while preserving both supporting domains without creating implementation work for them or treating the proposal as accepted.
- [ ] T007 [P] [US1] Verify the accepted ADR links, scoped Gate 3, pending Maintainer status and separate selection/change authority of the tool/dependency-guidance proposal, demand-created layers, deferred protocols, and the FR-018 extension rule against `specs/001-foundation-context/plan.md`, and record the implementation-start review in `specs/001-foundation-context/checklists/requirements.md`; do not mark proposed Product behavior as accepted or supporting-domain behavior as implemented.

**Checkpoint**: US1's design review is traceable. No speculative schema,
Capability concept, application API, or supporting-domain task has been added.

## Phase 4: User Story 2 — Implement Only the Core Domain Foundation (P2)

**Goal**: Implement Project Catalog, Local Context Resolution, and Context
Knowledge as pure domain rules with explicit facts. A complete business
workflow is not required.

**Independent Test**: Run the three domains' unit suites without a real Git
repository, Vault, database, Agent Client, generated file, or network. Catalogue
identity/selection, Task Context validation/resolution, and provenance/Scope
Match must each have standalone evidence.

### Tests Before Implementation

- [ ] T008 [P] [US2] Write portable-reference and registration tests in `tests/unit/catalog/test_registrations.py`: stable Repository/Resource identity, rename preservation, per-registration alias/canonical-key validation, and rejection of machine-specific state using both Windows and POSIX path examples without filesystem access.
- [ ] T009 [P] [US2] Write Workspace and Context Profile tests in `tests/unit/catalog/test_workspace_profiles.py`: exactly one primary Vault, distinct optional sources, supported representation version, cross-registration alias/canonical-key collisions, known same-Workspace references, Repository/Resource eligibility, blocked deletion of referenced registrations, and absence of Capability fields/defaults/behavior.
- [ ] T010 [P] [US2] Write Local Binding, immutable observation, and raw Task Context validation tests in `tests/unit/local_context/test_state_validation.py`: catalog identity cannot be redefined, removing a binding has no external effect, and wrong-Repository, conflicting, unknown, or ineligible explicit selections are rejected before resolution.
- [ ] T011 [P] [US2] Write resolution tests in `tests/unit/local_context/test_resolution.py` for the four ordered valid bases, Resolved/Ambiguous/Unavailable outcomes, candidate-order independence, invalid-input exclusion, and stale/unavailable observed facts without silent Checkout substitution.
- [ ] T012 [P] [US2] Write Evidence, source/review/validity, and directed Relation tests in `tests/unit/knowledge/test_provenance_relations.py`; cover required support, explicit source revisions, independent status dimensions, direction reversal, and unavailable targets without creating a generic Knowledge entity.
- [ ] T013 [P] [US2] Write Scope Match tests in `tests/unit/knowledge/test_scope.py` for matching, mismatched, and unknown dimensions, missing scope as unspecified rather than universal, and recomputation without mutating declared Scope or review/validity facts.
- [ ] T014 [P] [US2] Write context-result tests in `tests/unit/knowledge/test_context_result.py` requiring stable identity, source/Evidence, declared Scope, independent Scope Match/review/validity, and applicable Checkout-resolution basis; ensure relevance cannot disguise scope mismatch and do not test an invented JSON or ranking protocol.

### Domain Implementation

- [ ] T015 [P] [US2] Implement portable source references and Repository/Resource registration values in `src/devmeld/catalog/domain/references.py` and `src/devmeld/catalog/domain/registrations.py`, using domain-local validation and stable IDs to satisfy T008; perform no path discovery or I/O.
- [ ] T016 [US2] Implement Workspace/catalog consistency and the supported Context Profile subset in `src/devmeld/catalog/domain/workspace.py` and `src/devmeld/catalog/domain/profiles.py` after T015; enforce T009 plus cross-registration alias uniqueness using domain-owned rules and immutable catalog facts, with no Capability placeholders or pass-through application service.
- [ ] T017 [P] [US2] Implement local-only bindings and immutable Checkout observations in `src/devmeld/local_context/domain/bindings.py` and `src/devmeld/local_context/domain/observations.py`; accept explicitly supplied catalog IDs/observation facts and exclude client-location integration, Git execution, filesystem scans, and portable path leakage.
- [ ] T018 [US2] Implement raw Task Context validation and distinct valid/invalid results in `src/devmeld/local_context/domain/task_context.py` after T017; preserve rejected selections and reasons, and ensure invalid explicit input cannot become a resolution variant or fall back to another choice.
- [ ] T019 [US2] Implement the ValidTaskContext-only resolution policy in `src/devmeld/local_context/domain/resolution.py` after T018; satisfy T011 using pure observations, exactly three outcome variants, the documented precedence, and explainable candidates/bases without guessing through stale or ambiguous facts.
- [ ] T020 [P] [US2] Implement immutable provenance/resource facts and directed Relations in `src/devmeld/knowledge/domain/provenance.py` and `src/devmeld/knowledge/domain/relations.py` to satisfy T012; keep Evidence, Source Type, Review Status, and Validity Status distinct and external representations out of the domain.
- [ ] T021 [US2] Implement declared Scope and query-time matching in `src/devmeld/knowledge/domain/scope.py` after T020, satisfying T013; preserve unsupported or missing dimensions as explicitly unknown rather than inventing a version grammar or silently broadening scope.
- [ ] T022 [US2] Implement explainable context-result facts in `src/devmeld/knowledge/domain/context_result.py` after T021, satisfying T014; use explicit immutable inputs and stable identities, not foreign domain internals, a query service, serialized DTOs, or public operation/error catalogs.
- [ ] T023 [US2] Review T015–T022 for genuine coordination or external-fact needs and record the outcome in `specs/001-foundation-context/plan.md`; if pure inputs suffice, explicitly retain no application/port packages. If a real in-scope need is demonstrated, name the core rule and add bounded exact-path implementation/test tasks before creating any optional component; do not invent one to fill the tree.
- [ ] T024 [US2] Add cross-core fact-boundary tests in `tests/unit/test_core_fact_boundaries.py` after the three domain branches and T023: supply equivalent immutable facts from two independent test builders, verify the same outcomes and no foreign-state mutation, and reject inappropriate external representations at the relevant boundaries. If T023 introduced an actual port, add its two-substitute contract evidence; otherwise create no port or `tests/contract/` directory solely for this test.

**Checkpoint**: Each core unit suite passes independently and the pure
cross-core boundary test passes. Domain decisions do not live in an application
wrapper, fixture, provider object, or future adapter. Any new optional component
has named acceptance evidence and tasks, not just a directory.

## Phase 5: User Story 3 — Prove Core Safety and Dependency Rules (P3)

**Goal**: Protect the implemented core invariants and dependency directions with
focused, non-overlapping checks. Import Linter is activated only now, after the
three core packages contain real code.

**Independent Test**: Run safety regressions and prove that actual architecture
contracts reject seeded violations while allowing legitimate inward use.
Architecture probes operate only on isolated test fixtures.

- [ ] T025 [P] [US3] Add focused adversarial regressions in `tests/unit/test_core_safety.py` for invalid-selection no-fallback, reordered candidates, changed observations/revisions, portable/local separation, and conflicting provenance dimensions; verify that query-derived facts never replace source truth or mutate stored domain facts.
- [ ] T026 [US3] Add Import Linter to `pyproject.toml` only after Phase 4 and configure the minimum accepted dependency contracts: core-to-adapter/entrypoint, application-to-concrete-adapter where an application package exists, and cross-domain-internal imports; make optional-package absence valid without weakening constraints or creating placeholder production layers.
- [ ] T027 [US3] Implement `tests/architecture/test_dependency_contracts.py` after T026, exercising the actual configured contract rules against temporary fixture graphs with each forbidden import and legitimate inward/public-fact use; assert the expected violated contract, not just a nonzero exit caused by malformed configuration, and keep seeded modules outside production source.
- [ ] T028 [P] [US3] Implement `tests/architecture/test_scope_boundaries.py` to detect out-of-scope supporting packages, Capability-selection or tool-guidance placeholders, public protocol artifacts, and empty optional application layers within production source and this Feature's artifacts; exclude Spec Kit/tool templates and the design-only `specs/001-foundation-context/tool-use-semantics.md` artifact from production-code findings, and do not add line-count, complexity, or pattern-count rules as substitutes for semantic review.
- [ ] T029 [US3] Review invariant ownership and every actual application/port component against `specs/001-foundation-context/data-model.md` and record findings in `specs/001-foundation-context/quickstart.md`; verify coordination is real, no service merely forwards domain calls, and no conditional port/contract task remains unfinished before claiming US3 complete.

**Checkpoint**: Safety tests pass, every seeded forbidden dependency is detected
for the intended reason, legitimate dependencies pass, and optional layers are
absent unless justified. A passing scan over an empty graph is not evidence.

## Phase 6: Verification and Handoff

- [ ] T030 Run the Engineering Guide's Ruff format/check, mypy, pytest, and now-active `lint-imports` sequence, plus each core domain's unit suite independently, and record commands, results, interpreter/tool versions, and tested/unverified platforms in `specs/001-foundation-context/quickstart.md`; do not claim cross-platform or production integration coverage that was not run.
- [ ] T031 Reconcile implemented filenames, shared-kernel admissions, and actual conditional-layer/port decisions in `specs/001-foundation-context/plan.md`; remove stale claims of mandatory scaffolding and confirm the implementation has not expanded the Spec, either supporting domain, or design-only tool/dependency guidance.
- [ ] T032 Prepare the final evidence-backed review submission in `specs/001-foundation-context/quickstart.md`, referencing all Spec success criteria and ADR revisions; distinguish completed implementation checks from the Maintainer's final ACCEPT/REVISE decision, and leave that approval for the Maintainer rather than self-ratifying it.

## Dependencies and Execution Order

```text
Setup: T001 -> T002
    -> shared foundation: T003 -> T004 -> T005
    -> US1: T006 || T007
    -> US2 tests: T008 || T009 || T010 || T011 || T012 || T013 || T014
        -> Catalog:   T015 -> T016
        -> Local:     T017 -> T018 -> T019
        -> Knowledge: T020 -> T021 -> T022
        -> T023 -> T024
    -> US3: T025 || (T026 -> T027) || T028
        -> T029
    -> T030 -> T031 -> T032
```

- All US2 implementation branches require the shared foundation, the US1
  checkpoint, and their corresponding failing tests. They may then proceed
  independently using stable IDs and explicit input facts.
- T023 waits for all three core branches; T024 waits for any actual optional
  component work that T023 demonstrates is necessary.
- US3 needs the actual US2 packages. It is independently verifiable but not
  implementable before there is a real dependency graph to check.
- T026 must not be pulled forward into initial setup. T027 must test its real
  contracts rather than a separate hard-coded imitation.
- Final handoff waits for all stories, including any precisely scoped tasks
  added for an evidenced optional need.

## Parallel Examples

- **US1**: T006 writes acceptance-guide traceability while T007 writes the
  requirements checklist; neither rewrites the other task's file.
- **US2**: T008–T014 can author independent tests in separate files. Afterward,
  the Catalog, Local Context, and Knowledge implementation branches can run
  concurrently; preserve each branch's sequential dependencies.
- **US3**: T025 and T028 use different test files and can proceed alongside
  T026–T027 after US2. Join all evidence before the T029 semantic review.
- Tasks that edit `pyproject.toml`, `plan.md`, or `quickstart.md` at different
  phases remain sequenced; do not treat shared-document edits as parallel.

## Requirement and Success-Criterion Coverage

| Scope | Evidence tasks |
| --- | --- |
| FR-001–004, FR-009, FR-018, FR-022; SC-001–002, SC-011 | T006–T007, T031: existing domain map, lifecycles, ownership, extension review, and design-only tool/dependency guidance |
| FR-005–006, FR-019–021; SC-003, SC-009 | T007, T009, T016, T023, T028–T029, T032: scoped implementation, gates and demand-created layers |
| FR-007–008, FR-017; SC-008 | T003–T005, T023–T024, T027: stable boundaries, actual-need inventory and substitution evidence |
| FR-010–011; SC-005 | T008–T009, T015–T017, T025: portability/authority; generated/rebuildability design reviewed in T006 without building storage or writes |
| FR-012–013; SC-004 | T010–T011, T017–T019, T025: validation separated from resolution and no fallback |
| FR-014–015; SC-006 | T012–T014, T020–T022, T028: provenance, Scope Match and no frozen public protocol |
| FR-016; SC-007, SC-010 | T004, T008–T014, T024–T030: pure tests and effective seeded dependency checks |

## Implementation Strategy

US1 is the smallest reviewable design checkpoint, not a user-visible MVP and
not evidence that the code foundation exists. Proceed through the three US2
core branches with tests before code, then activate US3's dependency gate over
the real package graph. Complete all three stories and handoff evidence before
calling the foundation implemented.

Do not add a demo query, generated file, database, provider integration, public
API, or generic framework to make the foundation look complete. Future
user-visible Features will supply their own end-to-end acceptance scenarios.
Do not execute code implementation or commit changes merely because this task
list was generated.
