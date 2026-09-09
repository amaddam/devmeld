# 003 implementation evidence

Status: initial 003 implementation and required self-verification completed on 2026-09-08;
output-language follow-up verified on 2026-09-09 (see below);
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
- Same-root links remain relative; cross-drive local links use file URIs and
  readable local paths (T013). Redirected paths and unavailable original
  references fail explicitly. File metadata beyond content
  and existence is not a promised restore contract.

These limits match the feature-local contract and example instructions. The original 11
tasks were completed; future product/integration choices remain separate decisions.
Local Git checkpoints preserve the design and first implementation, followed by
the hardening/verification update. No remote push was performed.

## T012 — English/Chinese generated output (2026-09-09)

The Maintainer requested selectable English/Chinese index output, with formal
developer-facing text and technical terms preserved. The follow-up is recorded
in FR-015 / SC-007 and the local-file contract; it does not change Product scope,
source ownership, tool authority or the two-domain architecture.

The native Windows baseline `cargo xtask check` passed before changes (22 tests).
Focused cycles used `cargo test -p devmeld --test workflow <filter> --locked --offline`:

| Filter | Observed RED | Observed GREEN |
| --- | --- | --- |
| `chinese_publication` | init rejected `--language` | Chinese index/page/entry, valid links, unchanged source and repeated bytes/mtime |
| `language_changes_require` | language command unavailable | preview/cancel/confirm, config-only edit, explicit sync and exact English round-trip |
| `invalid_output_languages` | Serde's default enum parser accepted object-shaped `{"en":null}` | string-only codes; invalid types/codes/duplicate fields rejected without writes |
| `help_explains` | help lacked language option | init/change syntax, separate sync step and untranslated-content boundary documented |

Additional regression coverage (not separately claimed RED cycles) verifies
legacy default/explicit English golden bytes, process-locale independence,
unchanged authored `Original document` text, ssh/http/curl/JSON, mixed-language
summaries, attribute names/values, reference labels and multiple entries. Stale
configuration/publication previews and external edits fail without overwrites.
Existing fault injection verifies interrupted language configuration recovery
and all five mutation boundaries of language republication; a failed sync keeps
the previously committed language while restoring the previous generated files.

Windows and Linux run the same production path. Final gates:

| Host | Command | Result |
| --- | --- | --- |
| Native Windows, Rust/Cargo 1.98.1 | `cargo xtask check` | PASS: 9 storage + 17 workflow + 2 boundary tests, 28 total |
| Ubuntu / WSL Linux 6.18.33.1, existing stable Rust/Cargo 1.98.1 | `cargo xtask check --root /home/lrns1b/project/devmeld-language-check-S9G0dL` with `RUSTUP_TOOLCHAIN=stable` | PASS: 9 storage + 19 workflow + 2 boundary tests, 30 total |

Linux used copied current sources and an independent target directory at
`/home/lrns1b/project/devmeld-language-check-S9G0dL`. An initial attempt to reuse
the previous snapshot's target ran an old xtask with its compiled-in workspace
root; that result was discarded, not counted as verification of this change.
The independent build then exposed a test-helper race: a no-op process exited
before the helper wrote confirmation, yielding BrokenPipe. The helper now
tolerates only that pipe closure and still checks the actual process outcome;
production confirmation behavior was not changed. Full gates and an additional
Linux workflow run passed after this correction.

No dependencies or runtime tools were added, upgraded or installed. Message
sets and Markdown presentation are application-owned; domain facts retain an
optional authored summary instead of an English presentation fallback. English
is omitted in serialized configuration for legacy compatibility; old binaries
cannot read Chinese configuration, as documented. macOS and automatic Agent
loading remain unverified. CLI diagnostics/help and authored source translation
are outside this follow-up. README/example commands describe both language paths.

## T013 — Local cross-drive links (2026-09-09)

The Maintainer limited the current path to this machine, including different
local drives. Remote addresses may be authored resource information, not remote
indexes to fetch or federate. Product records that scope; FR-016 / SC-008 and
the local-file contract own this feature's concrete path and output behavior.

Before this change, native Windows `cargo xtask check` passed all 28 T012 tests.
Observed RED/GREEN cycles, using `cargo test -p devmeld --lib <filter> --locked --offline`:

| Filter | Observed RED | Observed GREEN |
| --- | --- | --- |
| `cross_drive_links` | cross-drive input rejected with the old shared-root error | normal and canonical Windows disk paths produce correctly encoded `file:///D:/...` |
| `remote_index_inputs` | URL path produced OS error 123 instead of local-input guidance | context and resource paths reject URLs before IO, with actionable guidance |

Additional regression tests cover same-drive normal/verbatim prefixes, drive
case, escaped link labels/readable paths, and Unix filenames containing literal
backslashes. Existing English golden bytes, Chinese output, schemas, ownership,
stale previews and source protection remain green. Source data is not translated
or copied. No Cargo manifest/lockfile change, dependency installation or runtime
helper was needed; rendering and native-path checks remain application-owned.

Two opt-in tests were explicitly run on **real C: and D: local NTFS volumes**
(both reported as fixed local disks), not a substituted drive or string-only test:

```text
cargo test -p devmeld cross_drive --locked --offline -- --ignored
```

`DEVMELD_TEST_OTHER_ROOT` selected the dedicated D: test directory. Test fixtures
used unique children; no existing project or knowledge files were used as targets.

- `cross_drive_workflow_preserves_sources_and_follows_offline_links`: PASS.
  Real binary preview/confirmation/publication, D: source and entry with C:
  output, then D: output with entries on both drives. All generated Markdown
  link destinations are decoded and compared with the exact original files.
  Covers descriptions/references/configuration, Chinese/spaces/`#`/`%`/parentheses,
  both output languages, unchanged bytes/mtime, no-op, external entry conflict
  and preserved author-owned sources. Remote endpoint remains descriptive data.
- `cross_drive_relocation_recovers_at_each_mutation_boundary`: PASS.
  All eight pre-commit interruption positions (six changed output/entry targets
  plus the ownership update) restore previous bytes/identities. Previous config
  commits survive, new outputs are removed, originals are unchanged, and retry
  succeeds with a subsequent no-op. Sibling staging requires no cross-drive
  rename or hard link; the production transaction algorithm was not replaced.

Final gates:

| Host | Command | Result |
| --- | --- | --- |
| Native Windows, existing Rust/Cargo 1.98.1 | `cargo xtask check` | PASS: 14 unit + 17 workflow + 2 boundary tests, 33 total; 2 two-drive tests ignored by default |
| Native Windows, C: + D: NTFS | opt-in command above | PASS: both two-drive tests, separately executed |
| Isolated WSL Linux, existing stable Rust/Cargo 1.98.1 | `cargo xtask check --root /home/lrns1b/project/devmeld-links-check-K4Z6Lj` with `RUSTUP_TOOLCHAIN=stable` | PASS: 12 unit + 19 workflow + 2 boundary tests, 33 total |

The Linux check used copied current sources and its own target directory, not
Windows binaries. Windows drive behavior is established by Windows evidence,
not attributed to Linux. macOS, automatic Agent loading and specific Markdown
viewers' clickable `file:` support remain unverified. Native path resolution and
permissions still apply; network mappings/mounts and remote-index support are
outside this scope. A readable absolute path accompanies each file-URI link.
This is not mount provenance enforcement or a filesystem sandbox.
