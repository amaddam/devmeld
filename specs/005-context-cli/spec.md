# Feature Specification: Context CLI and Resource Organization

**Feature Directory**: `specs/005-context-cli`

**Created**: 2026-09-09

**Status**: Maintainer authorized the discussed direction and next implementation step; incremental delivery, not implementation acceptance.

**Input**: Make local use practical without mandatory initialization; organize resources by logical paths with group descriptions, separate tags and descriptive fields, configurable inheritance, discoverable commands and explicit publication status.

## User Scenarios & Testing

### User Story 1 - Discover the command to use (Priority: P1)

A user can ask for help at the root, a command group, or a specific operation before choosing or creating a context.

**Why this priority**: Users must be able to discover current functionality without a setup prerequisite or guessing arguments.

**Independent Test**: Run root, resource-group and resource-add help in an empty directory; each succeeds, lists only implemented syntax and creates no files.

**Acceptance Scenarios**:

1. **Given** no context, **When** help is requested at a supported level, **Then** it succeeds without resolving or reading context data.
2. **Given** a selected but unavailable context location, **When** operation help is requested, **Then** the help remains available without creating that location.
3. **Given** an unknown command or help topic, **When** invoked, **Then** it fails with useful help direction and no managed writes; it does not present unrelated root help as success.

### User Story 2 - Add a real source without a setup ceremony (Priority: P1)

A user starts in a project, adds an existing source under a meaningful logical path and publishes readable navigation. They need not invent an internal ID or run init first.

**Why this priority**: This is the smallest practical registration-to-reading journey.

**Independent Test**: In an empty project, add a separate local document, inspect the pending registration, sync, and follow the generated navigation to the unchanged source without DevMeld running.

**Acceptance Scenarios**:

1. **Given** an empty project, **When** a valid first source is added, **Then** local context storage is created at the chosen location, its location is reported, and publication is reported as pending.
2. **Given** a context in a parent directory, **When** adding from a descendant directory without an explicit context, **Then** the nearest existing context is reused; relative input paths refer to the invoking directory.
3. **Given** an explicit context location, **When** adding a resource, **Then** that location takes precedence; no second project-local context is created.
4. **Given** invalid input, existing corrupt/unowned state, or a pending recovery, **When** an operation is attempted, **Then** it fails without silently reinitializing, adopting data, or partially registering the source.

### User Story 3 - Organize and describe resources (Priority: P2)

Users create nested groups, optional descriptions, tags and named descriptive fields; they can list, inspect and move resources independently of physical files.

**Why this priority**: Logical organization must remain useful as resources grow beyond a flat list.

**Independent Test**: Add two sources with the same leaf name under different groups, move one, and inspect generated navigation plus its existing access association.

**Acceptance Scenarios**:

1. **Given** a nested resource address, **When** adding it, **Then** missing groups may be created without invented descriptions; their creation is reported.
2. **Given** descriptions, tags and custom fields on a group or resource, **When** inspecting or publishing it, **Then** those distinct information kinds are presented without modifying the authored source.
3. **Given** an existing resource and associations, **When** moving its logical path, **Then** its stable identity, associations and source location remain unchanged; collisions fail without replacement.

### User Story 4 - Reuse descriptions with explicit inheritance (Priority: P2)

Users configure defaults for parent propagation and child inheritance, override either choice on individual items, and inspect where effective information came from.

**Why this priority**: Reuse must not turn classification into hidden resource facts or require repetitive entry.

**Independent Test**: Exercise both parent and child controls, same-name overrides, tag union, a three-level hierarchy and a defaults change; inspect provenance and synchronized output.

**Acceptance Scenarios**:

1. **Given** configurable defaults, **When** creating items, **Then** explicit choices win and omitted choices are resolved and persisted using creation-time defaults.
2. **Given** an existing item, **When** defaults change, **Then** its saved choices do not change.
3. **Given** a parent and child, **When** either disables the connecting inheritance edge, **Then** parent information is not inherited; the child's own information can still propagate to its children.
4. **Given** an enabled edge, **When** computing effective metadata, **Then** tags merge uniquely, child-local fields override same-name inherited fields, origins remain visible, and overall description, identity and source paths are not inherited.
5. **Given** a changed parent, **When** syncing, **Then** derived metadata is recomputed rather than copied into child declarations.

### User Story 5 - Understand and control publication (Priority: P2)

Users distinguish saved configuration from published navigation, preview or confirm publication, and explicitly attach a project entry when wanted.

**Why this priority**: Less typing must not weaken ownership or imply an Agent has already loaded the result.

**Independent Test**: Register, inspect status, preview, publish, publish again, attach/detach an instruction entry and inspect unchanged host text and sources.

**Acceptance Scenarios**:

