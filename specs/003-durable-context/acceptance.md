# 003 implementation evidence

Status: 003 implementation and required self-verification completed on 2026-09-08;
not an independently granted Maintainer acceptance or public API ratification.

## Native Windows behavior cycles (2026-09-08)

Rust/Cargo 1.98.1, native MSVC host. Focused commands use
`cargo test -p devmeld --test workflow <name> --locked --offline`.

| Cycle | Observed RED | Observed GREEN |
| --- | --- | --- |
| initialization | command unavailable | preview has zero writes; confirmation creates configuration; repeat refused |
| documents_publish | entry/registration command unavailable | real binary, Unicode/spaced links, offline files, unchanged bytes/mtime |
| publication_cannot | sync overwrote registered generated source | source overlap refused, original output preserved |
| reserved_surfaces | init accepted state as output | reserved paths and duplicate entries refused |
| interrupted_create (library test) | missing recovery API (intended E0599) | pending recovery detected; external bytes preserved; successful rollback |
| registration_entry | change commands unavailable | owned output/entry migration; source and earlier config preserved |
| descriptions_validate | description command unavailable | custom attributes rendered; invalid required attributes leave old output |
| access_guidance | access command unavailable | source/tool/declaration links, no execution, explicit association removal |
| allowed_large (library) | a 3 MiB output expanded into an unreadable receipt | compact text records, successful reload and unchanged no-op |
| partial_commit (library) | rollback stuck on incomplete commit JSON | recorded commit-file identity permits safe cleanup of a partial record |
| committed_recovery (library) | preview incorrectly promised restoration after commit | cleanup-only preview and later external edits preserved |
| help_explains | help omitted description syntax | usable command options; EOF confirmation changes no files |

Additional storage tests pass for every complete mutation boundary in a
replace/delete/receipt operation, identical external creation and cooperative lock.
These were added against the implemented journal as risk verification, not claimed
as separately observed RED cycles. Stale-preview/external-edit tests also pass.
Syntax/type errors encountered while implementing were corrected; they are not
counted as behavioral evidence.

Additional negative cases verify unsupported schema dialects/keywords, external
refs, required vocabularies, malformed schema controls, literal `$schema` data,
valid local fragment refs, hard links, lost ownership records, invalid/duplicate
IDs, unknown description fields, unsupported versions and oversized inputs.
Valid address/custom-field updates change output; invalid updates preserve it.

Recovery fault injection is private test instrumentation, not a production
command/environment hook. Adding stop points for recovery and post-commit cleanup
is harness setup, not a claimed behavioral RED by itself. Non-Unicode argument
handling and the other supplemental negative cases were verified after their
implementation; no unobserved RED is claimed.

## Dependencies

One Cargo workspace/lockfile. Domain crates remain standard-library-only.
Application: serde 1.0.229, serde_json 1.0.151, jsonschema 0.55.0 with default
features disabled, file-id 0.2.3. Cargo resolved 90 registry packages including
platform-specific dependencies. The inspected native graph has no reqwest,
tokio or HTTP retrieval feature. Schema interpretation has a significant normal
transitive dependency cost; this is not represented as a zero-dependency feature.
The developer-only xtask reuses serde_json 1.0.151 for Cargo metadata instead of
adding a TOML parser or another checker. Every member inherits workspace lints.

## Final executed gates

| Host | Command | Actual result |
| --- | --- | --- |
| Windows, native MSVC Rust/Cargo 1.98.1 | `cargo xtask check` | PASS: 8 storage + 12 real CLI/workflow + 2 boundary tests; 22 total |
| Ubuntu under WSL, Linux 6.18.33.1, existing stable Rust/Cargo 1.98.1 | `RUSTUP_TOOLCHAIN=stable cargo xtask check` | PASS: 8 storage + 14 workflow + 2 boundary tests; 24 total |
| Windows | `cargo build -p devmeld --locked --offline` and example command sequence | PASS: 3 resources, access association, optional file entry and manual sync |
| Windows | `git diff --check` | PASS |

Both full gates ran Cargo metadata boundaries, rustfmt, compiler checks,
conservative Clippy and tests (including doctest execution; zero doctests exist).
Linux adds redirected-path and non-Unicode argument cases. Windows reparse-point
rejection is implemented but no Windows junction/symlink fixture was executed.

Linux ran on its own filesystem, not the Windows-mounted checkout, at
`/home/lrns1b/project/devmeld-003-check-sADmbv`. An earlier temporary snapshot
stopped at formatting and was not counted as a pass. The initial attempt to invoke
the named WSL `1.98.1` installation triggered rustup repair of an already incomplete
toolchain and was interrupted. No completed reinstall is claimed: its manifest
remained unavailable. The already installed `stable` has the exact same verified
compiler version and supplied the final Linux checks without changing repository
toolchain settings. Downloading the reviewed Cargo lockfile dependencies was the
only completed bootstrap for that verification.

## Real example and ownership review

The checked-in source fixture contains a document, service description, access
guide, original instructions and optional attribute schema. The documented
commands were executed against `examples/team-context`; generated output is
ignored, not checked in as hand-authored expected output. All five generated
Markdown files were read after the process exited; all 14 links resolve to actual
source/configuration/generated files. Authored fixture bytes were checked before
and after and remain identical. Repeating sync reports `0 changed target(s)`.
State ends with only `lock` and `owned.json`, no pending journal/staging files.

The integration suite independently resolves generated links in its combined
document/service/tool fixture, including the original script/dependency declaration.
That script deliberately raises if executed; DevMeld only reads and links it.

Self-review retained the two-domain direction and removed duplicate association
removal logic from application coordination. Domain-source inspection found no
filesystem/process/environment/clock or serialization calls. This inspection is
not claimed to be an automatic purity proof. Real isolated Cargo fixtures prove
normal, renamed optional, build, dev and inactive-target edges are rejected; a
valid external ResourceId constructor probe compiles, while unchecked construction
fails with the intended E0603 private-constructor diagnostic.

## Limits and acceptance boundary

- macOS, automatic Agent discovery, real Agent consumption quality and Windows
  reparse fixtures remain unverified; no benchmark or effectiveness score.
- No server, scheduler, network/tool execution, installation, migration or forced
  state adoption. A copied/lost ownership record is not silently reconstructed.
- Ordinary-file entries only; users explicitly point readers at them.
- Synchronous local IO, bounded files/records, no cross-file atomic reads,
  hostile-editor isolation, power-loss or filesystem-hang guarantee.
- Pre-journal preparation failures/interruption may leave unique temporary files
  or empty directories while targets remain unchanged; no recursive orphan purge.
  Malformed/external recovery records require inspection, not guessed repairs.
- Relative file links cannot span Windows drive roots; redirected paths and
  unavailable original references fail explicitly. File metadata beyond content
  and existence is not a promised restore contract.

These limits match the feature-local contract and example instructions. All 11
tasks are complete; future product/integration choices remain separate decisions.
Local Git checkpoints preserve the design and first implementation, followed by
the hardening/verification update. No remote push was performed.
