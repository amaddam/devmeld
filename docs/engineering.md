# DevMeld Engineering Guide

- Status: engineering conventions retained through the 2026-09-08 domain reset
- Audience: contributors and maintainers implementing DevMeld
- Purpose: define implementation and quality practices without duplicating product behavior or contribution policy

## Sources of Direction

Implementation must comply with the stable principles in
[`constitution.md`](../.specify/memory/constitution.md), the product meanings in
[`product.md`](product.md), and, for a user-visible feature or material behavioral
change, its approved Feature Spec. If those artifacts conflict, stop and resolve
the owning decision before coding.

## Code Design

- Use names that express product intent and match the vocabulary in
  `docs/product.md`.
- Keep modules and functions focused on one responsibility with explicit inputs,
  outputs, ownership, and failure behavior.
- Prefer the smallest abstraction that serves a current approved scenario. Do
  not add speculative inheritance trees, generic extension containers, or
  framework layers that only rename simple operations.
- Follow [ADR-0001](adr/0001-domain-oriented-modular-monolith.md): organize code
  by domain, but create an `application/` package only for real coordination
  outside domain objects. Empty layers and pass-through services are not
  required architecture. Create ports only for implemented needs.
- Keep core product behavior independent of a particular Agent Client,
  Capability Provider, workflow tool, storage engine, or user interface. Put
  integration-specific behavior behind explicit contracts and adapters.
- Preserve the distinction between portable source data, machine-local locations and observations,
  rebuildable derived state, and generated artifacts. Do not turn a cache or
  machine path into an undeclared source of truth.
- Represent ambiguity, missing evidence, stale state, and conflicts explicitly.
  Do not choose a convenient default when it could return context for the wrong
  source or silently change user-managed content.

## Model-Driven Change and Refactoring

Existing code is evidence of an earlier design, not authority for the next one.
Before extending an affected path, reason from the current requirement, domain
language, invariants, ownership and data flow; then assess whether the existing
model expresses them correctly. Do not infer intended behavior from whichever
function or folder is easiest to patch. The same applies to code written by an
Agent in an earlier task.

- When a requirement exposes a mistaken concept, misplaced responsibility,
  duplicated policy or an abstraction that needs exceptions at every caller,
  reconsider the affected model before adding branches. Ordinary conditionals
  and platform-specific adapters are not inherently design failures.
- Choose a local change, preparatory refactoring or replacement of an affected
  implementation based on semantic clarity and current needs, not the smallest
  diff or preservation of existing names, signatures and file layout. Moving
  code into more files is not sufficient if the responsibility error remains.
- Refactoring may precede or accompany a feature. A cohesive module or its
  affected collaborators may need restructuring; small verified steps do not
  impose a small total diff. Remove superseded paths and duplication rather
  than retaining compatibility scaffolding for unneeded internal structures.
- Preserve accepted observable behavior and required contracts, not every
  historical accident. Establish a green baseline and add characterization
  coverage where needed. Structure-coupled tests may evolve, but passing tests
  do not justify an incorrect model, and assertions must not be weakened merely
  to accommodate a replacement. Treat defects as explicit regression fixes.
- If the newly understood rule changes Product meaning, Feature behavior or an
  accepted architecture boundary, resolve and record that owning decision first
  under CONTRIBUTING.md. Do not conceal semantic or persisted-data changes as
  behavior-preserving refactoring. Already-authorized internal restructuring
  does not require repeated approval just because several files change.
- Keep the work tied to the actual design problem and its affected dependency
  path. Do not expand it into unrelated cleanup, a speculative framework or a
  whole-project rewrite. Preserve recoverable Git history; follow the existing
  authorization for commits rather than assuming a refactor authorizes one.

