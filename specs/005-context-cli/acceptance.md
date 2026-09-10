# 005 Implementation Evidence

Date: 2026-09-09. This record retains incremental evidence; the final T012-T015 section summarizes complete implementation verification, not Maintainer acceptance.

## Baseline (T001)

- Actual Git branch: `main`; pre-existing edits in `README.md` and `README.zh-CN.md` preserved.
- Native Windows, existing pinned Rust; `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS.
- 54 tests passed, 3 Windows cross-drive tests explicitly ignored in the default command. Those skips are not new cross-drive verification.
- No dependency/tool installation, user context migration, commit or push.

## Current implementation status

- US1 command help: implemented and verified on native Windows (T002-T004).
- Source-first registration, distinct logical addresses/stable IDs, automatic parent groups, no-init context selection and read-only bootstrap previews: implemented (T005-T007).
- Group creation/empty removal and group/resource inspection/logical moves: implemented (T008).
- Local descriptions/tags/custom fields, shortcut equivalence, add/update/explicit removal, inspection and EN/zh-CN publication: implemented (T009).
- Metadata inheritance, configurable defaults and origin presentation: implemented and verified (T010-T011).
- Scoped saves, dry-run, one sync/recover confirmation, explicit `--yes`, status, config language and entry attach/create/remove: implemented and verified (T012-T013). Superseded `--apply` / apply-word interaction has been removed.
- Bilingual documentation/example replay, native Windows and isolated Linux checks, real terminal interaction and actual C:/D: verification completed (T014-T015). All tasks are complete; final Maintainer acceptance, macOS and additional client consumption remain unverified or pending as stated below.

The chronological sections below retain actual RED/GREEN and completed-slice evidence, including historical intermediate limitations. `tasks.md` records actual completion; planning artifacts alone are not evidence that commands exist.

## Command discovery RED/GREEN (T002-T003)

Each focused command used `cargo test --locked --offline -p devmeld --test cli <test-name>` through the actual binary.

| Test | Observed RED | Observed GREEN |
| --- | --- | --- |
| `resource_add_help_needs_no_context_and_creates_nothing` | exit 2, required `--context` before help | specific current resource-add help, empty fixture unchanged |
| `command_groups_and_operations_have_specific_current_help` | resource-group help required `--context` | 13 group/operation cases pass; no unbuilt `--as`/`--inherit` advertised |
| `help_ignores_unavailable_context_and_supports_short_form` | attempted to canonicalize nonexistent context | root/group/operation help with explicit nonexistent path and both help flags passes |
| `unknown_help_topics_fail_before_context_access_without_writing` | filesystem error instead of help-topic diagnostic | unknown root/subcommand/future group produces a useful error without accessing context |

Additional regression checks passed on first execution: root/no-argument/short-help equivalence, malformed selectors, and help against corrupt configuration without content/timestamp changes. No RED is claimed for those already-satisfied cases.

## Slice verification (T004)

- `cargo test --locked --offline -p devmeld --test cli`: 6 passed; native process tests with closed stdin and disposable directories.
- `cargo xtask check`: PASS on Windows; 60 passed, 3 explicitly ignored cross-drive tests. Formatting, compiler, conservative Clippy and architecture probes passed.
- `cargo build --release --locked --offline -p devmeld`: PASS; no new dependencies. Existing release binary rebuilt from current source.
- README help examples verified against the executable; both languages retain the existing, actually supported mutation journey. Future commands are only in 005 planning documents.
- Feature Markdown links and `git diff --check`: PASS.
- `.specify/extensions.yml` absent; before/after workflow hooks skipped. Spec quality checklist 16/16; 15 generated tasks (baseline 1, US1 3, US2 3, US3 2, US4 2, US5 2, final 2). T001-T004 complete; T005-T015 remain unbuilt.
- No migration, source/resource changes, entry edits, commit, push or paid Agent session. Linux/macOS and fresh cross-drive execution remain unverified for this slice.

## Next code step after US1 (historical checkpoint)

T005: separate stable resource identity from logical organization path and source location, and prove add/source-to-navigation with the new address. T006-T007 then provide safe default-context selection and no-init first use. Metadata, both configurable inheritance defaults and publication UX follow their checked-in tasks; do not treat this help delivery as those features already working.

## Registration and first use (T005-T007, 2026-09-09)

Baseline at the start of this continuation: `cargo xtask check` PASS on Windows, 60 passed and 3 explicit cross-drive skips.

| Test/boundary | Observed RED | Observed GREEN |
| --- | --- | --- |
| Pure `organization_addresses_are_separate_from_identity_and_allow_equal_leaf_names` | missing accepted organization API (`E0432`) | stable IDs, same leaf in different groups, failure-atomic collisions |
| Pure `restoring_organization_rejects_missing_parents_and_collisions` | missing accepted `from_parts` (`E0599`) | restored declarations reject missing parents and collisions |
| CLI `source_first_registration_publishes_organized_addresses_and_stable_associations` | old parser expected `--document`/`--description` | source + `--as`, missing groups, stable association IDs, deterministic links/no-op sync |
| CLI `first_resource_add_and_sync_need_neither_init_nor_context_selector` | required `--context PATH COMMAND` | two management commands establish configuration then navigation, no implicit entry |
| CLI `descendant_invocations_use_nearest_context_but_resolve_sources_from_cwd` | linked the root's same-name source rather than the invoking child's source | nearest context reused while source/schema/entry/output operands resolve from cwd |
| CLI `first_use_dry_run_does_not_create_even_the_requested_root` | `--dry-run` rejected as a registration option | preview succeeds without even creating the requested root |
| CLI `a_new_marker_after_bootstrap_preview_is_not_adopted` | bootstrap adopted a marker appearing after preview | atomic fresh-marker creation rejects it before writing state/configuration |

Additional regression cases passed without an invented RED: Unicode/default filename addresses, identity allocation skips old IDs and does not recycle removals, invalid-source/collision failure atomicity, explicit new location precedence, flat owned v0 record reads without configuration rewrite, corrupt/incomplete/unowned/pending marker rejection, Windows denied-read handling, stale bootstrap source rejection and cwd-relative schema/entry/output paths. Description references remain source-directory-relative. Entry detachment restores exact authored bytes.

Two full-suite findings were resolved before this checkpoint: the first init guard also blocked the existing explicit retry after completed rollback; that route was restored while inferred incomplete markers still block automatic initialization. A newly added host-path test initially assumed insertion at the end; it was corrected to the existing prepended-insertion contract and strengthened with exact-byte detachment verification. Original recovery and source-protection assertions remain in place.

### Verification

- Windows `cargo xtask check`: PASS, 76 tests passed, 3 cross-drive cases skipped in the ordinary gate; formatting, compiler, conservative Clippy and architecture probes pass.
- Separate `cargo test --locked --offline -p devmeld cross_drive_ -- --ignored`: PASS, all 3 cases on actual C:/D: volumes. Scratch parent: `D:\devmeld-cli-005-2fcf63bb9be64d2eadba0197131c9a09`; fixture-owned children cleaned by tests. No user data was deleted.
- Linux `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, 75 tests, 0 ignored; isolated native WSL source/build copy at `/home/lrns1b/project/devmeld-cli-005.dgBVwS`. Windows-specific cases are not presented as Linux evidence.
- Existing dependencies and toolchain reused offline. No runtime installer, daemon, executable resource capability, new owning domain, user/example context migration, commit, push or paid Agent session.
- `cargo build --release --locked --offline -p devmeld`: PASS. The updated service/tool/schema/association/entry examples and EN -> zh-CN -> EN publication passed using that executable in `C:\Users\lrns1\AppData\Local\Temp\devmeld-cli-docs-6e885ad6e94a4a3ba7a9aaad2498e525`, copying authored fixtures only; the repository's existing example context was not touched. Both README languages now show supported source-first commands and explicitly retain the interim confirmation limitation.
- T001-T007 complete. This is an independently verified registration slice, not full 005 delivery or Maintainer acceptance. T008 starts group/resource inspection and logical moves; metadata, both inheritance defaults, the simplified interaction and final complete-journey acceptance still follow.

