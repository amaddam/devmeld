# Feature Specification: Durable Context Publication

**Feature Branch**: current local branch (`main`); feature identity is independent of branch naming.
**Created**: 2026-09-08
**Status**: Implementation scope recorded from the Maintainer's direction to continue the reviewed file-based path; completion and acceptance remain separate.
**Input**: Deliver the reviewed resource-registration, access-guidance, preview, manual synchronization, independent file-reading and recovery path. Continue implementation and self-verification, preserving local Git checkpoints.

## User Scenarios & Testing

### User Story 1 - Prepare context that remains readable (Priority: P1)

A maintainer initializes a context, registers team-authored resources, previews the proposed files,
and publishes a common navigation and optional project entry. A reader follows those files without
running DevMeld or knowing its internal state.

**Why this priority**: This proves the actual product delivery model, not another in-memory foundation.
**Independent Test**: In an isolated real directory, register an existing document, publish with an
explicitly chosen entry, stop the command, and follow the entry to the original document.

**Acceptance Scenarios**:

1. Given an existing source document and empty context state, initialization and registration can be previewed without changing files; explicit application records their state.
2. Given registered resources, publication creates navigable files with purposes and editable sources; source bytes are unchanged.
3. Given a published entry, normal reading works while DevMeld is not running. Missing automatic client discovery is not represented as success.
4. Given unchanged inputs and rules, repeating publication produces identical bytes and does not rewrite unchanged targets.

### User Story 2 - Maintain reusable descriptions and tool guidance (Priority: P2)

A team describes a service and an existing tool, optionally validates custom descriptive fields,
and maintains the association through DevMeld. Changes update the derived information without
installing tools, connecting services or duplicating the project's dependency declarations.

**Why this priority**: It proves custom resources and usable access guidance rather than a document-only index.
**Independent Test**: Use team-authored service and tool descriptions and a local validation convention;
publish, edit an address, change an association and republish. US1 supplies the publication mechanism.

**Acceptance Scenarios**:

1. Given valid service fields and a registered tool, generated guidance links the source description, tool instructions and original dependency declaration.
2. Given missing required fields, generation explains the failure and leaves previous output unchanged; a permitted custom field needs no resource-specific implementation.
3. Given one tool association, guidance does not invent exclusive selection, availability or execution/installation authority.
4. Given a removed resource, affected navigation is updated without deleting source files; references to a removed tool require explicit resolution, not silent replacement.

### User Story 3 - Change and recover managed files safely (Priority: P1)

A maintainer can inspect changes before applying them, detect external edits and recover interrupted
writes without losing source data or undoing previously successful configuration maintenance.

**Why this priority**: Managed publication is not usable if it silently overwrites unrelated work.
**Independent Test**: With a previously published fixture, exercise a conflicting edit and a controlled
interruption. Recover only the changes belonging to that operation. This is required for US1, not optional polish.

**Acceptance Scenarios**:

1. Given an existing unowned target or a target overlapping source/configuration/recovery data, the operation refuses to adopt or overwrite it.
2. Given inputs or targets changed after preview, application refuses the stale preview.
3. Given interruption after a partial write sequence, the next operation reports pending recovery. Recovery restores the prior managed contents when safe, and reports external conflicts without overwriting them.
4. Given a successful configuration edit followed by failed publication, configuration remains updated, output is explicitly not current, and author-owned sources remain untouched.

### Edge Cases

- Empty contexts; duplicate identities/associations; missing descriptions; dangling internal references.
- Malformed input, unsupported control fields/versions/validation conventions, invalid custom fields.
- Spaces and Unicode in source paths; different current directories; portable identifiers and target-name collisions.
- Targets aliasing inputs, conflicting output/entry locations, links traversing redirected paths.
- External edits, missing ownership records, write errors, interrupted recovery and concurrent DevMeld writers.
- Partial availability of referenced original material is not proof of connectivity or current tool availability.

## Requirements

### Functional Requirements

