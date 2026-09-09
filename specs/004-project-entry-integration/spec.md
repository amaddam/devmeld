# Feature Specification: Project Instruction Entry Integration

**Feature Branch**: current local branch (`main`); no branch change is required to identify this feature.

**Created**: 2026-09-09

**Status**: Scope and revised [Plan/contracts](plan.md) authorized by Maintainer on 2026-09-09; implementation and required [verification](acceptance.md) complete, Maintainer acceptance pending.

**Input**: Continue from 003's completed file-publication tasks toward the Product's project-entry path. The proposed next slice connects generated context to a user-selected project instruction file, while preserving human-owned content. The Maintainer requested reassessing models rather than accumulating patches around old assumptions.

## User Scenarios & Testing

### User Story 1 - Connect a project's instructions to prepared context (Priority: P1)

A maintainer selects a project instruction file that their Agent Client reads,
such as AGENTS.md, and attaches a small context entry. A new session can follow
that entry to the existing common navigation and choose relevant resources.
The user need not repeatedly supply the context location, and DevMeld need not
remain running. This relies on the selected client's actual instruction-loading
behavior, not the filename alone.

**Why this priority**: 003 can publish independently readable files, but an
ordinary generated entry does not establish that a session will encounter it.

**Independent Test**: In an isolated project with existing instructions, preview
and confirm attachment/publication. Open a fresh session through a supported,
explicitly recorded client setup. Without supplying a separate DevMeld path in
the task prompt, follow the entry to a known source containing the task's answer.

**Acceptance Scenarios**:

1. **Given** an existing project instruction file with no managed entry, **when** attachment is previewed, **then** the proposal identifies the selected file and new entry content, changes nothing, and does not claim ownership of existing instructions.
2. **Given** explicit confirmation and valid context inputs, **when** the entry is published, **then** only the authorized insertion is added; all pre-existing bytes remain unchanged and the entry leads to the context's common navigation.
3. **Given** the selected file does not exist, **when** its creation is explicitly confirmed, **then** publication creates a file containing the entry without creating or rewriting other project instructions. Future user-authored text outside the entry remains user-owned.
4. **Given** a successfully published entry and a supported client setup, **when** a fresh session reads the project's instructions, **then** it can find a relevant original resource without a running DevMeld process or a prompt that explains where DevMeld stores context.
5. **Given** no entry attachment was requested, **when** a context is maintained or synchronized, **then** no project instruction file is discovered or modified automatically.

### User Story 2 - Maintain the entry without taking over the host file (Priority: P1)

A team continues editing its own project instructions while DevMeld maintains
only its entry. Changes to context output location or language update that entry;
detachment removes it without deleting the host file, resources or other entries.

**Why this priority**: Shared-file ownership is necessary for a usable entry,
not optional hardening after attachment is shipped.

**Independent Test**: Start from an attached fixture, edit instructions outside
the entry, update the context location/language, synchronize and detach. Compare
the author-owned bytes throughout and verify all remaining entry links.

**Acceptance Scenarios**:

1. **Given** the recorded entry is unchanged but the team edited text outside it before preview, **when** publication is prepared and applied, **then** those edits are valid input and are preserved; the whole file is not treated as an externally edited DevMeld-owned artifact.
2. **Given** an output-location or generated-language change, **when** the next publication is confirmed, **then** the entry reflects that change, keeps the same resource set, and leaves surrounding instructions unchanged. Local cross-drive destinations remain usable under 003's link rules.
3. **Given** unchanged inputs and entry content, **when** publication repeats, **then** no duplicate entry is added and the host file is not rewritten.
4. **Given** a verified managed entry, **when** removal is configured and the following publication is confirmed, **then** only the managed insertion is removed. The host file remains even if empty, and other entries, generated context and original resources are not removed because of this detachment.

### User Story 3 - Refuse uncertain edits and recover incomplete publication (Priority: P1)

A maintainer can distinguish an authorized entry update from an unsafe overwrite.
External edits to the entry, ambiguous ownership and interrupted publication
must be reported without silently adopting content or restoring over user work.

**Why this priority**: Sharing a file must not weaken the existing write and
recovery guarantees. US1 is not complete without these failure outcomes.

**Independent Test**: Use prepared shared-file fixtures to exercise entry edits,
stale previews, missing ownership, ambiguous entry boundaries and interruption.
Verify preserved author bytes, explicit failures and recoverable operation state.