## Organization inspection and moves (T008, 2026-09-09)

Baseline at the start of this continuation: Windows `cargo xtask check` PASS, 76 passed and 3 explicit cross-drive skips. Earlier uncommitted changes were preserved.

| Test/boundary | Observed RED | Observed GREEN |
| --- | --- | --- |
| Pure `moving_a_resource_preserves_identity_and_rejects_collisions_atomically` | missing accepted `move_resource` API (`E0599`) | readdressing retains ID; collision/missing-source failures leave organization unchanged |
| Pure `groups_have_an_explicit_lifecycle_without_recursive_removal` | missing accepted group lifecycle APIs (`E0599`) | explicit creation with parents, duplicate rejection, empty-only removal |
| Pure `moving_a_group_readdresses_exactly_its_subtree_and_retains_all_identities` | missing accepted `move_group` API (`E0599`) | subtree and empty groups move together; complete-segment boundaries, collisions and self-descendant rejection |
| CLI `resource_show_reports_registration_without_reading_or_rewriting_source` | unsupported inspection command | source references/identity remain inspectable even when the source is missing, without config/source writes |
| CLI `organization_lists_use_logical_subtrees_and_show_direct_group_children` | unsupported list command | sorted scoped lists, direct-child group show, wrong-kind/unknown-scope errors |
| CLI `logical_resource_move_retains_source_identity_associations_and_published_page_location` | unsupported resource move | registration address changes; ID/source/schema/associations/counter and page location stay stable; publication waits for sync |
| CLI `groups_can_start_a_context_and_only_empty_groups_can_be_removed` | unsupported group add | confirmed first group bootstraps config; dry-run creates nothing; nonempty remove rejected |
| CLI `group_move_updates_descendant_navigation_and_incoming_associations_only_after_sync` | unsupported group move | subtree/empty-group addresses update, outside siblings stay unchanged, navigation/association labels change only after sync |
| CLI `organization_help_describes_real_operations_and_read_only_queries` | unknown group help topic | implemented group/resource operation help works without context; read-only help advertises no apply confirmation |

