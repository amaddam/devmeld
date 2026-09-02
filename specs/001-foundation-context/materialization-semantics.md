# Materialization Semantics

**Feature**: [Domain Foundation](spec.md)  
**Status**: Non-binding supporting-domain semantic sketch; design only  
**Owner**: Managed Materialization

## Purpose

This document preserves the safety model for future controlled writes without
authorizing implementation. This foundation creates no Managed Materialization
package, port, test double, public operation, filesystem behavior, Agent Client
integration, or machine contract.

## Owned Concepts

### Materialization Plan

A Materialization Plan is one immutable, human-inspectable preview. Its identity
must cover the meaning-bearing inputs, proposed changes, ownership claims,
target preconditions, and reversal boundaries. Changed inputs or output produce
a different plan rather than mutating an approved preview.

### Planned Change

A Planned Change identifies one bounded Managed Surface and describes:

- the target identity without assuming a transport or filesystem shape;
- the proposed complete content or bounded difference;
- the claimed owner and currently observed ownership;
- the expected target state or other preconditions;
- the inputs and revisions used to derive the proposal;
- the maximum boundary that a future reversal may touch.

### Ownership and Preconditions

Matching content is not ownership evidence. Application eligibility requires an
explicitly accepted ownership policy and current target facts that still match
the preview's preconditions. Unknown ownership or observation fails closed.

### Materialization Manifest

After a future approved application succeeds, a human-inspectable Manifest
records the applied plan, exact owned surfaces, before/after facts, input
revisions, and enough evidence to verify or evaluate bounded reversal. It grants
no authority outside the applied boundary.

### Drift, Conflict, and Reversal Boundary

Verification compares current facts with the Manifest. It reports drift,
unavailability, or ownership conflict without adopting the observed change.
Reversal is eligible only when it can affect the Manifest-owned boundary without
harming unrelated human or provider content.

## Semantic State Model

```text
Previewed ── accepted authority + matching preconditions ──> Applied
    │                                                       │
    ├── stale / ownership conflict / unsafe target ──> Conflict
    └── intentionally discarded ─────────────────────> Closed

Applied ── matching owned boundary ──> Verified
Applied ── changed or unknown facts ──> Drifted / Conflict / Unavailable
Applied ── safe bounded reversal ─────> Reverted
```

These labels describe necessary distinctions, not public operation names or a
wire-level state enumeration.

## Invariants

1. No write is eligible without a complete human-inspectable preview.
2. Approval applies only to the exact immutable plan reviewed.
3. Preconditions and ownership are re-evaluated against current facts before
   any effect.
4. Unknown ownership, stale input, target drift, or overlapping incompatible
   ownership fails closed.
5. A person or another provider's surface is never adopted merely because its
   content resembles a proposal.
6. Verification reports drift; it does not silently update the expected state
   or expand ownership.
7. Reversal touches only the recorded owned boundary and stops when unrelated
   content could be harmed.
8. Partial external effects cannot be reported as ordinary complete success.
9. A generated artifact remains generated state and never replaces its portable
   or source-owned inputs as truth.
10. Materialization publishes context-discovery material; it does not instruct
    an Agent how to execute a development workflow.

## Deferred Application Decisions

The first approved materialization Feature must supply concrete scenarios and
decide any required application shape. This foundation does not define:

- preview, apply, verify, revert, or other operation names;
- request/response envelopes or transport;
- approval actor fields, authority tokens, or plan expiry;
- target-reference serialization;
- adapter receipts or partial-failure compensation APIs;
- public error/status codes and retry semantics;
- manifest storage or serialization format;
- compatibility and protocol versions;
- provider/client capability negotiation;
- filesystem, configuration-tree, or Agent Client adapters.

No code or fixture is required for this supporting domain in the current
feature. A later Feature may revise the application shape while preserving these
safety invariants and recording lasting architecture choices in ADRs.
