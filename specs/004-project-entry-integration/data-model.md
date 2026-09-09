# Data Model: Project Instruction Entry Integration

**Status**: Authorized implementation model for [004](spec.md), not a new Product
domain or a public library protocol. [The contract](contracts/shared-instructions.md)
owns serialized representation and exact byte rules.

## Responsibilities

| Fact | Owner / meaning | Not its responsibility |
| --- | --- | --- |
| Resource membership and access associations | Existing Resource Organization | Entry-specific filtering or selecting a tool for the current question |
| Desired entry attachment | Maintained publication configuration: local target and `file` or `instructions` kind | Proof the entry has been published or read |
| Whole-file claim | Context Publication: last owned complete content and physical identity | Permission to accept external edits |
| Insertion claim | Context Publication: exact published insertion at one context-bound target | Ownership of surrounding text or permanent host inode |
| Recognition result | Adapter observation: absent/unique matching insertion/conflicting bytes | Granting ownership because markers look familiar |
| Operation basis | Storage adapter: current full bytes, presence and physical identity of captured inputs/targets | A permanent claim over the complete shared host |
| Planned mutation | Application/storage coordination: after-image plus explicit ownership effect | Inferring whole-file ownership from an installed file |
| Recovery step | Existing transaction adapter: complete before/after observations and staging evidence | Merging later external edits or reverting earlier committed operations |

No new global entry identity, Repository, Profile, AgentSession or patch-processor
domain is introduced. Context-root provenance in local records is not a portable
resource identity or a machine-mapping mechanism.

## Ownership and operation facts must remain separate

An internal ownership value has two explicit alternatives:

- **Whole file**: previous complete UTF-8 bytes and physical identity, preserving
  003's existing conflict rules.
- **Instruction insertion**: previous exact UTF-8 insertion, including markers
  and separators. Context/target binding comes from its enclosing local record.
  Its newline style is derived from those bytes, not reselected on every sync.

For each preview, storage captures a fresh complete host observation. An editor
can replace the file while preserving the insertion before preview; an old
inode/file ID is not grounds to reject that shared-host edit. After preview,
any observed bytes/presence/identity change invalidates application, even when
the planned publication was a no-op.

Each planned write carries an independent next-ownership effect: retain/update
a whole-file claim, retain/update an insertion claim, or release the claim.
In particular, detach has a present after-image, possibly empty, plus release.
It is never represented as file deletion. The receipt itself remains internal
maintenance state, not a user publication or a recursively self-owned file.

## Attachment lifecycle

| Desired configuration | Existing maintenance evidence | Result of successful sync |
| --- | --- | --- |
| Instruction target added | No claim; no reserved marker; valid host or absent file | Insert entry, record insertion claim |
| Instruction target retained | Unique exact recorded insertion | Replace only insertion if generation changed; otherwise no write |
| Instruction target removed | Unique exact recorded insertion | Remove insertion, keep host, release claim |
| Target added then removed before any publication | No insertion claim | No host mutation |
| Target removed then re-added before detachment | Existing matching insertion claim | Maintain same claim; do not append another block |
| Any desired state requiring maintenance | Missing/edited/ambiguous insertion or unreliable record | Conflict, no adoption or repair |
| Requested kind differs from existing whole-file/insertion claim | Incompatible ownership mode | Conflict; no automatic conversion |

Successful registration/removal is one configuration transaction. Later sync is
another publication transaction. Its failure or recovery does not roll back the
earlier configuration. A pending journal blocks new normal operations until
verified recovery has completed.

## Validation boundaries

- Resolve paths through existing local-machine rules. Compare target paths and
  observed physical aliases against sources, other entries, generated files and
  reserved config/state. The same file must not acquire two ownership modes.
- Validate the text and reserved-marker envelope without interpreting Markdown.
  The publication policy consumes explicit recognition/evidence facts; do not
  pass artificial `evidence_matches=true` through whole-file classification.
- A missing receipt, wrong context provenance or copied-looking block is not a
  fresh installation opportunity. No automatic rebuilding of ownership by
  scanning existing generated content is allowed.
- Registration establishes intent, not present source freshness, client loading,
  reader permissions or execution authority. Sync validates current inputs and
  targets again before proposing any publication.

## Persistence and recovery

The typed claim map holds one kind per canonical target. All contexts initialized
by the new implementation use the same v0 config/receipt/journal baseline,
whether they publish whole files, insertions or both. There is no v1 adaptation
layer and no format transition when adding or removing an insertion.
Unsupported old records are evidence to preserve, not current ownership facts
to decode or adopt; exact rejection behavior is defined in the contract.

The transaction still records complete file effects. For a shared update this
means the host captured for this preview, with its current authored text, plus
the new insertion. A recovery step can restore that operation's before-image
only while its full evidence is still valid. If the author edits the host after
an uncommitted interruption, rollback stops, including when only outside text
changed. A committed operation instead needs verified staging/journal cleanup;
it does not restore targets or reject later author edits merely to finish cleanup.

Ordinary detach leaves a file that attachment created. Recovery of an interrupted
creation may instead restore verified prior absence: undoing an uncommitted
operation and detaching a committed entry are different lifecycle operations.