Additional regressions passed on their first execution; no RED is claimed: rejected/preview/no-op moves preserve configuration bytes and timestamps, changed configuration invalidates an earlier preview, associated resources cannot be removed implicitly, and inspection never bootstraps or bypasses ownership. Separate cross-drive logical-move coverage passed on the existing native-path implementation without adding another path resolver.

### Verification and boundaries

- Windows `cargo xtask check`: PASS, 87 tests passed, 4 cross-drive cases explicitly ignored in the ordinary gate. Formatting, compiler, conservative Clippy and architecture checks passed.
- Separate `cargo test --locked --offline -p devmeld cross_drive_ -- --ignored`: PASS, all 4 tests on actual C:/D: volumes, including `cross_drive_logical_moves_preserve_native_sources_and_link_destinations`. Scratch parent: `D:\devmeld-organize-005-4eb9cbba26e24934ad4afdc285780267`; fixture-owned children cleaned by tests. Source bytes/timestamps and file-URI destinations remain unchanged by logical moves.
- Linux `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, 86 tests, 0 ignored, in isolated WSL source/build copy `/home/lrns1b/project/devmeld-organize-005.9aMA1C`. This is Linux execution, not a claim that Windows-only tests ran there.
- `cargo build --release --locked --offline -p devmeld`: PASS. The newly documented quick-start-to-organization journey in both README languages ran using that executable in `C:\Users\lrns1\AppData\Local\Temp\devmeld-organize-docs-62eac3332f3c48adbaa682b133e1b5b3`, copying an authored fixture only. Read-only commands, exact moves, empty removal, sync and dry-run passed; source bytes/timestamp, identity and generated resource-page name were preserved.
- Domain review: logical paths and mutation rules remain in the std-only Resource Organization model. Read-only inspection is application presentation, not a new domain. Mutations reuse the existing configuration ownership/preview/recheck/recovery mechanism; no filesystem source moves, new persisted shape, dependency or native-path implementation were added for T008.
- Both READMEs, plan file ownership, CLI contract and this validation guide now distinguish implemented organization operations from unbuilt annotations/inheritance and confirmation UX. No user/example context migration, project entry edit, dependency/tool install, commit, push or paid Agent session.
- Markdown local links (25 checked) and `git diff --check`: PASS. `.specify/extensions.yml` absent; post-implementation hooks skipped. The specification checklist was read-only and remained 16/16.
- T001-T008 complete; T009-T015 remain open. This is incremental delivery, not complete US3/005 or Maintainer acceptance. macOS and additional Agent Client acceptance remain unverified.

## Local descriptions, tags and fields (T009, 2026-09-09)

Baseline: native Windows `cargo xtask check` PASS, 87 passed and 4 explicit cross-drive skips. Earlier uncommitted edits were preserved. The existing spec-quality checklist was read-only, 16/16 satisfied.

Focused pure commands used `cargo test --locked --offline -p devmeld-resources --test organization <test-name>`; command behavior used `cargo test --locked --offline -p devmeld --test cli <test-name>` against the real executable.

| Test/boundary | Observed RED | Observed GREEN |
| --- | --- | --- |
| `local_annotations_keep_description_tags_and_extensible_fields_distinct` | missing accepted `LocalAnnotations` (`E0432`) | distinct description, unique tags, extensible text fields; duplicate keys/invalid text rejected |
| `annotations_belong_to_nodes_and_follow_moves_without_implicit_inheritance` | missing accepted node annotation API (`E0599`) | metadata owned by the actual group/resource, retained through moves, no implicit parent inheritance |
| `group_annotations_are_saved_inspected_and_published_as_context_information` | group add rejected annotation arguments | first group add, owned JSON persistence, read-only show and generated index display local annotations; implicit parents stay empty |
| `resource_annotations_keep_source_attributes_separate_in_both_output_languages` | resource add rejected annotation arguments | source descriptor/schema plus annotation switches, cwd-relative operands after boolean switches, separately labeled source/annotation values in both languages; source bytes/time and no-op output preserved |
| `annotation_updates_preserve_unspecified_values_and_support_explicit_removal` | update command unsupported | group/resource update, omission preservation, selected removal and same-value no-op; stable resource identity/source |
| `annotation_help_explains_add_update_and_only_implemented_flags_without_context` | existing add help lacked annotation syntax | both command kinds explain add/update, equivalent shortcuts and explicit removal without context access; inheritance not advertised |

Additional regression tests passed on first execution; no RED is claimed: `annotation_shortcuts_and_fields_are_equivalent_and_conflicting_edits_never_write`, `annotated_nodes_retain_metadata_and_associations_across_saved_moves`, and the extended cross-drive logical-move test. These cover shortcut equivalence, duplicate/conflicting assignments, invalid first-use with no bootstrap, wrong-kind/unknown targets, forbidden source-option updates, dry-run, stale previews, saved moves/removals, locality and stable incoming/outgoing associations.

### Model and boundary review

- Groups now own their annotations; organized resources own stable identity plus annotations. There is no parallel annotation registry. Persistence adaptation retains unannotated v0 group strings and stores annotated nodes as documented in the data model; reading does not rewrite data and no real user/example context was migrated.
- On a green full-suite baseline (95 tests), the explicit update/omission/removal policy was moved from the argument adapter into the pure Resource Organization `AnnotationEdit`. The adapter only parses arguments/shortcuts. Existing behavioral tests stayed green; `annotation_edits_preserve_omissions_and_reject_contradictions_in_the_domain` adds direct characterization of this domain boundary, without a manufactured RED.
- Local fields remain descriptive strings; `shared` is not a permission and does not merge with a tag of the same name. Source attributes and local context annotations are presented separately. No inheritance/default flags, source rewrites, tool execution, new dependency, third domain or generic extension engine were introduced.
- The existing ownership/preview/recheck/recovery mechanism is reused. The native operand adapter recognizes zero-argument annotation switches so they cannot shift a following schema path into the wrong scope. Formatting/escaping remains presentation, not domain policy.

### Verification

- Windows `cargo xtask check`: PASS, 96 tests passed and 4 cross-drive cases ignored in the ordinary gate. Compiler, formatting, conservative Clippy and architecture probes passed.
- Isolated WSL Linux `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, 95 tests, 0 ignored, source/build copy `/home/lrns1b/project/devmeld-annotations-005.5RJjv8`. Windows-only checks are not reported as Linux evidence.
- Actual C:/D: `cargo test --locked --offline -p devmeld cross_drive_ -- --ignored`: PASS, all 4 cases. The logical-move case now includes annotations and an annotation update, retaining the original cross-drive file URI and source bytes/timestamps. Scratch parent `D:\devmeld-annotations-005-d8a6a840c5624bac9f62c4aaf3f46e4d`; test-owned children were cleaned by the tests.
- `cargo build --release --locked --offline -p devmeld`: PASS. New EN/zh-CN README command examples and explicit annotation-removal commands passed using that executable in `C:\Users\lrns1\AppData\Local\Temp\devmeld-annotations-docs-6350e7b0443d4e939ccdf02f989eaa51`. Only an authored fixture was copied; existing examples/user contexts were untouched.
- No dependency/tool installation, paid Agent session, commit or push. macOS and additional client acceptance remain unverified.
- T001-T009 complete. US3's local organization/annotation behaviors are implemented; T010-T015 and final Feature/Maintainer acceptance remain open.
- Final documentation checks: 25 local Markdown links and `git diff --check` passed. `.specify/extensions.yml` absent; post-implementation hooks skipped.

