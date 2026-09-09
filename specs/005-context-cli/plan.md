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
- `crates/devmeld/src/cli.rs`: implemented command/help definitions and later typed argument parsing. Add only definitions actually served by the current implementation.
- `crates/devmeld/src/annotation_args.rs`: parse shared annotation options, shortcuts and explicit inheritance switches into a domain-owned local edit and choices; no update/merge policy or filesystem resolution in this parser.
- `crates/devmeld/src/lib.rs`: bootstrap and registration/publication coordination; refactor existing argument matching when the typed commands replace it.
- `crates/devmeld/src/context.rs`: local marker selection and cwd-to-context native input references, shared by the CLI application path; no domain or filesystem discovery engine.
- `crates/devmeld/src/inspection.rs`: read-only presentation of saved group/resource declarations, inheritance choices/derived origins and identity-based associations; no new owning domain, source scan or publication status authority.
- `crates/devmeld/src/declarations.rs`: JSON adaptation, defaults, stable identity allocation and source loading.
- `crates/resources/src/organization.rs`: create when the first organization behavior needs it; logical paths, annotations and inheritance belong to Resource Organization.
- `crates/devmeld/src/render.rs` and `language.rs`: present resolved organization facts/provenance, preserve source links and EN/zh-CN wording.
- `crates/devmeld/src/storage.rs` and `crates/publication/`: reuse existing managed-write mechanisms; do not replace the transaction engine to simplify flags.
- `crates/devmeld/tests/cli.rs`: public process-level command tests.
- `crates/resources/tests/organization.rs`: pure model rules when implemented.
- Existing `workflow.rs` and `instructions.rs`: regression gates, not evidence for unbuilt new commands.

Only files needed by completed slices are created. There are no empty groups/services/ports crates.

## Delivery and Verification

1. Green baseline, then nested help RED/GREEN through the real executable. Retain current mutating syntax until its owning slice changes it.
2. Model organization addresses without changing source paths or resource identity; test collisions/moves through the pure domain boundary, then real add/show/sync.
3. Add context resolution and failure-atomic bootstrap. No filesystem creation during parsing, validation, help or preview.
4. Add metadata, defaults and computed inheritance with provenance; keep source descriptor attributes distinct from context annotations.
5. Replace CLI confirmation/status flow and update both README languages only after executable examples pass.

Record each actual failure/pass and platform in `acceptance.md`. No old test evidence is promoted to new completion. `tasks.md` is the completion authority; unchecked tasks remain future work even if this Plan describes them.

See [research](research.md), [data model](data-model.md), [CLI contract](contracts/cli.md) and [validation guide](quickstart.md).
