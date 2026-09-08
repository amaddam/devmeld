# DevMeld

DevMeld organizes resources and access guidance into durable, readable project
context. Agents follow a small project entry to the resources they need;
ordinary reading does not require DevMeld to be running.

The product name is **DevMeld**; the repository name is **devmeld**.

## Current State

The Maintainer authorized a design restart on 2026-09-08. The old 001/002 design
files, four domain crates and their tests have been removed. Their old domain
selection, APIs and test results are not requirements for the new design.

Only the Rust engineering scaffold remains: the workspace, toolchain settings
and developer-only `tools/xtask`. There is no product library, CLI, background
service, resource connector or context generator implemented yet.

## Read First

1. [Product](docs/product.md): the confirmed artifact-first product direction.
2. [Domain proposal](docs/domain-model.md): proposed ownership boundaries, for
   review before implementation; not a predetermined crate layout.
3. [Engineering](docs/engineering.md) and [Contributing](CONTRIBUTING.md):
   implementation practices and decision ownership.
4. [Discussion record](docs/notes/2026-09-08-context-generation-and-consumption.md):
   confirmed intent, rejected directions and remaining decisions.
5. [Reset record](docs/notes/2026-09-08-foundation-reset.md):
   what was removed, what remains, verification and workflow assessment.

The [Constitution](.specify/memory/constitution.md) remains a draft.
[ADR-0001](docs/adr/0001-domain-oriented-modular-monolith.md) retains general
domain-oriented modular-monolith principles, not the retired domain map.
[ADR-0003](docs/adr/0003-rust-runtime.md) retains the Rust runtime decision;
its old foundation layout is historical.

## Run Checks

With the existing pinned Rust toolchain, rustfmt, Clippy and native linker:

```text
cargo xtask check
```

This runs formatting, compiler checking, conservative Clippy and Cargo tests
using native Rust process APIs. No PowerShell, Bash or Python is required.
The remaining scaffold has no third-party dependencies.

After the reset this passed on native Windows, with zero product tests.
That is scaffold verification, not evidence that a product feature works.
The reset has not been verified on Linux/WSL or macOS.

## Workflow During Redesign

There is no active Feature or implementation task list. Spec Kit's installed
skills, scripts, templates and Constitution have been retained, but the old
active Feature pointer was removed. Do not resume an old 001/002 task sequence
or generate a new one before the new domain and delivery boundaries are reviewed.

Spec Kit's optional development helpers use the configured Python workflow;
they are not runtime dependencies of DevMeld or of its generated context.
Whether to keep using the full Spec Kit workflow remains a separate decision.

Restart and design snapshots are recorded in local Git history. Remote publishing
remains with the Maintainer.