## Explicit inheritance (T010-T011, 2026-09-09)

Baseline: native Windows `cargo xtask check` PASS, 96 passed and 4 explicit cross-drive skips. Earlier uncommitted work was preserved. The existing specification checklist was read-only and remained 16/16.

| Test/boundary | Observed RED | Observed GREEN |
| --- | --- | --- |
| Pure `effective_annotations_require_both_parent_propagation_and_child_inheritance` | missing accepted inheritance APIs (`E0599`) | all four edge combinations, field provenance, local values/identity preserved, missing-node rejection |
| Pure `creation_defaults_are_captured_for_new_nodes_and_never_retroactive` | missing accepted `InheritanceDefaults` / `set_defaults` (`E0432` / `E0599`) | creation-time capture, explicit override, no retroactive changes, moves retain choices and new parents capture defaults |
| CLI `inheritance_defaults_and_explicit_choices_are_saved_only_at_creation` | `config set defaults.inherit true` unsupported | both configuration defaults, explicit target switches, implicit parents, persistence/reload and omission preservation |
| CLI `inherited_navigation_is_derived_localized_and_separate_from_source_attributes` | resource show omitted inherited fields and origins | English/Chinese show/navigation, duplicate tag origins, parent changes, empty local override, no copied declarations or source writes |
| Extended annotation help test | add/update help did not advertise the newly implemented `--inherit` | supported inheritance flags shown on the proper command kinds without context access; future confirmation flags still excluded |

