# Context Semantics

**Feature**: [Domain Foundation](spec.md)  
**Status**: Non-binding domain semantic sketch  
**Owners**: Project Catalog, Local Context Resolution, and Context Knowledge

## Purpose

This document records facts and invariants that later context Features must
preserve. It is not an API specification, transport contract, serialization
schema, compatibility promise, or instruction to implement search, read, or
relation operations in this foundation.

## Semantic Flow

```text
portable catalog/profile facts
              +
local bindings and Checkout observations
              +
raw Task Context
              │
              ▼
     Task Context validation
        ├── InvalidTaskContext ──> stop with explainable validation failure
        └── ValidTaskContext
                   │
                   ▼
        Active Checkout resolution
        ├── Resolved
        ├── Ambiguous
        └── Unavailable
                   │
                   ▼
       Context Knowledge evaluation
                   │
                   ▼
         explainable Context Result
```

Validation and resolution are distinct domain decisions. An invalid explicit
selection never becomes Ambiguous or Unavailable and never falls back to a
weaker local preference.

## Task Context Validation

A raw Task Context may provide explicit Checkout selections and a current
working area for one request. Validation yields either:

- `ValidTaskContext`: selections are normalized, non-conflicting, known, and
  eligible for the named logical Repositories; or
- `InvalidTaskContext`: at least one selection names the wrong Repository,
  conflicts with another explicit selection, or names an unknown or ineligible
  observation.

Validation failures retain the rejected selection and an explainable reason.
They are request facts, not portable Workspace state.

## Active Checkout Resolution

Only a Valid Task Context enters resolution. For each required Repository, the
semantic outcome is exactly one of:

- `Resolved`: one eligible Checkout observation is selected, with the decision
  basis and considered candidates;
- `Ambiguous`: valid facts leave more than one equally eligible candidate and
  no decisive basis;
- `Unavailable`: valid facts provide no eligible observable candidate.

Resolution bases, from strongest to weakest, are a valid explicit task
selection, a valid current-workspace selection, a valid explicit local default,
and a sole eligible observed candidate. Candidate ordering is presentation only
and cannot select a winner.

## Context Result Facts

A context result is explainable only when it preserves the facts needed to
understand what was returned and why. Where applicable, it includes:

- stable object and Resource identity;
- source identity, Source Type, and observed source revision;
- Evidence and its locator or derivation basis;
- declared Scope;
- query-time Scope Match with matched, mismatched, and unknown dimensions;
- Review Status and Validity Status as independent facts;
- applicable Repository and Checkout-resolution basis;
- relation direction and target availability for relation results;
- relevance or ranking only as an additional derived explanation.

The exact application representation remains open.

## Invariants

1. Task Context validation precedes Active Checkout resolution.
2. Invalid Task Context never falls back to another Checkout selection.
3. Valid-input resolution never guesses through ambiguity.
4. Portable catalog/profile state never stores an Active Checkout or a local
   absolute path.
5. Source Type, Evidence, Scope, Scope Match, Review Status, and Validity Status
   retain independent meanings.
6. Query-time Scope Match never mutates the declared Scope, review, validity, or
   source facts.
7. Ranking cannot erase provenance or make an out-of-scope fact appear in
   scope.
8. A directed Relation is not reversed implicitly, and an unavailable target
   remains explainable.
9. External library, storage, provider, transport, and UI objects do not become
   domain facts.

## Deferred Application Decisions

The first approved Feature with a real consumer must decide, test, and own any
needed application protocol. This foundation deliberately does not define:

- operation or method names;
- request and response envelopes;
- transport choice or CLI/UI shape;
- pagination, cursors, limits, or ordering guarantees;
- ranking fields, scores, or algorithms;
- public error codes, payloads, or retry flags;
- partial-result behavior;
- correlation metadata;
- serialization schema or compatibility/versioning policy.

No example in this document may be treated as a wire contract. A later Feature
may choose a materially different application shape if it preserves the domain
semantics and records any lasting architecture decision in the owning artifact.
