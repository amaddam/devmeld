# 004 Implementation Evidence

**Status**: Implementation and required verification complete; Maintainer acceptance is not asserted.
**Date**: 2026-09-09. Work starts from `1429f11` plus the existing uncommitted
003 language/local-link work and 004 design documents. Those changes are preserved.
The ignored v1 example context is not an implementation fixture and is not migrated.

## Baseline and design gates

- T001: Native Windows `cargo xtask check` PASS: 33 tests passed, 2 Windows
  cross-drive tests explicitly ignored pending their separate environment run.
  Root `.gitignore` owns Rust, secret/local scratch and example-output exclusions.
- Task generation: 15 tasks, 3 per story, remaining baseline/cross-cutting work.
  All task IDs/paths validated. No extension hooks configured.
- Read-only analysis: 13 FRs and 7 SCs covered; no critical/high findings,
  unmapped tasks or Constitution conflicts. All 16 spec-quality checks complete.
- Maintainer authorized one real read-only Codex CLI acceptance using the
  existing account. The executed result is recorded below, separately from approval.

## Behavior evidence

- T002: `cargo test -p devmeld --test workflow new_contexts_use_one_current_ownership_format --locked --offline`
  first failed on the earlier config marker; passed after the typed baseline.
  The Maintainer subsequently clarified this is unreleased design evolution, so the
  current development-only marker is 0, not a second released format.
  `unsupported_records_are_rejected` first failed because recover accepted v1 config;
  now passes for config/receipt/journal v1 and unknown versions across four commands,
  preserving fixture files. Full workspace tests: 35 passed, 2 cross-drive ignored.

- T003: focused CLI attachment first failed on unsupported `--instruction-entry`;
  passed after explicit registration, exact insertion and typed ownership were connected.
- T004–T007: follow-up characterization passed for the coherent insertion primitive:
  absent/empty/BOM hosts, mixed line endings, no final newline, encoding/marker conflicts,
  pre-preview atomic saves, unchanged bytes/mtime, independent multi-host detach,
  registration roundtrips, moved output and en/zh-CN with unchanged SSH/HTTP sources.
  These cases were already supported by the minimum exact-byte implementation;
  no artificial failing stub was introduced. Actual cross-drive checks also passed under T013.
- T008: `even_noop_apply_rechecks_all_captured_inputs` failed because no-op apply
  accepted a post-preview host change. After moving recheck ahead of no-op return
  and invoking it from the CLI, all host/source/config/receipt variants pass.
  Affected workspace: 42 passed, 2 ignored cross-drive tests.
- T009: missing configuration ownership, physical aliases between shared targets,
  duplicate receipt keys and remove/re-add ownership-mode changes each exposed a
  failing regression before correction. Tests now also reject copied, edited,
  missing and malformed insertion evidence, source/obsolete-target aliases,
  and host aliases of configuration, receipt or the cooperative lock.
- T010: private failure-injection tests cover current initialization with incomplete
  config/receipt, shared-host creation/attachment/update/detachment, every implemented
  install step and commit boundary, interrupted rollback and intermediate swaps.
  Pre-journal staging leaves targets unchanged and reports possible remnants;
  uncommitted recovery verifies full before/after effects and refuses external
  changes; committed recovery only cleans up and preserves later author edits.
  Previously successful configuration survives publication rollback.
- T011: `cargo xtask boundaries` and the valid/forbidden dependency and ResourceId
  visibility probes passed. Resources and Publication remain std-only; no crate,
  production dependency or test dependency was added.
- T012: English/Chinese README, examples and quickstart match the implemented
  commands, ownership and unreleased v0 semantics. Existing ignored example state
  is preserved; tests use disposable current-model contexts.
- T015: final Spec/Plan/task review found a missing stale-basis case:
  `a_pending_operation_appearing_after_noop_preview_invalidates_it` first failed,
  then passed after capturing pending-journal absence. The transaction updates that
  captured basis when publishing its own journal. Lock alias checks use identity,
  not a content read under Windows' exclusive lock. All final gates below were rerun.

## Final platform checks (T013)

Rust `1.98.1` (`48a229cea`, 2026-09-01), Edition 2024, existing locked dependencies.

