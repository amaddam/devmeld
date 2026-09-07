---
description: "DevMeld behavior-slice task template"
---

# Tasks: [FEATURE NAME]

**Input**: Accepted design documents from `specs/[###-feature-name]/`
**Prerequisites**: `spec.md`, `plan.md`, `docs/engineering.md`, relevant ADRs and available supporting design documents.
**Convention**: Follow `docs/engineering.md#behavioral-development`. One behavior task owns its test/implementation cycles; do not create separate test and implementation batches or separate RED/GREEN/refactor tasks. Other task types use their appropriate verification method.

## Format

`- [ ] T001 [P?] [US?] Description with exact paths`

- IDs are unique and ordered by execution dependencies.
- `[US1]` maps story tasks to the Spec. Setup, shared prerequisites, cross-cutting and final acceptance tasks have no story label.
- `[P]` means the whole task is independent, has no conflicting files, and its prerequisites are satisfied. It does not authorize agent delegation.
- Each behavior task identifies the accepted rule/case, implementation and test paths, and relevant acceptance condition. The task may contain several small cycles.
- Use real paths and owning domains from the Plan. Do not invent application layers, base entities, services or adapters to fill this template.

<!-- Replace every sample and placeholder with approved work. Omit empty phases,
     use only real prerequisites, and renumber phases/tasks as needed. These are
     illustrative tasks, not new Foundation requirements or a fixed task count. -->

## Phase 1: Setup (When Needed)

**Purpose**: Only the environment/configuration work needed for this Feature.

- [ ] T001 Verify [required workspace configuration] in [exact configuration path] using [direct check command]

## Phase 2: Genuine Shared Prerequisites (When Needed)

**Purpose**: Work that demonstrably blocks specified behavior slices, not a generic infrastructure phase.

- [ ] T002 Establish [approved prerequisite] in [exact paths], verify with [task-appropriate check]; blocks [task IDs and reason]

**Checkpoint**: Listed dependents can begin when their prerequisites pass. Unrelated stories need not wait. A behavioral prerequisite still follows vertical TDD.

## Phase 3: User Story 1 - [Title] (Priority: P1)

**Goal**: [Accepted story outcome]
**Independent Acceptance**: [Observable result and Spec acceptance references]
**Boundary**: [Accepted public/intentionally exposed domain API and owning decision]

### Behavior Slices

- [ ] T003 [US1] Implement [behavior A] using vertical TDD in [implementation path] and [test path]; verify [acceptance condition]
- [ ] T004 [US1] Implement [behavior B / failure case] using vertical TDD in [implementation path] and [test path]; verify [acceptance condition]; depends on [actual task IDs]

### Story Integration / Acceptance (When Needed)

- [ ] T005 [US1] Verify [story-level scenario] through [accepted boundary] using [test/acceptance path and command], after T003 and T004

**Checkpoint**: [Story outcome demonstrated; required affected checks pass]

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [Accepted story outcome]
**Independent Acceptance**: [Observable result and Spec acceptance references]
**Boundary**: [Accepted boundary and owning decision]

### Behavior Slices

- [ ] T006 [US2] Implement [behavior C] using vertical TDD in [implementation path] and [test path]; verify [acceptance condition]

### Story Integration / Acceptance (When Needed)

- [ ] T007 [US2] Verify [story-level scenario] using [test/acceptance path and command], after T006

**Checkpoint**: [Story outcome demonstrated, including any real cross-story integration]

## Phase 5: Cross-Cutting / Architecture Verification (When Needed)

Keep only checks required by the accepted design. Run focused boundary checks
with the changes they protect; this phase collects cross-cutting verification,
not permission to defer all architecture feedback until the end.

- [ ] T008 Verify [accepted type/visibility guarantee] with a valid control and intended compile-fail diagnostic in [probe path]
- [ ] T009 Verify [accepted dependency rule] with valid and forbidden cases in [architecture probe path]

## Phase 6: Final Acceptance

- [ ] T010 Review and update [affected documentation paths] against delivered behavior
- [ ] T011 Run required full checks and [Feature acceptance scenarios]; record commands, outcomes and unavailable checks in [acceptance evidence path], and identify pending Maintainer decisions

**Checkpoint**: Implementation evidence is complete or explicitly reported as missing. Maintainer acceptance is not granted by passing automated checks.

## Dependencies and Execution Order

- [List actual task and story dependencies; justify shared prerequisites.]
- Default to story priority when dependencies permit.
- Within a behavior task: one failing case, minimum coherent implementation, focused suite, necessary refactor, green suite, then the next case.
- For bug fixes, reproduce the defect first. For refactors, preserve the green baseline. For boundaries, setup and documentation, use Engineering's task-specific verification.
- Preserve completed history; do not retrofit completed Foundation tasks.

## Parallel Opportunities

[List only whole independent tasks with non-conflicting implementation AND test
files. State "None" when no safe parallel work exists. Do not batch all tests or
all models, and do not manufacture independence between dependent stories.]

## Delivery Strategy

Complete the smallest accepted story with only its real prerequisites, verify
its outcome, then add the next behavior/story without regressing previous work.
Keep required final checks and Maintainer review; do not auto-deploy, commit or
push merely because a checkpoint passes.