Focused commands used `cargo test --locked --offline -p devmeld-resources --test organization <test-name>` and `cargo test --locked --offline -p devmeld --test cli <test-name>`. The owned legacy fixture used `cargo test --locked --offline -p devmeld --lib legacy_inheritance`.

Additional characterization/regression cases passed on first execution; no RED is claimed: pure three-level origin/override/break behavior, CLI edge combinations and moves, invalid/duplicate/opposite switches, defaults-only output no-op, removal exposing an inherited field, and stale creation/publication/no-op previews. Descriptive fields named `inherit`/`propagate` do not act as controls. The extended schema case verifies cwd-relative schema resolution after an inheritance switch. The cross-drive move case now verifies inherited origin changes while retaining its native source/link checks.

### Model and boundary review

- Saved group/resource choices and creation defaults belong to the std-only Resource Organization model. Effective tags retain every contributing origin; each field retains the nearest local value/origin. The calculation walks parent edges without recursion or a child-value cache. Overall descriptions, identities, source paths and authority never enter that derived view.
- Whole nodes are readdressed on moves instead of reconstructing a growing list of fields. This keeps annotations/choices with their owning node, preserves stable identities, and creates only actual missing destination parents. No generic inheritance engine, new owning domain, dependency or blanket implementation restriction was added.
- The data-model record defined this v0 evolution before persistence implementation. New groups use object records with explicit booleans; resources save their receipt choice. Plain group strings and absent old choices retain fixed false/true semantics, independently of current defaults. The owned legacy test proves reads, previews, no-op updates and sync do not normalize configuration. Defaults-only updates preserve existing group/resource JSON values.
- Generated output now includes saved choices and relevant derived provenance. Earlier exact group-shape and generated-byte assertions were updated for these intentional changes; stable ID/source/association, deterministic bytes, legacy configuration preservation and ownership assertions remain. No user/example context was migrated and no new release/version label was introduced.
- Parsing/persistence stay in the application adapters; presentation shares the domain calculation across show and generated files. Source-declared attributes remain separate. Defaults appear in CLI inspection as future-node policy, not in generated content, so changing only defaults does not rewrite unchanged publication.
- The existing transaction/receipt/recheck/recovery engine is reused without new mutation behavior; the storage change in this continuation is legacy-fixture verification. No source scan is added to show, and no tool installation/execution, scheduler, network indexing or automatic project entry was introduced.

### Verification