| Environment / command | Observed result |
| --- | --- |
| Native Windows: `cargo xtask check` | PASS: boundaries, fmt, check, Clippy and tests; 54 passed, 3 cross-drive cases intentionally ignored in this run |
| Native Windows: `DEVMELD_TEST_OTHER_ROOT=D:\` then `cargo test --workspace --locked --offline cross_drive -- --ignored` | PASS: all 3 real C:/D: cases, including instruction entries, original-source reading, output relocation, detachment and publication recovery |
| Linux in isolated WSL: `RUSTUP_AUTO_INSTALL=0 cargo xtask check` | PASS: boundaries, fmt, check, Clippy and all 54 Linux tests; no ignored tests |
| macOS, other Agent Clients, Agent reading across Windows drives | Unverified; filesystem cross-drive evidence does not imply client support |

The final Linux run used a fresh `/tmp/devmeld-004-verified-eAZEzB` source copy
and separate build artifacts, not the mounted Windows target directory. An earlier
toolchain probe caused rustup to complete a partial installed toolchain; this was
disclosed, and subsequent runs disabled auto-install. A separate attempt using an
expired WSL temporary path was stopped and is not counted as verification.
Final native and isolated Linux checks passed after those environment incidents.

## Real fresh Agent consumption (T014)

**PASS**, native Windows, Codex CLI `0.153.4`, model `gpt-6-astra`.
Fixture: `C:\Users\lrns1\AppData\Local\Temp\devmeld-004-agent-91a72ea9e403433eb0c32b97f5e1b0b6`.
The small Git project had an active AGENTS.md containing authored instructions and
the generated insertion, no masking override, and an empty global AGENTS.md.
Context and original knowledge were outside the project but on the same drive.
The harmless answer appeared only in the original handover document body.

The successful fresh run used `codex exec --cd <fixture>/project`, `--sandbox read-only`,
`--ignore-user-config`, `--ephemeral`, `--model gpt-6-astra`, and `--json`.
Per-run settings were `approval_policy="never"`, `web_search="disabled"`,
`windows.sandbox="unelevated"`; plugins, apps, hooks, memories and multi_agent were
disabled. Existing account authorization was reused without copying credentials.
DevMeld had finished publication and was not running during consumption.

The task prompt was only:

> What is the release-review code in the team handover note? Cite its source. Do not modify files or run project tools.

Recorded thread: `01a084dd-3bd8-7191-b0ef-e677b565f0fc` (ephemeral run).
The actual command events read, in order, relative to the project:

1. `../context/.devmeld/output/index.md`
2. `../context/.devmeld/output/r-handover.md`
3. `../knowledge/handover.md`, with line numbers

The final answer correctly returned `ORCHID-5842-RIVER` and cited the original
handover file at line 3. Exit status was 0. No DevMeld, MCP or indexed project tool
was invoked. Transient before/after comparisons confirmed all 9 non-Git fixture
files unchanged; no maintained checksum manifest was added to the repository.

The first read loaded a shell profile whose oh-my-posh cache write was denied by
the read-only sandbox; reading still succeeded. Subsequent reads used `-NoProfile`.
This is not a claim that the Agent attempted no writes anywhere, only that the
fixture remained unchanged and no project tool or successful cache write occurred.
An earlier attempt with the default CLI `0.145.0` was rejected before model work
because the selected model required a newer client; it is not counted as a pass.
The successful retry used an already-installed `0.153.4` executable. Afterwards,
the Maintainer separately requested a CLI update: the official installer updated
the default standalone CLI to `0.153.4`, verified by `codex --version`.
That environment update adds no DevMeld dependency and triggered no further model call.

## Handoff

T001–T015 are complete. Required platform and named-client evidence is present;
unsupported platforms/client combinations remain explicitly unverified. This
demonstrates fresh-session navigation, not comparative Agent answer quality.
No Maintainer acceptance, stable release, remote push or old-data migration is
implied. Existing Git history and old example data remain intact. Verification
finished in the local worktree; the Maintainer subsequently authorized a local
commit, recorded in Git, without remote push. No extension hooks are configured,
and post-implementation hook discovery found no extension file.
