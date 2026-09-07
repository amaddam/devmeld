---
description: "Generate an actionable, dependency-ordered tasks.md for the feature based on available design artifacts."
---


## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty).

## Pre-Execution Checks

**Check for extension hooks (before tasks generation)**:
- Check if `.specify/extensions.yml` exists in the project root.
- If it exists, read it and look for entries under the `hooks.before_tasks` key
- If the YAML cannot be parsed or is invalid, skip hook checking silently and continue normally
- Filter out hooks where `enabled` is explicitly `false`. Treat hooks without an `enabled` field as enabled by default.
- For each remaining hook, do **not** attempt to interpret or evaluate hook `condition` expressions:
  - If the hook has no `condition` field, or it is null/empty, treat the hook as executable
  - If the hook defines a non-empty `condition`, skip the hook and leave condition evaluation to the HookExecutor implementation
- When constructing command invocations from hook command names, replace dots (`.`) with hyphens (`-`). For example, `speckit.git.commit` → `$speckit-git-commit`.
- For each executable hook, output the following based on its `optional` flag:
  - **Optional hook** (`optional: true`):
    ```
    ## Extension Hooks

    **Optional Pre-Hook**: {extension}
    Command: `/{command}`
    Description: {description}

    Prompt: {prompt}
    To execute: `/{command}`
    ```
  - **Mandatory hook** (`optional: false`):
    ```
    ## Extension Hooks

    **Automatic Pre-Hook**: {extension}
    Executing: `/{command}`
    EXECUTE_COMMAND: {command}

    Wait for the result of the hook command before proceeding to the Outline.
    ```
    After emitting the block above you MUST actually invoke the hook and wait for it to finish before continuing. Run it the same way you would run the command yourself in this agent/session (the invocation may differ from the literal `{command}` id shown above, e.g. a skills-mode agent runs it as `/skill:speckit-...` or `$speckit-...`). Emitting the block alone does not run the hook.
- If no hooks are registered or `.specify/extensions.yml` does not exist, skip silently

## Outline

1. **Setup**: Run `python3 .specify/scripts/python/setup_tasks.py --json` from repo root and parse FEATURE_DIR, TASKS_TEMPLATE_CONTENT, TASKS_TEMPLATE, and AVAILABLE_DOCS list. `FEATURE_DIR` and `TASKS_TEMPLATE` must be absolute paths when provided. `AVAILABLE_DOCS` is a list of document names/relative paths available under `FEATURE_DIR` (for example `research.md` or `contracts/`). For single quotes in args like "I'm Groot", use escape syntax: e.g 'I'\''m Groot' (or double-quote if possible: "I'm Groot").

2. **Load design documents**: Read from FEATURE_DIR:
   - **Required**: plan.md (tech stack, libraries, structure), spec.md (user stories with priorities), `docs/engineering.md` (implementation and verification conventions)
   - **Optional**: data-model.md (entities), contracts/ (interface contracts), research.md (decisions), quickstart.md (test scenarios)
   - **IF EXISTS**: Load `.specify/memory/constitution.md` for project principles and governance constraints
   - Note: Not all projects have all documents. Generate tasks based on what's available.

3. **Execute task generation workflow**:
   - Load plan.md and extract tech stack, libraries, project structure
   - Load spec.md and extract user stories with their priorities (P1, P2, P3, etc.)
   - If data-model.md exists: Extract entities and map to user stories
   - If contracts/ exists: Map interface contracts to user stories
   - If research.md exists: Extract decisions for setup tasks
   - Generate tasks organized by user story (see Task Generation Rules below)
   - Generate dependency graph showing user story completion order
   - Create parallel execution examples per user story
   - Validate task completeness (each user story has all needed tasks, independently testable)