This follows DDD's refinement of models through deeper domain understanding
([DDD Reference, Refactoring Toward Deeper Insight](https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf))
and [preparatory refactoring](https://martinfowler.com/articles/preparatory-refactoring-example.html).
Neither requires a new tool, arbitrary complexity quota or a separate design
document for every small edit.

## Runtime and Automated Quality Gates

[ADR-0003](adr/0003-rust-runtime.md) retains stable Rust, Edition 2024, and
standard-library-first development. ADR-0002 is historical. The active file-based
implementation follows [the domain model](domain-model.md) and 003's Plan, not
old 001. `resources` and `publication` are pure domain crates; `devmeld` composes
them with local JSON, schema, rendering and filesystem adapters.

### Toolchain and Dependencies

Use the existing compatible native toolchain where possible. Record its host,
compiler, Cargo, rustfmt, Clippy and linker evidence in the Feature acceptance
guide. Pin the tested compiler in `rust-toolchain.toml`; record intentional
minimum support with `rust-version`, without claiming untested older releases.
An operational version pin is updateable, not a permanent architecture rule.
Do not silently install Rust, build tools, a second runtime, or dependencies.

Use one Cargo workspace, root lockfile and target directory. Declare shared
settings at the root and explicitly inherit them in every member. Start with
the standard library where adequate. The domain crates currently use std only;
application dependencies are listed in 003's Plan. `tools/xtask` reuses the
workspace's serde_json version to inspect Cargo metadata. New dependencies need
a concrete requirement; inspect existing declarations before adding one.
No blanket ban on third-party libraries is intended, and no shared-kernel or
test-only domain edge is preapproved for the new model.

### Rust Modeling Style

- Prefer ordinary structs, enums, newtypes, methods and named policies. Traits,
  generics and macros need an actual abstraction or substitution requirement;
  do not reproduce a Java inheritance framework or design every value as a trait.
- Protect validated state with private fields and validating constructors.
  Expose shared immutable views; do not return mutable references to protected
  fields or add unchecked constructors to make tests easier.
- Use distinct identity types where identity kinds must not interchange.
  Give failure cases owner-local typed errors; errors used at crate boundaries
  should have readable `Display` and appropriate `std::error::Error` support.
- Keep expected failures in explicit `Result` outcomes, not panics or fabricated
  success. A domain-result enum models mutually exclusive semantic outcomes;
  validation errors remain separate. Review wildcard matches on closed domain
  outcomes rather than globally prohibiting wildcard syntax.
- Prefer understandable ownership and owned snapshots. Use `Clone` where a
  snapshot or duplication is intentional; do not introduce lifetimes, `Arc`,
  interior mutability or async merely to avoid an unmeasured copy.
- Pure rules receive observation, freshness and time facts explicitly. They do
  not perform filesystem/network/process IO or read a hidden current clock.
  Standard-library path/time value types are not themselves IO.
- Keep serialization, SDK types and adapters outside core invariants. No
  universal error registry, metadata bag, entity base, or blanket service layer.
- Use `expect`/`unwrap` for clearly valid test fixtures if useful. In production,
  handle fallible external/domain inputs explicitly; an assertion needs a true
  internal invariant, not a convenient substitute for error handling.

### Conservative Check Policy

Use compiler checking, default rustfmt, Clippy, and Cargo's built-in tests:

- Rust compiler errors are gates. The retained `unsafe_code = "forbid"` policy
  applies to handwritten workspace code that inherits it, not claims about
  the standard library or future dependencies.
- Clippy `correctness`: deny; `suspicious` and `perf`: warn.
- Clippy `style`, `complexity`, `pedantic`, `nursery`, and `restriction`:
  allow initially. Use group priority `-1` so narrow justified overrides can
  take precedence. Do not enable preview/nightly-only rules by default.
- Each member opts into `[lints] workspace = true`; root settings alone do not
  propagate. Narrow overrides must be visible and justified.
- Do not use blanket `-D warnings`, redundant formatters/type checkers, blanket
  clone bans, or quotas for branches, arguments, returns, complexity, function/
  file length, or coverage percentage.
- Unit and integration tests verify behavior; compile-fail probes verify specific
  type/visibility guarantees. A failing compiler command is evidence only after
  a valid control succeeds and the intended diagnostic is confirmed.

### Cross-platform Check Entry Point

Project checks MUST NOT require an OS-specific shell. Use Rust filesystem and
process APIs with native paths and separate arguments; do not compose commands
for PowerShell/Bash/cmd, hard-code drive letters, or assume an executable suffix.
Platform details such as rejecting Windows junctions may be handled locally
behind cfg, without changing the common check entry point.

With the pinned Rust toolchain and native linker already available, run
`cargo xtask check`. Bootstrap reviewed lockfile dependencies with
`cargo fetch --locked` before offline checks on a fresh environment.
The entrypoint executes this equivalent sequence on the current native platform:

```text
cargo xtask boundaries
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked --offline
cargo clippy --workspace --all-targets --locked --offline
cargo test --workspace --locked --offline
```

The xtask alias is defined in .cargo/config.toml and runs with --locked --offline.
It requires neither PowerShell/Bash nor Python. Checks fail on nonzero native-command exit.
The default Cargo test command
includes doctests; if selecting `--all-targets`, run doctests separately.
Offline assumes required dependencies and toolchains already exist; Cargo
offline does not stop rustup from trying to acquire a missing toolchain.

Record platform and test evidence in the active Feature's acceptance document.
The old reset's zero-test result and old 001 platform evidence do not establish
current behavior. A local check does not imply another OS or Agent client passed.

Spec Kit's selected workflow helpers use `.specify/scripts/python` and an
existing Python 3 interpreter. Both integration settings select `script: py`.
Official integration refreshes may install PowerShell auxiliary scripts under
`.specify/scripts/powershell`; keep them as upstream-managed infrastructure.
Their presence does not change the selected Python workflow or introduce a
PowerShell dependency into Rust builds and checks.
Python may display shell-specific environment-variable hints, but does not
execute a shell or depend on a PowerShell script to perform its work.

### Dependency Direction Checks

The old four-crate scope checker and its probes have been removed. Do not
restore that package inventory as a requirement for the new product. The current
gate protects the two implemented std-only domain crates; it does not prohibit
future application adapters or prescribe a folder count. Add or change checks
only for reviewed boundaries, with valid and forbidden examples.
The relevant technical design owns allowed normal/build/dev edges.
Check all declarations, including optional, renamed and target-specific ones;
do not only inspect currently activated features or the current host's graph.
Use Cargo metadata JSON, not hand-written TOML parsing or source-text import
guessing. Treat metadata IDs as opaque.

Cargo metadata does not expose workspace lint inheritance. Review the member
manifests explicitly; add focused compiler probes if an effective policy needs
automated verification. Current probes cover declared dependency edges and the
private ResourceId constructor, not every future type or policy.
This does not prove every Clippy setting or forbid all future local lint overrides.

Prove the real gate using isolated copies/fixtures containing forbidden edges
and private-import attempts, with valid controls. Test-only dependencies do not
authorize production coupling, examples, benchmarks or supporting-domain code.
Do not create fake production layers to make a checker look effective.

Cargo graph checks cannot prove that core code never calls `std::fs`,
`std::process`, or a clock. Code review and focused behavior tests still own
that purity check. Dependency checks do not replace semantic review.

Sonar, Import Linter, additional analyzers, property-test libraries, hook
frameworks and async runtimes are not initial requirements.

## Human Inspection and Documentation

Code, public contracts, generated artifacts, manifests, and error messages must
be understandable without access to an opaque internal store. Comments should
explain non-obvious domain rules, constraints, or trade-offs rather than repeat
syntax.

When a change accepts a durable decision, update the artifact that owns it:

- cross-feature product meaning: `docs/product.md`;
- feature behavior and scope: the relevant Feature Spec;
- lasting architecture trade-off: an ADR;
- implementation convention: this guide;
- contribution or review process: `CONTRIBUTING.md`.

Create an ADR only when there is an accepted architecture decision to record; do
not create placeholder decision files or empty directories.

## Behavioral Development

For new or changed domain/application behavior, use vertical TDD by default.
Start from an accepted behavior and its owning Spec, Plan and ADRs, following
this guide. Test through a public or intentionally exposed domain boundary.
Do not add a facade, application layer or public API merely to enable a test.
Do not ask the Maintainer to reconfirm an already accepted boundary; clarify
only unresolved scope, conflicting decisions or undecided boundary choices.

If the current structure obstructs the accepted behavior, first perform the
necessary preparatory refactoring on a green baseline as described above.
The cycle below is not a requirement to patch the old structure first or to
postpone design correction until after implementing the feature.

Within each behavioral task:

1. Select one observable rule or case from the accepted behavior.
2. Add the smallest test that demonstrates the missing behavior.
3. Run it and confirm failure for the intended reason. Environment failures
   are not a behavioral RED. A compile failure caused specifically by an
   intentionally missing, already-accepted API may serve as the initial RED.
4. Implement the minimum coherent behavior needed to pass that test.
   Once the API compiles, ensure the test actually asserts the required behavior
   and passes before treating the behavior as GREEN. Do not add a knowingly
   incorrect stub solely to manufacture a runtime RED.
5. Run the focused test and affected suite. Make necessary refactors in verified steps,
   keeping the suite green, then continue with the next case.

One task owns a bounded behavior slice and may contain several small cycles.
Do not batch all tests for a feature or story before implementing any behavior,
or turn RED, GREEN and refactor into separate checklist tasks. Do not use a
cycle as permission to implement future behavior or speculative abstractions.
Expected values must come from accepted rules or independently worked examples,
not repeat the implementation's calculation. Avoid tests coupled to private
structure when observable behavior is the subject under test.

Choose verification by task type:

| Task | Execution and evidence |
| --- | --- |
| New or changed behavior | Small failing behavior test, implementation, focused suite, necessary refactor, green suite |
| Bug fix preserving accepted behavior | Reproduce the defect with a failing regression test, fix it, verify the affected suite |
| Existing behavior refactor or rename | Establish a green baseline, add characterization coverage if needed, refactor, keep green; do not remove working code to manufacture RED |
| Compile-time type or visibility boundary | Successful valid control plus a compile-fail probe with the intended diagnostic |
| Architecture dependency rule | Focused architecture probe with valid and forbidden cases |
| Tooling, setup or CI configuration | Direct configuration, command or workflow verification appropriate to the change |
| Documentation | Review consistency, references and examples; validate executable examples when applicable |

Boundary probes test the boundary itself and are not ordinary behavioral TDD.
This distinction does not exempt them from verification. Record justified
departures from behavioral TDD and unavailable checks; do not claim RED was
observed when it was not. Report the focused command and relevant RED/GREEN
outcome with the task's evidence, without creating separate evidence artifacts
for every cycle. Full checks and Feature acceptance remain required as applicable.

Apply this convention to subsequent new behavior and fixes. Historical test
passes are not evidence for the new domain model. Spec Kit's retained
project-owned templates and command sources express this convention;
see [workflow maintenance](../CONTRIBUTING.md#spec-kit-customizations).

## Testing and Evidence

Tests must be proportional to behavioral risk and must verify observable
outcomes rather than internal structure alone.

- Unit tests cover focused rules and failure cases.
- Contract tests cover public machine contracts and generated artifact shapes.
- Integration tests cover the actually implemented source reading, validation,
  synchronization, source attribution, adapters and Managed Surface writes.
- End-to-end acceptance tests cover each approved user scenario through the
  smallest complete vertical slice.
- A claim that DevMeld improves context accuracy, traceability, or navigation
  cost requires a realistic benchmark against a recorded baseline.

Tests for managed writes must cover preview, conflicts, repeated application,
and recovery. Tests for derived state must show that it can be rebuilt from its
declared sources. A generated file or passing internal test is not sufficient
evidence when the Feature Spec requires a user-visible outcome.

## Change Quality

Before review, keep the implementation inside the approved Feature Spec when one
is required, remove unused abstractions, run the relevant automated checks, and
record any check that could not be run. Review findings should identify whether
the issue belongs to the product baseline, Feature Spec, architecture,
engineering guidance, or implementation, so the correction is made in the
owning artifact.

For material design changes, explain the old assumption or responsibility that
was wrong, why the revised structure fits the current rules, and which behavior
and contracts were preserved or intentionally changed. Review the resulting
model and maintenance burden, not merely diff size or whether tests pass.