- Windows `cargo xtask check`: PASS, 104 tests passed and 4 cross-drive cases explicitly ignored in the ordinary gate. Formatting, compiler, conservative Clippy and architecture probes passed; the gate was repeated after final code/help cleanup.
- Isolated WSL Linux `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, 103 tests, 0 ignored, source/build copy `/home/lrns1b/project/devmeld-inheritance-005.LZgq6c`. Windows-specific tests are not claimed as Linux evidence.
- Actual C:/D: `cargo test --locked --offline -p devmeld cross_drive_ -- --ignored`: PASS, all 4 cases. Scratch parent `D:\devmeld-inheritance-005-1e49102cc6014ea69e23cbc7c1d98d66`; fixture-owned children cleaned by tests. Cross-drive source bytes/timestamps and file-URI destinations remain intact while logical ancestry/origins change.
- `cargo build --release --locked --offline -p devmeld`: PASS. Both READMEs' new inheritance examples ran using that executable in `C:\Users\lrns1\AppData\Local\Temp\devmeld-inheritance-docs-e5df7228a9ad40fb914a991d5d49f319`. Separate en/zh-CN fixtures verified source-first add, explicit inheritance, show/origins, sync, both defaults, explicit group overrides and defaults-only no-op output. Source bytes/timestamps remained unchanged; only an authored example was copied.
- No tool/dependency installation, paid Agent session, commit or push. macOS and additional client acceptance remain unverified.
- T001-T011 complete. US4 is implemented and verified; T012-T015, complete 005 delivery and Maintainer acceptance remain open. Next: simplify scoped saves/publication confirmation and add honest status, preserving managed boundaries.
- Final document checks: 24 local links in both READMEs and the 005 spec/plan/tasks/contract/model/quickstart/acceptance set passed; `git diff --check` passed. `.specify/extensions.yml` is absent, so post-implementation hooks were skipped.

## Save, inspect and publish (T012-T015, 2026-09-09)

Baseline: local commit `9dadac8`, clean worktree, native `cargo xtask check` PASS (104 passed, 4 explicit cross-drive skips). The specification checklist remained read-only, 16/16 satisfied. No new commit or push was requested in this continuation.

Focused behavior commands used `cargo test --locked --offline -p devmeld --test interaction <test-name>` against the real executable. Cases were implemented one at a time; the following RED failures were actually observed before their corresponding changes.

| Case | Observed RED | Observed GREEN |
| --- | --- | --- |
| Scoped save / dry-run bootstrap | no-flag resource add only previewed; configuration absent | direct save, dry-run leaves no context, source unchanged and no implicit entry/publication |
| Noninteractive publication / no-op | no-flag sync incorrectly succeeded as preview | changes require terminal confirmation or explicit `--yes`; no-op rechecks without prompt/rewrite; contradictory/removed flags rejected |
| Read-only publication status | unsupported status command | pending/current generation, saved configuration and entries distinguished; missing source blocks verification without writes |
| Config language | `config set language zh-CN` rejected by boolean-default parser | language saves without publishing, old `language VALUE` rejected, sync produces Chinese wording |
| Entry attach/create | entry operation expected add/remove | explicit insertion/whole-file registration; sync publishes; detach restores authored host bytes and retains independent ordinary entry |
| Optional init path | init treated the positional path as an invalid option | explicit native location, no-write preview, no implicit entry; duplicate context selectors and existing state rejected |
| Root help cleanup | root help still advertised removed `--apply` | save semantics and sync/recover confirmation flags match implementation, without creating context |

Additional regression/characterization coverage passed on first execution; no RED is claimed: actual modified-output conflicts cannot be overridden by `--yes`; status/preview preserve bytes/receipts; linked-document body changes are not represented as historical source-freshness tracking; empty recovery does not prompt. The existing interruption loop now also checks status is blocked and preserves the pending journal before successful rollback. The private terminal-confirmation test covers y/YES, n/blank/EOF/old apply-word and verifies nonterminal input is not read.

### Ownership and refactor review

- This is application interaction work. No change to either std-only owning domain, manifests, lockfile, toolchain, generated record version or transaction algorithm. No additional dependency or runtime service.
- The argument adapter handles one terminal flag and explicit context selection; optional init PATH uses the same existing explicit-context path. Removed human syntax has no permanent compatibility wrapper. Main flushes previews before saves and before its one confirmation; even empty plans still run apply/recheck.
- Status and sync share publication preparation. Status never applies it: current sources and owned files are inspected/rechecked; conflicts, missing inputs and pending recovery are errors. No dirty flag, timestamp, source hash registry or persisted snapshot was added. Generated-content equivalence and actual Agent consumption are explicitly distinct.
- Entry attach/create retain existing insertion/whole-file ownership. Configuration saves do not publish or modify source/host files. Exact BOM, mixed-newline, no-final-newline, independent-entry, stale/no-op, cross-drive, interruption and recovery assertions remain in the regression suites.
- Existing test helpers were updated from pipe-based apply-word input to direct scoped saves, explicit dry-run and `sync/recover --yes`. Old syntax rejection assertions were updated deliberately; no ownership/source/recovery assertions were dropped. One remaining pure-plan language invocation and one pending-recovery assertion were migrated so they test the accepted command rather than accidentally failing on removed syntax.
- Both READMEs, examples, CLI contract, model, research and validation guide describe the implemented journey. Research labels its old-command evidence as a historical baseline. No user/example context was migrated or edited; only disposable copies of authored fixtures were used.

### Verification

- Final Windows `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, **113 passed**, 4 cross-drive cases explicitly ignored in the ordinary gate. Formatting, compiler, conservative Clippy and architecture probes passed.
- Final isolated WSL Linux `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, **112 passed**, 0 ignored, source/build copy `/home/lrns1b/project/devmeld-publication-final-005.cManal`. Windows-only checks are not claimed as Linux evidence.
- Actual C:/D: `cargo test --locked --offline -p devmeld cross_drive_ -- --ignored`: PASS, **all 4 cases**, covering relocation/recovery, organization/inheritance, instruction entries and offline source links. Scratch parent `D:\devmeld-publication-cli-005-c0fcb7cf0ade4502b4e452da583140cc`; test-owned children cleaned by the tests. The later root-help-only fix does not change these paths or behaviors.
- `cargo build --release --locked --offline -p devmeld`: PASS. Both README languages (10 fenced CLI blocks each), 005 quickstart (2 CLI blocks) and examples ran using that release executable: **156 successful invocations** in `C:\Users\lrns1\AppData\Local\Temp\devmeld-publication-docs-005-5806a2792a074b299caef43b1bf5ff60`. Placeholder paths were replaced with disposable local fixtures; documented `sync --yes` was used for noninteractive replay. Source bytes/timestamps, attach/detach host restoration, source-first setup, metadata/inheritance, status, language and C:/D: file-URI output were checked. The replay script remains in that disposable directory, not in the repository.
- Real Windows terminal, release executable, `C:\Users\lrns1\AppData\Local\Temp\devmeld-terminal-005-2d4856f49a2f49a2b839090dcb328ecf`: answering n cancelled with no output directory; changing configuration through a separate invocation while the next sync waited for y caused **stale preview** rejection, still without output; a newly prepared sync accepted y once and published successfully. Blank/EOF and nonterminal refusal additionally have automated coverage.
- 26 local Markdown links and tracked/untracked whitespace checks passed. `.specify/extensions.yml` is absent; no post-implementation hooks apply.
- No tool/dependency installation, paid Agent run, commit or push. macOS and additional Agent Client consumption remain unverified. Existing generated files remain independently readable without DevMeld running.

**Delivery status:** T001-T015 implemented and verified. US5 completes the 005 CLI journey. This is implementation evidence, not final Maintainer acceptance; that decision remains pending.

## Help argument naming clarification (2026-09-10)

Baseline: local commit `87d8cda`, clean worktree. This follow-up clarifies existing help and documentation only; command parsing, requiredness, domain behavior, persisted data and dependencies are unchanged. No new Feature scope or task was introduced.

- Root, command-group and operation help distinguish native directories/files (`CONTEXT_DIR`, `OUTPUT_DIR`, `SOURCE_FILE`, `SCHEMA_FILE`, `ENTRY_FILE`) from logical addresses (`RESOURCE_PATH`, `GROUP_PATH`, `TOOL_RESOURCE_PATH`, and explicit move endpoints). Shared notation explains required/optional values and optional flag groups. A tool resource address is not an executable path. Both READMEs, examples, the CLI contract and quickstart use the same names.
- Baseline focused help suite: 8 passed. The new executable regression `help_distinguishes_native_files_directories_and_logical_addresses` first failed because root help lacked `init [CONTEXT_DIR]`, then passed. It checks 19 operations across help levels, naming/notation and no context writes. Existing help expectations were updated without removing behavioral or ownership assertions.
- Final native Windows `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, 114 tests passed, 4 cross-drive cases explicitly ignored. Formatting, compiler, conservative Clippy and architecture checks passed.
- `cargo build --release --locked --offline -p devmeld`: PASS. The rebuilt executable's root, `resource add` and `access add` help was inspected successfully. Local Markdown link check: 16 links passed; `git diff --check` passed.
- Linux, macOS and explicit cross-drive acceptance were not rerun for this help-only change; earlier platform evidence above is historical. No dependency installation, context migration, commit or push.

