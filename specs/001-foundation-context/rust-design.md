# Rust Design: Domain Foundation

**Status**: Implemented mapping; native Windows verification complete, Maintainer acceptance pending
**Date**: 2026-09-07

**Authority**: [Product](../../docs/product.md), [Spec](spec.md), [Domain Model](data-model.md), [ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md), [ADR-0003](../../docs/adr/0003-rust-runtime.md)

This is an internal implementation design, not a new Product vocabulary or
public Rust SDK/serialization contract. Conceptual fields and invariants remain
in data-model.md; this document prevents their loss during the runtime restart.
Do not copy the partial WSL experiment as a completed foundation.

## Crate and API Boundaries

Use the four library members and dependency allowlist in [Plan](plan.md#allowed-edges).
Each core depends normally only on the shared identity crate. Modules default
to private; lib.rs re-exports intentionally exposed immutable facts, aggregates,
errors and policies. Do not expose an entire internal module to simplify a test.
The minimal shared kernel exports distinct RepositoryId and ResourceId newtypes
with private fields, validated construction, equality/hash semantics and readable
access. No path, Scope, status or universal error type belongs there.

Single-owner identities stay owner-local validated values. Choose a newtype when
mixing identity kinds is a real risk, not a blanket trait/type framework. Names
and aliases remain separate from identity. Text rules belong to the owning
concept; two crates using strings do not justify a generic shared utility crate.

## Domain 1: Project Catalog

| Planned module | Values and rules to preserve |
| --- | --- |
| references.rs | Required primary Vault reference, optional source references, portable source identity/locator validation |
| registrations.rs | Repository identity, Workspace association, canonical key, display name, aliases, optional declared source locator; Resource identity/source/type/locator/eligibility |
| profiles.rs | Profile identity, name, Workspace association, Repository/Resource selections; no Capability field or behavior |
| workspace.rs | Supported schema version, exactly one primary Vault, distinct sources, registration uniqueness, same-Workspace eligible references and checked updates/removals |

Validated fields are private. Update operations build and validate a candidate
snapshot before publishing it; failed updates leave the original unchanged.
Renaming does not change stable identity. Alias/canonical-key collisions are
checked across registrations, not merely inside one object. Referenced objects
cannot be silently cascade-deleted. No local path, machine/developer selection,
Active Checkout, credentials or derived-index location is portable Catalog state.

### Lexical Input Boundaries

Pin exact accepted/rejected examples in tests before implementing validators.
Preserve meaningful Unicode names; reject blank, boundary-whitespace and control
input. Do not equate Rust is_control with Python isprintable or silently normalize
opaque IDs. Retain a documented supported character policy rather than claiming
all Unicode edge cases were established by the previous implementation.

Initial shared identity examples: accept `repo-1`, `项目/主仓`, internal spaces
and distinct composed/decomposed Unicode spellings unchanged. Reject empty,
leading/trailing Unicode whitespace, and Unicode control characters (Rust
char::is_control). Equality is exact and case-sensitive; neither display names
nor locators inherit identity semantics simply because they contain text.

The initial portable locator subset covers relative source paths and credential-
free `http`, `https`, `ssh` and `git` URIs with plain ASCII DNS/IPv4 host spelling
and an optional numeric port. IPv6 literals, IDNA host input, SCP shorthand,
query parameters and fragments are outside this initial lexical subset.
Reject Windows drive/UNC/absolute and
POSIX absolute paths, home shorthand, file URIs, credentials, traversal segments,
malformed escapes, and encoded traversal/control characters. Validate decoded
meaning before accepting, and reject ambiguous repeated encoding instead of
guessing. This is lexical validation, not path existence, source parsing, network
access or a generic URL library. Do not narrow support to relative paths merely
because that was all the WSL sample implemented. If safe parsing requires a
dependency, document the actual need before changing the std-first baseline;
do not silently invent a general-purpose URI parser or accept unsafe input.

## Domain 2: Local Context Resolution

| Planned module | Values and rules to preserve |
| --- | --- |
| bindings.rs | Machine/developer identity, known Repository IDs, local paths/binding identity, explicit workspace/default selection, registry revision and checked local transitions |
| observations.rs | Observation ID, RepositoryId, local path, optional branch/ref and commit, working-tree state/details, observed time, observer revision, availability/freshness with reasons |
| task_context.rs | Raw selections and optional working area; normalization; rejected selection/reason; ValidatedTaskContext owning the checked snapshot |
| resolution.rs | Exactly Resolved/Ambiguous/Unavailable, RepositoryId, selected/considered candidates and basis/reason |

Removing a binding changes only the registry, never the external Checkout.
Registry revision changes on accepted mutation; observations are immutable and
later inspections create new facts. No filesystem/Git discovery occurs here.
Local native path values are not portable references; host/dialect differences
must be explicit in pure fixtures. Do not canonicalize, inspect existence or
convert paths into Repository identity.

LocalPath only checks absolute spelling in its supplied Windows/POSIX dialect;
it does not certify that the OS can open the path or resolve aliases, device
paths, dot segments or symlinks. Eligibility comes from supplied observation
facts, never from successful lexical construction alone.

Availability and freshness are explicit supplied facts, never implicit true
defaults or a hidden clock calculation. Preserve unknown/stale/unavailable
explanations. A timestamp is observation evidence, not a global transaction ID.

### Validation and Resolution

Raw task validation returns a success containing a private ValidatedTaskContext
or an InvalidTaskContext containing rejected selections and reasons. These are
not resolution variants. Identical duplicate selections may normalize; conflicting
selections, unknown observations, wrong Repository and ineligible explicit
choices fail before fallback. Contradictory snapshots such as duplicate observation
IDs are rejected instead of being overwritten during map construction.

The validated value owns the exact known-catalog IDs, normalized task selections
and immutable observations checked together. No public mutable getters or
unchecked conversion can replace them. Resolution must not accept a second,
unrelated observation slice: that would allow validated snapshot A to be used
with unvalidated snapshot B. Changed observations require revalidation.

For a required known Repository, use valid explicit task selection, valid workspace
selection, valid local default, then sole eligible candidate. Invalid weaker
preferences do not beat eligible observations; invalid explicit input cannot
reach this policy. Multiple eligible candidates without a decisive preference
remain Ambiguous, regardless of input order. No candidates means Unavailable.
An unknown requested Repository is a request-validation error, not a fourth
resolution state. All outcomes remain inspectable with Repository and evidence.

## Domain 3: Context Knowledge

| Planned module | Values and rules to preserve |
| --- | --- |
| provenance.rs | ResourceId at source revision, Source Type, Evidence locator/range or derivation, optional content hash, independent review and validity |
| relations.rs | Relation identity/derivation basis, typed directed endpoints, relation kind, Scope, source and supporting Evidence |
| scope.rs | Repository, Checkout/revision, environment, working context and version interval; per-dimension Match/Mismatch/Unknown with explanations |
| context_result.rs | Resource and Relation result facts, query Scope/Scope Match, source/review/validity, applicable Checkout basis, target availability and optional relevance explanation |

Represent independent status dimensions with separate types. Review does not
imply validity, a content hash does not imply trust, and matching scope does not
promote either. Non-declared inferred/asserted relations need traceable Evidence.
Reversed endpoints have different direction; unavailable targets remain visible
with a reason rather than being dropped or substituted.

Scope matching is pure and recomputed per query. Exact supported dimensions can
match or mismatch; missing information is unspecified, not universal. A known
mismatch is not hidden by another unknown dimension. Version-interval matching
remains Unknown until there is an accepted grammar/meaning; do not introduce
SemVer rules as accidental Product behavior. Preserve per-dimension explanations.

The internal `overall()` summary compares dimensions mentioned by either Scope.
Both-absent dimensions remain Unknown with `compared() == false`; they neither
claim universal scope nor prevent a match of the explicitly mentioned dimensions.
A one-sided missing value or a mentioned unsupported interval makes the summary
Unknown unless a known mismatch takes precedence. An entirely unspecified
comparison is Unknown. Consumers must retain the dimension details; this summary
is not a public query filter or an assertion of applicability in omitted contexts.

Evidence content hashes and derived Relation identities retain supplied opaque
values and their supporting facts. The foundation does not compute/verify hashes
or execute derivation algorithms, and does not infer trust from their presence.

Knowledge owns its consumer-facing immutable selected-Checkout facts. Test
composition maps Local Context's output into these facts explicitly; it does not
introduce a normal dependency on the Local Context crate. A Repository-only query
does not fabricate a Checkout requirement. When the query claims a Checkout or
revision, check compatible Repository/revision and retain the observation and
resolution basis. Both Resource and Relation results use this rule.

## Verification Design

Behavior tests cover cross-field validation and state transitions. Compile-fail
tests cover identity-kind mismatch, private validated construction, private
foreign internals and non-exhaustive result handling. Confirm exact intended
diagnostics after compiling a valid consumer control; missing tools or imports
are not evidence of a protected invariant.

Per-domain tests run independently. Cross-core acceptance uses two independently
constructed, equivalent immutable fact sets in Knowledge integration tests,
with the specific dev edges from Plan. If no external-fact port is needed, test
supplied facts directly instead of inventing a port to demonstrate substitution.

The real dependency gate is introduced after the cores exist and tested against
temporary copies. It is not an AST purity checker: std IO, misleading re-exports,
source inclusion, unchecked constructors and domain ownership still need review.
The gate and its harness live in the developer-only `tools/xtask` package and use
native Rust APIs without an OS shell. This tool is not a domain or product CLI;
its approved JSON parser dependency cannot enter the four core libraries.
See [Tasks](tasks.md) and [Acceptance Guide](quickstart.md).

## Non-Goals

Implementation and verification followed the documentation reset; Maintainer
acceptance is still pending. See Tasks and the Acceptance Guide for evidence.
No production executable/public SDK,
no protocol/schema freeze, no language benchmark, no capability selection, no
supporting-domain package, no database, no async runtime or GUI. Future adapters
must traverse these core rules, but are not scaffolded now.
