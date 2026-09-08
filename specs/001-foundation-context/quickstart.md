# Foundation Acceptance Guide

**Feature**: [Domain Foundation](spec.md)  
**Last Updated**: 2026-09-07
**Status**: Tool/dependency-guidance Product behavior accepted on 2026-09-07;
committed baseline and review correction have recorded Windows/Linux verification;
subsequent type refinements verified on native Windows only; Maintainer acceptance pending
**Audience**: Maintainers and contributors reviewing the first code foundation

## What This Guide Proves

The design review proves that DevMeld has a coherent five-domain map and an
honest three-domain implementation boundary. After implementation is approved,
the runnable checks prove that the three core domains enforce their invariants
and dependency direction.

This guide does not prove real Git or Vault access, production persistence,
public context operations, Managed Materialization, Capability Integration,
capability selection, generated Agent artifacts, a CLI/Desktop application, or
benchmark improvement.

## 1. Confirm Accepted Decisions and Deferred Scope

Review these owning artifacts:

1. [Domain Foundation specification](spec.md)
2. [Implementation plan](plan.md)
3. [Research decisions and proposals](research.md)
4. [Conceptual domain model](data-model.md)
5. [Context semantic sketch](context-semantics.md)
6. [Materialization semantic sketch](materialization-semantics.md)

Gate 1 was accepted on 2026-09-02; Gate 2 was replaced on 2026-09-07. Before implementing
[the task list](tasks.md), confirm the authoritative records:

1. [ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md):
   domain-oriented modular monolith and package-by-domain organization;
2. [ADR-0003](../../docs/adr/0003-rust-runtime.md): stable Rust, Edition 2024,
   standard-library-first core; ADR-0002 is superseded;
3. [Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates):
   the concrete progressive check policy.

These ADRs resolve the two general prerequisites; they are not implementation
acceptance. Section 11 records fresh native Rust evidence, not old test passes.

Gate 3, Capability Product meaning, may remain undecided. It blocks only work
that depends on that meaning in any domain. Such work would require a Maintainer
decision recorded in `docs/product.md` and an approved Feature scope. The current
foundation excludes that work, so neither task generation nor core acceptance
requires the Capability decision. Resolving Gate 3 does not automatically add
capability selection or Capability Integration to this Feature.

## 2. Confirm the Design Boundary

The domain map must assign clear ownership and invariants to all five domains:

| Domain | Design | First implementation |
| --- | --- | --- |
| Project Catalog | Required | Required; Context Profile limited to the Capability-independent subset |
| Local Context Resolution | Required | Required |
| Context Knowledge | Required | Required |
| Managed Materialization | Required | Not allowed |
| Capability Integration | Required | Not allowed |

Reject the foundation plan if either supporting domain gains a package, port,
test double, fixture, or public protocol merely to mirror the design diagram.
Also reject generic entity, CRUD, provider, storage, or metadata abstractions
that erase the three core domains' language.

An `application/` package is allowed only when an implemented scenario needs
real cross-object/domain coordination outside domain objects. Reject empty
application packages and services that merely forward calls. A domain with only
pure rules is complete for its approved scope without an application layer.

The Context Profile model retains its full Product design, but this foundation
implements only identity, naming, Repository/Resource selections, and portable
rules independent of Capability semantics. Do not introduce Capability-related
placeholder fields, default-empty lists, no-op behavior, or test fixtures.

### Review Tool and Dependency Guidance

