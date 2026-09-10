# Implementation Plan: Context CLI and Resource Organization

**Working branch**: `main` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

The workflow helper's `BRANCH=005-context-cli` is a feature label, not the actual Git branch. No branch change, commit or push is implied.

## Summary

Deliver the discussed CLI incrementally, starting with side-effect-free command discovery. Then decouple stable identity from organization paths, add project-local bootstrap, metadata/inheritance and publication UX. Do not implement the new domain model as a string-to-old-command wrapper. Existing publisher ownership, recovery and cross-drive links remain the baseline.

## Technical Context

- Language/version: existing pinned stable Rust 1.98.1, Edition 2024; no install or upgrade.
- Dependencies: reuse current std, serde/serde_json, jsonschema and file-id. No new parser, i18n, inheritance engine or database dependency is selected.
- Storage: current local JSON configuration and owned publication records; unreleased format 0. Do not alter existing fixtures or development contexts implicitly.
- Tests: Cargo behavior tests and `cargo xtask check`; real CLI processes for interaction claims.
- Platforms: Windows first, isolated Linux verification for completed behavior; macOS unverified until run. Existing cross-drive tests remain explicit Windows cases.
- Performance: bounded existing source/record limits; help must not inspect context files. No unmeasured optimization claim.
- Scope: five stories in Spec; each completed slice can ship independently. Live help and README describe implemented syntax only.

## Constitution Check

Pre-design and post-design checks pass:

1. Context, not execution: no resource/tool execution, scheduling or network indexing.
2. Grounding: separate authored source facts from local annotations; inheritance exposes origin.
3. Local/portable: stable identity is not a drive/path; navigation remains independently readable.
4. Writes: dry-run remains available, publication retains preview/confirmation, receipts and rechecking. Bootstrap never adopts existing unknown state.
5. Inspection: help, list/show/status and generated Markdown are human-facing, not a new public machine API.
6. Slices: command discovery is independently useful; subsequent slices include real registration-to-publication outcomes, not a batch of empty layers.

The user authorized the discussed direction and starting the next step. This does not ratify the Constitution, grant final acceptance or authorize migrations/commits. No additional ADR or owning domain is needed.

## Project Structure

- `crates/devmeld/src/main.rs`: process IO and confirmation, delegating help/parsing instead of owning domain policy.
- `crates/devmeld/src/cli/`: binary-owned adapter. `mod.rs` handles invocation flags; `parse.rs` maps syntax to typed requests; `annotation_args.rs` parses annotation syntax only; `help.rs` owns layered help; `render.rs` presents application results and previews. No filesystem IO or domain mutation in parsing/rendering.
- `crates/devmeld/src/lib.rs` and `request.rs`: reusable typed application boundary. Native paths and logical addresses have distinct roles; callers supply an absolute input base and optional context selection.
- `crates/devmeld/src/application.rs`: bootstrap and registration/publication coordination; share read-only publication preparation between publish and status. No argv parsing or terminal formatting.
- `crates/devmeld/src/context.rs`: local marker selection and typed native input references; no hidden process cwd or general filesystem discovery engine.
- `crates/devmeld/src/inspection.rs`: structured read-only registration and publication views. List/show do not read sources; status reads current inputs via publication preparation and rechecks the snapshot. Neither is a new owning domain or persisted authority.
- `crates/devmeld/src/declarations.rs`: JSON adaptation, defaults, stable identity allocation and source loading.
- `crates/resources/src/organization.rs`: create when the first organization behavior needs it; logical paths, annotations and inheritance belong to Resource Organization.
- `crates/devmeld/src/render.rs` and `language.rs`: present resolved organization facts/provenance, preserve source links and EN/zh-CN wording.
- `crates/devmeld/src/storage.rs` and `crates/publication/`: reuse existing managed-write mechanisms; do not replace the transaction engine to simplify flags.
- `crates/devmeld/tests/cli.rs`: public process-level command tests.
- `crates/devmeld/tests/interaction.rs`: real executable saves, dry-run, confirmation modes, status, language and entry registration/publication boundaries.
- `crates/devmeld/tests/application.rs`: direct typed non-CLI callers, including native input bases, structured previews/results, inheritance origins and stale/no-op protection. Existing fixture helpers reuse the binary parser only under test configuration.
- `crates/resources/tests/organization.rs`: pure model rules when implemented.
- Existing `workflow.rs` and `instructions.rs`: regression gates, not evidence for unbuilt new commands.

Only files needed by completed slices are created. There are no empty groups/services/ports crates.

## Delivery and Verification

### Authorized adapter refinement (2026-09-10)

The Maintainer requested an application boundary that future GUI callers can use
without constructing CLI arguments or parsing terminal reports. Retain the two
domains, file formats and mutation/confirmation behavior. This is a refinement
of the existing modular-monolith boundary, not a new GUI, protocol or dependency.

- The binary owns CLI parsing, help, terminal confirmation and renderers.
- The library accepts typed mutation/query requests and explicit input/context
  locations. Resolve native paths by their typed role, not by rescanning argv.
- Queries return owned inspection data; prepared changes expose structured
  previews. Display strings are not the application result contract.
- Keep prepare/inspect/apply separate: apply consumes the captured plan and
  retains ownership, conflict, stale-input and recovery checks for every caller.
- Remove production string-command entrypoints. Existing fault-injection tests
  may reuse the real CLI parser through test-only fixture helpers; direct typed
  application tests must demonstrate independent callers without that helper.
- Implement the selected `docs/cli-style.md` layout without changing executable
  command syntax. No per-command trait hierarchy, generic dispatcher framework,
  empty ports or new domain crate is required.

T016-T018 track this refinement separately from the completed T001-T015 history.

1. Green baseline, then nested help RED/GREEN through the real executable. Retain current mutating syntax until its owning slice changes it.
2. Model organization addresses without changing source paths or resource identity; test collisions/moves through the pure domain boundary, then real add/show/sync.
3. Add context resolution and failure-atomic bootstrap. No filesystem creation during parsing, validation, help or preview.
4. Add metadata, defaults and computed inheritance with provenance; keep source descriptor attributes distinct from context annotations.
5. Replace CLI confirmation/status flow and update both README languages only after executable examples pass.

Record each actual failure/pass and platform in `acceptance.md`. No old test evidence is promoted to new completion. `tasks.md` is the completion authority; unchecked tasks remain future work even if this Plan describes them.

See [research](research.md), [data model](data-model.md), [CLI contract](contracts/cli.md) and [validation guide](quickstart.md).
