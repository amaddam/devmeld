# Implementation Plan: Durable Context Publication

**Branch**: `main` | **Date**: 2026-09-08 | **Spec**: [spec.md](spec.md)

## Summary

Implement the reviewed file-based path with two pure domain crates and one
application crate. Resource Organization validates identities and associations;
Context Publication owns change/conflict classification. The application reads
local declarations, renders Markdown and applies explicitly confirmed changes.
It is not a daemon, connector or general transaction framework.

The Maintainer delegated continuation and self-verification of this path with
local Git history. This does not ratify the Constitution or a permanent public API.

## Technical Context

- Language: existing Rust 1.98.1 / edition 2024 baseline (ADR-0003).
- Dependencies: application-only Serde/serde_json for JSON, `jsonschema` with
  default features disabled and offline resolution for attribute schemas,
  `file-id` for portable physical-file identity during managed recovery.
- Storage: original sources, managed JSON configuration, Markdown output and
  local ownership/operation records. No database or manual checksum maintenance.
- Testing: Rust unit and real binary/filesystem tests; `cargo xtask check`.
- Target platforms: normal local Windows and Linux filesystems; isolated WSL
  Linux is acceptable. macOS remains unverified unless actually executed.
- Performance/constraints: 8 MiB per source/output, 128 MiB captured operation data
  and internal records;
  no hung-filesystem latency or atomic multi-file/power-loss guarantee.
- Scope: manual whole-context synchronization and ordinary-file entry only.

## Constitution Check

Pre-research and post-design: PASS within the authorized scope.

1. Context, not execution: file links; no installation, connection or daemon.
2. Grounded: authored fields and source links; no invented availability/facts.
3. Local-first: deterministic, independently readable output.
4. Explicit writes: preview, confirmation, stale-input/ownership checks, lock and
   journaled recovery. No forced adoption or overwrite option.
5. Inspectable: ordinary JSON/Markdown and actionable errors.
6. Vertical slices: real CLI-to-files tests per behavior, including failures.

## Project Structure

```text
crates/
  resources/src/lib.rs       # pure organization invariants
  publication/src/lib.rs     # pure change/conflict rules
  devmeld/
    src/main.rs              # arguments and human confirmation
    src/lib.rs               # command coordination
    src/declarations.rs      # JSON and local schema adapter
    src/render.rs            # deterministic Markdown presentation
    src/storage.rs           # paths, bounded IO, ownership and recovery
    tests/workflow.rs        # real binary/filesystem behavior
tools/xtask/                 # developer checks and real Cargo boundary probes
```

`tools/xtask/src/boundaries.rs` reuses serde_json to check Cargo metadata; this is
developer-only and does not add a runtime quality-control framework.

No shared kernel, per-resource domain or empty application/port layer. Domain
crates use only std and never depend on each other or the application. Cargo
dependency checks protect this concrete rule.

See [research](research.md), [data model](data-model.md),
[contract](contracts/local-files.md), [acceptance guide](quickstart.md) and tasks.

## Complexity Tracking

No Constitution exception. The journal supports required managed-write recovery;
schema interpretation uses an offline library, not a handwritten validator.
File identity prevents confusing another writer's identical new file with ours.
