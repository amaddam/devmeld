# Tasks: Durable Context Publication

Input: this feature's Spec, Plan, contracts and Engineering guide. Each behavior
task owns small RED/GREEN/refactor cycles; setup/docs use direct verification.

## Setup

- [x] T001 Configure the three planned crates in Cargo.toml and crates/*/Cargo.toml, resolve reviewed dependencies into Cargo.lock and inspect their feature graph; blocks T002 onward.

## US1 — Durable files (P1)

Goal: real registration → publication → offline reading, through the CLI.
Boundary: Resource Organization and Context Publication, composed by application.

- [x] T002 [US1] Implement previewed/confirmed initialization with ownership protection using vertical TDD in crates/devmeld/tests/workflow.rs, crates/devmeld/src/main.rs, crates/devmeld/src/lib.rs and crates/devmeld/src/storage.rs; preview writes nothing and repeat init cannot adopt a context.
- [x] T003 [US1] Implement document registration and deterministic publication with source links using vertical TDD in crates/resources/src/lib.rs, crates/devmeld/src/declarations.rs, crates/devmeld/src/render.rs and crates/devmeld/tests/workflow.rs; include optional entry and unchanged no-op; depends on T002.

## US3 — Safe changes (P1; required before US1 is complete)

Goal: stale/conflicting writes fail; interruption can be recovered without losing
author changes. Independent acceptance uses real files and storage fault injection.

- [ ] T004 [US3] Implement ownership, stale-basis and path-alias rejection using vertical TDD in crates/publication/src/lib.rs, crates/devmeld/src/storage.rs and crates/devmeld/tests/workflow.rs; depends on T003.
- [ ] T005 [US3] Implement journaled interruption/recovery and cooperative locking using vertical TDD in crates/devmeld/src/storage.rs and crates/devmeld/tests/workflow.rs; verify every mutation boundary, retry and external conflicts; depends on T004.
- [x] T006 [US1] Implement explicit entry/output changes and unregister/sync deletion using vertical TDD in crates/devmeld/src/lib.rs and crates/devmeld/tests/workflow.rs; original sources survive and prior config commits survive sync failure; depends on T005.

## US2 — Descriptions and access guidance (P2)

Goal: custom resource facts and original tool references, without executing tools.
Independent acceptance: service/tool fixture, schema failures and explicit changes.

- [x] T007 [US2] Implement authored descriptions and offline local attribute validation using vertical TDD in crates/devmeld/src/declarations.rs and crates/devmeld/tests/workflow.rs; custom fields accepted, invalid fields/unsupported controls rejected before writes; depends on T006.
- [x] T008 [US2] Implement validated access associations and grounded guidance using vertical TDD in crates/resources/src/lib.rs, crates/devmeld/src/render.rs and crates/devmeld/tests/workflow.rs; no implicit authority/availability, dangling removal rejected; depends on T007.

## Cross-cutting verification

- [ ] T009 Verify pure-domain dependencies with valid/forbidden probes in tools/xtask/src/main.rs; run the check with each manifest change, then the completed workspace.
- [ ] T010 Review and update README.md and specs/003-durable-context/quickstart.md against real commands, and record actual behavior evidence in specs/003-durable-context/acceptance.md.
- [ ] T011 Run cargo xtask check on Windows and isolated WSL Linux, full CLI fixtures and git diff --check; record outcomes/unverified platforms in specs/003-durable-context/acceptance.md and preserve local Git checkpoints.

## Dependencies / parallel opportunities

T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T010 → T011.
T009 starts when manifests exist and reruns at completion. No independent whole
implementation task is marked parallel: these slices share composition/storage
and integration tests. This applies to US1, US2 and US3 alike.

## Delivery strategy

Finish each real behavior cycle before adding another. US1 is not deliverable
without its US3 safety requirements. Preserve focused evidence and affected suite
results; final tests are not Maintainer acceptance. Local commits are authorized
by the Maintainer; remote push is not.
