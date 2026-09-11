# Organization and CLI Model

This is the intended model for the complete Feature, not a claim that all fields exist yet.

## Ownership

- Resource Organization owns logical membership, group paths, local annotations, inheritance and association integrity.
- Context Publication receives resolved facts/provenance and owns output/entry maintenance.
- Application code owns context selection, typed use-case requests/results and persistence coordination. The CLI adapter owns argument syntax, terminal rendering and confirmation input. No third domain is introduced. Prepared changes preserve inspect/apply and rechecking for every caller; internal result types are not a public wire protocol.

## Values and relationships

- `ResourceId`: stable internal identity, retained across logical moves. Allocate independently from the display path; no random ID or source hash requirement is introduced.
- `OrganizationPath`: slash-separated relative logical segments, not an OS path. Reject empty/dot/parent segments and resource/group collisions. Native source paths remain independent.
- `Group`: path, optional overall description, local tag set, named context fields, saved `inherit` and `propagate` booleans.
- `ResourceRegistration`: identity, logical path, source reference, local annotations and saved `inherit`. Existing access associations continue to use identities.
- `Defaults`: saved `inherit` and `propagate`; omitted CLI choices are resolved at creation, not dynamically at read time. Missing parents created during an explicit add use the same defaults and have no fabricated descriptions.
- `EffectiveAnnotations`: tags/fields plus per-value origin (current node or ancestor path). Computed, never an independently maintained registry.

## Annotation boundaries

An authored description file retains its own title, summary, attributes and schema. Registration/group annotations describe context organization and do not rewrite that file or claim inherited data was declared there. Presentation keeps source-declared attributes and context annotations identifiable; a matching key does not silently merge the two authorities.

All new context fields are descriptive. Convenient CLI flags and `--field KEY=VALUE` produce the same named field, not separate storage. A standard boolean flag such as `--shared` is a descriptive value, not an execution permission.

## Effective inheritance

For each immediate parent-child edge, inherit only if the parent propagates AND the child receives. Merge parent effective tags with local tags uniquely. Overlay child-local named fields on parent effective fields; preserve origins for effective values. Do not inherit identity, source path or overall description.

A disabled edge cuts more distant ancestors too. A child refusing its parent can still propagate its own values. Defaults changes leave saved choices unchanged; parent-value changes are reflected at next computation/publication. No per-field exclusions or arbitrary merge expressions in this Feature.

## Lifecycle and failure

- Parse/select/preview do not create storage. Successful first mutation establishes only the selected context, using existing owned-write machinery.
- Logical move updates organization path and affected descendant addresses, never source locations or stable associations. Collision and self-descendant moves are errors.
- Whole publication recomputes affected navigation deterministically; source/metadata changes before apply invalidate a stale plan.
- Empty groups can be removed explicitly; nonempty group removal must not recursively remove registrations by default. Removing a registration never deletes its source.
- Existing context files are not rewritten merely by selecting or reading them. Any incompatible v0 record evolution is documented and rejected untouched unless an explicit future migration is authorized.

## T005 persistence decisions

Retain draft `format_version: 0`. Add optional registration `path`, a `groups` list and `next_resource_id` counter to the existing configuration. On older records, missing `path` means the existing flat ID is also the organization address, missing groups means an empty list, and the counter starts at 1. Read/preview never materializes these defaults on disk. New registrations store a separate address and an allocated `resource-N` identity; allocation skips occupied identities and persists a checked monotonic counter so removals do not recycle them. No source hash or random dependency is required.

Existing ownership and publication records retain their shape and IDs. Source descriptors remain unchanged. New configuration fields may be rejected by older development builds; there is no new release/version label or automatic migration command. Logical paths are case-sensitive Unicode names, permit internal spaces, and reject empty/dot segments, surrounding whitespace, controls, backslashes and colons. The 2026-09-10 readable-page amendment replaces stable-ID filenames: the publication adapter maps validated logical segments to portable page paths, without treating the raw logical name as a native path or changing resource identity. The CLI contract owns escaping and collision behavior.

## T006-T007 context references and first use