1. **Given** a valid scoped registration/configuration command, **When** executed, **Then** its change is inspectable and saved without the old redundant apply-word ceremony; dry-run writes nothing.
2. **Given** pending publication, **When** syncing interactively, **Then** actual changes are previewed and require one confirmation; explicit noninteractive confirmation skips only the prompt, not conflicts or stale checks.
3. **Given** a no-op sync, **When** executed, **Then** it neither prompts nor rewrites output and still checks freshness/ownership.
4. **Given** no chosen project entry, **When** setting up storage or publishing navigation, **Then** project instructions are not edited automatically.
5. **Given** a registered entry, **When** inspecting status, **Then** configured, published and client-consumed are not conflated.

### Edge Cases

- Nearest context is malformed, inaccessible, incomplete or nested inside another context; do not fall through to a different context.
- Explicit location does not exist; failed validation/dry-run must not leave newly initialized state.
- Paths contain spaces, Unicode or different local drive roots; retain the local-only link contract.
- Duplicate logical paths, resource/group collisions, moving a group into itself, and missing associations fail without partial edits.
- Parent changes while a publication is being confirmed; stale input rejects the application.
- Noninteractive confirmation is missing; return an actionable error rather than wait indefinitely.
- User-authored source attributes and context annotations are distinct inputs; do not rewrite the source or silently label inherited annotations as source-declared facts.

## Requirements

### Functional Requirements

- **FR-001**: Provide side-effect-free help at each implemented command level, without context prerequisites (US1).
- **FR-002**: Support explicit context selection and nearest-ancestor reuse, with current-directory initialization only on a valid creating operation when no context exists (US2).
- **FR-003**: Preserve native local/cross-drive paths; resolve command-line relative paths from the invoking directory, not an accidentally selected ancestor (US2).
- **FR-004**: Keep stable resource identity distinct from user-facing logical organization paths and source locations (US2/US3).
- **FR-005**: Support nested groups, optional descriptions, separate tags and extensible named fields without requiring a new core implementation for every field (US3).
- **FR-006**: Apply the two independent configurable inheritance choices, creation-time default capture, merge rules and inspectable origins described in US4.
- **FR-007**: List/show organization and allow logical moves without source moves or identity/association changes (US3).
- **FR-008**: Distinguish saved, pending publication, published and client consumption; provide non-mutating inspection and previews (US2/US5).
- **FR-009**: Preserve managed ownership, conflict rejection, stale checking and recovery while simplifying confirmation (US5).
- **FR-010**: Do not execute tools, fetch remote indexes, invent resource facts, implicitly edit project instructions or create inherited permissions (all stories).
- **FR-011**: Preserve English/Chinese generated wording, untranslated authored/technical information and independent file consumption (US3/US5).

### Key Entities

- **Context**: one explicitly selected organization and publication boundary, not one per Agent question or entry.
- **Resource identity / organization path / source reference**: three distinct facts with different lifetimes.
- **Group**: organization node with optional annotations and propagation/receiving choices.
- **Local annotations**: description, tags and named fields explicitly maintained for a group or registered resource.
- **Effective annotations**: derived tags/fields with provenance, not another editable fact source.
- **Defaults**: creation-time choices for inheritance and propagation; changing defaults does not mutate existing choices.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Root, group and operation help succeed in an empty directory with zero filesystem mutations.
- **SC-002**: A user can add an existing source and publish its navigation in two management commands without an explicit initialization command or internal ID.
- **SC-003**: Every organization/move scenario preserves source bytes, resource identity and existing valid associations.
- **SC-004**: All four combinations of parent propagation and child inheritance, plus a three-level break and creation-default change, produce the specified effective metadata with visible provenance.
- **SC-005**: Read-only commands, cancelled publications, conflicts and invalid first operations do not write managed content; no-op publication leaves bytes and timestamps unchanged.
- **SC-006**: Real entrypoint verification records actual Windows/Linux results independently; macOS and unexecuted client cases remain explicitly unverified.

## Assumptions

- Existing 003/004 ownership, local link and recovery behavior remains applicable unless this Spec explicitly replaces a CLI interaction.
- Default storage is project-local, not a global registry or a background service. Initial propagation/inheritance defaults are true/false respectively, both configurable.
- Extra fields are descriptive, not executable plugins. Overall group description is not automatically inherited.
- This is unreleased v0 design evolution; no v2 branding or destructive migration of existing user/example contexts is authorized.
- The first delivered slice can be command discovery alone. Later unbuilt commands must not appear in live help as supported.

## Non-Goals

GUI, scheduler, network indexing, cross-machine mapping, a new owning domain, arbitrary executable extensions, per-field inheritance expressions, inherited authorization, automatic source copying, global context merging, paid Agent benchmark, automatic Git commit or push.
