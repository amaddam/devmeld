# Conceptual Domain Model: DevMeld Foundation

**Feature**: [Domain Foundation](spec.md)  
**Status**: Reviewed foundation design; architecture/runtime accepted; five domains designed, three core domains selected for first implementation  
**Date**: 2026-09-01

**Last Updated**: 2026-09-02

## Purpose

This is a conceptual domain model: it assigns language, state, decisions, and
invariants to owners. It is not a database schema, serialization schema, class
diagram, or instruction to create a class for every noun.

Implementation names may be narrower than product terms when necessary, but
they must preserve the semantics in `docs/product.md`. Persistence records,
transport messages, parser nodes, provider objects, and UI models are mappings
of this model rather than the model itself.

The accepted [architecture](../../docs/adr/0001-domain-oriented-modular-monolith.md)
does not require every domain to have an application layer. Create application
coordination only for actual cross-object/domain coordination outside domain
objects. Empty packages and pass-through services do not implement a domain
boundary; pure domain policies need no application wrapper.

## Context Map

```text
Capability Integration ── capability registration facts ──┐
                                                          │
Project Catalog ── catalog selections and stable IDs ─────┼──> Context Knowledge
      │                                                   │
      └── repository facts ──> Local Context Resolution ──┘
                                      │
                                      └── resolution facts ──> Context Knowledge

Project Catalog ── profile/resource selections ───────────┐
Context Knowledge ── context/resource snapshots ──────────┼──> Managed Materialization
Capability Integration ── client/capability facts ────────┘
```

Arrows describe immutable facts or stable identities that a future feature may
need. They do not require a port or package in this foundation and do not grant
the receiver authority to mutate the publisher's state. No module uses another
module's storage adapter as an integration API.

| Publisher | Public fact | Consumer | Contract rule |
| --- | --- | --- | --- |
| Project Catalog | Repository identity and aliases | Local Context Resolution | Local state refers to `RepositoryId`; it cannot redefine the Repository |
| Project Catalog | Context Profile selection | Context Knowledge | Selection is copied as an immutable request snapshot |
| Local Context Resolution | Checkout resolution result | Context Knowledge | Result includes status, basis, candidates and observed revision |
| Context Knowledge | Context/resource snapshot | Managed Materialization | Snapshot is input evidence, not mutable knowledge state |
| Capability Integration | Provider, provisional capability-declaration, and Agent Client compatibility facts | Project Catalog / Managed Materialization | Design only; the standalone Product meaning of `Capability` requires Maintainer approval |

## Shared Kernel Admission Rule

The shared kernel may contain only values whose meaning and validation are
identical across all consuming domains:

- opaque stable identifiers with explicit kinds;
- portable representation-version values whose meaning is genuinely shared;
- non-domain-specific result correlation identifiers;
- immutable timestamp/revision primitives only if every domain uses the same
  semantics.

The following do **not** belong in the shared kernel merely because several
modules use similar shapes:

- filesystem paths, URLs, source locators, content hashes, status enums;
- generic entity/audit bases;
- validation helpers tied to one domain rule;
- error-code registries for unrelated operations;
- DTOs, persistence records, or provider types.

Similarity is not semantic identity. Duplicating a small value type is cheaper
than coupling domains through a misleading abstraction.

## Domain 1: Project Catalog

### Responsibility

Own stable project identity and portable composition: what a Workspace is, which
logical Repositories and Resources exist, and what a Context Profile selects.
It does not know where a current machine stores a Checkout or how content is
indexed.

### Aggregate: Workspace

| Member | Meaning | Constraints |
| --- | --- | --- |
| `WorkspaceId` | Stable identity | Opaque and immutable |
| name | Human-facing label | Non-blank; not identity |
| primary Vault reference | Required portable knowledge-source reference | Exactly one; no absolute local path |
| additional source references | Optional declared knowledge sources | Stable references; no duplicate identity |
| Context Profile references | Named selections available in the Workspace | Every reference belongs to the same Workspace |
| schema version | Portable representation version | Explicit and supported |

**Invariants**:

1. A Workspace has exactly one primary Vault reference.
2. Portable Workspace state contains no developer, machine, absolute path,
   Active Checkout, credential, or derived-index location.