CLI native operands resolve from the invocation directory. Resolved inputs within the selected context are persisted relative to that context; outside inputs retain an absolute native local path, including other drives. Description-file references still resolve from the description's directory. Logical addresses are never passed to native path resolution.

The first successful add saves configuration and ownership evidence in one existing transaction. Preview and validation create no directory; application atomically creates a previously absent `.devmeld` marker and refuses a marker appearing after preview. Inferred incomplete state blocks fallback and implicit init. Explicit `--context PATH init` (or `init PATH`) after a completed rollback remains supported when configuration and owned surfaces are absent; this creates new configuration, not ownership of unrelated remnants.

## T009 annotation representation

Keep `format_version: 0`. Each resource registration may contain `annotations` with optional `description`, a tag list and a string-valued `fields` object. A group with annotations is represented as `{ "path": "database", "annotations": { ... } }`; a plain group path remains readable and is still written as a string when it has no annotations. Reading does not rewrite records. Older development builds may reject annotated groups/registrations; no migration or user/example-context rewrite is implicit.

In the pure organization model, annotations belong to their actual group/resource node and move with that node. Missing parent groups receive empty annotations. Fields are descriptive text, including `shared=true`/`shared=false`; shortcuts and `--field` store identical string values. No type guessing, field-specific core implementation or authorization is derived from these values. Tags are unique and fields sort by key. Description/value text may contain line breaks; names reject controls, empty/surrounding whitespace and field keys containing `=`. Descriptions must be nonblank when supplied; an explicitly empty field value is allowed. Source-description attributes retain their independent meaning/schema.

## T010-T011 inheritance and persistence

Keep draft `format_version: 0`. Context `defaults` stores `inherit` and `propagate` booleans (initial false/true). New resource registrations explicitly save `inherit`; new groups save both booleans in their object record, even when annotations are empty. This extends the T009 group representation; plain group strings and older object/resource records remain readable. Missing legacy choices always mean fixed false/true, never the current context defaults. Read/preview does not materialize them. A successful organization mutation may serialize those same fixed choices explicitly; a defaults-only change leaves all existing node records unchanged. Older builds may reject new records; no migration or v2 label is introduced.

Organization owns creation-time defaults and saved node choices. Explicit add flags affect the target only; automatically created parents (including move destinations) use the current defaults. Moves preserve existing nodes' choices and recompute effective annotations under the new ancestry. Updates with omitted flags preserve choices. `config set defaults.inherit/defaults.propagate` changes one value in an existing context; it does not implicitly initialize a new context.

Effective fields retain one origin, replaced by the nearest local declaration even when its text is identical or empty. A unique effective tag retains all contributing node paths. The derived view contains tags/fields only: no overall description, identity, source location or authority. Computation follows the immediate-parent chain without recursion or cached child declarations. Removing a local annotation may reveal an inherited value; per-value inheritance suppression is outside 005.

## T012-T013 publication inspection

T024 adds a derived group document, not a new stored domain object. Every group
maps to `resources/<group>/<leaf>.md`, including implicit and empty groups.
The document presents the group's description/effective annotations and links
to direct children and its parent. The root index links only top-level groups
and ungrouped resources. Group/card destination paths are validated together;
both use existing publication ownership and recovery. Configuration, identities
and inheritance rules are unchanged.

Status is a read-only application result, not a persisted entity or new domain. It derives expected generated content from current configuration and sources using the same preparation as sync, compares against owned files and rechecks the captured inputs. Entry registration belongs to configuration; publication evidence belongs to the existing ownership receipts. Missing inputs, conflicts or pending recovery block verification.

No timestamp, counter, source snapshot or new format is stored for status. A changed document body can leave its generated reference unchanged; status reports generated-content equivalence, not historical source freshness or Agent consumption. Entry attach/create map to the existing insertion/whole-file ownership modes, with no receipt or transaction-engine redesign.

T026 refines only permanent whole-file evidence: SHA-256 plus physical identity
replaces duplicated bodies. Exact instruction insertions and temporary recovery
images remain. The [CLI contract](contracts/cli.md#compact-ownership-evidence-2026-09-11-t026)
owns the narrow preceding-TOML conversion; configuration and domain data are unchanged.
