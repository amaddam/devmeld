# Research: Domain Foundation

**Feature**: [Domain Foundation](spec.md)  
**Date**: 2026-09-01  
**Last Updated**: 2026-09-02  
**Status**: Architecture/runtime accepted; Capability Product meaning remains deferred

This document records foundation reasoning. Accepted architecture/runtime
decisions are owned by ADR-0001 and ADR-0002; detailed quality rules are owned by
the Engineering Guide. Research alone does not accept Product concepts or
public protocols.

## Decision 1: Use a Domain-Oriented Modular Monolith

**Status**: Accepted on 2026-09-02; Gate 1 recorded in
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md).

**Decision**: Use one locally deployable application organized by domain. Map
all five domains, but create code only for Project Catalog, Local Context
Resolution, and Context Knowledge in the first foundation.

**Rationale**:

- DevMeld's main complexity is preserving distinct meanings, authorities, and
  lifecycles while external tools remain replaceable.
- Package-by-domain keeps rules beside the language that owns them and avoids
  global `models`, `services`, and `repositories` dumping grounds.
- A monolith keeps local deployment and refactoring simple while boundaries are
  still being learned.
- Designing a supporting boundary does not justify creating its package before
  an approved feature needs it.
- Creating `application/` requires actual coordination outside domain objects.
  Pure domain rules need no wrapper; empty layers and pass-through services
  would add structure without solving a coordination problem.

**Rejected alternatives**:

- transaction-script or use-case-first core that distributes invariants across
  handlers and adapter branches;
- horizontal layers across the whole application that hide domain ownership;
- microservices per domain before an operational need exists;
- empty packages, ports, and test doubles that make a diagram look implemented.

**Acceptance consequence**: The architecture ADR now owns this lasting choice;
the Plan and tasks apply its demand-created layer rules to the first code.

## Decision 2: Treat DDD as an Ownership Discipline

**Decision**: Use DDD to define ubiquitous language, ownership, invariants,
value objects, aggregates where consistency requires them, domain policies, and
anti-corruption boundaries. Do not require a domain class for every noun.

**Rationale**: The objective is to prevent accidental coupling and misplaced
logic, not to maximize pattern count. Some Product concepts are categories,
results, facts, or external registrations rather than long-lived entities.

**Rejected defaults**:

- one aggregate and one repository per noun;
- a common `Entity`, `Resource`, or `Knowledge` base hierarchy;
- event sourcing or a global domain-event bus;
- a generic CRUD service or Generic Repository;
- a universal metadata map used to avoid explicit concepts.

## Decision 3: Create Ports Only for Implemented Core Needs

**Decision**: An implemented core domain or its application layer may own a
narrowly named port only when one of its current rules requires an external
fact. The corresponding substitute exists to test that rule. A future Git,
Vault, filesystem, storage, Codex, CLI, Desktop, or provider adapter does not by
itself justify a port now.

**Rationale**: A need-owned port can keep a core rule stable when an external
tool changes. A future-facing port instead freezes guesses about operations,
data shape, and failure semantics before there is acceptance evidence.

**Rejected alternatives**:

- creating ports and substitutes for all five designed domains;
- generic `Provider`, `Repository<T>`, `Storage`, or `Client` interfaces;
- passing library objects or database rows into domain APIs;
- letting entrypoints choose domain outcomes through workflow-specific
  condition branches.

Managed Materialization and Capability Integration therefore have no ports,
test doubles, or packages in the first implementation.

## Decision 4: Keep State Lifecycles Separate

**Decision**:

| State class | Authority | Foundation rule |
| --- | --- | --- |
| Portable | Versioned project/Vault source | Human-readable, machine-independent, usable without DevMeld |
| Local | Current developer/machine | Human-inspectable, separate from portable truth, may contain local paths and selections |
| Derived | Declared portable/local sources | Disposable, rebuildable, never authoritative over its sources |
| Generated | An approved materialization plus ownership evidence | Human-readable, ownership-bounded, verifiable and reversible by design |
| Query-derived | Declared facts plus current query context | Recomputed for the query; never persisted as underlying truth |

**Rationale**: Combining these lifecycles would leak machine details into shared
project data or make a cache, query result, or generated artifact an accidental
source of truth.

**Rejected alternatives**:

- one database for authoritative, local, indexed, and generated state;
- portable Repository records containing absolute Checkout paths;
- treating generated Agent artifacts as independently editable project truth;
- persisting Scope Match as if it changed Evidence or Validity Status.

## Decision 5: Validate Task Context Before Resolving Active Checkout