3. Additional sources cannot replace or silently redefine the primary Vault.
4. Removing a referenced object is rejected until affected profiles are revised
   explicitly; cascading deletion is not implicit domain behavior.

### Aggregate: Repository Registration

| Member | Meaning | Constraints |
| --- | --- | --- |
| `RepositoryId` | Stable logical-repository identity | Independent of clone, worktree, path and current branch |
| canonical key | Portable unique key | Unique within the Workspace catalog |
| display name | Human-facing label | Mutable without identity change |
| aliases | Recognized portable names | Cannot collide with another Repository's canonical key or alias |
| declared source locator | Optional source-owned locator | Portable form only; not a local Checkout path |

**Invariants**:

1. One logical Repository may correspond to zero, one, or many local Checkouts.
2. Branch, commit, working-tree state and local path are never Repository
   properties.
3. Identity survives display-name and alias changes.

### Aggregate: Context Profile

This is the complete conceptual design. The first implementation covers only
identity, naming, Repository selections, Resource selections, and portable
rules independent of Capability semantics. Capability selections remain design
only. Do not add placeholder Capability IDs, fields, default-empty lists, or
no-op selection behavior; the implemented subset does not redefine the full
Context Profile meaning in `docs/product.md`.

| Member | Meaning | Constraints |
| --- | --- | --- |
| `ContextProfileId` | Stable selection identity | Unique within Workspace |
| name | Human-facing label | Non-blank |
| Repository selections | Eligible logical Repositories | Stable IDs only |
| Resource selections | Eligible project resources | Stable IDs only |
| capability selections | Requested context functions; standalone `Capability` definition is pending Product approval | Design only; stable references in the conceptual model, absent from first implementation |
| portable selection policy | Cross-machine selection semantics | Cannot contain local paths or current-machine choices |

**Invariants**:

1. Every selected ID must be known to the Workspace's accepted catalog view.
2. A Context Profile defines eligibility, not an execution workflow or ordered
   task pipeline.
3. Selecting a Repository does not persist an Active Checkout.
4. A capability selection names required semantics, not one provider instance;
   this design does not yet establish `Capability` as a standalone Product
   concept.

### Concept: Resource Registration

Project Catalog owns only a Resource's stable identity, declared source
reference, type discriminator, stable locator, and selection eligibility.
Context Knowledge owns the contextual content/provenance view. These are public
facts joined by `ResourceId`, not one cross-module mutable object.

## Domain 2: Local Context Resolution

### Responsibility

Own current-developer and current-machine facts used to map portable project
identities to concrete local Checkouts and to choose one Checkout for a task.
It does not own Repository identity, content truth, or Git command execution.

### Aggregate: Local Binding Registry

| Member | Meaning | Constraints |
| --- | --- | --- |
| `MachineId` | Current machine identity | Local-only |
| `DeveloperId` | Confirmed local developer identity | Local-only; not inferred as portable truth |
| Repository bindings | `RepositoryId` to candidate local locations | Multiple candidates allowed |
| workspace selections | Optional current-workspace preference | Local and explicit |
| client locations | Optional Agent Client locations | Design only in this foundation; local and capability-scoped |
| registry revision | Optimistic consistency token | Changes on accepted local mutation |

**Invariants**:

1. A binding references a catalog identity but cannot create or redefine it.
2. Absolute paths may exist here because this is local state; they never flow
   into portable catalog facts.
3. A default or preference is explicit data with a basis, not a guess derived
   from iteration order.
4. Removing a binding does not delete or modify the external Checkout.

### Immutable Fact: Checkout Observation

| Member | Meaning |
| --- | --- |
| `CheckoutObservationId` | Identity of this observation, not the Checkout's portable identity |
| `RepositoryId` | Logical Repository observed |
| local path | Concrete observed location; local-only |
| branch/ref | Observed Git reference, if available |
| commit | Observed revision, if available |
| working-tree state | Clean/dirty/unknown plus explainable details |
| observed at | Observation time |
| observer revision | Source adapter/version fact |

An observation is immutable. A later scan creates a new observation rather than
mutating historical evidence in place.

### Value: Task Context and Validation