- **FR-001**: Provide a human-facing way to initialize a context, register/remove resources, maintain access associations, choose an optional file entry, preview, publish and recover.
- **FR-002**: Preserve source ownership. Descriptions and original documents/scripts remain human/AI-owned; registration, associations and generated outputs use DevMeld maintenance mechanisms.
- **FR-003**: Organize all resources in one common context. Multiple entries do not filter resources. Separate contexts remain separately configured.
- **FR-004**: Support existing documents directly and general descriptions with author-provided titles, purposes, custom attributes and file references; do not require a database or Git-specific domain.
- **FR-005**: Apply optional local validation conventions to descriptive attributes without executing code or retrieving remote schemas. Invalid or unsupported semantic controls must fail explicitly.
- **FR-006**: Generated guidance must distinguish associations, applicable explicit selections, observations and authority; preserve Product's existing-option reuse rule and do not assert unobserved availability.
- **FR-007**: Publish traceable navigation, resource pages and an optional ordinary-file entry; existing source files and dependency declarations remain the facts of record.
- **FR-008**: Normal file consumption must require neither a live DevMeld process nor internal maintenance records; generated information must not claim to reflect unsynchronized changes.
- **FR-009**: Equal relevant inputs and generator rules must produce equal output bytes; unchanged files must not be rewritten.
- **FR-010**: Every managed modification must be previewable, explicitly applied within the selected surface, checked against the preview basis, and recoverable through recorded before/after intent.
- **FR-011**: Reject unowned/external edits and source-target overlaps; never silently adopt, merge, delete sources or select replacement tools.
- **FR-012**: Report configuration and publication outcomes separately. Failed publication cannot roll back successful earlier configuration changes or author-owned edits.
- **FR-013**: Detect unfinished operations and cooperative concurrent writers. Recovery must not overwrite external changes; no adversarial filesystem isolation or cross-file atomic snapshot is promised.
- **FR-014**: Provide actionable human diagnostics and non-success outcomes for invalid input, conflicts and pending recovery. Do not expose credentials or fabricate successful verification.

### Key Entities

- **Resource registration**: stable reference to a document or authored description, with optional descriptive validation convention.
- **Resource facts and access association**: input-backed description and declared relationship, distinct from current availability/authority.
- **Publication**: deterministic derived files and their source correspondence.
- **Managed change**: selected target, observed original, intended result and recovery information; not another source of resource truth.

## Success Criteria

### Measurable Outcomes

- **SC-001**: One real fixture containing a document, service description and tool can be maintained, published and read entirely through the human entrypoint and filesystem path.
- **SC-002**: All links in that generated entry/navigation resolve to the intended fixture files without a running DevMeld process; source bytes remain unchanged.
- **SC-003**: Two consecutive unchanged publications are byte-identical and report zero changed targets.
- **SC-004**: Address edits, permitted custom fields, association changes and removals produce the expected navigation/guidance with no source deletion or environment change.
- **SC-005**: Invalid-input, external-conflict, stale-preview, interruption and conflicting-recovery cases all fail explicitly without silently overwriting unrelated bytes.
- **SC-006**: Platform verification reports actual executed hosts. Unexecuted platforms and Agent automatic loading remain explicitly unverified.

## Assumptions

- Operates on explicitly selected, user-authorized local context and project surfaces. No hostile metadata sandbox, global source discovery or network access is required.
- Maintainers own their input facts and ensure publishable descriptions do not contain secrets; validation is not a universal secret detector.
- Initial synchronization is manual and whole-context; targeted/periodic triggers remain future capabilities using the same domain rules.
- Native filesystem errors are reported; no bounded-latency promise for a hung network-mounted filesystem or power-loss-proof multi-file transaction is made.
- No GUI, runtime query server, tool executor/installer, Git observation, embedding/indexing engine, autonomous AI generation, project-instruction patcher or Skill adapter is included.
- This scope instantiates the already discussed product path. It does not ratify the Constitution, approve a stable public machine API, change governance authority or revive old 001/002 requirements.