4. **Generate tasks.md**: Use TASKS_TEMPLATE_CONTENT (from the JSON output above) as the structure. For compatibility with older setup scripts that omit TASKS_TEMPLATE_CONTENT, read TASKS_TEMPLATE instead. Fill with:
   - Correct feature name from plan.md
   - Setup tasks only where needed by the accepted plan
   - Genuine shared prerequisites, with the specific dependent tasks identified; omit this phase when none exist
   - Story phases: One phase per user story (in priority and dependency order from spec.md)
   - Each story includes: goal, independent acceptance criteria, bounded behavior slices, and story-level integration/acceptance when needed
   - Cross-cutting architecture checks and final acceptance; schedule focused boundary checks with the work they protect, not only at the end
   - All tasks must follow the strict checklist format (see Task Generation Rules below)
   - Clear file paths for each task
   - Dependencies section showing story completion order
   - Parallel execution examples per story
   - Implementation strategy section (MVP first, incremental delivery)

## Mandatory Post-Execution Hooks

**You MUST complete this section before reporting completion to the user.**

Check if `.specify/extensions.yml` exists in the project root.
- If it does not exist, or no hooks are registered under `hooks.after_tasks`, skip to the Completion Report.
- If it exists, read it and look for entries under the `hooks.after_tasks` key.
- If the YAML cannot be parsed or is invalid, skip hook checking silently and continue to the Completion Report.
- Filter out hooks where `enabled` is explicitly `false`. Treat hooks without an `enabled` field as enabled by default.
- For each remaining hook, do **not** attempt to interpret or evaluate hook `condition` expressions:
  - If the hook has no `condition` field, or it is null/empty, treat the hook as executable
  - If the hook defines a non-empty `condition`, skip the hook and leave condition evaluation to the HookExecutor implementation
- When constructing command invocations from hook command names, replace dots (`.`) with hyphens (`-`). For example, `speckit.git.commit` → `$speckit-git-commit`.
- For each executable hook, output the following based on its `optional` flag:
  - **Mandatory hook** (`optional: false`) — **You MUST emit `EXECUTE_COMMAND:` for each mandatory hook**:
    ```
    ## Extension Hooks

    **Automatic Hook**: {extension}
    Executing: `/{command}`
    EXECUTE_COMMAND: {command}
    ```
    After emitting the block above you MUST actually invoke the hook and wait for it to finish before continuing. Run it the same way you would run the command yourself in this agent/session (the invocation may differ from the literal `{command}` id shown above, e.g. a skills-mode agent runs it as `/skill:speckit-...` or `$speckit-...`). Emitting the block alone does not run the hook.
  - **Optional hook** (`optional: true`):
    ```
    ## Extension Hooks

    **Optional Hook**: {extension}
    Command: `/{command}`
    Description: {description}

    Prompt: {prompt}
    To execute: `/{command}`
    ```

## Completion Report

Output path to generated tasks.md and summary:
- Total task count
- Task count per user story
- Parallel opportunities identified
- Independent test criteria for each story
- Suggested MVP scope (typically just User Story 1)
- Format validation: Confirm ALL tasks follow the checklist format (checkbox, ID, labels, file paths)

Context for task generation: $ARGUMENTS

The tasks.md should be immediately executable - each task must be specific enough that an LLM can complete it without additional context.

## Task Generation Rules

**CRITICAL**: Tasks MUST be organized by user story to enable independent implementation and testing.

**Verification follows `docs/engineering.md`**: New or changed behavior uses vertical TDD by default, even when the Spec does not explicitly request tests. Put testing and implementation in the same behavior task. Do not require implementation methodology in the Feature Spec. Refactors, boundary probes, setup and documentation use the verification appropriate to their task type.

### Checklist Format (REQUIRED)

Every task MUST strictly follow this format:

```text
- [ ] [TaskID] [P?] [Story?] Description with file path
```

**Format Components**:

1. **Checkbox**: ALWAYS start with `- [ ]` (markdown checkbox)
2. **Task ID**: Sequential number (T001, T002, T003...) in execution order
3. **[P] marker**: Include ONLY if task is parallelizable (different files, no dependencies on incomplete tasks)
4. **[Story] label**: REQUIRED for user story phase tasks only
   - Format: [US1], [US2], [US3], etc. (maps to user stories from spec.md)
   - Setup phase: NO story label
   - Shared prerequisites phase: NO story label
   - User Story phases: MUST have story label
   - Cross-cutting and final acceptance phases: NO story label