Task Context carries explicit Checkout selections and an optional current
working area for one resolution request. It is request-scoped local input, not a
stored workflow and not portable project state.

Validation occurs before Active Checkout resolution:

```text
TaskContextValidation
├── ValidTaskContext
│   └── normalized, non-conflicting selections eligible for resolution
└── InvalidTaskContext
    ├── selection names the wrong Repository
    ├── explicit selections conflict
    ├── observation is unknown
    └── observation is ineligible
```

An invalid Task Context is a validation failure, not an Ambiguous or Unavailable
resolution. It cannot fall through to workspace preference, local default, or a
sole candidate.

### Domain Result: Active Checkout Resolution

The resolution policy accepts only `ValidTaskContext`.

```text
ActiveCheckoutResolution
├── Resolved
│   ├── RepositoryId
│   ├── selected CheckoutObservation
│   ├── basis
│   └── considered candidates
├── Ambiguous
│   ├── RepositoryId
│   ├── candidates
│   └── missing decision reason
└── Unavailable
    ├── RepositoryId
    ├── known bindings/observations
    └── reason
```

Allowed resolution bases, from strongest to weakest, are:

1. valid explicit task selection;
2. valid current-workspace selection;
3. valid explicit local default binding;
4. sole eligible observed candidate.

For valid input, multiple equally decisive eligible candidates produce
Ambiguous, while absence of an eligible observable candidate produces
Unavailable. Candidate order never selects a winner. Invalid explicit
selection never reaches this policy, and lower-priority evidence cannot
silently replace it.

## Domain 3: Context Knowledge

### Responsibility

Own the semantics required to explain and scope context: Resource context,
typed directed Relations, Evidence, Scope, independent provenance/status
dimensions, Scope Match, and stable query-result facts. It does not crawl Git,
parse a Vault, or persist an index.

### Context Resource Fact

A Context Resource Fact is an immutable view of a cataloged `ResourceId` at a
declared source revision. It may include a title, content reference, media/type
facts, and provenance. It is not a universal `KnowledgeEntity` and need not be a
long-lived aggregate.

### Relation

| Member | Meaning | Constraints |
| --- | --- | --- |
| `RelationId` | Stable or deterministically derived identity | Identity method is explicit |
| source ID | Stable project-object identity | Required |
| relation type | Directed domain meaning | Not a free-form display label |
| target ID | Stable project-object identity | Required |
| Evidence references | Support for the assertion | At least one for asserted/derived relations unless explicitly declared |
| applicable Scope | Where the relation is claimed to hold | Explicit |

Reversing source and target produces a different semantic relation unless the
relation type explicitly declares symmetry.

### Evidence

Evidence records a traceable source reference, locator/range or derivation
description, observed revision, and optional content hash. Evidence supports a
claim; it does not by itself determine review, validity, or current Scope Match.

### Scope

Scope is a composable value describing where a fact may apply, such as logical
Repository, resolved Checkout/revision, environment, version interval, or named
working context. Missing dimensions mean unspecified, not universal.

### Independent Dimensions

| Dimension | Question answered |
| --- | --- |
| Source Type | What kind of source produced or declared this fact? |
| Review Status | What human review has occurred? |
| Validity Status | Is the fact currently accepted, superseded, invalid, or unknown? |
| Scope Match | Does the fact's declared Scope match this query context now? |

`ScopeMatch` is computed for a query and contains matched, mismatched, and
unknown dimensions plus an explanation. Computing it never mutates the other
three dimensions.

### Query Result Fact

A query result must include stable object identity, source identity, Evidence,
declared Scope, Scope Match, independent review/validity facts, and applicable
Checkout-resolution basis. Relevance/ranking is a separate derived fact and may
not erase provenance.

**Invariants**:

1. Evidence, Scope, Review Status, Validity Status, and Scope Match retain their
   independent meanings in every context result where they apply.
2. Query-time computation cannot mutate source-owned or portable facts.
3. Ranking or relevance cannot erase provenance or turn a scope mismatch into an
   ordinary in-scope result.
4. Relations retain direction and unavailable targets remain explainable.

## Domain 4: Managed Materialization

