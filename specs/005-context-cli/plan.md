# Implementation Plan: Context CLI and Resource Organization

**Working branch**: `main` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

The workflow helper's `BRANCH=005-context-cli` is a feature label, not the actual Git branch. No branch change, commit or push is implied.

## Summary

Deliver the discussed CLI incrementally, starting with side-effect-free command discovery. Then decouple stable identity from organization paths, add project-local bootstrap, metadata/inheritance and publication UX. Do not implement the new domain model as a string-to-old-command wrapper. Existing publisher ownership, recovery and cross-drive links remain the baseline.

## Technical Context

- Language/version: existing pinned stable Rust 1.98.1, Edition 2024; no install or upgrade.
- Dependencies: std, serde/serde_json, jsonschema, file-id and TOML records (T020); T025 adds pulldown-cmark and pulldown-cmark-to-cmark for publication. No new CLI parser, i18n engine, inheritance engine or database dependency.
- Storage: TOML configuration and owned publication records; unreleased format 0. Legacy JSON contexts remain intact and are explicitly rejected, not implicitly migrated.
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
- `crates/devmeld/src/declarations.rs`: declaration validation, defaults, stable identity allocation and authored JSON source loading.
- `crates/devmeld/src/records.rs`: TOML encoding/decoding of managed configuration and maintenance evidence.
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

### Library-based Markdown (T025)

Use `pulldown-cmark` events and `pulldown-cmark-to-cmark` only in the application
publication adapter. Build headings, paragraphs, lists, links and code spans as
structured events; remove the handwritten text escape helper. Keep native-path
URI encoding and the existing publication transaction. Reading metadata omits
origin annotations; domain calculations and CLI inspection keep them. Verify
raw field-key readability, literal text and link round trips, both languages,
preserved input bytes, no-op sync, external conflicts and recovery. This supersedes
T022/T024's reading-origin presentation, not their domain or path decisions.

### Group documents (T024)

Extend the publication destination map to include group pages and validate all
file/ancestor claims together. Replace the flat expanded index with direct-child
navigation built from existing organization membership. Render each group's
description/effective annotations using the existing metadata presenter; no
domain changes or new inheritance policy. Group pages use the current publication
writer, ownership checks and recovery. Resource destinations and card bodies stay
unchanged. Verify nested/empty groups, local links in both languages, collisions,
move/removal, source preservation, no-op, edited/unowned pages and interrupted sync;
update affected tests/docs and regenerate Shop only through the real CLI.

### Reading-oriented publication (T022)

Replace the publication renderer's maintenance-shaped metadata formatting with
a reading view of existing effective domain values. Keep the separate CLI show
renderer unchanged. Simplify card prose and localized labels, centralize full
maintenance rules at entry/index, and remove configuration links from reading
surfaces. Verify both languages, source/context authority separation, ancestor
origins without duplicate local/effective output, original files/configuration
unchanged, idempotence, conflicts and recovery. Reuse existing link and writer
code; no new dependency, domain, format or path convention.

### Readable publication paths (2026-09-10)

Add a small publication-path adapter, not a domain identity change. Compute one
ID-to-page map from current organization, with portable segment escaping and
case-folded ancestor/target collision validation, then reuse it for all generated
links. Keep logical-name policy in Resource Organization and filesystem spelling
in the adapter. Existing publication preparation withdraws obsolete owned files;
reuse its conflict/stale/recovery transaction rather than adding a migration or
recursive directory deletion. Verify first publication, resource/group moves,
same-leaf resources, Unicode/special names, target collisions, old-layout upgrade,
source preservation and interrupted publication. No dependency or format bump.

### Compact ownership receipts (2026-09-11)

The Maintainer authorized removing whole-file body copies from `owned.toml`.
Keep configuration and ownership separate. Store a SHA-256 content fingerprint
and physical identity per whole-file target; keep exact small instruction-entry
insertions. In-memory observations and temporary recovery journals/backups retain
full bytes for preview, stale checks and rollback. Fingerprints detect change,
not authority, and never authorize adoption of missing/unowned files.

Use the maintained RustCrypto [sha2 crate](https://docs.rs/sha2/0.11.0/sha2/)
in the application storage adapter; the existing dependency graph has no digest
implementation. Do not implement a hash or use an unstable standard-library hasher.
Domain crates remain std-only. A narrow reader accepts the preceding full-body
TOML claim, hashing its recorded bytes, not current disk contents. Actual writes
serialize compact claims. With otherwise unchanged publication, sync previews a
receipt-only change and uses the existing confirmation/journal/recovery path.
Reads, status, cancellation and preview never rewrite state; subsequent sync is
a no-op. V0 remains an unreleased design marker; older builds reject the new claim
shape. Existing TOML recovery journals remain usable; JSON recovery is unchanged.

### Managed TOML records (2026-09-10)

Use the Serde-compatible `toml` crate only in the application adapter for the
requested readable configuration/ownership/journal format. Keep JSON for authored
descriptions, schema and Cargo metadata. Keep the existing transaction algorithm,
exact byte evidence, physical file identities and pure-domain dependency boundary.
The existing dependency graph contained no TOML codec; one direct application
dependency supplies parsing and serialization, using the maintained
[toml crate](https://docs.rs/toml/1.1.5+spec-1.1.0/toml/). Paths can use TOML's
[literal strings](https://toml.io/en/v1.0.0#string); no escaping logic is handwritten.
TOML serialization is shared by managed records; it must not be used to rewrite
authored resource content. Block legacy JSON markers before preparing normal or
recovery operations, including mixed-format contexts, and capture their absence
so concurrent legacy creation invalidates a prepared plan. Do not add a migration
framework or change the v0 marker. Verify real registration/publication plus the
full interrupted-write/recovery suite, multiline text and Windows path spelling.

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
