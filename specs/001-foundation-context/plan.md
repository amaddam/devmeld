# Implementation Plan: Domain Foundation — Rust

**Branch**: main (existing checkout; feature identifier: 001-foundation-context)

**Created**: 2026-09-01

**Last Updated**: 2026-09-07

**Status**: Committed Rust foundation has recorded Windows/Linux verification;
review correction commit verified on Windows/Linux; subsequent type refinements
verified on native Windows only;
Maintainer acceptance pending (see quickstart.md)
**Spec**: [Domain Foundation](spec.md)

## Summary

Preserve the five-domain conceptual design and implement only Project Catalog,
Local Context Resolution and Context Knowledge. The Maintainer accepted the
Rust restart in [ADR-0003](../../docs/adr/0003-rust-runtime.md), superseding the
Python runtime. [ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md)
still owns the modular-monolith architecture.

The preceding reset removed the Windows Python implementation after recoverable
backup and updated documents only. The subsequent Maintainer start request now
authorizes the implementation tasks below. Existing Python completion evidence and the partial WSL Rust sample
are not acceptance evidence for the planned implementation. No further language
comparison is required before implementation, and no dual core is maintained.

Context Profile implementation remains the identity/naming/Repository/Resource
subset. Managed Materialization, Capability Integration, capability selection
and Tool Guidance implementation remain excluded. Accepted Tool Guidance Product
behavior stays in [Product](../../docs/product.md#tool-and-dependency-guidance);
the [semantic detail](tool-use-semantics.md) preserves scoped reuse, separate
selection/change authority and Agent Client execution ownership.

## Technical Context

**Language/Version**: Stable Rust, Edition 2024; exact supported toolchain and
Cargo rust-version recorded at T001. No nightly dependency or permanent patch
pin in ADR prose. Native Windows development first; not Windows-only support.

**Primary Dependencies**: Standard library and internal workspace crates only
for the four foundation libraries and their domain tests. Cargo compiler/tests,
rustfmt and conservative Clippy. The approved developer-only tools/xtask package
uses serde_json to parse Cargo metadata and diagnostics; its locked transitive
dependencies do not enter core manifests or product binaries. This replaces the
PowerShell verification prerequisite without hand-writing a JSON/TOML parser.

**Storage**: None. Portable, local, derived, generated and query-derived state
remain distinct conceptual lifecycles; no database, physical schema or index.

**Testing**: Per-domain pure behavior tests, compile-fail visibility/type probes,
and Cargo-metadata architecture checks after real crates exist. Follow the
[Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates).

**Target Platform**: Native Windows first; Windows/macOS/Linux remain targets.
WSL is optional, never the implicit toolchain for native Windows acceptance.
Record each tested host; path-string examples do not prove execution on that OS.

**Project Type**: Virtual Cargo workspace with four internal library packages
and one developer-only xtask binary; no product executable in Feature 001.
One root Cargo.lock and target directory; no separate tool workspace/environment.

**Performance Goals**: No latency, throughput, ranking or language-speed claim.
Pure domain tests require no real Git/Vault/database/client/network. Architecture
tests may run Cargo/rustc on isolated test files, not application integrations.

**Constraints**: Preserve Product and Constitution; no IO in domain decisions;
no hidden clock; no Capability placeholders, speculative layers, generic entity/
repository/service framework, UI, provider SDK, installer, storage or public API.
No complexity/size quotas. Copying the partial WSL experiment is not completion.

**Scale/Scope**: Five designed domains, three core libraries, two shared identity
types; application/ports only if a named implemented core rule demonstrates need.

## Maintainer Decision Gates

### Gate 1: Initial Application Architecture

Accepted on 2026-09-02 in
[ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md):
domain-oriented modular monolith, inward dependencies, demand-created layers.
Rust library boundaries do not create separate services or deployables.

### Gate 2: Runtime and Quality Baseline

Rust accepted on 2026-09-07 in
[ADR-0003](../../docs/adr/0003-rust-runtime.md). The original
[ADR-0002](../../docs/adr/0002-initial-python-runtime.md) is superseded, not deleted.
Engineering owns operational checks. Runtime approval is not code acceptance.

### Gate 3: Capability Product Meaning

Deferred. It blocks any work that depends on a standalone Capability meaning,
including Context Profile capability selection, regardless of owning domain.
It does not block this Capability-independent foundation.

Tool/Dependency Guidance was accepted on 2026-09-07 in Product, but neither
resolves Gate 3 nor authorizes Domain 5 implementation. Accepting Gate 3 later
would still not expand Feature 001 automatically. No Capability IDs, empty
lists, no-op selection methods, ports or fixtures may disguise unsupported work.

## Constitution Check

Pre-planning and post-design review: **PASS for the document/three-core scope**.
This is neither implementation acceptance nor Constitution ratification.

| Principle | Design evidence |
| --- | --- |
| Context, Not Workflow or Execution | Core evaluates supplied context facts; no command runner or execution owner |
| Grounded and Explainable Context | Rejected selections, snapshot identity, candidates, Evidence and Scope Match stay explicit |
| Local-First, Portable, and Rebuildable | No hosted dependency; portable values exclude local bindings; no store is introduced |
| Explicit and Reversible Writes | Implementation reset is backed up; managed-write design remains deferred; no user-project writes |
| Human-Inspectable by Default | Owning ADRs, conceptual model, Rust mapping, typed errors and review records remain inspectable |
| Deliver Value in Vertical Slices | Internal foundation makes no user-visible outcome claim; no supporting-domain implementation |

Post-design: no Product/Constitution change, no public contract, no new Capability
meaning, no required application wrapper, and no exception/complexity waiver.

## Domain Strategy and Delivery Scope

| Domain | This Feature |
| --- | --- |
| Project Catalog | Workspace, source references, Repository/Resource registration, Capability-independent Profile |
| Local Context Resolution | Binding Registry, observations, raw/valid Task Context, resolution |
| Context Knowledge | Source/Evidence/Relation/Scope, independent statuses, explainable query-derived results |
| Managed Materialization | Design only; no code, port or test double |
| Capability Integration | Design only, including Tool Guidance; no discovery/installer/execution |

The existing [Domain Model](data-model.md) owns the complete concept/invariant
inventory. [Rust Design](rust-design.md) maps the three cores to implementation
types and records lexical/path/state boundaries without accepting a public API.

## Project Structure

### Documents

    docs/
      engineering.md
      adr/
        0001-domain-oriented-modular-monolith.md
        0002-initial-python-runtime.md          # historical, superseded
        0003-rust-runtime.md                   # current runtime
    specs/001-foundation-context/
      spec.md
      plan.md
      research.md
      data-model.md
      rust-design.md
      context-semantics.md
      materialization-semantics.md
      tool-use-semantics.md
      quickstart.md
      tasks.md
      checklists/requirements.md

No contracts directory: there is no public machine contract in this Feature.

### Source Layout

    Cargo.toml                                # virtual workspace, resolver 3
    Cargo.lock                                # generated by Cargo
    rust-toolchain.toml                        # tested operational toolchain
    crates/
      shared-kernel/                          # package devmeld-shared-kernel
        Cargo.toml
        src/lib.rs
        src/identity.rs
        tests/identity.rs
      catalog/                                # package devmeld-catalog
        Cargo.toml
        src/lib.rs
        src/references.rs
        src/registrations.rs
        src/profiles.rs
        src/workspace.rs
        tests/registrations.rs
        tests/workspace_profiles.rs
      local-context/                          # package devmeld-local-context
        Cargo.toml
        src/lib.rs
        src/bindings.rs
        src/observations.rs
        src/task_context.rs
        src/resolution.rs
        tests/state_validation.rs
        tests/resolution.rs
        tests/safety.rs
      knowledge/                              # package devmeld-knowledge
        Cargo.toml
        src/lib.rs
        src/provenance.rs
        src/relations.rs
        src/scope.rs
        src/context_result.rs
        tests/provenance_relations.rs
        tests/scope.rs
        tests/context_result.rs
        tests/core_fact_boundaries.rs
    .cargo/config.toml                        # cargo xtask alias
    tools/xtask/                              # developer-only; not a core domain
      Cargo.toml
      src/main.rs
      src/architecture.rs
      src/probes.rs

Additional error modules or focused test fixtures may be introduced with real
content. Do not create an empty root facade, binary, application, ports, adapters,
entrypoints or either supporting domain. Internal modules default to private;
lib.rs selectively re-exports the domain's intended API.

Four library members are justified by three actual ownership boundaries and a
shared identity need, not a rule that every future noun/layer needs a crate.
The detailed layout is a revisable Plan choice, not a public SDK contract.

The subsequent type refinement stays within these existing modules: ProfileId
and SUPPORTED_SCHEMA_VERSION belong to Catalog; ObservationId and RejectionReason
belong to Local Context; WorkingTreeState belongs to Knowledge. Existing tests
and the compiler-consumer harness cover the changed signatures. No new crate,
dependency, shared-kernel value, adapter or public protocol is introduced.

## Dependency and Modeling Rules

### Shared-Kernel Inventory

Only RepositoryId and ResourceId: Catalog/Local Context/Knowledge share the former;
Catalog/Knowledge share the latter. Use distinct validated immutable newtypes.
Single-owner IDs, schema versions, timestamps, paths, Scope, status enums and
errors remain with their owners. No generic validation/utilities framework.

### Allowed Edges

| Package | Normal dependencies | Dev dependencies beyond its normal ones |
| --- | --- | --- |
| devmeld-shared-kernel | None | None |
| devmeld-catalog | devmeld-shared-kernel | None |
| devmeld-local-context | devmeld-shared-kernel | None |
| devmeld-knowledge | devmeld-shared-kernel | devmeld-catalog and devmeld-local-context, only for core_fact_boundaries tests |
| xtask (developer tool, not a domain) | serde_json (registry; tool-only) | None |

Foundation dependency-shape decision (post-commit review): each of Catalog,
Local Context and Knowledge declares exactly one normal shared-kernel dependency.
It must be unconditional, non-optional and unrenamed. Additional target-specific
or optional declarations to that same allowed destination are rejected; renaming
the required declaration is also rejected. The shared kernel has no dependencies.
Three real-manifest probes cover the allowed destination with an extra target
declaration, an extra optional target declaration and a renamed declaration.
Existing Knowledge test-only domain edges retain their current policy.

No core may depend on xtask. It has no dependency on core libraries; it inspects
their metadata and builds isolated test consumers. All subprocesses use Rust
Command with separate arguments, not shell command strings. Paths use native
Path/PathBuf operations; temporary fixtures are owned and bounded. Rust/Cargo
and the native linker suffice for project verification on each supported host.
Python is required only for optional Spec Kit workflow commands, not cargo xtask.

No build-dependencies or build scripts in the foundation. Cross-core acceptance
maps public immutable facts into consumer-owned values in test code only;
Knowledge production code does not depend on the other cores. No examples or
benchmarks use these dev edges. No production coordination is introduced merely
to host a test. These test edges are added at T024, not initial scaffolding.

Workspace members and dependencies are explicit, with one root lockfile.
Each member inherits workspace lint/package policy. Do not bypass boundaries
through path includes, foreign source modules, broad re-exports or build-time
code generation.

### Enforcement After Real Crates Exist

The architecture check reads cargo metadata --format-version 1 --no-deps and
inspects every packages[].dependencies declaration, not resolve (null in this
mode). Cover normal/dev/build, optional, renamed and target-specific edges.
Resolve allowed local packages by member identity and manifest/path; do not
infer identity by splitting Cargo's opaque package IDs or trusting import aliases.
Reject unexpected members, third-party declarations in core crates,
unknown dependency kinds or paths outside the expected workspace.
Only xtask's explicitly admitted serde_json declaration is a tool exception.

Probes use isolated copies of the real manifests/source/checker and generate
fixture-local lockfiles when adding seed dependencies. Never mutate the real
working tree or lockfile for a negative test. Test valid graphs and allowed dev
edges, then forbidden core-to-core/kernel-outward/adapter/entrypoint edges,
including hidden target/optional/build/renamed variants. Separately compile
consumer fixtures proving private modules/fields and validated-state boundaries.
Check intended diagnostics after valid controls compile successfully.

Cargo metadata omits lint settings. Member manifest review checks explicit
inheritance; separate unsafe compiler probes in all four copied members verify
the effective safety gate. The checker also bounds the current four flat
library/source/test layouts; approved future layers require updating this
Feature-local allowlist. This is not a permanent architectural folder quota.

Graph checks do not detect every possible std IO call or prove architectural
correctness. Review domain purity, source inclusion, semantic ownership and
public surfaces separately; tests and compiler checks are complementary.

## Rust Modeling and Coverage

Preserve the full approved foundation, not the narrower experimental subset:

- Catalog includes schema version, names, aliases, source references, supported
  locators, same-Workspace identity/selection rules and immutable update checks.
- Local Context includes local bindings and registry revision, full observation
  facts, validation errors retaining rejected selections, working area and
  candidate/basis explanations. Validated context owns the exact checked snapshot.
- Knowledge keeps Evidence/revision/hash or derivation, directed relation facts,
  independent status dimensions, all designed Scope dimensions and explanations.
  Unsupported interval matching remains Unknown, never invented semantics.
- Errors, type/visibility checks and behavior tests all matter; a successful
  compiler run cannot substitute for schema, cross-field or scope validation.

T023 implementation review: all current rules accept owner-local immutable values
or explicitly supplied facts. No application coordination or external-fact port
is necessary. T024 translates the exposed facts in test code only; no production
facade or layer is introduced. Future needs must name their rule and exact tasks.

## Delivery Sequence

1. T001–T005: native setup, reproducible Cargo policy, shared identity tests/code.
2. T006–T007: record fresh implementation-start review of existing design/gates.
3. T008–T024: tests-first implementation of the three complete core domains,
   followed by test-only fact-boundary validation.
4. T025–T028: adversarial regressions, real graph checks and seeded compiler/
   dependency violations; no production IO or speculative integration.
5. T029–T032: semantic review, native checks and honest acceptance submission.

The reset task list was implemented and its baseline verification recorded for
native Windows and Linux in WSL. Post-commit corrections and their separate
verification limits are recorded in tasks.md and quickstart.md; Maintainer
acceptance remains pending.
A document checkpoint or empty workspace is not a product MVP or foundation acceptance.

## Design Artifacts

- [Research](research.md)
- [Conceptual model](data-model.md)
- [Rust mapping and invariants](rust-design.md)
- [Context semantics](context-semantics.md)
- [Materialization semantics](materialization-semantics.md)
- [Tool Guidance semantics](tool-use-semantics.md)
- [Acceptance guide](quickstart.md)
- [Tasks](tasks.md)

## Complexity Tracking

No Constitution exception. Four library boundaries use the already selected
Cargo toolchain; no new service, framework or mandatory technical layer is added.
