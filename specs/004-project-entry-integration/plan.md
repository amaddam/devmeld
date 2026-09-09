# Implementation Plan: Project Instruction Entry Integration

**Branch**: `main` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Status**: Revised design and continued implementation authorized by Maintainer on 2026-09-09; [tasks](tasks.md) and required [verification](acceptance.md) complete, Maintainer acceptance pending.
The Plan itself is not implementation evidence. The Spec Kit feature
identifier is `004-project-entry-integration`, not a newly created Git branch.

## Summary

Add an explicit `instructions` entry alongside 003's whole-file `file` entry.
DevMeld inserts a small navigation block into a selected project instruction
file; a fresh supported Agent session follows it to existing durable context.
DevMeld does not run during consumption or select resources for the session.

Correct the whole-file ownership assumption before extending publication:
long-term evidence owns either a whole file or one exact insertion, while each
operation still captures and checks the entire file. Reuse the existing staged
write/recovery mechanism, with an explicit ownership effect for each mutation.
See [research](research.md), [model](data-model.md),
[contract](contracts/shared-instructions.md) and [acceptance guide](quickstart.md).

## Technical Context

- **Language**: Stable Rust, Edition 2024; current workspace toolchain `1.98.1`.
  [ADR-0003](../../docs/adr/0003-rust-runtime.md) remains the runtime authority.
- **Dependencies**: Existing serde/serde_json, jsonschema and file-id adapters;
  no additional production or test dependency. Domain crates remain std-only.
- **Storage**: Local JSON config/receipt/journal and UTF-8 publications. All new
  contexts use one v0 maintenance baseline, including whole-file-only contexts.
  No v1 runtime path or automatic conversion; unsupported state stays intact.
- **Testing**: Cargo behavior/contract/integration tests, `cargo xtask check`,
  real CLI fixtures and one authorized fresh Agent Client session.
- **Platforms**: Native Windows and Linux (isolated WSL accepted) are required
  implementation checks; macOS remains a target but unverified until executed.
  Real Windows cross-drive IO is required separately from path-string tests.
- **Type**: Existing local CLI; no service, client SDK or new executable.
- **Limits**: Retain 003's 8 MiB file and 128 MiB operation/record bounds.
  Scan bounded host bytes linearly; no new latency or throughput claim.
- **Scope**: One common context, multiple explicitly selected hosts, at most
  one managed insertion per host. UTF-8 plain Markdown/text only. No resource
  subsets, automatic discovery, Skill installation, scheduler or remote index.

## Constitution Check

Pre-research and post-design checks both pass within the proposed contract;
these are design checks, not acceptance evidence or Constitution ratification.

| Principle | Design evidence |
| --- | --- |
| I: Context, not execution | Publish navigation; no runtime Agent call, tool execution or session routing |
| II: Grounded context | Link to common navigation and original sources; preserve ownership and conflict evidence |
| III: Local/rebuildable | Independent ordinary-file reading; local records are not portable resource facts; retain 003 link rules |
| IV: Explicit/reversible writes | Selected insertion only, preview/confirmation, full basis checks and verified operation recovery |
| V: Human inspection | Readable markers/JSON, visible attachment and unsupported-format diagnostics, no opaque registry |
| VI: Vertical value | Registration → publication → fresh-session reading; failure/recovery cases are part of the slice |

No quality-improvement or navigation-cost advantage is claimed without a
comparison. Real reading proves this feature works, not that it outperforms
another context system.

## Review Gates

- **Scope**: Accepted; do not ask to reconfirm the same Product boundary.
- **Compatibility direction**: The Maintainer accepted replacing development-era
  formats without mandatory old-version support on 2026-09-09, recorded in
  [FR-011 and the Spec assumptions](spec.md). Apply one current model rather than
  retaining dual read/write paths. This supersedes the first-attachment upgrade
  proposal; it is not permission to reset or migrate actual data.
- **Design**: The revised single-v0 contract and continued implementation were
  authorized on 2026-09-09. Do not reopen the accepted ownership/compatibility
  direction merely to begin tasks. Actual data conversion, if later
  needed, remains separate work under [CONTRIBUTING](../../CONTRIBUTING.md#review-gates).
- **Implementation acceptance**: Maintainer decision pending; the [required checks](acceptance.md)
  passed, including a separately authorized real named client and platform evidence.
  Local client availability alone is not paid-use authority or consumption evidence.

## Project Structure

```text
specs/004-project-entry-integration/
├── spec.md
├── checklists/requirements.md
├── plan.md
├── research.md
├── data-model.md
├── contracts/shared-instructions.md
└── quickstart.md

crates/
├── resources/src/lib.rs       # existing resource facts; no new selection rules
├── publication/src/lib.rs     # whole-file vs insertion maintenance policy
└── devmeld/
    ├── src/
    │   ├── main.rs            # explicit CLI options and preview presentation
    │   ├── lib.rs             # configuration/publication coordination
    │   ├── declarations.rs    # entry kind and versioned config adaptation
    │   ├── instructions.rs    # bounded byte recognition/composition
    │   ├── render.rs          # common entry meaning and existing file links
    │   ├── language.rs        # en/zh-CN generated labels; no host translation
    │   └── storage.rs         # typed claims, captured basis, staged transactions
    └── tests/                 # public workflow and isolated CLI fixtures
```

Module names are working design, not a mandatory folder inventory. No new
domain, crate, generic patch API or duplicate language/link renderer is needed.
Byte recognition is an adapter fact; authorization to create/update/detach
belongs to Context Publication, not CLI branches or Markdown templates.

## Implementation Approach

1. Establish the green 003 baseline and characterize whole-file behavior.
   Refactor ownership effects separately from actual bytes installed; do not
   disable conflict assertions merely to admit shared hosts. Reinitialize test
   fixtures through the new model; old serialized shapes are not behavior rules.
2. Replace the old maintenance representation with the single current baseline
   and add explicit instruction registration. Remove superseded decoding/writing
   paths rather than carrying compatibility branches. Keep unsupported-record
   rejection tests. Configuration commands still do not publish.
3. Deliver attachment end to end: validate targets, render the small common
   navigation entry, recognize host bytes, obtain publication authorization,
   prepare full before/after effects and apply under existing checks.
4. Extend that path to outside edits, output/language changes, no-op and
   detachment. Retire the assumption that every obsolete target is a file to
   delete, and that every installed after-image becomes whole-file evidence.
5. Verify interruptions and conflicts throughout, then actual Agent reading.
   These are not deferred hardening after shipping attachment.

Subsequent tasks follow Engineering's vertical TDD: each bounded behavior owns
its RED/GREEN/refactor cycle. Preparatory refactoring starts green. Do not create
separate domain/parser/storage phases that postpone the working path or batch
all tests before behavior. [Tasks](tasks.md) record the implementation sequence.

## Complexity Tracking

The Maintainer's 2026-09-09 clarification keeps this work in the unreleased v0
development stage. The maintenance discriminator is a draft implementation detail,
not a new public release. Design evolution does not require a version ladder or
backward-compatibility framework. See the contract for current record validation.

No Constitution exception is requested. Two ownership kinds, a fixed byte
envelope and one supported format address demonstrated assumptions in current code.
Keep the existing transaction engine; do not add Markdown AST processing,
three-way merge, cross-file atomicity, hostile-editor isolation, a global lock
service, dual-format maintenance, a migration framework or client-specific orchestration.