**Acceptance Scenarios**:

1. **Given** an existing entry is edited, removed, duplicated, malformed or lacks reliable ownership evidence, **when** maintenance would modify that file, **then** it reports the conflict without repairing, appending a replacement entry or adopting matching-looking content.
2. **Given** the host file or other publication inputs change after preview, **when** the preview is applied, **then** application rejects the stale basis without overwriting those changes; a fresh preview is required.
3. **Given** an interrupted publication and unchanged recovery basis, **when** recovery is confirmed, **then** it restores that operation's verified prior effects, including shared-file content, while retaining previously successful configuration changes.
4. **Given** external changes make recovery uncertain, **when** recovery is attempted, **then** it preserves those changes and reports the unresolved operation; it does not restore an old whole-file copy over author edits merely to complete rollback.

### Edge Cases

- Empty or absent host file; no final newline; differing line endings; Unicode and spaces in paths or instructions.
- Apparent entry delimiters inside examples; malformed, repeated or ambiguous managed regions. Their exact recognition contract belongs in Plan/contracts, not guesswork during application.
- Existing generated whole-file entry at the selected path; shared-file and whole-file ownership must not be silently interchanged.
- Another context's entry in the host file; missing/copied maintenance records; a manually copied generated-looking section.
- Unsupported maintenance formats, including an old pending operation; format incompatibility must not trigger deletion, adoption or automatic migration.
- Host files aliasing registered sources, output, configuration or recovery data; redirected paths retain existing restrictions.
- Outside-entry edits before preview versus any changes after preview; interrupted attachment, update, detachment or recovery.
- One context attached to multiple projects; one attachment removed while others remain; a file becoming empty after detachment.
- Existing instructions that conflict with reading the context, disabled client discovery, reader permissions or blocked file links. None is proof of successful automatic consumption.

## Requirements

### Functional Requirements

- **FR-001**: Provide explicit, previewable, confirmed maintenance of an entry in a user-selected local project instruction file, alongside 003's ordinary whole-file entry. No automatic search for or modification of instruction files is allowed.
- **FR-002**: Own only the identifiable managed insertion, not the surrounding file. Preserve author-owned bytes, ordering and formatting, including existing line endings. Any separators introduced by attachment belong to the insertion, not retroactively to the author's content.
- **FR-003**: Emit a small navigational entry pointing to the existing common context, with enough reading and maintenance guidance to distinguish source edits from managed updates. Do not copy the resource catalog into every project or require the reader to run DevMeld.
- **FR-004**: Keep context resources and selection semantics unchanged across entry forms. The entry does not select tools for a session, grant execution or installation authority, override higher-priority instructions, or introduce a project-specific resource subset.
- **FR-005**: Keep entry configuration maintenance separate from publication. A successfully saved registration/removal is not evidence that the host file was updated. A failed publication preserves earlier successful configuration changes and reports the unpublished result.
- **FR-006**: Recompute managed entry content from current authorized inputs, using the selected English/Chinese generated language and established local relative/cross-drive link behavior. Do not translate or normalize surrounding authored text. A successful publication must lead to the intended published navigation, not an unresolved or unrelated target.
- **FR-007**: Accept independently edited outside-entry content observed before preview while requiring the existing managed insertion to match its ownership evidence. Changes after preview invalidate that preview; no implicit merge or overwrite is allowed.
- **FR-008**: Require reliable evidence for update/removal of an existing managed insertion. Ambiguous boundaries, edits to the insertion, missing owned content or absent ownership records fail explicitly. Appearance or equal text alone does not authorize adoption.
- **FR-009**: Detachment removes only the verified insertion and releases its maintenance claim. Never delete the host file as a consequence of entry detachment, including a file initially created by attachment. Preserve other publications and original sources.
- **FR-010**: Retain source/target overlap protection, cooperative-writer checks, stale-preview rejection and operation-scoped recovery. Restore only verified effects of an interrupted operation; stop on uncertain external changes rather than overwrite them. No cross-file atomicity, hostile-editor isolation or automatic conflict merging is promised.
- **FR-011**: Repeated unchanged publication must neither duplicate the insertion nor rewrite the host file. Preserve ordinary whole-file entry behavior in the current ownership model; compatibility with development-era maintenance formats is not required. Unsupported existing records must be rejected before writes and left intact, without automatic conversion, deletion or adoption.
- **FR-012**: Allow multiple host files to refer to one common context, but at most one DevMeld-managed entry per host file in this slice. An existing entry owned by another context is a conflict, not permission to merge, replace or federate contexts. Replacing whole-file ownership with shared-file ownership requires a separate explicit transition; it is not automatic in 004.
- **FR-013**: Distinguish published-file correctness from actual client discovery and consumption. Support claims must name the client setup and executed evidence; creating a correctly named file is insufficient proof that an Agent reads it.

