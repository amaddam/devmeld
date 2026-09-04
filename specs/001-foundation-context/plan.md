# Implementation Plan: Domain Foundation

**Branch**: `001-foundation-context` (feature identifier; no Git branch created)  
**Date**: 2026-09-01  
**Last Updated**: 2026-09-04
**Status**: Proposed tool/dependency-guidance Product change prepared for
Maintainer review; architecture/runtime accepted; implementation not started
**Spec**: [Domain Foundation specification](spec.md)

**Input**: Revised Feature Specification from
`specs/001-foundation-context/spec.md`

## Summary

Design the complete DevMeld domain map while implementing only the three core
domains: Project Catalog, Local Context Resolution, and Context Knowledge.
Managed Materialization and Capability Integration remain documented supporting
boundaries with responsibilities, invariants, and negative constraints, but no
code, ports, test doubles, or stable machine protocols are created for them in
this feature.

Capability Integration design now includes a proposed evidence-backed
tool/dependency-guidance model: explicit applicable selections are preserved;
otherwise existing eligible project-managed options are preferred. Selection
authority is separate from change authority, so any dependency, installation,
environment, or system change remains a separately authorized proposal. Project
declarations remain authoritative, and Agent Clients retain execution
responsibility.

Context Profile code covers only identity, naming, Repository/Resource
selections, and Capability-independent portable rules. Capability selections
remain design only, without placeholder fields or behavior.

The foundation fixes domain semantics rather than future application APIs.
Context and materialization documents are non-binding semantic sketches: they
do not select operation names, JSON envelopes, pagination, public error codes,
ranking formats, approval fields, or compatibility versions. Application
protocols belong to the first Feature that needs them.

## Technical Context

**Language/Version**: Python 3.14.x, accepted in
[ADR-0002](../../docs/adr/0002-initial-python-runtime.md).

**Primary Dependencies**: Standard library only in the production core. The
initial quality baseline is Ruff, mypy, and pytest, with settings owned by the
[Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates).
No tool configuration or dependency environment has been implemented yet.

**Storage**: No production persistence adapter is in scope. Pure values and only
the minimum in-memory substitutes required by an implemented core rule may be
used. The design still separates portable, local, derived, generated, and
query-derived lifecycles so future persistence cannot collapse their authority.

**Testing**: Pytest invariant/behavior tests, conservative Ruff format/lint
checks, and mypy strict for core domain code and any genuinely needed
application/port code. Import Linter is introduced only after real packages
exist and only for accepted dependency contracts.

**Target Platform**: Local Windows, macOS, and Linux development environments;
no required hosted service. Platform verification remains implementation work.

**Project Type**: Domain-oriented modular monolith, accepted in
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md). The first code
contains a minimal shared kernel and three core domain packages only.

**Performance Goals**: No production throughput or latency target. Core behavior
tests are deterministic and require no external process, repository, database,
network, Vault, Agent Client, or generated target. Architecture tests may invoke
local verification tools over isolated temporary fixtures, not production
adapters or user data.

**Constraints**:

- preserve the accepted architecture/runtime decisions and the bounded
  Capability-independent implementation scope;
- domain code does not import adapters, entrypoints, filesystem, Git, database,
  Codex, UI, or another domain's internals;
- portable objects do not contain machine-absolute paths, current-machine
  selection, developer identity, credentials, or Active Checkout;
- invalid Task Context is rejected before Active Checkout resolution;
- valid resolution produces only Resolved, Ambiguous, or Unavailable and never
  guesses through ambiguity;
- application coordination does not own domain invariants;
- create `application/` only for real coordination outside domain objects;
  neither an empty package nor a pass-through service is required;
- no generic entity hierarchy, CRUD service, Generic Repository, catch-all
  metadata map, or shared utilities package replaces domain language;
- no public application protocol, supporting-domain port, or speculative test
  double is created in this feature;
- tool/dependency guidance is design only: no environment discovery, dependency
  installation, generic command runner, enforcement integration, or duplicate
  dependency registry is implemented;