**Delivery classification**: Supporting domain, design only. This feature
creates no package, port, test double, public operation, or machine contract for
it.

### Responsibility

Own the decision model for controlled generated changes. It decides whether a
change is eligible to apply, verify, or revert; future adapters inspect and
modify actual files or client configurations.

### Aggregate: Materialization Plan

| Member | Meaning | Constraints |
| --- | --- | --- |
| `MaterializationPlanId` | Identity of one immutable preview | Changes if inputs or proposed output change |
| provider/capability identity | Producer semantics | Registered and compatible |
| input revisions | Facts used to render | Complete enough to reproduce/verify |
| Planned Changes | Ordered set of bounded target changes | At least one, no overlapping ownership claims |
| created at / expiry policy | Preview validity | Apply cannot ignore expiration or changed preconditions |

### Value: Planned Change

Each Planned Change names a Managed Surface, target identity, ownership claim,
expected-before hash/state, proposed-after hash/content or semantic operation,
human-readable preview, and reversal boundary.

### Fact: Materialization Manifest

The manifest records the applied plan identity, exact owned surfaces, before and
after hashes, input revisions, application receipt, and revert information. It
does not grant ownership over content outside the applied plan.

### State Model

```text
Previewed ── valid ownership + matching preconditions ──> Applied
    │                                                    │
    ├── conflict/stale/expired ──> Conflict              ├── exact match ──> Verified
    │                                                    ├── changed ─────> Drifted
    └── discarded ──────────────> Expired                └── eligible revert ─> Reverted
```

**Invariants**:

1. Apply requires the exact immutable preview and matching preconditions.
2. A provider cannot claim a surface already owned by a person or another
   provider without an explicit governance-approved transfer contract.
3. Verify compares the managed boundary; it does not adopt drift automatically.
4. Revert touches only manifest-owned content and requires compatible current
   state. Conflict is safer than destructive guessing.
5. A materialization plan never instructs an Agent how to execute development
   work; it publishes context-discovery material only.

## Domain 5: Capability Integration

**Delivery classification**: Supporting domain, design only. This feature
creates no package, port, test double, public operation, or machine contract for
it. A standalone `Capability` remains a proposed Product concept.

Gate 3 applies to any implementation that depends on that meaning, regardless
of domain. It does not block Capability-independent core tasks or acceptance.
Resolving it does not add this supporting domain to the current Feature scope.

### Responsibility

Own stable registration and compatibility facts for the external capabilities
DevMeld may expose or consume. It does not own provider runtime objects, client
execution, authentication, or generated surfaces.

### Aggregate: Capability Provider Registration

| Member | Meaning |
| --- | --- |
| `CapabilityProviderId` | Stable registration identity |
| provider kind and version | Compatibility fact, not runtime instance |
| provisional capability declarations | Named context functions offered; not yet accepted as a standalone Product concept |
| required inputs / emitted semantic forms | Compatibility facts without fixing a public protocol |
| status | Enabled/disabled/unsupported with reason |

### Aggregate: Agent Client Registration

An Agent Client registration names the client kind/version, supported context
functions or semantic forms, and local binding reference if needed. It never
contains the client's workflow, prompt history, or execution authority.

### Compatibility Result

Compatibility is an explicit compatible, incompatible, or unknown semantic
result with reasons. Unknown is not treated as compatible. Provider-specific
feature flags must be translated at the boundary rather than becoming Product
language.

**Invariants**:

1. Provider and Agent Client runtime objects never enter a domain API.
2. Unknown compatibility is never treated as compatible.
3. Registration does not grant execution, authentication, or write authority.
4. No provisional capability declaration becomes cross-feature Product
   language until a Maintainer accepts its meaning in `docs/product.md`.

## State Lifecycle Matrix

