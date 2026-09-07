# ADR-0002: Initial Python Runtime and Quality Baseline

- Status: Superseded by [ADR-0003](0003-rust-runtime.md) on 2026-09-07
- Decision date: 2026-09-02
- Scope: Initial DevMeld runtime and progressive quality-tooling baseline
- Supersedes: none

## Decision Record

Historical record only. The Maintainer accepted Rust as the replacement runtime
on 2026-09-07. [ADR-0003](0003-rust-runtime.md) owns the current runtime and
quality baseline; the original decision below is retained, not rewritten as if
Python had never been accepted. Previous Python test results do not constitute
Rust implementation evidence.

The Maintainer approved this runtime and quality-baseline decision (Gate 2)
on 2026-09-02. The architecture decision is recorded separately in
[ADR-0001](0001-domain-oriented-modular-monolith.md).

The decision is permission to use this baseline, not evidence that the code or
quality configuration has already been implemented or tested.

## Context

The initial foundation contains pure identity, validation, resolution, and
provenance rules. It does not need a production framework, parser, database,
CLI/UI toolkit, or provider SDK. Deterministic local checks should protect
correctness without imposing overlapping analyzers or mechanical code-size
limits.

On the decision date, the official [Python version status](https://devguide.python.org/versions/)
lists Python 3.14 as a stable bugfix-maintained release line, with end of life
scheduled for October 2030. The local planning interpreter reports Python
3.14.6; this is an environment observation, not a project-wide patch pin.

## Decision

1. Use Python 3.14.x as the initial runtime baseline. Select maintained patch
   releases during setup and record the tested interpreter version. Older or
   future minor-version support is not implied by this decision.
2. Use standard-library types and facilities for the production core. This
   foundation adds no production third-party libraries. Later dependencies
   require an actual feature need and review in their owning Plan or ADR.
3. Start with three required quality tools: Ruff for formatting and conservative
   linting, mypy for core static typing, and pytest for behavioral/invariant
   evidence. Detailed settings belong in the
   [Engineering Guide](../engineering.md#runtime-and-automated-quality-gates),
   not duplicated as permanent configuration in this ADR.
4. Keep logical-simplification, complexity, branch-count, function/file-size,
   and numeric-coverage gates out of the initial baseline. Do not add a second
   formatter or type checker to duplicate an existing responsibility.
5. Add Import Linter only after actual core packages exist, with a small set of
   accepted and demonstrably effective dependency contracts. Sonar and other
   quality platforms remain deferred until a distinct need exists.
6. Keep local Windows, macOS, and Linux development as the target. Cross-platform
   verification is future implementation evidence, not guaranteed by choosing
   Python alone. No hosted service or special interpreter build is required.

Dependency-manager choice, distributable packaging, and concrete tool-version
pins remain implementation details. Setup must select compatible stable tool
releases and record a reproducible development environment without silently
changing this baseline.

## Compatibility Evidence

- Ruff's [target-version setting](https://docs.astral.sh/ruff/settings/#target-version)
  includes `py314`.
- Mypy's [release notes](https://mypy.readthedocs.io/en/stable/changelog.html#initial-support-for-python-3-14)
  document running/testing on Python 3.14; support for every new language
  construct must not be assumed from runtime support alone.
- Pytest's [release notes](https://docs.pytest.org/en/stable/changelog.html)
  document Python 3.14 compatibility work. The selected tool versions still need
  a local smoke test during implementation setup.

These sources support feasibility, not a claim that DevMeld's future test suite
already passes.

## Alternatives Considered

- **A prerelease runtime**: unnecessary uncertainty for an initial foundation
  while a stable 3.14 line is available.
- **A full production framework from the outset**: no approved infrastructure
  need currently justifies coupling the domain to it.
- **Sonar or several overlapping analyzers immediately**: adds operational and
  diagnostic overhead before a distinct repository need is demonstrated.
- **Prompt-only style enforcement**: cannot provide repeatable local/CI checks.
- **Maximal lint rules and complexity thresholds**: could pressure coherent
  domain behavior into artificial fragments instead of improving correctness.

## Consequences and Revisit Triggers

The project now requires a suitable Python environment and must check future
dependencies for runtime/platform compatibility. Standard-library-first is not
a permanent ban on dependencies, and the initial tool trio is not a requirement
to solve every future quality problem with those tools.

Revisit the runtime or tooling when measured compatibility, deployment,
performance, or uncovered quality needs justify the change. Do not silently
lower the runtime or broaden lint gates to work around an isolated issue.
