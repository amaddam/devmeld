# Implementation Rationale

## Current repository evidence

Inspected on 2026-09-09: `main.rs` handles only root help, then requires leading `--context` and trailing `--apply`; `lib.rs` coordinates already-authorized plans; `declarations.rs` stores flat registration IDs; `ResourceId` is a portable slug. These are current implementation facts, not authority for the new CLI model.

## Command discovery before parser replacement

- Decision: first extract current help into one small command-aware module, using std and real executable tests. Recognize only actual command paths; root/group/operation help must not resolve a context or call `prepare`.
- Rationale: an independently useful result and a bounded refactor on a green baseline. A parser-library decision is unnecessary for this first slice.
- Alternatives: adding speculative future commands to help misrepresents implementation; adding an interactive wizard or parser framework first does not prove better first use.

## Resource identity versus organization

- Decision: retain a separate stable `ResourceId`; introduce logical organization paths, groups and computed context annotations in Resource Organization. Source paths stay with the source reference.
- Rationale: moves must not invalidate associations or relocate source documents. Rendering cannot become the owner of grouping/inheritance rules.
- Alternatives: slash-containing IDs couple identity to classification; copying parent values into child declarations destroys provenance.

## Defaults and propagation

- Decision: both defaults are stored, initial `propagate=true`, `inherit=false`. Explicit flags win at creation; resolved choices are saved. Effective metadata follows enabled parent-child edges, tags union and child-local field override. Overall descriptions are displayed at their own node.
- Rationale: changing a default should not rewrite existing meaning. No per-field expression language or schema registry is needed for this scope.

## Local placement and write flow

- Decision: explicit context wins; otherwise use the nearest ancestor with context state, or current directory for a successful creating operation. A corrupt/inaccessible marker is an error, not permission to fall through or recreate it. Input relative paths use the invoking directory.
- Rationale: no global implicit shared context; use from subdirectories does not create accidental duplicates.
- Alternatives: global remembered context is easy to mix across projects; Git-root inference adds an unrelated dependency and ambiguous monorepo behavior.

## Preserve actual managed-write protections

- Decision: configuration operations can save their explicitly requested scope with a visible preview and dry-run option. Publication/recovery still use one confirmation, with an explicit noninteractive override that bypasses only prompting.
- Rationale: reduce ceremony without weakening conflicts, stale-input checks or recovery. Entry attachment remains explicit.

No external technology comparison or new dependency was needed for these decisions. Future technical changes must be justified by an actual task, not silently adopted here.