| Fact/object | Class | Source of truth | Rebuild/recovery rule |
| --- | --- | --- | --- |
| Workspace, Repository, Context Profile, Resource registration | Portable | Versioned project/Vault metadata | Read directly; migrate explicitly by schema version |
| Developer/Machine identity, Local Binding, local client location | Local | Human-inspectable local registry | Re-enter or rediscover without changing portable state |
| Checkout Observation | Local observed fact | External Checkout plus observation receipt | Re-observe; never synthesize as portable truth |
| Search index, extracted relation candidate, rank | Derived | Declared portable/local sources | Delete and rebuild; no unique authoritative fact allowed |
| Materialized client artifact | Generated | Preview inputs plus manifest | Verify, regenerate, or bounded revert |
| Evidence in source-owned content | Portable/external source-owned | Declared source | Preserve locator/revision; do not copy authority into index |
| Scope Match | Query-derived | Scope plus current query context | Recompute; never persist as underlying validity |

## Identity, Equality, and Revision Rules

1. Stable IDs identify product objects across name, path, or display changes.
2. Local filesystem paths identify locations only within a machine context; they
   are not stable project identity.
3. Content hashes prove byte/content equality, not semantic validity or
   ownership.
4. Revisions make observations and semantic facts comparable; they do not imply one
   global transaction across domains.
5. Human-facing names are mutable labels unless a domain explicitly defines a
   canonical key invariant.
6. Derived deterministic IDs must name their derivation inputs and version so a
   rule change cannot silently collide with an older identity.

## Cross-Domain Consistency

Where an implemented scenario genuinely needs coordination outside domain
objects, use explicit coordination rather than a distributed aggregate:

- an application operation obtains immutable facts from each owner;
- the owning domain validates its state transition against those facts;
- the result names the input revisions it used;
- partial external effects are handled by explicit receipts/compensation in a
  later adapter feature, not by pretending all domains share one transaction;
- stale facts produce conflict or retry, never silent last-write-wins across
  owners.

Domain events may later publish accepted facts, but only when a named consumer
and delivery semantics are known. Events do not replace explicit semantic
boundaries or become a default integration mechanism.

## Negative Model Constraints

- `Knowledge` is a product semantic category, not a required entity, aggregate,
  inheritance root, or table.
- `Resource` is a product term covering selectable project content; it is not a
  mandatory base class for Repository, file, note, relation, or provider object.
- `Active Checkout` is a resolution outcome, not persisted portable state.
- An invalid explicit Checkout selection is a Task Context validation failure,
  not an Active Checkout resolution variant.
- `Capability` is a provisional implementation-facing label until its
  standalone cross-feature meaning is accepted in `docs/product.md`.
- Capability-dependent values, fields, rules, ports, and fixtures are excluded
  from the first implementation, including those in Project Catalog or the
  shared kernel. Do not disguise unsupported selection behavior with defaults.
- `Extension` is not a stable domain entity until a future requirement proves
  identity and lifecycle distinct from Capability Provider or adapter.
- `Benchmark Run` belongs to verification evidence unless a later product
  decision makes benchmarking a user-managed capability.
- Provider SDK objects, Git objects, Markdown AST nodes, database records,
  command-line options, and UI view models cannot cross into domain APIs.
- A free-form `metadata` or `type` switch cannot carry facts whose invariants are
  known enough to deserve an explicit value or boundary.
- A domain map is not a required package inventory. Application coordination and
  ports exist only for demonstrated needs, never for layer symmetry.

## Maintainer Review Questions

Before task generation, Maintainers should answer yes to all of the following:

1. Does every invariant have one owning domain?
2. Does the design avoid sharing mutable objects or persistence details between
   modules?
3. Does every port and test double serve a rule in one of the three implemented
   core domains rather than a planned external system?
4. Are portable, local, derived, and generated facts impossible to confuse in
   the public model?
5. Do ambiguity, unknown compatibility, stale facts, drift, and conflicts remain
   explicit failure/result states?
6. Does every shared abstraction have identical semantics rather than merely
   similar implementation shapes?
7. Can the three core domains be tested without implementing either supporting
   domain or claiming that a user-visible workflow has been delivered?
8. Are Task Context validation failures kept separate from the three valid-input
   Active Checkout outcomes?
9. Are public operations, envelopes, pagination, error catalogs, and protocol
   versions still deferred to later Features?
10. Can the declared core subset proceed after the architecture/runtime gates
    without a Capability decision, while capability-selection implementation
    remains explicitly excluded?
11. Does every application coordination component serve a real need outside
    domain objects, with no empty package or pass-through service?
