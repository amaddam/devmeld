# Feature Specification: Domain Foundation

**Feature Branch**: Not created (specification only)

**Created**: 2026-09-01

**Last Updated**: 2026-09-02

**Status**: Design reviewed — architecture/runtime accepted; implementation not started

**Decision Record**: The project decision-maker accepted the two architecture
and runtime proposals on 2026-09-02. Their owning ADRs and the still-deferred
Capability gate are recorded in [Plan](plan.md#maintainer-decision-gates).

**Input**: User description: "Plan DevMeld's complete domain map and constraints
first, but keep the first code foundation limited to the core domains. Do not
freeze future application protocols or implement supporting domains before a
real feature needs them."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Establish the Complete Domain Map (Priority: P1)

As a maintainer, I want DevMeld's core and supporting domain boundaries mapped
before implementation so that future features have explicit language,
ownership, lifecycle, and dependency constraints without requiring speculative
code.

**Why this priority**: The complete map exposes boundary conflicts early, while
separating design coverage from implementation coverage prevents a diagram from
turning automatically into five packages of unused abstractions.

**Independent Test**: Review the domain model against `docs/product.md` and
verify that every accepted foundational concept has one owner, every provisional
term is visibly marked, and each of the five domains declares its responsibilities,
forbidden ownership, and critical invariants.

**Acceptance Scenarios**:

1. **Given** the accepted Product terminology, **When** the complete domain map
   is reviewed, **Then** Project Catalog, Local Context Resolution, Context
   Knowledge, Managed Materialization, and Capability Integration each have a
   distinct responsibility and lifecycle boundary.
2. **Given** a rule that spans several concepts, **When** its ownership is
   reviewed, **Then** it belongs to one domain or an explicit semantic boundary
   rather than being duplicated in several workflows.
3. **Given** a proposed term that is not yet accepted in `docs/product.md`,
   **When** the map uses it, **Then** it is identified as provisional and cannot
   silently become a cross-feature Product concept.
4. **Given** a supporting domain with no approved implementation need, **When**
   the foundation scope is reviewed, **Then** its design remains documented but
   no package, port, persistence abstraction, or test double is required.

---

### User Story 2 - Implement Only the Core Domain Foundation (Priority: P2)

As a contributor, I want the first code foundation limited to Project Catalog,
Local Context Resolution, and Context Knowledge so that the most important
identity, resolution, and provenance rules become executable without building
future materialization or provider integrations speculatively.

**Why this priority**: These three core domains contain the rules that later
features must trust. The supporting domains can remain design boundaries until
a real write or provider feature supplies concrete requirements.

**Independent Test**: Exercise the three core domains using pure domain values
and only the minimum test substitutes required by those core rules, without a
real Git repository, Vault, database, Agent Client, generated file, or
supporting-domain implementation.

**Acceptance Scenarios**:

1. **Given** portable catalog values, **When** Workspace, Repository,
   Resource-registration, and the supported Context Profile subset are
   evaluated, **Then** identity, Repository/Resource selection, and portable
   policy invariants hold without local paths or infrastructure types; no
   capability-selection support is claimed.
2. **Given** a valid Task Context and Checkout observations, **When** Active
   Checkout resolution is evaluated, **Then** it returns exactly one of
   Resolved, Ambiguous, or Unavailable with its basis and candidates.
3. **Given** an invalid explicit Checkout selection, **When** Task Context is
   validated, **Then** validation fails before resolution and no fallback
   candidate is selected.
4. **Given** Resource, Relation, Evidence, and Scope facts, **When** query-time
   applicability is evaluated, **Then** Scope Match remains separate from source,
   review, and validity status.
5. **Given** a future-adapter substitute required by a core rule, **When** a
   second conforming substitute is used, **Then** the core domain outcome remains
   unchanged and no external representation enters the domain API.
6. **Given** an approved foundation scope and accepted architecture/runtime
   decisions, but an undecided Capability meaning, **When** implementation tasks
   are scoped, **Then** Capability-independent core work may proceed while
   capability-selection and other Capability-dependent behavior remain excluded.
7. **Given** a core rule with no coordination need outside domain objects,
   **When** the rule is implemented, **Then** no empty application layer or
   pass-through service is introduced merely to mirror the domain map.

---

### User Story 3 - Prove Core Safety and Dependency Rules (Priority: P3)

As a maintainer, I want the implemented core invariants and dependency direction
represented by executable checks so that later features cannot silently regress
portability, provenance, or Checkout ambiguity handling.

**Why this priority**: The foundation is useful only if later code is forced to
traverse its accepted rules instead of reproducing them in adapters or
application condition trees.

**Independent Test**: Run the core verification suite and, after real domain
packages exist, seed representative forbidden imports to prove the architecture
gate detects dependency reversal.

**Acceptance Scenarios**:

1. **Given** multiple eligible Checkouts with no decisive valid selection,
   **When** resolution is evaluated, **Then** the result is Ambiguous and no
   Checkout is guessed.
2. **Given** portable project data, **When** it is validated, **Then** it contains
   no machine-specific absolute path, developer/machine selection, or Active
   Checkout.
3. **Given** a context result, **When** its explanation is inspected, **Then**
   source identity, Evidence, Scope, Scope Match, review/validity facts, and
   resolution basis remain distinct and traceable.
4. **Given** a seeded dependency from core domain code to an adapter, entrypoint,
   or another domain's internals, **When** the architecture gate runs, **Then**
   the violation is detected.

### Edge Cases

- A Product term appears useful to several domains but has different authority
  or lifecycle in each one.
- A provisional implementation term is mistaken for accepted Product language.
- An undecided Capability meaning blocks unrelated core work, or an incomplete
  Context Profile is incorrectly presented as full capability-selection support.
- An external provider exposes an object resembling a DevMeld concept but with
  incompatible identity or validity rules.
- A contributor proposes a generic entity, repository, service, resource
  hierarchy, or metadata map that erases domain-specific language.
- Portable state accidentally includes a local path, developer identity,
  machine identity, or task-scoped Active Checkout.
- An explicit Checkout selection names the wrong Repository, repeats with
  conflicting candidates, or names an unknown observation.
- A valid selected Checkout later becomes unavailable or its observation is
  stale.
- Source Type, Review Status, Validity Status, and Scope Match disagree; none may
  overwrite or imply another.
- A future Managed Surface is human-owned, claimed by another provider, or
  changed after preview; the design must preserve an explicit conflict even
  though this feature does not implement that domain.
- A future requirement does not fit an existing domain without weakening its
  invariants.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The foundation MUST define a ubiquitous language consistent with
  `docs/product.md` and MUST label implementation-facing or proposed Product
  terms explicitly.
- **FR-002**: Every accepted foundational concept and invariant MUST have exactly
  one owning domain or one explicitly named semantic boundary.
- **FR-003**: The complete design MUST distinguish the core Project Catalog,
  Local Context Resolution, and Context Knowledge domains from the supporting
  Managed Materialization and Capability Integration domains.
- **FR-004**: Each of the five domain designs MUST declare what it owns, what it
  may reference, what it must not own, and at least three critical invariants.
- **FR-005**: The first code foundation MUST implement only the three core
  domains. Supporting-domain packages, ports, contracts, and test doubles MUST
  remain absent until an approved Feature needs them. Context Profile
  implementation MUST be limited to identity, naming, Repository/Resource
  selections, and portable rules independent of Capability semantics;
  capability-selection behavior and placeholders MUST remain absent.
- **FR-006**: Domain behavior MUST use domain-specific concepts and invariants
  rather than generic entity hierarchies, generic persistence operations, or
  workflow-specific condition trees.
- **FR-007**: Cross-domain collaboration MUST use stable identities, immutable
  facts, or explicit semantic boundaries; one domain MUST NOT mutate another
  domain's internal state directly.
- **FR-008**: External representations and failures MUST be translated before
  reaching core domain decisions and MUST NOT become authoritative Product
  concepts.
- **FR-009**: The foundation design MUST classify project state as portable,
  local, derived, generated, or query-derived, with a distinct authority and
  lifecycle for each class.
- **FR-010**: Portable project state MUST NOT contain machine-absolute paths,
  current-machine selections, local credentials, or an Active Checkout.
- **FR-011**: Derived state MUST remain disposable and rebuildable from declared
  sources; generated-state design MUST identify inputs, ownership, and a bounded
  replacement or reversal model without implementing materialization now.
- **FR-012**: Task Context validation MUST reject an explicit Checkout selection
  that names the wrong Repository, conflicts with another explicit selection,
  or names an ineligible/unknown observation before resolution begins.
- **FR-013**: For valid Task Context only, Active Checkout resolution MUST return
  Resolved, Ambiguous, or Unavailable with a basis and considered candidates;
  ambiguity MUST NOT become a guessed success.
- **FR-014**: Evidence, Scope, Source Type, Review Status, Validity Status, and
  query-time Scope Match MUST remain independent concepts where they apply.
- **FR-015**: The foundation MUST fix domain semantics and invariants without
  fixing public operation names, request/response envelopes, pagination,
  ranking protocols, public error catalogs, or transport compatibility rules.
- **FR-016**: The implemented core foundation MUST include executable examples
  for its critical invariants and, once packages exist, automated checks for the
  accepted dependency direction.
- **FR-017**: Only an external fact genuinely required by an implemented core
  rule may introduce a domain-owned port and test substitute. No port may be
  created solely to prepare a future adapter or supporting domain.
- **FR-018**: The design MUST document an extension test: extend an existing
  domain when its language and lifecycle still fit; introduce a new or revised
  boundary when ownership is genuinely distinct; never weaken a model solely to
  avoid a boundary change.
- **FR-019**: Completion MUST NOT be presented as delivery of real-source
  querying, materialization, provider integration, capability selection,
  benchmarking, a public machine protocol, a desktop application, or another
  user-visible workflow.
- **FR-020**: Architecture and runtime decisions MUST be accepted before
  foundation implementation tasks are generated. The Capability Product
  decision MUST block only work that depends on that meaning, regardless of
  owning domain; it MUST NOT block Capability-independent core tasks within the
  approved scope.
- **FR-021**: Application coordination MUST be introduced only when an
  implemented scenario needs actual cross-object/domain coordination outside
  domain objects. Empty layers or pass-through services MUST NOT be created to
  mirror an architecture diagram; domain invariants MUST remain domain-owned.

### Scope Boundaries

**Design scope** includes the complete five-domain map, ubiquitous language,
ownership, invariants, state lifecycles, dependency direction, negative model
constraints, and non-binding semantic sketches for future context and managed
write behavior.

**Implementation scope** includes only the minimum pure domain foundation for
Project Catalog, Local Context Resolution, and Context Knowledge; application
coordination only where genuinely needed; core-required ports and test
substitutes; invariant tests; and dependency checks activated after real packages
exist.

Context Profile is an explicit implementation subset: identity, naming,
Repository selections, Resource selections, and portable rules that do not
depend on Capability semantics. Capability selections remain design only; no
placeholder field, default-empty representation, or no-op behavior may imply
support. This delivery boundary does not narrow the full Product definition.

It excludes production Git, Vault, filesystem, database, Codex, CLI, Desktop,
and hosted-service adapters; Managed Materialization and Capability Integration
code; capability-selection implementation; public machine contracts; complete
search/read/relation workflows; generated Agent artifacts; and benchmark claims.

### Key Concepts

- **Domain Module**: A cohesive ownership boundary with its own language,
  invariants, lifecycle, and allowed dependencies.
- **Project Catalog**: The core domain owning stable project identity, Workspace
  composition, logical Repositories, Context Profiles, and Resource identity.
- **Local Context Resolution**: The core domain owning Local Bindings, Checkout
  observations, Task Context validation, and task-scoped Active Checkout
  resolution.
- **Context Knowledge**: The core domain owning Relations, Evidence, Scope,
  provenance dimensions, and query-time context semantics.
- **Managed Materialization**: A design-only supporting domain boundary for
  preview, ownership, preconditions, manifest, conflict, verification, and
  reversal semantics.
- **Capability Integration**: A design-only supporting domain boundary for
  Capability Provider and Agent Client registration/compatibility. Any separate
  `Capability` concept remains provisional until accepted in `docs/product.md`.
- **Task Context Validation**: Validation that distinguishes malformed or
  contradictory explicit selections from valid inputs that can be resolved.
- **Active Checkout Resolution**: Resolved, Ambiguous, or Unavailable outcome
  produced only from a valid Task Context.
- **Invariant**: A rule that must hold for a domain state or transition,
  independent of an adapter or workflow.
- **State Lifecycle Class**: Portable, local, derived, generated, or
  query-derived authority and recovery behavior.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A domain review assigns 100% of accepted foundational concepts to
  one owner and lists every provisional term as a decision gate.
- **SC-002**: Each of the five domain designs documents owned language,
  forbidden ownership, dependencies, and at least three critical invariants.
- **SC-003**: Implementation artifacts exist only for the three core domains;
  no supporting-domain package, port, persistence abstraction, test double, or
  public protocol is created. Context Profile acceptance covers only its
  declared subset and finds no capability-selection behavior or placeholders.
  Every application coordination component has a demonstrated need, with no
  empty layer or pass-through service created solely for structural symmetry.
- **SC-004**: The core verification suite covers valid Resolved, Ambiguous, and
  Unavailable outcomes plus invalid Task Context, and never falls back after an
  invalid explicit selection.
- **SC-005**: All portable-state fixtures exclude machine-specific paths,
  current developer/machine selections, and Active Checkout state.
- **SC-006**: All representative context fixtures preserve separate source,
  Evidence, Scope, Scope Match, Review Status, and Validity Status facts.
- **SC-007**: Automated dependency checks detect every seeded core-to-adapter,
  core-to-entrypoint, and cross-domain-internal violation after real packages
  exist.
- **SC-008**: Every core-required external fact can be supplied by a second test
  substitute without changing the owning core domain's rules; no unused future
  port is required to meet this outcome.
- **SC-009**: Maintainer review finds no production adapter, user interface,
  supporting-domain implementation, or business workflow acting as the source
  of a core invariant.
- **SC-010**: The implemented core foundation can be inspected and verified
  without a real Git Checkout, Vault, database, Agent Client, generated target,
  hosted service, or desktop application.

## Assumptions

- The accepted `docs/product.md` vocabulary owns cross-feature Product semantics;
  this initiative may propose but cannot silently add a Product concept.
- The full domain map is a design boundary, not an instruction to create one
  package, aggregate, repository, port, or service for every named area.
- Domain-driven design here means explicit language, ownership, invariants, and
  dependency direction. It does not require event sourcing, a global event bus,
  an aggregate for every noun, or a repository for every object.
- Application architecture and implementation language/runtime remain protected
  Maintainer decisions required before foundation implementation tasks are
  generated; accepted lasting decisions must be recorded in ADRs.
- The formal meaning of `Capability` remains a protected Product decision, but
  blocks only tasks that depend on it. Accepting that meaning does not
  automatically add capability selection or Capability Integration to this
  foundation; such work also needs an approved Feature scope.
- Supporting-domain code and user-visible features will be planned separately
  when real requirements provide acceptance evidence.