**Decision**: Explicit task selections are validated before resolution. Wrong-
Repository, conflicting, unknown, or ineligible explicit selections produce a
Task Context validation failure. Only a valid Task Context enters the Active
Checkout policy, which returns exactly `Resolved`, `Ambiguous`, or
`Unavailable` with its basis and considered candidates.

**Rationale**: An invalid request and a valid request with insufficient local
facts are different problems. Combining them would allow a malformed explicit
choice to fall through to a weaker default and could select the wrong Checkout.

**Rejected alternatives**:

- mapping an invalid explicit selection to Ambiguous or Unavailable;
- a global mutable `active=true` flag on a Checkout;
- first path wins, latest modified wins, or alphabetical fallback;
- merging several Checkouts without explicit task intent.

## Decision 6: Keep Provenance Dimensions Independent

**Decision**: Source Type, Evidence, Scope, Review Status, Validity Status, and
query-time Scope Match are separate values. A context result composes them but
does not persist one as an inferred replacement for another.

**Rationale**: A human-reviewed note may be out of scope; an automatically
derived relation may be valid; a source may be authoritative for one fact but
not another. One status cannot safely summarize all dimensions.

**Rejected alternatives**:

- one trust, quality, or confidence field;
- updating Review Status when Scope Match changes;
- treating derived evidence as inherently unreviewed or invalid;
- allowing ranking to hide provenance or turn a mismatch into an ordinary hit.

## Decision 7: Preserve Managed-Write Semantics as Design Only

**Decision**: Keep Managed Materialization as a documented supporting-domain
boundary. Its design owns immutable preview, target preconditions, ownership,
manifest, verification, drift/conflict, and bounded revert semantics. This
feature creates no package, port, fixture, operation, or adapter for it.

**Rationale**: Write safety needs an explicit owner, but its application API and
implementation should be shaped by the first approved materialization feature,
not by a foundation-only hypothetical workflow.

**Rejected alternatives**:

- omit write-safety constraints until after filesystem code exists;
- implement write helpers or a generic target port now;
- freeze preview/apply/verify/revert operation signatures in this foundation;
- treat matching content as owned without explicit ownership evidence.

The non-binding design is recorded in
[Materialization Semantics](materialization-semantics.md).

## Decision 8: Defer Application Protocols

**Decision**: Record non-binding semantic sketches instead of versioned machine
contracts. The foundation may state facts and invariants that a later API must
preserve, but it does not select:

- operation names or request/response envelopes;
- CLI, desktop, IPC, HTTP, or provider transport;
- pagination or cursor behavior;
- ranking representation;
- public error codes or error payloads;
- approval identity fields, receipts, or compensation APIs;
- compatibility or schema-version rules for a public protocol.

**Rationale**: Stable application contracts require real consumers and
workflows. Freezing them now would turn guesses into compatibility obligations
and could make transport DTOs look like the domain model.

**Rejected alternatives**:

- retaining `context-contract-v1.md` and `materialization-contract-v1.md` as
  foundation contracts;
- using planned CLI output or JSON examples as the canonical boundary;
- exposing provider-native objects or an unversioned free-form dictionary.

The retained design artifacts are [Context Semantics](context-semantics.md) and
[Materialization Semantics](materialization-semantics.md).

## Decision 9: Use Python 3.14 for the First Code Foundation

**Status**: Accepted on 2026-09-02; Gate 2 recorded in
[ADR-0002](../../docs/adr/0002-initial-python-runtime.md).

**Decision**: Target Python 3.14.x, use standard-library types in the production
core, and avoid production parser, database, CLI, UI, and provider libraries in
this phase.

