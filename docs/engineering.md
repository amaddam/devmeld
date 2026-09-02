# DevMeld Engineering Guide

- Status: initial engineering baseline
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
- Preserve the distinction between portable source data, Local Bindings,
  rebuildable derived state, and generated artifacts. Do not turn a cache or
  machine path into an undeclared source of truth.
- Represent ambiguity, missing evidence, stale state, and conflicts explicitly.
  Do not choose a convenient default when it could return context for the wrong
  Checkout or silently change user-managed content.

## Runtime and Automated Quality Gates

[ADR-0002](adr/0002-initial-python-runtime.md) accepts Python 3.14.x and
standard-library-first core development. The initial foundation has no
production third-party dependencies. Tool configurations are created with the
first code; this guide defines their accepted baseline.

- **Ruff**: use its formatter and rule families `E`, `F`, `I`, `UP`, and `B`,
  ignoring `E501`. Target Python 3.14. Do not enable `ALL`, preview rules, `SIM`,
  duplicated annotation checks, or complexity/size thresholds by default.
- **mypy**: target Python 3.14 and use strict checking for core domain code and
  any implemented application/port code. Tests and future adapters may use
  narrow, explained overrides where needed; avoid repository-wide ignores or
  untyped core boundaries. Strict typing is not a requirement to split domain
  logic into more functions or classes.
- **pytest**: verify observable invariants and failure outcomes. Validate
  configuration and registered markers; do not impose a numeric coverage target
  or implementation-shape quota instead of testing meaningful behavior.

No initial hard limits apply to branches, arguments, returns, statements,
cyclomatic complexity, function length, or file length. Rule exceptions should
be local and explained; new rule families need an identified risk rather than
automatic adoption of every available check.

After setup, local and CI checks must use the same documented sequence:

```text
python -m ruff format --check src tests
python -m ruff check src tests
python -m mypy src/devmeld
python -m pytest
```

Add Import Linter only once real domain packages exist. Limit it to accepted
dependency directions, prove rejection with controlled failing examples, and
do not create empty application, port, adapter, or entrypoint packages merely
to satisfy its configuration. Run `lint-imports` as an additional check only
after this gate is introduced.

Sonar, additional type checkers, property-testing libraries, and hook frameworks
are not initial requirements. Add one only when it covers a distinct actual
need. Record compatible development-tool versions and interpreter versions so
checks are reproducible, without freezing patch versions in governance prose.

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

## Testing and Evidence

Tests must be proportional to behavioral risk and must verify observable
outcomes rather than internal structure alone.

- Unit tests cover focused rules and failure cases.
- Contract tests cover public machine contracts and generated artifact shapes.
- Integration tests cover Active Checkout resolution, persisted or restored
  state, source attribution, adapters, and Managed Surface writes.
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