### Key Entities

- **Entry attachment**: a maintained association between one context's common navigation and one explicitly selected project instruction file.
- **Shared instruction surface**: author-owned surrounding content containing an identifiable DevMeld-managed insertion; distinct from a wholly generated file.
- **Entry maintenance evidence**: the facts needed to identify the authorized insertion, compare its last published state and recover verified changes. It does not own the project's instructions or resource facts.

These describe responsibilities, not mandatory new classes or domains. Resource
Organization continues to own resource facts and associations; Context
Publication owns the entry's maintenance boundary. Client-specific file discovery
is not a new resource-selection rule.

## Success Criteria

### Measurable Outcomes

- **SC-001**: In fresh and existing project fixtures, preview, confirmed attachment, update and detachment preserve every pre-existing byte outside the authorized insertion. The fixture set includes mixed authored languages, differing line endings and absent final newlines.
- **SC-002**: Two project instruction files can reach the same context and its original resources with DevMeld stopped. Test local same-drive and real Windows cross-drive locations and both generated-output languages; do not infer drive IO support from string tests alone.
- **SC-003**: Two successive unchanged publications produce zero changed host files and exactly one managed insertion per attachment; modification times remain unchanged.
- **SC-004**: Author edits outside the insertion survive subsequent maintenance. Entry edits, stale previews, missing ownership and ambiguous-boundary cases fail without overwriting external content or adding replacement entries.
- **SC-005**: Interruption at each implemented mutation boundary is recoverable when its basis remains valid. Tests with external changes demonstrate refusal to overwrite them; previously committed configuration changes survive failed publication and recovery.
- **SC-006**: At least one named Agent Client setup demonstrates a fresh session finding a known original resource through project instructions, without being given a separate context path and without calling DevMeld to read. Record the host, client setup and observed read path. Unavailable integration testing remains an explicit acceptance gap, not a simulated pass.
- **SC-007**: Ordinary-entry scenarios continue to pass on fixtures initialized under the current maintenance model, without converting ownership or changing resource content. Unsupported-format fixtures fail without changing their files or creating maintenance artifacts. No source edits, tool execution, dependency installation or remote-index access result from entry publication.

## Assumptions

- On 2026-09-09 the Maintainer clarified that this is unreleased design evolution,
  not a second product or stable format release. The implementation is in the v0
  development stage. Indexes and resource content carry no release label; any
  internal format discriminator is only a draft marker, not a compatibility promise.

- The Maintainer accepted the shared-file write scope and, on 2026-09-09, continuing model evolution without mandatory compatibility with development-era implementations/formats. This removes the earlier requirement to keep old contexts maintainable by the new implementation; it does not authorize deleting, migrating or adopting existing data. It also does not declare 003's excluded instruction-file patching already implemented or grant implementation acceptance.
- Projects explicitly choose the instruction file appropriate to their reader. AGENTS.md is an example, not a universal discovery contract; no client-specific precedence or startup behavior is assumed without verification.
- Plan/contracts must choose the precise attachment command, insertion placement, boundary recognition, supported text representation, ownership-record compatibility and recovery behavior. They must reassess the current whole-file ownership assumption rather than silently treating a shared file as wholly generated.
- Only current local-machine scope is included. No remote indexing/mapping, mandatory daemon, Skill installation, automatic project discovery, instruction-file migration, legacy maintenance-format converter, multi-context shared-file arbitration, scheduler or targeted-resource synchronization is introduced.
- Existing resource registration, source validation, common navigation, language and cross-drive link capabilities come from 003. They are reused as behavior contracts, not requirements to preserve every current implementation signature or module layout.
- Generation remains deterministic and contains no runtime AI calls. Selecting an external Agent for acceptance does not authorize installation, paid invocations or access to real team secrets; execution must use existing authorized tooling and isolated fixtures.