**Rationale**: The runtime's typing and data-model facilities are
sufficient for a framework-independent domain core. Restricting dependencies
keeps the first implementation review focused on language and invariants.
The [Python versions status](https://devguide.python.org/versions/) was rechecked
on 2026-09-02 and lists 3.14 in stable bugfix maintenance. The local planning
interpreter reports 3.14.6; a supported patch version and compatible tool
versions must still be recorded and verified during implementation setup.

**Rejected alternatives**:

- adopting a prerelease runtime;
- adding an application framework before an adapter or entrypoint is in scope;
- choosing persistence or CLI libraries as part of domain design.

**Acceptance consequence**: ADR-0002 and the Engineering Guide now own the
runtime and tooling baseline. This decision does not claim that implementation
or cross-platform verification has happened.

## Decision 10: Adopt a Progressive, Non-Overlapping Tool Baseline

**Status**: Accepted as part of Gate 2. The authoritative settings are in the
[Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates).

**Decision**: Begin with three quality tools only:

- Ruff with conservative `E`, `F`, `I`, `UP`, and `B` rule families, ignoring
  `E501` and leaving logical simplification and complexity rules disabled;
- mypy strict for core domain and any implemented application/core-owned port code, with
  narrow documented overrides only where genuinely necessary;
- pytest for invariant, behavior, and failure-case evidence.

Do not initially enable Ruff `ALL`, `SIM`, duplicate annotation linting, or hard
gates for complexity, branches, arguments, returns, statements, function
length, or file length. Add Import Linter only after the three core packages
actually exist and only for accepted dependency contracts. Defer Sonar,
additional type checkers, Hypothesis, and commit-hook frameworks until a
concrete non-duplicated need exists.

**Evidence retained from the initial repository comparison**:

- FastAPI combines Ruff, strict mypy, and pytest while explicitly ignoring
  Ruff's `C901` complexity rule.
  [Source](https://github.com/fastapi/fastapi/blob/master/pyproject.toml)
- Starlette uses a narrow Ruff selection with strict mypy and pytest rather than
  enabling every lint or complexity rule.
  [Source](https://github.com/encode/starlette/blob/master/pyproject.toml)
- Pydantic uses multiple type checkers because typing behavior is part of its
  product and still configures a more permissive McCabe threshold; that
  specialized burden is not a suitable DevMeld baseline.
  [Source](https://github.com/pydantic/pydantic/blob/main/pyproject.toml)
- attrs lets mypy own annotations and disables several mechanical complexity
  and size rules, illustrating that mature projects tune tools around their
  model rather than accepting every available metric.
  [Source](https://github.com/python-attrs/attrs/blob/main/pyproject.toml)

These tools have distinct responsibilities and run locally without a
persistent service. Sonar can be reconsidered when code history, CI governance,
or cross-repository quality reporting creates a concrete need.

## Decision 11: Defer Production Storage

**Decision**: No production storage adapter, physical schema, or derived-index
implementation belongs to this foundation. Only the rebuildability and
authority boundaries are accepted now. A later Feature evaluates storage using
its actual query, portability, and recovery requirements.

**Rationale**: A future store may be useful, but selecting its schema before
core invariants and real retrieval behavior exist would make persistence shape
the domain.

**Rejected alternatives**:

- create a SQLite or other database adapter in the foundation;
- define domain objects around tables;
- mandate vector or hosted search without measured need;
- leave derived-state authority unspecified.

## Decision 12: Validate the Foundation Without a Business Workflow

**Decision**: Acceptance uses pure values and only the minimum substitutes
required by implemented core rules. It verifies Project Catalog, Task Context
validation and Local Context Resolution, Context Knowledge, and dependency
direction. Managed Materialization and Capability Integration are reviewed as
design only.

Context Profile acceptance covers only identity, naming, Repository/Resource
selections, and Capability-independent portable rules. It does not require a
Capability Product decision or capability-selection fixtures, and cannot claim
the full profile behavior has been implemented.

**Rationale**: A non-user-visible foundation should be testable without
pretending to deliver a vertical product workflow or building supporting-domain
machinery as demonstration code.

**Rejected alternatives**:

- mark the foundation complete based only on diagrams;
- implement all five domains to make the design look symmetrical;
- include a thin end-to-end workflow that bypasses unfinished domain rules;
- create adapter-shaped fixtures unrelated to an implemented core invariant.

## Protected Decision Gates

Gate 1 and Gate 2 were accepted on 2026-09-02 and are recorded in
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md) and
[ADR-0002](../../docs/adr/0002-initial-python-runtime.md). The following
prerequisites are resolved for the reviewed foundation scope:

1. Gate 1: domain-oriented modular-monolith architecture;
2. Gate 2: Python 3.14.x and the three-tool baseline.

Tooling guidance is recorded in [the Engineering Guide](../../docs/engineering.md).

Gate 3 is a scoped blocker: the standalone Product meaning of `Capability` must
be approved and recorded in `docs/product.md` before any work relies on that
meaning, including Context Profile capability selection and a future Capability
Integration Feature. It does not block unrelated core task generation or
acceptance. This foundation excludes Capability-dependent implementation and
placeholders; approving Gate 3 alone does not expand its scope.

The rationale is dependency-based governance: architecture and runtime affect
the first code directly, whereas Capability-independent rules do not need a
future Capability definition. The decision remains protected without forcing
an unrelated dependency into the core foundation. The approval of Gate 1/Gate 2
does not accept Gate 3 or ratify the Constitution.

## Deferred Questions

Parser choice, CLI or desktop framework, physical storage schema, concrete Vault
layout, context operations, paging, ranking, error catalogs, public protocol
versioning, materialization operations, Codex rendering, benchmarks, and
supporting-domain ports are intentionally deferred to the Features that need
them. They are not unresolved requirements for this foundation.