Review the accepted [Product behavior](../../docs/product.md#tool-and-dependency-guidance),
the [semantic sketch](tool-use-semantics.md), and its representation in the
[Domain Model](data-model.md#tool-and-dependency-guidance) using at least these
cases:

1. an explicitly selected option is verified and eligible in the current scope;
2. an explicitly selected option is unavailable or conflicts with a project
   constraint;
3. no option is selected and a maintained script or declared, verified
   dependency already satisfies the need;
4. no adequate existing option exists and a bounded dependency/environment
   change is proposed;
5. the only evidence is global discovery, another project's observation, or
   stale evidence, with no current-scope verification;
6. no project-managed option is suitable, but an existing local/system tool is
   verified callable, compatible, and authorized in the current task scope and
   requires no new dependency, environment, or installation change. Guidance
   selects it before proposing a change; reusing it in a different machine or
   scope requires fresh verification.

For every case, identify the authoritative project declaration, local
observation and freshness, project/task scope, selection authority, separate
change authority, and execution owner. Reject silent substitution, duplicate
dependency registries, implicit installation permission, and claims that
DevMeld itself executed or enforced the choice.

This is a design review only. Do not discover tools, inspect unrelated machine
locations, create a Capability Integration package, or introduce a placeholder
provider/port/fixture to perform it.

## 3. Review Task Context Semantics

Confirm the model and semantic sketch preserve this order:

```text
raw Task Context
    -> validation
        -> InvalidTaskContext: stop, explain, never fall back
        -> ValidTaskContext
            -> Active Checkout resolution
                -> Resolved | Ambiguous | Unavailable
```

Wrong-Repository, conflicting, unknown, and ineligible explicit Checkout
selections are validation failures. Ambiguous and Unavailable describe only
valid input evaluated against local facts.

## 4. Review Deferred Protocol Choices

The context and materialization sketches may state semantic facts and
invariants, but they must not define a v1 contract, JSON envelope, operation
catalog, pagination/cursor format, ranking representation, public error-code
catalog, approval payload, or compatibility-version policy.

Those decisions belong to the first approved Feature with a real consumer.

## 5. Verification Environment After Implementation

### Current Planning Observation — Not Implementation Evidence

On 2026-09-07 the Windows checkout contained the previous Python implementation.
It was backed up and removed for the Rust design reset. Rust/Cargo 1.98.1 for
x86_64-pc-windows-msvc and rustfmt/Clippy components were found in the user's
standard Cargo directory, but cargo/rustc were not on this terminal's PATH.
No PATH, toolchain, system or linker installation was changed. A native project
build/link test had not yet been run at that planning checkpoint. T001 evidence
below supersedes that observation. WSL observations
are not native Windows evidence.

### Historical Native Setup Evidence — T001–T002

On 2026-09-07 the implementation request started the reset task list. A native
Rust program was compiled, linked and executed successfully outside the repository.
The four workspace libraries also passed the initial Cargo test and Clippy runs
(zero domain tests at setup; not foundation acceptance).

- Host: x86_64-pc-windows-msvc.
- rustc: 1.98.1 (48a229cea 2026-09-01); Cargo: 1.98.1.
- rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- MSVC linker: Visual Studio 2022 Community, VC Tools 14.44.35207,
  Hostx64/x64/link.exe, discovered by the existing Visual Studio installer tooling.
- Operational pin and intentionally tested minimum: 1.98.1. Older versions are
  not claimed supported merely because they understand Edition 2024.
- No tools, external packages or system settings were installed or changed.

This machine registers that exact compiler under stable-x86_64-pc-windows-msvc,
not the version-named toolchain. For checks in this session, reuse it only after
verifying the version matches rust-toolchain.toml. The override is process-local,
not a repository or user-wide rustup override. If stable changes, re-evaluate
the pin deliberately rather than bypassing a mismatch:

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;" + $env:PATH
$env:RUSTUP_TOOLCHAIN = 'stable-x86_64-pc-windows-msvc'
if ((rustc --version) -notmatch '^rustc 1\.98\.1 ') { throw 'Toolchain does not match pin' }
```

### Historical Setup Procedure at T001–T002

1. Recheck the installed native Windows Rust toolchain and MSVC linker. Use an
   existing authorized developer shell/path; stop with the precise missing
   prerequisite before installing anything or changing system policy.
2. Choose a supported stable compiler, record rustc -Vv, cargo -V, rustfmt and
   Clippy versions, and create the operational rust-toolchain.toml pin. Declare
   the intentionally supported rust-version; do not promise earlier versions
   just because they support Edition 2024.
3. Create the virtual workspace and four minimal library entries from Plan.
   Each listed member must have a real Cargo manifest and lib.rs entry; ordinary
   compiler-required package setup does not justify any speculative sublayer.
   The core rules and tests remain to be implemented after this setup.
4. Generate Cargo.lock with cargo generate-lockfile --offline once the manifests
   are valid. Commit it only when a later commit is requested. No third-party
   production/test package, alternate runtime or new environment is needed.
5. Apply Engineering's workspace lints explicitly in each member. Verify tooling
   availability without claiming an empty test run as foundation acceptance.

### Current Cross-platform Check Procedure — T033–T036

With the pinned Rust toolchain, rustfmt, Clippy and native linker available, run
from the repository root on Windows, macOS or Linux using the same commands:

```text
cargo fetch --locked
cargo xtask check
```

The first command is the initial bootstrap for the explicitly approved
`tools/xtask` dependency closure. No third-party dependency was added to a core
library. The workspace has one lockfile and target directory; no parallel tool
environment is created. After bootstrap, the alias runs locked/offline.

`cargo xtask check` runs formatting, Cargo checking, conservative Clippy, workspace
tests (including doctests), the metadata gate and its regression harness, stopping
on the first failed command. Individual boundary checks are also available:

```text
cargo xtask architecture
cargo xtask test-architecture
```

The Rust tool replaces the previous project PowerShell scripts. It uses native
process/filesystem APIs, separate command arguments and OS paths, not shell
commands. No PowerShell, Bash or Python is required for these project checks.
The Windows override in the historical setup evidence is a host-specific way to
reuse an already installed exact compiler, not a universal setup requirement.
Section 11 records actual Windows and Linux execution; macOS remains unverified.

Cargo offline does not prevent rustup toolchain downloads. Do not run a newly
pinned absent toolchain as an implicit installation step. If future reviewed
dependencies require fetching, record and authorize that setup explicitly before
returning to locked/reproducible checks.

Spec Kit uses only `.specify/scripts/python/`. Both integration settings select
`py`; the six redundant PowerShell scripts and their integration inventory entries
were removed. The Python scripts are not the DevMeld runtime or reason to recreate
the removed project .venv. Use an existing authorized interpreter if the
documentation workflow needs them. Shell-specific environment assignment hints
printed for users do not introduce a shell dependency in the Python workflow.

## 6. Exercise Project Catalog

Using pure values and only core-required substitutes, verify:

1. a Workspace requires exactly one primary Vault reference;
2. portable Workspace, Repository, Context Profile, and Resource-registration
   values reject machine paths and current-machine choices;
3. Repository identity survives display-name and alias changes;
4. aliases cannot collide across logical Repositories;
5. the implemented Context Profile subset selects stable Repository and
   Resource references, enforces Capability-independent portable rules, and
   cannot persist an Active Checkout;
6. removing a referenced registration is rejected until dependent selections
   are revised explicitly;
7. schema version, source locators, aliases, Resource type/eligibility and
   same-Workspace references are present; no field is silently dropped to match
   the partial experiment.

Review evidence must also confirm that capability-selection fields, behavior,
and placeholders are absent. This is subset acceptance, not a claim that the
complete Product Context Profile is implemented.

## 7. Exercise Local Context Resolution

Validate raw Task Context before invoking resolution:

| Fixture | Expected semantic result |
| --- | --- |
| Explicit selection belongs to another Repository | InvalidTaskContext; no resolution or fallback |
| Explicit selections conflict | InvalidTaskContext; no resolution or fallback |
| Explicit observation is unknown or ineligible | InvalidTaskContext; no resolution or fallback |
| Valid explicit task selection | Resolved with explicit-selection basis |
| No task selection, valid workspace selection | Resolved with workspace-selection basis |
| No stronger selection, valid local default | Resolved with local-default basis |
| Exactly one eligible candidate | Resolved with sole-candidate basis |
| Two equally eligible candidates | Ambiguous with both candidates retained |
| No eligible candidate | Unavailable with an explanation |

Candidate order must not change an outcome, and a later Checkout observation
must not mutate an earlier observation. Local Binding Registry transitions,
working area and full observation provenance must also be covered. Validation
and resolution use the same immutable snapshot; changing observations requires
revalidation, never a second unvalidated observation input.

## 8. Exercise Context Knowledge

Create representative Resource, Relation, Evidence, and Scope facts without a
real parser, index, or public query API. Verify:

1. Source Type, Review Status, Validity Status, and Scope Match can disagree and
   remain independently visible;
2. Scope Match is recomputed for a new query without mutating stored review,
   validity, Evidence, or Scope facts;
3. reversing a directed Relation changes its meaning unless the relation type
   explicitly declares symmetry;
4. an explainable context result preserves stable identity, source revision,
   Evidence, Scope, Scope Match, and applicable Checkout-resolution basis;
5. relevance cannot turn out-of-scope content into an ordinary match;
6. an unavailable relation target remains explainable rather than being
   substituted or erased.

The evidence must demonstrate [Context Semantics](context-semantics.md), not a
preselected wire representation.

## 9. Activate Dependency Enforcement Only After Crates Exist

Implement the actual Cargo metadata gate for the [allowed edges](plan.md#allowed-edges).
Check all declarations, not only the currently active graph: normal, dev, build,
optional, renamed and target-specific edges. Unknown members/paths/kinds and
malformed metadata fail explicitly. Core-to-core normal dependencies are
forbidden even when only public symbols are used; the specified Knowledge
dev-dependencies are allowed only for test composition.

In temporary copies of real manifests/source, using the actual compiled Rust
checker, demonstrate:

| Probe | Expected |
| --- | --- |
| Four core libraries plus the isolated developer-only xtask package | PASS |
| Specified Knowledge test-only dependencies | PASS |
| xtask to its admitted JSON parser dependency | PASS |
| Core to xtask, xtask to core, or xtask library leakage | REJECT |
| Core to another core as normal dependency | REJECT |
| Shared kernel to a domain | REJECT |
| Core to isolated adapter/entrypoint fixture | REJECT |
| Forbidden build/optional/target-specific/renamed edge | REJECT |
| Consumer accessing foreign private module/field | Compiler REJECT with intended diagnostic |
| Raw or fabricated validated context passed to resolution | Compiler REJECT with intended diagnostic |
| ResourceId passed where RepositoryId is required | Compiler REJECT with intended diagnostic |
| Closed resolution match omits a variant | Compiler REJECT with intended diagnostic |

Generate probe-local locks after adding seeds; never mutate the actual workspace
or its lockfile. Valid controls must succeed, and missing dependencies/linkers
must fail the probe harness rather than count as detected violations. Temporary
adapter/entrypoint fixtures are checker test inputs, not production scaffolding.
No real Git repository, user files, credentials or hosted service is involved.

Compiler and metadata checks do not prove that std IO or a hidden clock is
absent. Review core source, included modules, public surfaces and ownership too.
No Python Import Linter or extra analyzer is introduced.

## 10. Confirm Honest Scope

The first code foundation passes only when:

- production code exists only for the minimal shared kernel and three core
  domains;
- each port and substitute is justified by an implemented core rule;
- each application package is justified by actual coordination outside domain
  objects; absent application layers need no placeholder or substitute service;
- Context Profile covers only its declared subset; no Capability-dependent
  values, selection behavior, or placeholders have entered any core domain or
  the shared kernel;
- no `materialization/`, `capabilities/`, production `adapters/`, or
  `entrypoints/` package exists as empty architecture;
- no tool/environment discovery, dependency installation, command execution,
  enforcement integration, duplicate dependency registry, or tool-guidance
  placeholder is implemented as product behavior; the developer verification
  tool's own Cargo subprocesses and isolated fixtures are not Domain 5;
- no public machine contract or real Git, Vault, filesystem, database, Codex,
  CLI, Desktop, or network behavior is claimed;
- no benchmark improvement or user-visible workflow is claimed;
- later Features must traverse accepted core rules rather than bypass them for
  delivery speed.

## 11. Implementation Status and Required Rust Evidence

The baseline Rust foundation was implemented in the native Windows working tree
on 2026-09-07 and subsequently committed as 4da1911. Previous Python and WSL
experiment results are not credited. The
cross-platform tooling correction was additionally compiled and tested from an
isolated copy of this working tree on Linux in WSL, not from the old experiment.
No commit, push or system-policy change was made by the implementation agent.
The Maintainer-approved `serde_json` tool dependency closure was fetched against
the shared lockfile in both environments; no core dependency was added.

| Item | State at baseline submission |
| --- | --- |
| Product and five-domain design | Retained; scope unchanged |
| Architecture / runtime | ADR-0001 / ADR-0003 Accepted |
| Constitution ratification / Capability Gate 3 | Still deferred |
| Implementation | Three core libraries plus the two shared identity values |
| T001–T036 | Implemented, verified and submitted; not Maintainer ACCEPT |
| Native Windows checks | Passed on x86_64-pc-windows-msvc |
| Linux checks in WSL Ubuntu | Passed on x86_64-unknown-linux-gnu from a fresh copy |
| Supporting domains / Tool Guidance implementation | Absent |
| macOS, distribution, real integrations, performance | Not verified |

### Cross-platform Verification Results — T030, T034–T036

This is the historical baseline evidence for commit 4da1911, not a claim that
later edits passed both platforms. See the post-commit review section below.

`cargo xtask check` and every constituent command below exited 0 on both native
Windows and Linux in WSL. Windows used the exact existing-toolchain procedure in
section 5: Rust/Cargo 1.98.1, rustfmt 1.9.0, Clippy 0.1.98 and MSVC 14.44.35207.
Linux used the verified existing `stable` toolchain, Rust/Cargo 1.98.1 for
x86_64-unknown-linux-gnu, compiling and linking Linux binaries.

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --workspace --all-targets --locked --offline` | PASS |
| `cargo clippy --workspace --all-targets --locked --offline` | PASS; one distinct non-blocking performance warning |
| `cargo test --workspace --locked --offline` | 37 core behavior tests + 3 tool unit tests + 1 compile-fail doctest PASS |
| `cargo xtask architecture` | Four core libraries plus xtask; six admitted dependency declarations PASS |
| `cargo xtask test-architecture` | 40 graph/schema/scope/compiler/path probes PASS |

At T030 each core package was also tested independently on Windows with
`cargo test -p <package> --locked --offline --quiet` (all exits 0):

| Package | Behavior tests | Doctests |
| --- | --- | --- |
| devmeld-shared-kernel | 4 | 1 compile-fail |
| devmeld-catalog | 9 | 0 |
| devmeld-local-context | 12 | 0 |
| devmeld-knowledge | 12 | 0 |

These are test-function counts, not coverage percentages. The Knowledge status
matrix exercises 36 source/review/validity combinations within one test.
Local Context additionally exercises all six orderings of three candidates.

The 40-probe harness retains the previous 35 cases and contains 19 graph/layout
controls and cases, seven metadata failure cases, nine public-consumer compilation
controls/cases, four core unsafe-policy probes and one fixture path-escape guard.
Every probe uses a temporary path containing spaces and Unicode. Intended
diagnostics include ARCH_EDGE, ARCH_TARGET,
ARCH_MEMBERS, ARCH_SCOPE, ARCH_PATH, ARCH_KIND, metadata command/JSON/schema
failures, and Rust E0603/E0616/E0308/E0599/E0596/E0061/E0004/unsafe_code.
Valid controls compile first; an arbitrary failed command is not credited.
Fixtures are isolated copies and removed after the run, not production packages.

The Linux verification ran in `/tmp/devmeld-linux-review-20260907b`, copied from
the current Windows tree. It did not modify the user's existing WSL experiment.
An initial WSL compiler probe triggered rustup synchronization from the repository
pin and was interrupted; final checks explicitly used the already installed,
version-verified `stable` toolchain. No claim is made that Cargo's offline flag
prevents rustup activity. macOS execution was not available and is not credited.

Spec Kit smoke checks passed on Windows using the existing Python interpreter:
prerequisites with tasks, task-template output, template resolution, feature
creation dry-run and plan-command help. The local Feature pointer hash was
unchanged. Linux Python 3.14.4 also passed the prerequisite check against its
copied Feature documents. No new Feature, plan or branch was created by these
checks. The six tracked PowerShell scripts were deleted; only an empty local
directory may remain because directory cleanup was blocked by tool policy.

Clippy's `large_enum_variant` warning concerns the owned Resolution snapshot.
It is visible and non-blocking under the accepted perf=warn policy. No measured
performance problem justifies adding indirection yet; no blanket warnings-as-
errors or suppression was added.

### Resolved Native Execution Interruption

An earlier full run was blocked before launching Catalog registrations tests
(OS error 4551, Code Integrity event 3077 at 17:58:38 local time on 2026-09-07).
After the user reported resolving the system setting, the same repository
target and normal Cargo commands ran successfully. The agent did not change
security policy, rename binaries, move the target directory or substitute WSL.
The complete rerun above supersedes the earlier blocked status.

### Semantic and Ownership Review — T003–T007, T023, T028–T029

The five-owner map and provisional-concept inventory were checked against
Product. RepositoryId is shared by all three cores; ResourceId by Catalog and
Knowledge. Owner-local paths, statuses, errors and other identities were not
promoted into the shared kernel. The six Tool Guidance design cases preserve
selection/change authority and local-observation/project-declaration separation;
their Product acceptance does not resolve Gate 3 or authorize implementation.

All production modules, their exports and constructors were reviewed. They use
pure supplied values with no filesystem/network/process IO, hidden clock,
include/path-module trick, interior mutable escape, unsafe block, generic
persistence layer or Capability placeholder. Cross-core composition occurs only
in the Knowledge integration test and maps facts explicitly into consumer-owned
values. Two independent equivalent fact builders yield identical outcomes.
No current rule needs an application service or external-fact port.

Two implementation gaps found during review were corrected with regressions:
encoded home shorthand in portable locators is rejected; Scope summaries no
longer become permanently Unknown solely because both sides omit a dimension.
The latter retains per-dimension Unknown and explicit comparison participation,
one-sided unknowns and unsupported intervals. Its bounded summary meaning is
documented in [Rust Design](rust-design.md#domain-3-context-knowledge), not promoted
into Product or a public query protocol.

Further regressions cover atomic rejection when replacing a Profile's eligible
Resource or redirecting its source, independent statuses across query changes,
Relation Checkout consistency, invalid-explicit-selection no fallback, and
immutable validated snapshots under changed caller inputs.

| Requirements / criteria | Evidence |
| --- | --- |
| FR-001–004, FR-009, FR-018, FR-022; SC-001–002, SC-011 | Five-owner/lifecycle/extension review; six Tool Guidance design cases; no supporting implementation |
| FR-005–006, FR-019–021; SC-003, SC-009 | Four-library layout, Profile subset, full source review; ADR decisions remain authoritative |
| FR-007–008, FR-017; SC-008 | Independent fact builders and explicit mapping; no foreign SDK or unnecessary port |
| FR-010–011; SC-005 | Catalog portable-locator/Workspace tests; separate local bindings/observations; generated-state design remains deferred |
| FR-012–013; SC-004 | State validation, four resolution priorities, all three outcomes, adversarial/permutation/snapshot tests |
| FR-014–015; SC-006 | Evidence/Relation/Scope/result tests, 36 status combinations; no public protocol introduced |
| FR-016; SC-007, SC-010 | Windows/Linux behavior/doctest suites and 40 controlled architecture probes |

### Limitations Requiring Honest Review

- Automated checks cover manifest edges, target/layout scope and specific Rust
  visibility/type/safety examples. They do not prove all semantic ownership,
  source expansion, IO purity, correct facts or re-export intent.
- Cargo metadata omits lint settings. All four manifests explicitly inherit the
  workspace policy; actual unsafe probes verify the effective unsafe gate.
  Other Clippy settings and narrow overrides still require manifest/source review.
- The initial flat source/layout gate is a revisable 001 scope check, not a
  permanent rule against justified future application/port/adapter layers.
- Portable locators support a bounded relative/remote lexical subset; local paths
  are dialect-tagged absolute spellings, not proof of OS validity or existence.
  See Rust Design for exclusions. No actual source parsing/discovery occurs.
- Observation freshness and availability are supplied facts. Old snapshots do
  not automatically age or re-inspect themselves; changed facts need revalidation.
- Unsupported version intervals remain Unknown. Evidence hashes/derived IDs
  retain supplied provenance, not verified cryptographic trust.
- Windows and Linux in WSL were executed here. macOS builds, packaging,
  signing, MSRV below 1.98.1 and performance remain unverified. There is no product CLI,
  UI, integration adapter or user-visible vertical slice in this submission.

## Acceptance Record

Original submission and subsequent commit, retained as history:

```text
Submitted snapshot: originally reviewed as an uncommitted tree based on bb7523d4ece4c42a0da98516ebe7cfd653fb9400
Committed snapshot: 4da191195b3edafefb84d6f2896f0e56fc74e400
Original Code/Spec/Plan/ADR revisions: verification.sha256 as stored in that commit
Architecture: ADR-0001 accepted; unchanged by this implementation
Runtime: ADR-0003 accepted; unchanged by this implementation
Context Profile acceptance scope: Capability-independent subset only
Capability-dependent implementation: excluded; Gate 3 remains deferred
Tool/dependency-guidance Product decision: accepted by Maintainer on 2026-09-07
Tool/dependency-guidance Product source: docs/product.md#tool-and-dependency-guidance
Tool/dependency-guidance implementation: excluded
Verification: section 11 commands; all exit 0 on native Windows and Linux in WSL
Passing tests: 37 core behavior + 3 tool unit + 1 compile-fail doctest
Architecture/type/scope/path probes: 40 passed with intended diagnostics
Developer tooling: Rust xtask, serde_json tool-only; Spec Kit Python-only
Maintainer: pending reviewer entry
Decision: PENDING — reviewer to choose ACCEPT or REVISE
Notes: the original submitting agent reported no commit or push at submission time; the snapshot was committed subsequently
```

Before these corrections, all 68 entries in the original manifest matched the
checkout after CRLF-to-LF normalization (three matched raw bytes; 65 differed
only in line endings). Git recorded LF in the index and CRLF in the working
tree under core.autocrlf=true. This links the committed contents to the original
reviewed snapshot without changing the historical record.

### Post-commit Review Verification (2026-09-07)

This section records the review correction snapshot committed as 64dfb74.
The later type refinement and its verification are recorded separately below.

Review snapshot: corrections based on commit 4da1911. At verification time,
these changes were uncommitted. The user subsequently authorized a local commit
and reserved push for themselves. The Git commit containing this record and its
fingerprint manifest identifies the submitted revision. Maintainer acceptance
remains PENDING; commit authorization is not an ACCEPT decision.

The changes retain SelectionSource on rejected preferences and enforce exactly
one unconditional, required, unrenamed shared-kernel dependency per consuming
core. Snapshot-wide errors retain None for both selection and source. Error
reason strings, selection precedence and supporting-domain exclusions are unchanged.

New regressions first failed with missing SelectionSource/source APIs (E0433,
E0599). The real graph control passed, then the original checker incorrectly
accepted the additional-target-kernel fixture with seven declarations; the new
probe correctly identified that as a failure. After the correction, all three
new allowed-destination probes fail with the intended ARCH_EDGE diagnostic.

Environment and reproducible commands:

- The initial native Windows attempt used Rust/Cargo 1.80.1. `cargo +stable test -p
  devmeld-local-context --locked --offline` failed before compilation because
  Edition 2024 is not supported. The successful native rerun below supersedes
  that blocked state for this revision.
- While inspecting Ubuntu-24.04 in WSL from the repository directory, rustup
  unexpectedly synchronized and installed the pinned 1.98.1 toolchain and its
  Cargo/rustc/rust-std/rustfmt/Clippy components. This was an unintended environment
  change during toolchain inspection, not reuse of an already installed version.
- Subsequent Linux commands explicitly set `RUSTUP_TOOLCHAIN=1.98.1` and
  `CARGO_TARGET_DIR=/tmp/devmeld-review-20260907-target`, and ran against the
  current checkout at `/mnt/d/RUST/project/devmeld`.
- `cargo fetch --locked` fetched the existing approved lockfile dependency closure
  because the WSL cache lacked serde_json 1.0.151. No manifest or lockfile changed.
- `cargo xtask check` exited 0 on Linux in WSL. Formatting, workspace checking,
  Clippy, 38 core behavior tests, three tool unit tests, one compile-fail doctest,
  the six-declaration graph check and 43 architecture probes all passed.
  The existing large_enum_variant performance warning remains non-blocking.
- The new behavior test exercises identical rejected Workspace/LocalDefault
  selections through Resolved, Ambiguous and Unavailable. Existing validation
  tests also check explicit-task provenance and source-free snapshot failures.
- macOS and CI execution remain unverified. CI and locator fuzz testing are
  follow-ups, not part of this correction.

Native Windows rerun after the user reported updating the environment:

- Rust/Cargo 1.98.1, rustfmt 1.9.0-stable and Clippy 0.1.98; compiler host
  `x86_64-pc-windows-msvc`. Rustup automatically installed the version-named
  1.98.1 toolchain components when the repository pin was activated.
- The first `cargo xtask check` could not resolve serde_json from the Windows
  offline cache. `cargo fetch --locked` completed the approved dependency cache;
  Cargo.toml, Cargo.lock and rust-toolchain.toml were unchanged.
- `cargo xtask check` then exited 0 in the native Windows PowerShell session at
  `D:\RUST\project\devmeld`. It compiled and ran Windows `.exe` files under the
  repository's `target` directory, with no WSL invocation in this rerun.
- Formatting, workspace checking, Clippy, 38 core behavior tests, three tool
  unit tests, one compile-fail doctest, six admitted dependency declarations
  and all 43 architecture probes passed. The existing large_enum_variant
  warning remains non-blocking. No source changes were needed for this rerun.

### Type Modeling Refinement (2026-09-07)

Historical snapshot: changes based on 64dfb74, authorized by the user's request
to replace overly broad string-based business fields with appropriate Rust types.
The refinement was subsequently committed as 75aa138. Its verification and commit
do not grant Maintainer ACCEPT.

- Knowledge WorkingTreeState preserves Clean, Dirty and Unknown; the latter two
  require valid explanation text. Resource and Relation results retain the type.
  The independent fact-builder test now covers all three mapped states.
- ProfileId and ObservationId validate identity spelling in their owning domains.
  Profile construction/removal/map access and observation input/selection/map access
  require the correct types. CheckoutSelection::new now returns Self because both
  IDs are already constructed; TaskContext still validates their relationship.
- RejectedSelection::reason returns RejectionReason, with eight variants and
  Display text preserving previous explanations. Duplicate observation failures
  retain their typed ID; selection sources and resolution precedence are unchanged.
- SUPPORTED_SCHEMA_VERSION names the existing Catalog schema version 1. Names,
  aliases, descriptions and open Resource type labels remain text. No dependency,
  workspace member, shared-kernel value or public protocol was added.

Validation on native Windows, using the already installed Rust/Cargo 1.98.1
(`x86_64-pc-windows-msvc`), rustfmt 1.9.0-stable and Clippy 0.1.98:

- New behavior tests first failed with the expected missing type/constant APIs
  and incompatible constructor signatures, before implementation.
- `cargo xtask check` exited 0 from `D:\RUST\project\devmeld`, compiling and running
  Windows executables under `target`. All dependencies were available offline.
- Formatting, workspace checking, Clippy, 43 core behavior tests (Catalog 10,
  Local Context 15, Knowledge 14, Shared Kernel 4), three tool unit tests and one
  compile-fail doctest passed. The existing large_enum_variant warning remains.
- The real six-declaration dependency graph and 46 architecture probes passed.
  Three new E0308 probes reject mixed Profile/Workspace IDs, Repository/Observation
  IDs and assigning a String to a working-tree state after the valid control passes.
- No WSL command was used for this refinement. WSL/Linux and macOS are not
  credited for the changed snapshot; earlier Windows/Linux evidence stays historical.

### Rejection Receipt Correction (2026-09-08)

T037–T038 correct the local working tree based on 08f115d. When a selection revokes
an earlier preference for the same Repository, normalization retains that earlier
selection and its source with ConflictingSelection, alongside the current
selection's own reason. Explicit conflicts still fail validation; weak preference
conflicts do not become a first/last-wins selection. No resolution priority, public
signature, dependency or domain boundary changed.

Verification on native Windows in `C:\java\project\DevMeld`, using the existing
Rust/Cargo 1.98.1 toolchain with a process-local
`RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-msvc` override:

- RED: `cargo test -p devmeld-local-context --test safety conflicting_selections_retain_all_participants_and_sources --locked --offline`
  failed before the implementation change: the assertion expected two retained
  selections and received one. This was a behavioral failure, not a compiler or
  environment failure.
- GREEN: the same command exited 0 after the correction. It covers both input
  orders for explicit task, Workspace and LocalDefault sources, with an extra
  unselected observation to prevent reconstructing receipts from all candidates.
  The affected package suite also passed.
- Additional regression: `cargo test -p devmeld-local-context --test safety invalid_preference_preserves_its_cause_and_the_revoked_preference --locked --offline`
  exited 0. An unknown preference retains UnknownObservation, the revoked valid
  preference retains ConflictingSelection, and sole-candidate fallback is unchanged
  in both input orders. No separate RED was claimed for this added boundary case.
- `cargo xtask check` exited 0: formatting, workspace checking, conservative Clippy,
  45 core behavior tests (Catalog 10, Local Context 17, Knowledge 14, Shared Kernel 4),
  three tool unit tests, one compile-fail doctest, the six-declaration graph check
  and all 46 architecture probes passed. The existing large_enum_variant warning
  remains non-blocking.
- No tool installation, dependency fetch, WSL execution, commit, push or Maintainer
  ACCEPT occurred during this correction. Linux and macOS were not rerun; earlier
  platform evidence remains attached to its historical snapshots.

[Current fingerprints](verification.sha256) identify the correction snapshot,
including this guide. Hash UTF-8 file bytes after normalizing CRLF to LF so a
normal Git checkout on Windows does not invalidate the comparison. The original
manifest remains available in commit 4da1911. The manifest excludes itself and
build artifacts; hashes identify contents, not successful test execution.
Any subsequent edit requires fresh relevant checks and updated fingerprints.

An ACCEPT decision means the three-domain code foundation is ready to support a
later adapter-backed Feature. It does not approve either supporting-domain
implementation, capability-selection implementation, tool-guidance implementation,
the standalone Capability Product meaning, or any future application protocol.
