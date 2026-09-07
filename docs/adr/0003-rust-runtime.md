# ADR-0003: Rust Runtime and Foundation Restart

- Status: Accepted
- Decision date: 2026-09-07
- Scope: DevMeld implementation language, native development baseline, and reset of Feature 001 implementation
- Supersedes: [ADR-0002](0002-initial-python-runtime.md)

## Decision Record

The Maintainer accepted Rust as DevMeld's main implementation language on
2026-09-07 and authorized removal of the existing implementation and a return
to documentation/planning in the Windows checkout. This is an engineering
decision accepting trade-offs, not a claim that comparative benchmarks or a
complete Rust implementation have passed.

[ADR-0001](0001-domain-oriented-modular-monolith.md) remains the architecture
baseline. The Constitution and Product behavior are unchanged. Gate 3 remains
deferred; this decision does not authorize Capability selection, Tool Guidance
implementation, or either supporting domain.

## Context and Rationale

DevMeld is a local-first project context layer, not an application whose core
requires a particular scripting or AI library ecosystem. Its foundation needs
explicit identity, validated observation snapshots, mutually exclusive outcomes,
and separate source and query-derived facts. Rust's types, ownership and native
artifact model fit this direction and the project's preference for explicit,
maintainable domain boundaries over minimum initial authoring effort.

Rust cannot prove that a domain model is correct or prevent all architectural
coupling. Learning, compilation, integration and platform-specific distribution
costs are accepted. Choosing Rust does not imply that TypeScript or Python
cannot implement the same behavior well. No numeric language ranking or
performance superiority is asserted.

## Decision

1. Use maintained **stable Rust**, Edition **2024**, with Cargo. No nightly
   requirement and no permanently frozen compiler patch in governance prose.
   At implementation setup, record the selected native toolchain, pin it in
   `rust-toolchain.toml` for reproducible work, and declare the intentionally
   supported `rust-version` in Cargo metadata. Update these operational files
   with compatibility checks; ordinary supported upgrades do not amend this ADR.
2. Develop natively on Windows first. Windows, macOS and Linux remain target
   platforms; host/architecture execution evidence is recorded independently.
   WSL is optional and its installed Rust is not proof of native Windows setup.
   Cross-platform build/link/package verification remains implementation work.
3. Use standard-library-first production domain code. The first foundation
   plans no third-party production or test library, framework, database, CLI,
   async runtime, provider SDK, or UI. This is not a permanent dependency ban:
   a real Feature need may justify a reviewed dependency in its owning Plan.
4. Use Cargo's compiler checks and tests, default rustfmt, and conservative
   Clippy. The [Engineering Guide](../engineering.md) owns lint choices and the
   check sequence. Do not impose blanket warnings-as-errors, complexity or
   code-size quotas, or blanket bans on cloning and test fixture expectations.
5. Realize the existing three core boundaries with a Cargo workspace and a
   minimal shared kernel. These are library compilation boundaries, not separate
   services or distributables. The [Plan](../../specs/001-foundation-context/plan.md)
   owns exact members and test-only edges. Do not add facade, application, port,
   adapter, entrypoint, or supporting-domain packages merely to complete a tree.
6. Enforce actual crate dependency directions after code exists using Cargo
   metadata and a small project check, plus compiler visibility probes. No
   Python Import Linter or additional persistent analysis service is required.
   Compiler and graph checks complement domain tests and human review.
7. Remove the previous Windows Python source, tests, manifest, lockfile and
   project-local runtime/cache outputs after recoverable backup. Keep language-
   independent design and historical decisions. Reset all Feature 001 code and
   acceptance tasks; neither old Python passes nor the partial WSL Rust sample
   satisfies the new Rust acceptance requirements.
8. This migration turn delivers documents only. Do not scaffold Cargo files or
   copy experimental Rust code into the Windows checkout. Keep the WSL sample
   outside the authoritative project; it is not a second production codebase.
   Spec Kit's own Python scripts remain development-workflow infrastructure,
   not a DevMeld runtime dependency.

## Alternatives Considered

- **TypeScript as the core**: viable domain modeling and integration ecosystem;
  not selected because this project's priorities favor native core delivery and
  explicit ownership over JS runtime alignment. No parallel TS core is planned.
- **Retain Python**: a real previous implementation, not a zero-code starting
  point. Its work informs the semantic checklist, but its runtime baseline is
  superseded by the Maintainer's direction. No Python migration benchmark gate
  or ongoing dual implementation is required.
- **Rust core plus immediate TS UI/Tauri**: premature; no UI is in Feature 001.
- **Copy the WSL sample as completed foundation**: rejected; it has documented
  semantic and field-coverage gaps and no full native Windows acceptance.
- **One crate with conventions alone**: fewer manifests, but weaker visibility
  boundaries between core domains. Small library members use Cargo's existing
  compiler boundaries without a new analyzer or speculative runtime layers.

## Consequences and Revisit Triggers

Contributors need the selected Rust toolchain and a native linker. Target builds
may still have system-library, signing, installer and upgrade requirements; a
native artifact does not eliminate distribution engineering. Cargo dependencies
belong to DevMeld's workspace, never to a connected user's project.

Review actual compatibility, deployment or maintenance problems if they arise.
No further language-comparison experiment is a prerequisite for implementation;
ordinary tests and target-platform verification remain required. A future
runtime reversal requires its own accepted decision, not a silent workaround.

## Technical References

- [Cargo workspaces and inherited settings](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)
- [Rustup on Windows](https://rust-lang.github.io/rustup/installation/windows.html)
- [Clippy usage](https://doc.rust-lang.org/clippy/usage.html)
