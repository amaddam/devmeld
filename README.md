# DevMeld

DevMeld is a local-first project context layer for software development.
It owns context, not workflow or execution.

## Current State

The Maintainer accepted Rust as the main implementation language on 2026-09-07.
The previous Python implementation was removed during the design reset. Rust
implementation is complete and verified on native Windows and Linux in WSL: the
four core libraries, shared identities and three core domains exist. Maintainer acceptance
is pending; this is not a finished application.

Both environments pass 37 core behavior tests, one compile-fail doctest, three
developer-tool unit tests and 40 architecture/type/scope probes. Formatting, Cargo checking and conservative
Clippy pass; one non-blocking performance warning remains documented. See the
[acceptance evidence](specs/001-foundation-context/quickstart.md#11-implementation-status-and-required-rust-evidence).

The product name is **DevMeld**; the repository name is **devmeld**. The existing
Windows checkout directory need not be renamed to express that distinction.

## Read in This Order

1. [Product](docs/product.md): what DevMeld means and where its boundaries are.
2. [Architecture ADR](docs/adr/0001-domain-oriented-modular-monolith.md) and
   [Rust runtime ADR](docs/adr/0003-rust-runtime.md): current accepted decisions.
3. [Foundation Spec](specs/001-foundation-context/spec.md) and
   [Domain Model](specs/001-foundation-context/data-model.md): complete scope and owners.
4. [Rust Design](specs/001-foundation-context/rust-design.md) and
   [Plan](specs/001-foundation-context/plan.md): type boundaries and planned workspace.
5. [Engineering](docs/engineering.md), [Tasks](specs/001-foundation-context/tasks.md)
   and [Acceptance Guide](specs/001-foundation-context/quickstart.md): how to implement and verify.

Contributions follow [CONTRIBUTING.md](CONTRIBUTING.md) and the
[Constitution](.specify/memory/constitution.md). The Constitution is still a draft;
choosing Rust does not ratify it.

## First Implementation Boundary

Only Project Catalog, Local Context Resolution and Context Knowledge, plus two
shared identity values. Managed Materialization and Capability Integration
(including Tool Guidance implementation) remain design only. No production CLI,
UI, database, provider adapter or public protocol is in Feature 001.

The next step is Maintainer review of this foundation, not automatic acceptance
or integration work. Cargo commands and the existing-toolchain reuse procedure are
in the acceptance guide. WSL is not required; its Linux results supplement native
Windows evidence. macOS has not been tested. No commit or push has been performed.

## Run Checks

With the pinned Rust toolchain, rustfmt, Clippy and the platform's native linker
available, run from the repository root:

```text
cargo fetch --locked
cargo xtask check
```

The first command bootstraps the approved locked developer-tool dependencies.
Subsequent checks run locked/offline. The same entrypoint uses native Rust APIs
on each OS; no PowerShell, Bash or Python is required for project checks.
`tools/xtask` is a developer-only package, not a product CLI or domain.
Its only direct third-party dependency is `serde_json`; the four core libraries
still have no third-party dependencies. See [Engineering](docs/engineering.md#cross-platform-check-entry-point).

Spec Kit uses only `.specify/scripts/python/` as workflow infrastructure; the
redundant PowerShell scripts have been removed. Use an existing Python interpreter
when invoking Spec Kit, not when building or checking the Rust project.
These scripts do not make Python a DevMeld runtime dependency. The old runtime decision
is retained as [superseded ADR-0002](docs/adr/0002-initial-python-runtime.md).
