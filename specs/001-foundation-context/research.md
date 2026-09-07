# Research: Domain Foundation

**Feature**: [Domain Foundation](spec.md)  
**Date**: 2026-09-01  
**Last Updated**: 2026-09-07
**Status**: Rust runtime accepted on 2026-09-07; post-reset verification is recorded in [Acceptance Guide](quickstart.md);
tool/dependency-guidance Product behavior accepted; Capability Product meaning deferred

This document records foundation reasoning. Accepted architecture/runtime
decisions are owned by ADR-0001 and ADR-0003 (superseding ADR-0002); quality rules are owned by
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

## Decision 9: Use Stable Rust for the Restarted Foundation

**Status**: Accepted on 2026-09-07 in
[ADR-0003](../../docs/adr/0003-rust-runtime.md), superseding the Python baseline
in ADR-0002. This is a Maintainer decision, not a measured language ranking.

**Decision**: Stable Rust, Edition 2024, standard-library-first core and native
Windows development first. Keep Windows/macOS/Linux targets; version pins,
supported rust-version and actual platform evidence belong to setup artifacts.

**Rationale**: Explicit identity/state ownership and native local-tool delivery
fit DevMeld's context-engine direction. The project accepts Rust's learning,
compilation and integration costs. Types protect parts of a correct design;
they do not substitute for domain design, tests, architecture checks or review.

**Alternatives**: TypeScript remains technically viable but is not selected as
a parallel core. Python's previous implementation is real historical work, not
a zero-code starting point; it is removed after recoverable backup rather than
maintained as a second baseline. No additional comparative experiment is a gate.

**Consequence**: No Rust source is written during this documentation reset.
Prior Python passes and the limited WSL sample do not prove the new implementation.
Do not narrow accepted invariants to match experimental code. Spec Kit's Python
scripts are workflow tools, not DevMeld runtime code, and remain untouched.

## Decision 10: Use Cargo's Tools and Focused Boundary Checks

**Authority**: [Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates).

**Decision**: Compiler checks, default rustfmt, conservative Clippy and Cargo
tests. Correctness is a gate; suspicious/perf remain warnings; style/complexity/
pedantic/nursery/restriction are not imposed as initial logic/size constraints.
No global warnings-as-errors, blanket clone ban or numeric complexity target.

After the three core crates exist, use the Rust tools/xtask Cargo-metadata
check and isolated negative probes. The approved 2026-09-07 portability correction
replaces the original PowerShell checks; serde_json is a developer-tool-only
dependency, not a core library dependency or a second runtime. The gate checks all declarations, including
optional/target-specific/build/renamed edges, not only active dependencies.
Use valid controls and explicit expected errors; do not equate any failing
command with a successful negative test. Metadata checks cannot prove std IO
purity; domain review and behavior tests remain necessary.

**Evidence**: Cargo supports shared workspace settings and an explicit resolver
for virtual workspaces. Members opt into lint inheritance. Cargo metadata exposes
dependency kinds and declarations even when --no-deps omits the resolved graph.
Cargo's default test selection includes doctests; --all-targets must not silently
replace that coverage. Windows MSVC toolchain/linker setup is independent of WSL.

- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)
- [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Clippy usage](https://doc.rust-lang.org/clippy/usage.html)
- [Rustup Windows](https://rust-lang.github.io/rustup/installation/windows.html)

**Rejected defaults**: Import Linter for Rust, Sonar, another formatter or type
checker, third-party test/architecture frameworks, async runtime and hook stack.
These would need a distinct demonstrated requirement.

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

## Decision 13: Model Tool and Dependency Guidance Without Implementing It

**Status**: Cross-feature Product behavior accepted by the Maintainer on
2026-09-07 and owned by
[`docs/product.md`](../../docs/product.md#tool-and-dependency-guidance).
Supporting-domain detail remains design only; public protocols are not fixed.

**Decision**: Extend the Capability Integration design with evidence-backed
tool/dependency guidance. Preserve explicit applicable selections; otherwise
prefer an eligible existing option requiring no new dependency, environment,
or installation change. Among those options, project-managed options have the
strongest reuse preference; a local/system tool verified callable, compatible,
and authorized in the current project/task scope is also eligible. This avoids
adding a dependency merely because a suitable existing tool is system-managed.
Keep project declarations, local discovery, verified availability,
compatibility, selection authority, and change authority as separate facts. A
request to use an option controls selection but does not authorize installing
it or changing a dependency, environment, or system. Treat any such change as a
separate proposal rather than an existing option.

Project manifests, lockfiles, maintained scripts, and configuration remain the
sources of truth for project declarations. Local observations retain scope,
source and freshness. DevMeld supplies the decision and evidence as context;
Agent Clients remain responsible for invocation, installation, and enforcement.

**Rationale**: The same access need may be satisfied by a command, an existing
declared dependency, a maintained script, a runtime-provided solution, or a
future provider. Choosing without project and environment evidence encourages
duplicate dependencies and parallel environments. Treating guidance as context
preserves DevMeld's boundary while still making the Agent's choice explainable.

**Alternatives rejected**:

- maintain a DevMeld-owned duplicate of every project's dependency inventory;
- treat every executable, library, or script as a Capability Provider;
- apply one universal command/library preference order across all projects;
- treat machine-wide discovery as proof that an option is usable in the current
  project or Checkout;
- treat an explicit tool selection as authority for any installation,
  dependency, environment, or system change, even a narrowly scoped one;
- claim prompt guidance alone can enforce an Agent Client's execution behavior;
- implement discovery adapters, installers, ports, or placeholder supporting
  packages in the three-domain foundation.

The non-binding design is recorded in
[`tool-use-semantics.md`](tool-use-semantics.md). This decision does not resolve
the standalone Product meaning of `Capability`, choose supported package
ecosystems, or expand the implementation scope.

## Decision 14: Use Compilation Boundaries for the Three Core Owners

**Decision**: A virtual workspace with shared-kernel, catalog, local-context
and knowledge library members; explicit member lists and resolver 3. Normal
dependencies go only from a core to the admitted identity kernel. No root facade,
product executable or additional domain layer is necessary. The developer-only
xtask member is verification tooling, not a fifth domain. See the [Plan](plan.md).

**Rationale**: These are actual ownership boundaries requiring isolation,
not speculative domains. Crate visibility and declared dependencies supply
compiler support; a small metadata gate rejects adding forbidden dependencies.
One deployment boundary can contain several library crates.

**Test-only exception**: Knowledge's core_fact_boundaries integration test may
use Catalog and Local Context as explicit dev-dependencies. No normal/build edge
or examples/benchmarks are authorized. This tests real public fact translation
without creating a production coordinator or extra harness library.

**Rejected alternatives**: A single crate protected only by naming conventions;
one crate per technical layer; a universal facade for tests; unrestricted dev
edges; regex-based TOML/import analysis presented as a complete architecture gate.

**Limits**: Cargo cannot determine whether a dependency is semantically justified
or whether a std call performs forbidden IO. Review remains necessary. Negative
probes must use copies of real configuration and cover disguised/conditional
declarations; architecture evidence is future work, not a planning result.

## Protected Decision Gates

Gate 1 remains accepted on 2026-09-02 in
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md). Gate 2 is now
owned by [ADR-0003](../../docs/adr/0003-rust-runtime.md), accepted on 2026-09-07
and superseding ADR-0002. The general prerequisites are:

1. Gate 1: domain-oriented modular monolith;
2. Gate 2: stable Rust and the progressive Cargo/rustfmt/Clippy/test baseline.

Tooling guidance is recorded in [the Engineering Guide](../../docs/engineering.md).

Gate 3 is a scoped blocker: the standalone Product meaning of `Capability` must
be approved and recorded in `docs/product.md` before any work relies on that
meaning, including Context Profile capability selection and a future Capability
Integration Feature. It does not block unrelated core task generation or
acceptance. This foundation excludes Capability-dependent implementation and
placeholders; approving Gate 3 alone does not expand its scope.

The accepted tool/dependency-guidance rules do not resolve Gate 3. They describe
how evidence and separate selection/change authority constrain a future tool
choice without requiring a universal `Capability` entity or capability-selection
implementation.

The rationale is dependency-based governance: architecture and runtime affect
the first code directly, whereas Capability-independent rules do not need a
future Capability definition. The decision remains protected without forcing
an unrelated dependency into the core foundation. The approval of Gate 1/Gate 2
does not accept Gate 3 or ratify the Constitution.

## Deferred Questions

Parser choice, CLI or desktop framework, physical storage schema, concrete Vault
layout, context operations, paging, ranking, error catalogs, public protocol
versioning, materialization operations, Codex rendering, benchmarks, supported
dependency ecosystems, safe observation adapters, execution-client enforcement,
tool-change authorization contracts, and supporting-domain ports are
intentionally deferred to the Features that need them. They are not unresolved
requirements for this foundation.
