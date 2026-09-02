<!--
Sync Impact Report
- Version change: 0.3.1-draft -> 0.3.2-draft
- Modified principles: none
- Modified governance:
  - separated cross-feature product semantics from feature-specific behavior
- Added sections: none
- Removed sections: none
- Follow-up TODO:
  - TODO(RATIFICATION_DATE): ratify after Maintainer review
-->

# DevMeld Constitution

## Core Principles

### I. Context, Not Workflow or Execution

DevMeld MUST own the discovery, connection, retrieval, explanation, and optional
materialization of project context. It MUST NOT prescribe a development
workflow, own another tool's workflow state, or become the runtime that executes
development work. Workflow and execution tools MUST remain replaceable
collaborators.

Rationale: a context layer remains broadly useful only when it complements,
rather than competes with, the tools that plan and perform work.

### II. Grounded and Explainable Context

Context returned by DevMeld MUST retain enough source, evidence, and scope for a
person or Agent to understand why it applies. Ambiguity, inference, and stale or
out-of-scope evidence MUST be visible rather than silently resolved. Derived
context MUST NOT replace its declared source of truth.

Rationale: more context is valuable only when its relevance and basis can be
checked.

### III. Local-First, Portable, and Rebuildable

DevMeld MUST remain fully useful in local operation without requiring a hosted service.
Portable project data MUST remain independent of machine-specific bindings.
Derived state MUST be rebuildable from declared sources, and generated artifacts
MUST NOT depend permanently on one machine or the DevMeld source checkout.

Rationale: changing machines, clients, or installations must not strand project
knowledge or make ordinary development depend on DevMeld.

### IV. Explicit and Reversible Writes

Every DevMeld-managed write MUST target an authorized surface, be inspectable
before application, and leave enough information to explain and reverse the
change. DevMeld MUST NOT silently overwrite, adopt, or merge content owned by a
person or another tool. Conflicts and permission boundaries MUST be explicit.

Rationale: bounded, recoverable writes let independent tools share a project
without hidden ownership or irreversible side effects.

### V. Human-Inspectable by Default

Project knowledge, generated artifacts, public contracts, and recorded
decisions MUST remain readable and reviewable by people without requiring an
opaque internal store. Implementation choices MUST preserve a clear path from a
reported result to its source and governing decision.

Rationale: DevMeld is infrastructure for collaboration, so people must be able
to audit, correct, and maintain what it provides.

### VI. Deliver Value in Vertical Slices

User-visible features and material behavioral changes MUST proceed through the
smallest end-to-end slice that produces an independently testable user outcome.
Their completion MUST be supported by acceptance evidence, and claims about
better context quality or lower navigation cost MUST be checked with a realistic
comparison or benchmark. Future capability MUST remain outside the current
slice until it has a demonstrated need.

Rationale: measured user value, not architectural breadth, determines whether a
context capability belongs in the product.

## Governance

This Constitution defines DevMeld's stable engineering principles. Cross-feature
product terms, boundaries, and semantics belong in `docs/product.md`;
implementation and quality guidance belongs in `docs/engineering.md`;
contribution workflow and review gates belong in `CONTRIBUTING.md`; accepted
feature-specific behavior belongs in Feature Specs; lasting architecture
decisions belong in ADRs.

It governs development of DevMeld itself. It MUST NOT impose Spec Kit or these
engineering rules on projects whose context DevMeld connects or manages.

A Maintainer is a project maintainer authorized under project governance to
approve protected changes, including governance and cross-feature
product-boundary changes. `CONTRIBUTING.md` defines which changes require
Maintainer approval. A contributor or current Agent user does not gain that
authority merely by requesting a change.

An amendment MUST state its rationale and compatibility impact and MUST be
approved by a Maintainer before it takes effect. Versions follow semantic
versioning:

- MAJOR: removal or incompatible redefinition of a principle or governance rule;
- MINOR: any material semantic change that does not require MAJOR, including
  adding, expanding, or narrowing a rule;
- PATCH: wording or clarification that does not change governed behavior.

Plans and reviews MUST check the applicable work against this Constitution.

**Version**: 0.3.2-draft | **Ratified**: TODO(RATIFICATION_DATE) | **Last Amended**: 2026-09-01