- no initial hard gate on complexity, branch count, argument count, return
  count, statement count, function length, or file length.

**Scale/Scope**: Five designed domains; three implemented core domains; one
minimal shared kernel; core-required ports only; pure invariant fixtures; no
supporting-domain or production-adapter package

## Domain Strategy and Delivery Scope

| Domain | Classification | Design ownership | This feature's code |
| --- | --- | --- | --- |
| Project Catalog | Core | Workspace composition, Vault/source references, logical Repository identity, Context Profile selections, Resource identity | Minimal foundation and core-required ports/tests; Context Profile excludes capability selections |
| Local Context Resolution | Core | Local Bindings, Checkout observations, Task Context validation, Active Checkout resolution | Domain rules/tests; coordination and ports only when actually needed |
| Context Knowledge | Core | Relations, Evidence, Scope, independent provenance dimensions, Scope Match and context-result semantics | Domain rules/tests; coordination and ports only when actually needed |
| Managed Materialization | Supporting | Preview, ownership, preconditions, manifest, conflict, verification and reversal semantics | Design only; no package, port, fixture or protocol |
| Capability Integration | Supporting | Capability Provider and Agent Client registration/compatibility; evidence-backed tool/dependency guidance; provisional capability-declaration semantics | Design only; no package, discovery adapter, installer, port, fixture or protocol |

The Verification Harness is test support, not a production domain or source of
Product truth.

The accepted modules form one application/deployment boundary. Their
boundaries protect language and dependency direction; they do not imply
microservices, remote calls, event sourcing, distributed transactions, or one
database per domain.

## Maintainer Decision Gates

Gate 1 and Gate 2 are prerequisites for foundation implementation tasks: both
were accepted on 2026-09-02 and are recorded in
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md) and
[ADR-0002](../../docs/adr/0002-initial-python-runtime.md). Within the approved
foundation scope, Gate 3 blocks only work that depends on formal Capability
Product semantics, not unrelated core tasks.

### Gate 1: Initial Application Architecture

**Status**: Accepted on 2026-09-02.

**Decision**: domain-oriented modular monolith with package-by-domain ownership,
inward adapter dependencies, and demand-created application/port packages.

**Record**: [ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md).

### Gate 2: Initial Runtime

**Status**: Accepted on 2026-09-02.

**Decision**: Python 3.14.x, standard-library-first core code, and a progressive
Ruff/mypy/pytest baseline without initial complexity or logical-simplification
gates.

**Record**: [ADR-0002](../../docs/adr/0002-initial-python-runtime.md) and
[Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates).

### Gate 3: Capability Product Meaning

**Status**: Deferred; not approved by the Gate 1/Gate 2 decision.

**Scope**: Capability-dependent work in any domain, including Context Profile
capability selection and a future Capability Integration Feature. It does not
block task generation or acceptance for this Capability-independent foundation.

`docs/product.md` currently defines Capability Provider and uses lowercase
`capabilities` in Context Profile, but does not define a standalone Capability
concept.

The proposed tool/dependency-guidance behavior is documented in
`tool-use-semantics.md` without making each tool a Capability Provider. It is
pending Maintainer Product review and does not resolve this Gate: the standalone
`Capability` meaning and capability selection remain deferred.

**Proposal**: define Capability as a named context function selectable by a
Context Profile and supplied by one or more Capability Providers, independent of
provider implementation.

**On acceptance**: update `docs/product.md`. Until then, `Capability Declaration`
in the design remains a provisional implementation-facing term. Acceptance does
not automatically expand this Feature: capability selection and Capability
Integration still require an approved scope before implementation. No
placeholder Capability IDs, fields, default-empty lists, no-op behavior, ports,
or fixtures may be introduced to bypass the unresolved meaning.

## Constitution Check

*GATE: Passed for the bounded foundation. Gate 1 and Gate 2 are accepted and
recorded. Gate 3 remains a scoped blocker for Capability-dependent work, which
is excluded from this implementation. This is not implementation acceptance or
Constitution ratification.*