5. **Description**: Identify the accepted behavior or task type, exact implementation/test or verification paths, and relevant acceptance condition or dependency. A behavior task contains its own test/implementation cycles; do not make separate RED, GREEN and refactor tasks.

**Examples**:

- CORRECT: `- [ ] T001 Verify workspace setup in Cargo.toml using the planned Cargo checks`
- CORRECT: `- [ ] T010 [US1] Implement ambiguous-checkout resolution using vertical TDD in crates/local-context/src/task_context.rs and crates/local-context/tests/resolution.rs; preserve all eligible candidates without selecting a winner (acceptance A2)`
- CORRECT: `- [ ] T011 [US1] Fix invalid explicit selection falling back using a failing regression test in crates/local-context/tests/resolution.rs and the correction in crates/local-context/src/task_context.rs (acceptance A3; depends on T010)`
- CORRECT: `- [ ] T020 Verify the accepted private-type boundary with a valid control and compile-fail probe in tools/xtask/src/probes.rs`
- ❌ WRONG: `- [ ] Create User model` (missing ID and Story label)
- ❌ WRONG: `T001 [US1] Create model` (missing checkbox)
- ❌ WRONG: `- [ ] [US1] Create User model` (missing Task ID)
- ❌ WRONG: `- [ ] T001 [US1] Create model` (missing file path)

### Task Organization

1. **From User Stories (spec.md)** - PRIMARY ORGANIZATION:
   - Each user story (P1, P2, P3...) gets its own phase
   - Derive bounded observable behavior slices from acceptance conditions, including relevant failure cases.
   - Each behavior task includes the smallest failing test, minimum coherent implementation, focused verification and necessary refactoring. Several small cycles may serve one bounded behavior.
   - Record the owning domain and accepted test boundary; an internal domain rule may be observable through an existing public domain API, without a new UI or application layer.
   - Mark actual story dependencies; preserve independent verification without pretending genuinely dependent stories are independent.

2. **From Contracts**:
   - Map each contract's observable rules to the behavior slices that implement them, including contract tests inside those tasks.
   - Keep type/visibility and architecture checks explicit where the boundary itself is the subject. Do not split all contract tests into a preceding batch.

3. **From Data Model**:
   - Map entities and relationships to the first behavior that needs them. Shared use alone does not make something a setup prerequisite.
   - Add a shared prerequisite only when it genuinely blocks identified slices. Do not generate base entities, services or other layers solely from template structure.

4. **From Setup/Infrastructure**:
   - Include only setup and infrastructure required by the accepted plan.
   - State which tasks each genuine prerequisite blocks; unrelated stories need not wait.
   - Keep story-specific setup within that story and classify its verification appropriately.

### Phase Structure

- **Setup / shared prerequisites**: Include only needed work with actual dependencies.
- **User stories**: In priority and dependency order, organize Behavior Slice A, Behavior Slice B, then story-level integration/acceptance when needed. Never group a story into all tests, all models, all services and all endpoints.
- **Cross-cutting verification**: Architecture/dependency checks and other accepted cross-story requirements. Run relevant focused checks during their owning slices as well.
- **Final acceptance**: Required full checks, acceptance evidence, documentation and Maintainer handoff. Passing tests does not grant Maintainer acceptance.
- Mark `[P]` only for whole independent tasks with non-conflicting files and satisfied dependencies, not to batch tests separately from implementation. The marker does not authorize spawning agents.
- Preserve completed task history. Apply this structure to new work, not retroactive rewrites of Foundation tasks.

## Done When

- [ ] tasks.md generated with all phases, task IDs, and file paths
- [ ] Extension hooks dispatched or skipped according to the rules in Mandatory Post-Execution Hooks above
- [ ] Completion reported to user with task count, story breakdown, and MVP scope
