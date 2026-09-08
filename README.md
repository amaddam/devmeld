# DevMeld

DevMeld organizes resources and access guidance into durable, readable project
context. Agents follow a small project entry to the resources they need;
ordinary reading does not require DevMeld to be running.

The product name is **DevMeld**; the repository name is **devmeld**.

## Current State

The Maintainer authorized a design restart on 2026-09-08. The old 001/002 design
files, four domain crates and their tests have been removed. Their old domain
selection, APIs and test results are not requirements for the new design.

The first replacement implementation is in `003-durable-context`: native Rust
commands maintain registration/access associations and publish durable Markdown.
The two domain crates have no third-party dependencies; filesystem, JSON and
schema adapters belong to the application. There is no background service,
resource connector, tool executor or runtime query API.

## Read First

1. [Product](docs/product.md): the confirmed artifact-first product direction.
2. [Domain model](docs/domain-model.md): the two reviewed ownership boundaries;
   concrete implementation choices are recorded in the active Feature.
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
The application dependencies are recorded in the workspace lockfile. Bootstrap
once with `cargo fetch --locked`; subsequent checks/builds use the local cache.

See [003 evidence](specs/003-durable-context/acceptance.md) for actual test results,
remaining checks and platform limitations. Old foundation tests are historical.

## Try the File-Based Path

Follow the [runnable example](examples/README.md). Initialize, register documents
and service/tool descriptions, associate them, preview and explicitly synchronize.
Then follow the generated entry without keeping DevMeld running.

Commands default to read-only preview; `--apply` asks for confirmation. Sources
remain authored files. Do not edit generated output; interrupted operations have
an explicit `recover` path. No tool execution, network connection or installation
is performed by this feature.

## Workflow During Redesign

The active path is [003 Durable Context Publication](specs/003-durable-context/spec.md),
with its [plan](specs/003-durable-context/plan.md) and
[behavior tasks](specs/003-durable-context/tasks.md). Do not resume old 001/002
tasks. The Maintainer delegated continuation and self-verification of the reviewed
file-based path; implementation progress and acceptance are recorded separately.

Spec Kit's optional development helpers use the configured Python workflow;
they are not runtime dependencies of DevMeld or of its generated context.
Whether to keep using the full Spec Kit workflow remains a separate decision.

Restart and design snapshots are recorded in local Git history. Remote publishing
remains with the Maintainer.