| Principle | Plan evidence | Result |
| --- | --- | --- |
| Context, Not Workflow or Execution | Core semantics and tool guidance describe context and decisions only; Agent Clients retain invocation, installation and enforcement | PASS |
| Grounded and Explainable Context | Evidence, Scope, source identity, resolution basis, ambiguity, tool availability and decision basis are explicit | PASS |
| Local-First, Portable, and Rebuildable | State lifecycle classes retain distinct authorities; local tool observations are scoped and rebuildable; no hosted service or production store is required | PASS |
| Explicit and Reversible Writes | Managed-write invariants remain visible; a dependency/environment change is only a proposal until separately authorized | PASS |
| Human-Inspectable by Default | Domain models, semantic sketches, validation results, tool decisions and decision gates are readable documents | PASS |
| Deliver Value in Vertical Slices | Supporting-domain code and public protocols are deferred; the non-user-visible core foundation does not claim product value | PASS |

### Post-Design Recheck

- Full design coverage is not treated as full implementation scope.
- Active Checkout receives only a valid Task Context and remains a resolution
  result rather than portable state.
- `Knowledge` remains a Product category, not a base entity.
- `Capability Declaration` is visibly provisional pending Product approval.
- Context Profile implementation and acceptance explicitly exclude capability
  selection; this does not narrow its full Product meaning.
- No operation names, transport envelopes, error catalogs, or protocol versions
  are accepted by this foundation.
- Ports and test doubles require an implemented core rule; future adapters do not
  justify them.
- Application packages require real coordination outside domain objects, not
  merely a matching box in the architecture diagram.
- Proposed tool guidance preserves user/project selections, authoritative
  dependency sources, scoped observations, and separate selection/change
  authority without claiming that DevMeld executes or enforces the choice.
- Tool/dependency guidance does not make every executable, library or script a
  Capability Provider and does not resolve the standalone Capability meaning.
- No Constitution exception or complexity waiver is required.

## Project Structure

### Documentation (this feature)

```text
specs/001-foundation-context/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── context-semantics.md
├── materialization-semantics.md
├── tool-use-semantics.md
├── quickstart.md
├── tasks.md
└── checklists/
    └── requirements.md
```

There is no `contracts/` directory because this feature accepts no public
machine contract. The two accepted decisions are now recorded in `docs/adr/`.

[Tasks](tasks.md) decomposes the reviewed scope after Gate 1/Gate 2 approval and
ADR/Engineering updates. Gate 3 remains open; no Capability-dependent or
tool-guidance implementation and no placeholder supporting-domain code belongs
in these tasks.

### Source Code (planned, not yet implemented)

```text
pyproject.toml

src/devmeld/
├── shared_kernel/
│   └── stable identity and schema-version primitives only
├── catalog/
│   ├── domain/
│   ├── application/    # only for real coordination outside domain objects
│   └── ports/          # only when an implemented core rule requires one
├── local_context/
│   ├── domain/
│   ├── application/    # only for real coordination outside domain objects
│   └── ports/          # only when an implemented core rule requires one
└── knowledge/
    ├── domain/
    ├── application/    # only for real coordination outside domain objects
    └── ports/          # only when an implemented core rule requires one

tests/
├── unit/
├── contract/           # only if a core-owned port actually exists
├── architecture/       # activated after real packages exist
└── fixtures/           # only for fixtures actually used by tests
```

Do not create `materialization/`, `capabilities/`, `adapters/`, or
`entrypoints/` as empty architecture. The first approved Feature requiring one
of those boundaries creates the necessary package.

**Structure Decision**: package by domain. Each owner has domain rules;
application coordination and ports are conditional, not mandatory folders.
Do not create an empty `application/` or a pass-through service to mirror this
tree. Avoid global `models/`, `services/`, and `repositories/` directories
because they hide ownership.

The first task list starts with pure domain objects/policies and explicitly
supplied immutable facts; it identifies no mandatory application package or
external-fact port yet. If implementation demonstrates a genuine need, record
the named core rule and add exact-path tasks before creating the component.
Passing values between tests is not evidence that a production port is needed.

## Dependency and Modeling Rules

