# Acceptance Guide: Project Instruction Entry Integration

**Status**: Implemented command guide; actual outcomes are recorded in
[acceptance.md](acceptance.md), not inferred from these instructions.
The current model is an unreleased v0 development design, not a new stable release.

## Prerequisites and fixture

Use the selected native Rust toolchain and existing project checks. Prepare a
disposable local test area outside real projects, using an editor or test fixture
helpers; do not rewrite this repository's own instruction files for acceptance.
Use fresh context/output/entry paths, not an old generated example directory.
Do not clear existing `.devmeld` records to make a fixture look fresh.

```text
<fixture>/
├── context/                 # existing empty directory for explicit --context
├── project-a/AGENTS.md       # author-owned ordinary Markdown instructions
├── project-b/AGENTS.md       # different author-owned instructions
└── knowledge/
    ├── handover.md          # title plus a unique, harmless test answer in its body
    └── unrelated.md         # unrelated resource, without that answer
```

Keep original host/source bytes and metadata in the test runner's evidence.
The answer must not appear in the Agent's task prompt, project files, registration
ID or a generated title. Never use credentials or real private team content.
All paths below are placeholders to replace with actual local native paths.
Examples use the built `devmeld` executable without requiring global installation.

## Main user path

From the DevMeld repository, establish the baseline:

```text
cargo xtask check
cargo build -p devmeld
```

Then use `target/debug/devmeld` (`devmeld.exe` on Windows) by its absolute path.
For each `--apply` command, inspect the displayed proposal and type `apply` only
when the listed fixture changes are correct:

```text
devmeld --context "<fixture>/context" init --apply
devmeld --context "<fixture>/context" resource add handover --document "<fixture>/knowledge/handover.md" --apply
devmeld --context "<fixture>/context" resource add unrelated --document "<fixture>/knowledge/unrelated.md" --apply
devmeld --context "<fixture>/context" entry add "<fixture>/project-a/AGENTS.md" --kind instructions
devmeld --context "<fixture>/context" entry add "<fixture>/project-a/AGENTS.md" --kind instructions --apply
devmeld --context "<fixture>/context" entry add "<fixture>/project-b/AGENTS.md" --kind instructions --apply
devmeld --context "<fixture>/context" sync
devmeld --context "<fixture>/context" sync --apply
```

Initialization creates the current v0 config/receipt even without any entry.
Registration does not change formats; neither preview nor confirmed registration
changes the hosts. Sync preview shows the exact
insertion and changes nothing. Confirmed sync adds one insertion to each host
and publishes the same common resource set. Check author/source bytes unchanged
and follow entry → index → resource guide → original source using ordinary file
reading, with DevMeld no longer running.

Repeat confirmed sync twice: zero host changes, unchanged host modification times,
no duplicate markers and no unnecessary receipt refresh. Add outside instructions
with an ordinary editor, including a save that replaces the physical file, then:

```text
devmeld --context "<fixture>/context" output "<fixture>/context/published" --apply
devmeld --context "<fixture>/context" language zh-CN --apply
devmeld --context "<fixture>/context" sync --apply
devmeld --context "<fixture>/context" entry remove "<fixture>/project-a/AGENTS.md" --apply
devmeld --context "<fixture>/context" sync --apply
```

The new insertion uses the new output link and Chinese generated labels; authored
text and resource membership stay unchanged. After detachment, project A keeps
all its current outside bytes, project B still reaches the context, and sources
remain untouched. Repeat the absent-host case: normal detachment leaves an empty
file, while verified recovery of an interrupted creation can restore absence.

## Required behavior and failure matrix

Use automated public-boundary fixtures where timing, byte comparison or failure
injection is needed. Do not expose a production fault-injection CLI merely for
these tests. Every case must assert results, not only a successful exit code.