## CLI adapter and layered help (T016-T018, 2026-09-10)

Baseline: local HEAD `87d8cda`; prior help-name/README/style edits were already
uncommitted and were preserved. Native `cargo xtask check` passed before this
refactor (114 tests, 4 explicit cross-drive skips). The 005 requirements checklist
remained read-only, 16/16 satisfied.

### Boundary and implementation review

- Production library entrypoints now accept `ContextLocation`, `Mutation` and
  `Query`; there are no exported argv-based `prepare_in`/`inspect_in` wrappers.
  Native inputs resolve from an explicit absolute base; context discovery is a
  separate selection choice, never a hidden process cwd read.
- `application.rs` owns use-case coordination, `inspection.rs` returns owned
  facts, and `Plan::preview` returns borrowed before/after data. The binary's
  `cli/` owns parsing, help and terminal renderers; main owns process IO and
  confirmation. Generated Markdown remains product output, not a terminal report.
- Preparation remains non-mutating. Applying consumes the original captured plan
  and keeps stale-input, source/target alias, ownership, lock, journal and recovery
  checks, including no-ops. No mutable transaction internals are exposed.
- Removed the old production string-dispatch path and top-level annotation parser;
  the parser now lives under `cli/`. Historical fault-injection fixtures reuse
  that real parser only in test builds. Four direct application tests deliberately
  do not use it or parse terminal reports.
- Domain crates, manifests, lockfile, toolchain, persisted v0 records and transaction
  algorithms are unchanged. No GUI, public wire protocol, generic dispatcher,
  per-command class/trait hierarchy, new dependency or permanent compatibility
  interface was introduced.