1. The shared kernel admits only values with identical semantics in every
   consuming core domain; convenience is insufficient.
2. A domain package depends only on its own domain code and the minimal shared
   kernel. Cross-domain references use stable IDs or immutable public facts.
3. Application code coordinates its own domain and genuinely required ports; it
   cannot update another domain's state or reproduce its invariants. Introduce
   it only for actual cross-object/domain coordination outside domain objects.
   Multiple input values alone do not justify moving a domain policy outward.
4. A port is named for an implemented core need. There is no generic persistence,
   provider, client, or CRUD port and no port created for a future adapter.
5. Future adapters translate external identities, states, and failures through
   anti-corruption mappings. External objects never enter core APIs.
6. Persistence records, transport DTOs, and semantic sketches are not domain
   objects or accepted public protocols.
7. Domain invariants live in aggregates, value objects, domain services, or
   policies named in the owning language.
8. Domain events require a demonstrated producer, consumer, and delivery need;
   this foundation adds no event bus or event sourcing.
9. A requirement that does not fit triggers a boundary review rather than an
   `extra`, `metadata`, type switch, or cross-domain mutation.
10. Proposed tool/dependency guidance references project-owned declarations and
    scoped observations. It does not own manifests or lockfiles, treat global
    presence as project eligibility, derive change authority from selection
    authority, or execute the selected option.

## First Code Foundation

With Gate 1 and Gate 2 accepted and recorded, this foundation may provide only:

1. stable identity and schema-version primitives admitted to the shared kernel;
2. Project Catalog values and invariants for portable Workspace, Repository,
   Resource registration, and the declared Context Profile subset, without
   capability-selection fields or behavior;
3. Task Context validation, immutable Checkout observations, and
   Resolved/Ambiguous/Unavailable resolution policies;
4. Context Knowledge values and policies for Relation, Evidence, Scope,
   provenance dimensions, Scope Match, and explainable context-result facts;
5. a port and substitute only where one of the above implemented rules genuinely
   needs an external fact;
6. pure invariant tests and the accepted Ruff/mypy/pytest configuration;
7. the minimum Import Linter contracts after packages exist, including seeded
   violations proving the gate works.

It will not implement Managed Materialization, Capability Integration,
capability selection, tool/environment discovery, dependency installation or
enforcement, real storage/source/client adapters, public query operations,
materialization operations, a production CLI, or benchmark claims.

## Delivery Sequence

1. Confirm the accepted Gate 1/Gate 2 ADRs, Engineering guidance, and the
   design-only tool/dependency-guidance boundary. Keep the task scope
   Capability-independent; Gate 3 remains deferred.
2. Create the minimal project/tooling baseline and shared-kernel primitives.
3. Implement Project Catalog language and invariants with pure tests, limiting
   Context Profile to its declared Capability-independent subset.
4. Implement Task Context validation and Local Context Resolution outcomes with
   pure observations and explicit ambiguity.
5. Implement Context Knowledge provenance and Scope Match with pure facts.
6. Add only the ports, substitutes, and contract tests demonstrated necessary by
   steps 3–5. Add application coordination only if a real need remains outside
   domain objects; otherwise leave those packages absent.
7. Once the three core packages exist, add the minimum Import Linter contracts
   and prove them with seeded dependency violations.
8. Run the Maintainer acceptance guide without creating supporting-domain or
   public-protocol artifacts.

Every step preserves the accepted dependency direction. Temporary workflow code
or speculative ports are not allowed to bypass the domain simply because a
future Feature may need them.

## Design Artifacts

- [Research decisions and proposals](research.md)
- [Conceptual domain model](data-model.md)
- [Context semantic sketch](context-semantics.md)
- [Materialization semantic sketch](materialization-semantics.md)
- [Tool and dependency guidance semantic sketch](tool-use-semantics.md)
- [Foundation acceptance guide](quickstart.md)

## Complexity Tracking

No Constitution violation is requested. The revised scope intentionally trades
early breadth for three implemented core boundaries while keeping supporting
domain knowledge inspectable and revisable.