| Case | Required result | Spec coverage |
| --- | --- | --- |
| Explicit opt-in; no attachment; canceled preview/apply | Only selected, confirmed targets change; registration is not publication | FR-001/005, SC-001 |
| Absent/empty/BOM-only, LF/CRLF/mixed endings, Unicode, no final newline | Exact preserved outside bytes across attach/update/detach | FR-002/009, SC-001 |
| Two hosts, both output languages, moved output | Same common navigation/resources; only generated text changes | FR-003/004/006/012, SC-002 |
| Same-drive and real C:/D: host/output/source combinations | Follow encoded relative/file-URI links and native-path fallback through actual IO | FR-006, SC-002 |
| Two no-ops; outside edit and atomic-save before preview | No duplicate/mtime churn; legitimate outside changes survive | FR-007/011, SC-003/004 |
| Outside, insertion, source, config or receipt changed after preview; stale no-op | Reject before publication writes; require fresh preview | FR-007/010, SC-004 |
| Entry missing/edited, duplicate/nested/truncated/unknown markers; raw marker examples | Conflict, no repair, replacement block or adoption | FR-008, SC-004 |
| Missing receipt, copied block/receipt from another context, other context in host | Conflict; no ownership inferred from appearance | FR-008/012, SC-004 |
| Host aliases source/output/config/state/another entry, including hard links | Reject without writing protected data | FR-010/012, SC-004/007 |
| Removed before first sync, removed/re-added before detach; whole-file/shared mode collision | No spurious insertion/deletion or silent conversion | FR-005/009/011/012 |
| Fresh whole-file-only, instruction and mixed contexts; add/remove last insertion | One v0 baseline throughout; no feature-triggered upgrade or downgrade | FR-011/012, SC-007 |
| V1/unknown/malformed/mixed config, receipt and pending records; init/sync/config/recover attempts | Reject unsupported state without changing the tree or creating lock/staging files; never follow an unsupported journal's targets | FR-008/010/011, SC-007 |
| Interrupted current initialization with config/receipt not fully installed | Valid v0 journal can recover without first requiring a complete normal baseline | FR-010, SC-005 |
| Pre-journal staging interruption | Targets unchanged; report possible staging remnants without automatic orphan cleanup | FR-010, SC-005 |
| Every implemented uncommitted install/commit/recovery mutation boundary | Verified operation effects roll back; prior committed config remains | FR-005/010, SC-005 |
| External host edit during uncommitted recovery, including outside-only edit | Refuse overwrite and retain unresolved rollback evidence | FR-008/010, SC-005 |
| Committed operation interrupted during cleanup, including later host edits | Clean only verified staging/journal artifacts; preserve committed targets and later edits | FR-005/010, SC-005 |
| Ordinary entries/resources/schema/access/path cases initialized under the current model | Preserve accepted behavior, not old serialization; no source writes, installs or remote access | FR-004/011, SC-007 |

Legacy-shaped records are negative fixtures only; there is no supported old
reader to preserve. Include old config, receipt and journal independently, not
only a made-up version number. Inspect the full fixture tree before/after and
use harmless sentinel targets to verify that unsupported recovery does not touch
recorded paths. Keep current-format success controls, especially interrupted
initialization, so rejection tests cannot hide a broken recovery path.

For actual old data, follow [the non-destructive handling boundary](contracts/shared-instructions.md#existing-development-data).
Do not run a historical executable, reset the worktree, delete generated state
or migrate a real context merely to satisfy these checks. Rebuilding disposable
fixtures does not authorize deleting original knowledge or existing publications.

## Real Agent consumption (required, not simulated)

The executed acceptance used Codex CLI `0.153.4` on Windows; see the exact setup
and observations in [acceptance.md](acceptance.md#real-fresh-agent-consumption-t014).
For a repeat run, record the actually used version, OS, model, effective
permissions and instruction setup. Use an authorized existing account/client;
if an invocation needs new paid-use or other authority, obtain it first or record
this check as blocked. Do not install another client or copy credentials.

For the first supported setup, use the still-attached project B as a small isolated Git project,
with AGENTS.md as its selected active instruction file and no masking override
in the fixture. Inspect relevant instruction precedence and available read
permissions without changing global settings. The official discovery rules
include startup loading and size limits; a fresh session is required.
[Codex instruction discovery](https://learn.chatgpt.com/docs/agent-configuration/agents-md)

After publication has exited, an authorized sample invocation is:

```text
codex exec --cd "<fixture>/project-b" --sandbox read-only --json "What is the release-review code in the team handover note? Cite its source. Do not modify files or run project tools."
```

Use a fresh run, not resume. Keep captured JSON events outside the project/context
inputs. The source note must contain the uniquely chosen fixture answer. The
task prompt must not contain its value, a DevMeld context path, or instructions
explaining the navigation layout. No DevMeld process, MCP call or explicit
DevMeld invocation may be used to supply context during this read.

Inspect actual read events for navigation/resource/source paths and the cited
answer. Record the active instruction-file setup; do not accept only the model's
claim that it read AGENTS.md or a fixture program traversing links. File-reading
commands used by the Agent are allowed; running the indexed tools is not.
The CLI's event output is an evidence aid, not an assertion that every client
exposes identical traces. [Non-interactive execution documentation](https://learn.chatgpt.com/docs/non-interactive-mode)

A masking override, unavailable external-directory permission, failed model
call or unreadable link is an integration failure/gap, not grounds to alter
higher-priority instructions or report a pass. Record other client/platform
combinations as unverified. The cross-drive file check and the named-client
check are distinct; do not infer that the client supports cross-drive reads
unless that combined setup was actually tested.

## Acceptance record

At implementation handoff, record in `acceptance.md`: commit/worktree basis,
commands, Rust/OS/client versions, focused behavior outcomes, failure-injection
coverage, host/source byte comparisons and relevant read traces. Summarize:

- Native Windows full gate and host workflows: PASS on 2026-09-09.
- Linux environment (isolated WSL), with separate build artifacts: PASS.
- Real Windows cross-drive IO: PASS on C:/D:.
- Named fresh Agent Client consumption: PASS, Windows Codex CLI `0.153.4`.
- macOS, other Agent Clients and Agent cross-drive reading: unverified.

All required checks must pass for an implementation ACCEPT; a missing required
environment is not waived by completed tasks. Acceptance does not approve Skill
installation, automatic discovery, scheduling, new resource selection or future
entry formats. These executed checks do not themselves grant Maintainer acceptance.
