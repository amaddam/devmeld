# Feature Specification: Context CLI and Resource Organization

**Feature Directory**: `specs/005-context-cli`

**Created**: 2026-09-09

**Status**: Maintainer authorized the discussed direction and next implementation step; incremental delivery, not implementation acceptance.

**Input**: Make local use practical without mandatory initialization; organize resources by logical paths with group descriptions, separate tags and descriptive fields, configurable inheritance, discoverable commands and explicit publication status.

## User Scenarios & Testing

### Managed record readability (2026-09-10 amendment)

The Maintainer requested TOML instead of JSON for DevMeld-owned configuration and
maintenance records. New contexts use `context.toml`, `state/owned.toml` and
`state/pending.toml` under `.devmeld`. Authored JSON resource descriptions and
JSON Schema are unchanged. The reading-view amendment below supersedes the
original requirement to link managed configuration from generated Markdown.
Configuration references use readable local path spellings, with `/` separators
on Windows where equivalent; internal canonical paths retain their exact meaning.
Record serialization must round-trip source/output bytes exactly, including CRLF,
quotes and backslashes, without weakening ownership, no-op or recovery checks.

This is an unreleased v0 format replacement, not v1/v2 ratification. Existing JSON
contexts are explicitly rejected and left intact, including pending recovery;
there is no automatic migration, adoption, deletion or mixed-format fallback.
New contexts may be created at a separate location. Help remains available without
opening records. Both formats present is also an error, never a precedence rule.

### Readable publication paths (2026-09-10 amendment)

The Maintainer accepted resource pages named after their logical organization
addresses instead of internal IDs: `services/shop` publishes to
`<output>/resources/services/shop.md`. The shared entry and `<output>/index.md`
remain the reading starting points. IDs and associations stay stable; moving a
resource/group changes affected page paths only on sync, which updates generated
links and withdraws unchanged old owned pages. External manually saved links are
not redirected. Source files and registration identities are not moved or renamed.

Publication must handle same leaf names in different groups and retain readable
Unicode names. Escape nonportable filename characters without altering logical
names, and reject case-insensitive or file/directory target collisions before
any writes. Existing target conflicts, edited old pages, stale previews and
interrupted-write recovery retain their protection. This is v0 output-layout
evolution, not a new persisted format, compatibility alias or source migration.

### Reading-oriented publication (2026-09-10 amendment)

The Maintainer approved separating generated reading material from configuration
inspection. Cards contain a short generated/do-not-edit comment, meaningful
description when supplied, source links and applicable context information;
omit the generic "Original document" placeholder, repeated maintenance prose,
configuration links and saved inherit/propagate controls. Full maintenance rules
belong in the project entry and total index, not every card. CLI show retains
the detailed registration, defaults, local/effective values and saved choices.

Publish final effective tags/fields once, without inheritance-origin labels;
origins remain available through CLI show. Omit empty metadata sections. Do not discard real
inherited values or confuse context fields with source-declared attributes.
Descriptions appear as readable prose, not a "local context annotations" wrapper;
if both an authored summary and registration description exist, label the latter
as context notes so their meanings remain distinct. No source/configuration,
identity, path, language selection or inheritance-rule changes are authorized.

The 2026-09-11 reading refinement uses a maintained Markdown serializer, with
field keys and technical literals in standard code spans rather than blanket
backslash escaping. Authored prose remains literal text, not interpreted Markdown
instructions; necessary syntax protection and URI encoding must remain correct.
This changes generated presentation only, not the persisted v0 format.

### Group documents and hierarchical navigation (2026-09-10 amendment)

The Maintainer approved a generated document for every registered group, including
empty groups. `code` publishes `resources/code/code.md`; nested `code/http`
publishes `resources/code/http/http.md`. These are managed reading artifacts,
not new editable sources or resource registrations. Resource card paths stay unchanged.

The total index lists only top-level groups and ungrouped resources with descriptions
and links. A group document shows its own description and effective tags/fields,
then links/descriptions for direct child groups and resources, plus navigation to
its parent (or the total index). Do not expand descendant metadata into every
ancestor. All registered groups/resources must be reachable by following local files.

Reuse portable filename encoding and validate the combined group/resource target
set before publication. A resource such as `code/code` conflicts with the group
page; reject with a clear path conflict rather than overwrite or silently rename.
Moves/removals update links and withdraw only unchanged owned former pages through
normal sync/recovery. Preserve source/configuration bytes on sync, all inheritance
rules, CLI inspection, source/access links, EN/zh-CN wording and write protections.
No new command, dependency, persisted format or automatic source discovery.

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
4. **Given** an enabled edge, **When** computing effective metadata, **Then** tags merge uniquely, child-local fields override same-name inherited fields, origins remain inspectable through show (not repeated in generated reading pages), and overall description, identity and source paths are not inherited.
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
