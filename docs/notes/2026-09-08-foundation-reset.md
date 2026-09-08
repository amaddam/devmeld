# Foundation reset — 2026-09-08

## Decision and scope

The Maintainer authorized removing the old 001 design, its four crates and tests,
and the old 002 design, keeping the engineering scaffold and the current design discussion.
Product/domain design restarts from durable context artifacts and Agent-owned consumption,
not from preserving the former domain map. No new product implementation is authorized by
the domain proposal alone.

Removed:

- 31 files under the former shared-kernel, catalog, local-context and knowledge crates.
- 11 files under the former 001-foundation-context design directory.
- 9 files under the former 002-local-context-integration design directory.
- The old architecture checker and its probe suite, which encoded the retired model/layout.
- The local Spec Kit active-feature pointer to the removed 002 directory.

The Cargo workspace now contains only tools/xtask. Its unused serde_json dependency
and dependency closure were removed through an offline Cargo lockfile regeneration.
The native fmt/check/clippy/test entrypoint remains; no replacement product crate was created.

## Retained and recoverability

Git history was not reset. The pre-reset HEAD was
`0003a418fd73d4b982dbbeec7288fdf98ab22109`.
Previously committed versions of the old 001 files remain recoverable from Git.
Uncommitted edits to those files and the untracked 002 documents are not recoverable
from existing commits. No extra backup was created.

The Rust toolchain pin, conservative lint settings, Cargo alias, general engineering
conventions, Constitution and historical ADR records remain. ADR annotations distinguish
retained general decisions from the now-retired foundation layout.
Existing build caches were left alone; any binaries there are not a current product deliverable.
Removing the now-empty retired directories was rejected by the execution policy.
They were left in place; the listed files are gone, and no retired crate remains
in Cargo metadata. Empty directories are not tracked by Git.

The [discussion record](2026-09-08-context-generation-and-consumption.md) is preserved.
[Product](../product.md) now records the confirmed intent; the new
[domain proposal](../domain-model.md) is for review, not an approved package inventory.

## Spec Kit assessment

This reset does not uninstall Spec Kit or install a replacement framework.
There is no active Feature and no new spec/plan/tasks generation during domain discovery.
Whether to use the full workflow for later Features remains a Maintainer choice.
Retained workflow templates include illustrative paths from the old model;
those examples are not instructions to recreate removed code. They were not
regenerated or repurposed as a new domain framework during this reset.

The [official workflow](https://github.com/github/spec-kit#-what-is-spec-driven-development)
organizes specification, planning, task breakdown, implementation and convergence.
It also supports customization and an optional idea-assessment extension; the tool is
not intrinsically limited to one rigid process.

The local specify skill starts from a feature description and requires feature-oriented
scenarios and measurable outcomes. The plan skill then generates research, model and
validation artifacts. The analyze skill checks cross-artifact consistency and coverage.
Those checks are useful, but do not independently establish that the product model is right.

The local task template already discourages invented layers and batches of tests before
implementation. The reset therefore is not evidence that Spec Kit inherently forbids DDD
or forces overengineering. Our use of it carried an unvalidated product interpretation
through mutually consistent documents, code and tests. That modeling error remains ours.

Recommendation: keep product clarification and domain exploration outside automatic feature
generation for now. Review the product, domain proposal and concrete examples first.
Then use only the needed specification, tests and delivery records for an approved Feature;
the existing workflow tool can be reconsidered then, without adding new tooling now.

## Verification

- `cargo generate-lockfile --offline`: passed; remaining workspace has no third-party dependencies.
- `cargo xtask check`: passed on native Windows after the reset: fmt, check, Clippy and test.
- Cargo tests report zero tests in the remaining developer tool; there are no product tests.
- `cargo metadata --no-deps --locked --offline`: only xtask remains, with no dependencies.
- `cargo xtask --help`: passed; the retired architecture command and an unknown command both exit nonzero.
- `git diff --check` and 41 local documentation links: passed; zero source/design files remain under the old crates/specs roots.
- This is scaffold verification, not acceptance of any new DevMeld behavior.
- Linux/WSL and macOS were not run for this reset. Earlier platform passes do not transfer.
- At this initial reset-verification stage, no commit, push, new task, toolchain
  install or dependency installation was performed. Subsequent local design
  checkpoints are recorded in Git; this line describes the reset operation only.