### Focused evidence

- `cargo test --locked --offline -p devmeld --test application`: observed initial
  compile RED specifically for the newly approved typed API's missing exports;
  final GREEN, 4 tests. Covers direct register/query/publish, structured previews,
  explicit native input bases, exact instruction-host restoration, inherited
  origins and associations after moves, missing-source registration inspection,
  discarded plans and stale/no-op refusal without CLI invocation.
- `cargo test --locked --offline -p devmeld --test interaction help`: observed
  layout RED because root help still expanded leaf usage and configuration keys;
  GREEN after root/group direct-child summaries and local operation help.
- Existing executable help assertions were adjusted to their owning help page,
  rather than requiring every page to repeat every term. Required/optional names,
  all implemented flags, short-help equivalence, unavailable/corrupt context,
  unknown-topic errors and no-write assertions remain. Ownership/recovery
  assertions were retained; the committed-recovery preview assertion now checks
  structured committed state instead of searching terminal text.
- `group list --help` now explains logical scope, descendants/exclusion, examples
  and applicable global options only. It does not describe tool paths, Schema or
  inheritance. Mutating help retains local risk/confirmation guidance.

### Verification

- Windows `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS, **118 passed**, 4
  cross-drive skips in the ordinary gate. Formatting, compiler, conservative
  Clippy, domain-dependency and type-boundary probes passed.
- Isolated WSL Linux `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS,
  **117 passed**, 0 ignored. Build/source copy:
  `/home/lrns1b/project/devmeld-adapter-005.raWI2I`. The first snapshot shell
  invocation failed before tests because of argument quoting; the explicit-path
  copy and full Linux gate subsequently succeeded. No install was needed.
- Actual C:/D: `cargo test --locked --offline -p devmeld cross_drive_ -- --ignored`:
  PASS, **all 4 cases**. Scratch parent:
  `D:\devmeld-adapter-005-4ad8016850bc438cb5c277eb5ff013f9`.
  Source links, logical moves, instruction entries and interruption recovery passed.
- Release build passed. **29 release invocations** exercised root/group/leaf help,
  en/zh-CN register/update/show/config/status, dry-run, publish, no-op and entry
  attach/detach in
  `C:\Users\lrns1\AppData\Local\Temp\devmeld-adapter-smoke-4408a52a227e4892b01d045130b90233`.
  Original source and authored host bytes were unchanged after detach; Chinese
  navigation and honest client-consumption status were verified.
- Local Markdown links and tracked/untracked whitespace checks passed.
  `.specify/extensions.yml` is absent, so no post-implementation hooks apply.
- macOS, an actual GUI and new Agent Client consumption remain unverified.
  No user context migration, tool/dependency install, paid Agent session, commit
  or push. This process does not currently resolve `devmeld` on PATH; release
  verification used the explicit repository executable.

T016-T018 are implemented and verified. Prior task/evidence history remains
intact; implementation evidence is not final Maintainer acceptance.

## Quick-reference help (T019, 2026-09-10)

The Maintainer clarified that a dedicated website will carry the user manual.
Help now keeps a one-line purpose, usage, precise operand/option summaries, at
most one useful example and immediate write risks. It no longer explains the
inheritance algorithm or recovery lifecycle; all implemented flags remain
discoverable. No documentation URL is invented before a website exists.

- Change scope: help strings/layout, corresponding help assertions, CLI style,
  README terminology, CLI contract and this task/evidence record. Parser,
  application/domain behavior, generated output and storage formats are unchanged.
  Previous uncommitted work remains intact; no commit or push.
- Baseline: 8 CLI help tests and 2 interaction help tests passed. The new
  `command_help_is_a_quick_reference_not_a_manual` first failed on resource add's
  five-line introductory tutorial, then passed. Final focused help suites:
  **11 passed**, retaining options, defaults, schema requirements, empty-group/source
  protection, confirmation limits and no-write checks.
- Native Windows `RUSTUP_AUTO_INSTALL=0 cargo xtask check`: PASS,
  **119 tests passed**, 4 cross-drive cases explicitly skipped. Format/compiler,
  conservative Clippy and architecture checks passed.
- Release build passed; actual resource-add/group-list/init help was inspected.
  Printed lines excluding trailing blank lines changed from 45/19/29 to 30/14/23.
  These are observations, not permanent line-count quotas.
- `git diff --check` passed. Requirements checklist remains read-only, 16/16
  satisfied. `.specify/extensions.yml` is absent; no post-implementation hook applies.
- Linux, macOS and actual cross-drive tests were not rerun for this help-text-only
  refinement. Earlier platform results remain historical; no fresh claim is made.
