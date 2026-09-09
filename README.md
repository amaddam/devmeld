# DevMeld

English | [简体中文](README.zh-CN.md)

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

### Generated-output language

Select English (`en`, the default) or Simplified Chinese (`zh-CN`) for one
context. For a new context, use `init --language zh-CN`. For an existing context:

```text
devmeld --context PATH language zh-CN --apply
devmeld --context PATH sync --apply
```

Confirm each preview with `apply`. The first command saves the language; the
second updates the index, resource pages and entries. Use `language en` to switch
back. Current-model configurations without this setting remain English, independently
of the system locale.

Only generated explanatory text is localized. Authored content, ssh/http/curl,
other technical names, commands, field names, IDs and links remain unchanged.
This is not source translation or an instruction about the Agent's reply language.
CLI help and diagnostics remain English. See the [example](examples/README.md)
for commands runnable directly from this checkout.

### Entry inside project instructions

Select an ordinary UTF-8 instruction file explicitly; DevMeld does not discover it:

```text
devmeld --context PATH entry add /path/to/project/AGENTS.md --kind instructions --apply
devmeld --context PATH sync --apply
```

For a fresh context, `init --instruction-entry PATH` registers the same entry.
Registration changes configuration only; confirmed sync inserts the small navigation
section. Surrounding author text is preserved. `entry remove PATH` followed by sync
removes only the insertion and retains the host file, even if empty. `--entry` and
default `entry add` still select an entirely generated file.

This is unreleased **v0 development**, not a second product release. The internal
draft marker is not a stable format guarantee; indexes/resources carry no release
label. Incompatible old development records are left intact and rejected, not
migrated. Use separate fresh paths rather than deleting or relabeling old records.
See [004 verification](specs/004-project-entry-integration/acceptance.md) for the
actually tested platforms and client setup; a filename alone does not prove discovery.

### Local files across directories and drives

Sources stay in their existing locations. Register an absolute native path when
needed, for example `resource add notes --document "D:/knowledge/notes.md"`.
Sources, output and ordinary entry files can be on different local Windows drives.
Generated links are relative to the containing Markdown file when roots match;
across drives they use `file:///D:/...` plus a readable local path. Spaces,
Unicode and URL-reserved characters are encoded without changing the target.

Only this machine is in scope. Remote service addresses can be authored resource
attributes; DevMeld does not fetch remote indexes or map paths between machines.
Some Markdown viewers block file links; use the displayed path with an authorized
local reader. No running DevMeld process is required, and no universal viewer or
cross-machine portability is claimed. See the [example](examples/README.md).

## Workflow During Redesign

The active path is [003 Durable Context Publication](specs/003-durable-context/spec.md),
with its [plan](specs/003-durable-context/plan.md) and
[behavior tasks](specs/003-durable-context/tasks.md). Do not resume old 001/002
tasks. The Maintainer delegated continuation and self-verification of the reviewed
file-based path; implementation progress and acceptance are recorded separately.

The active extension is [004 Project Instruction Entry Integration](specs/004-project-entry-integration/spec.md):
maintain a small entry inside a selected project instruction file while preserving
its authored content. Scope was accepted on 2026-09-09; the [Plan](specs/004-project-entry-integration/plan.md)
uses a single current draft maintenance model. Implementation and verification
are recorded in its [tasks](specs/004-project-entry-integration/tasks.md) and
[evidence](specs/004-project-entry-integration/acceptance.md). It does not automatically migrate existing data.
Spec Kit's current feature pointer selects 004.

Spec Kit's optional development helpers use the configured Python workflow;
they are not runtime dependencies of DevMeld or of its generated context.
Whether to keep using the full Spec Kit workflow remains a separate decision.

Restart and design snapshots are recorded in local Git history. Remote publishing
remains with the Maintainer.
