# Research: Project Instruction Entry Integration

**Date**: 2026-09-09 | **Status**: Design proposal for the approved [004 scope](spec.md).

This file records reasoning, not additional Product authority. The
[contract](contracts/shared-instructions.md) owns exact formats and maintenance
rules; [Plan](plan.md) owns implementation boundaries and review gates.
No 004 implementation or client-consumption test ran during this research.

## 1. Correct the ownership model, retain the transaction mechanism

**Decision**: Distinguish whole-file ownership from insertion ownership, and
distinguish both from the full-file observation used by one operation.

**Rationale**: Current `storage.rs` uses `Observed { bytes, identity }` both as
persistent whole-file evidence and as the preview basis. Its apply path records
every installed file as wholly owned; `lib.rs` deletes obsolete owned files.
Simply allowing an externally edited host through those paths would eventually
claim or delete the entire instruction file.

For a shared host, persist only the exact generated insertion under its target
and context binding. Capture the current full host for each preview, including
its file identity. This permits ordinary editor atomic-save outside the entry
before preview, while retaining exact stale-preview detection afterward.
Represent the intended ownership effect separately from the installed bytes.
Detachment writes remaining host bytes and releases the claim; it never deletes
the host. Whole-file publication retains its existing evidence requirements.

Keep sibling staging, complete before/after images and verified recovery.
Those images describe one operation's effects, not permanent ownership of the
host. External changes that conflict with uncommitted rollback still stop it;
a committed journal needs only verified cleanup, not target restoration. No
region merge or second rollback engine is needed.

**Alternatives**: Reject a permissive flag on whole-file classification, storing
the merged host as owned content, and a generic patch/executor framework.
The two real surface kinds justify a small explicit model, not new domains.

## 2. A strict byte insertion, not a Markdown editor

**Decision**: Use one fixed begin/end marker pair and exact insertion evidence.
Initially prepend after an optional UTF-8 BOM. Own the entire insertion,
including its trailing separators; preserve all original bytes as a suffix.
Support UTF-8 plain Markdown/text instruction files, not special front-matter
formats. The contract fixes encoding, newline selection and ambiguous cases.

**Rationale**: Prepending keeps a small navigation entry early in the file and
avoids appending inside an unfinished fence. This is a placement choice, not a
promise to understand arbitrary Markdown or a client's full instruction budget.
CommonMark specifies that an unclosed fenced code block can extend to the end
of its container. [CommonMark specification](https://spec.commonmark.org/0.31.2/#fenced-code-blocks)

Ansible's `blockinfile` demonstrates the established marker-delimited block
pattern, including previewable updates/removal. DevMeld still requires its own
stricter ownership evidence and byte-preservation policy; marker appearance is
not permission to adopt a block. Ansible is a reference, not a dependency.
[Ansible blockinfile documentation](https://docs.ansible.com/projects/ansible/latest/collections/ansible/builtin/blockinfile_module.html)

**Alternatives**: EOF insertion risks fenced/commented-out entries; parsing and
rewriting Markdown risks authored formatting and adds no necessary capability.
Configurable placement, templating and multiple blocks per host are deferred.
Raw reserved markers in examples fail explicitly rather than requiring an AST
to guess whether their author meant a managed block.

## 3. One current model, without a legacy compatibility branch

**Decision**: Use v0 config, ownership receipt and journal for every context
created by the 004 implementation, including whole-file-only contexts. Do not
implement a v1 reader/writer, feature-triggered upgrade, downgrade or converter.
Unsupported existing records are rejected before writes and left intact.

**Rationale**: On 2026-09-09 the Maintainer accepted revising development-era
models and formats instead of making old implementation compatibility a default
requirement. Whole-file and insertion ownership are two valid cases of one
current model, not two generations of application logic. The earlier proposal
to keep v1 contexts indefinitely and upgrade on first attachment is superseded.
An explicit version still prevents incompatible records being interpreted as
current ownership evidence; it does not require multiple supported versions.

This drops a compatibility obligation, not the source-protection or recovery
rules. Original resources and already published files remain readable. If actual
old maintenance state needs handling, preserve it and explicitly use a separate
fresh context/unused publication targets, or design a bounded migration for that
concrete need. Never delete records or adopt old outputs to make initialization
succeed. A pending old operation needs its matching implementation or a separate
recovery decision; the new runtime must not guess its semantics.

Bind current local records to the canonical context root and claims to canonical
target paths. This remains local provenance, not a new resource identity, machine
mapping or protection against deliberately forged maintenance records.

**Alternatives**: Reject permanent dual-format operation, automatic migration,
silent reinitialization, and retaining an obsolete recovery decoder merely for
development fixtures. Re-express ordinary whole-file behavior tests in the new
model; keep legacy-shaped fixtures only to verify non-destructive rejection.

## 4. Client discovery is evidence, not a filename guarantee

**Decision**: Select an existing Codex CLI setup as the first real-client
acceptance candidate; do not add a Codex adapter or invoke a model during planning.

**Rationale**: Official documentation describes startup instruction discovery,
root-to-working-directory loading, override/fallback precedence and a combined
instruction-size limit. Thus an arbitrary generated file is not necessarily
loaded, and an `AGENTS.md` can be masked or truncated. Use a fresh, explicitly
recorded fixture setup and preserve any higher-priority instructions.
[Official AGENTS.md guidance](https://learn.chatgpt.com/docs/agent-configuration/agents-md)

Local read-only checks found Codex CLI `0.145.0` and `exec` options for a working
directory, read-only sandbox and JSON events. This is an environment observation,
not a permanent supported-version pin or a passed integration test. The official
non-interactive interface supports recording execution events for inspection.
[Official non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode)

**Alternatives**: A fixture reader alone proves links, not Agent discovery.
No mandatory MCP/API call, startup hook, Skill install, or global client setting
change is needed. Test one named client honestly before claiming more.

## 5. Verification proportional to this ownership change

**Decision**: Use existing Rust tests and `cargo xtask check`, plus isolated native
Windows, Linux (WSL accepted), real Windows cross-drive and named-client checks.

**Rationale**: The highest risks are confusing ownership with operation state,
accepting ambiguous markers, mistaking unsupported state for fresh state and recovery over external
edits. Exercise those through public behavior, not just a renderer test. Include
outside atomic-save before preview, stale no-op application, and hard-link aliases
between hosts, sources and other protected targets. Current no-op application
returns before rechecking its basis; adjust that ordering for FR-007.

**Alternatives**: Do not introduce snapshot libraries, property-test frameworks,
a new lint service, OS sandboxing or an Agent test framework. Existing assertion
helpers and narrowly scoped test seams suffice. Missing platforms or client
authorization remain recorded gaps, never inferred passes.
