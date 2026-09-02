# Contributing to DevMeld

DevMeld welcomes small, reviewable changes that preserve its role as a context
layer. Before contributing, read the project
[`constitution`](.specify/memory/constitution.md),
[`product baseline`](docs/product.md), and
[`engineering guide`](docs/engineering.md).

## Classify the Change

Record a decision where it will remain discoverable and authoritative.

| Change | Required artifact |
| --- | --- |
| Stable engineering principle or governance rule | Constitution amendment |
| Cross-feature product boundary, term, or semantic rule | `docs/product.md` |
| User-visible feature behavior, scope, or acceptance condition | Feature Spec |
| Provisional implementation design for one feature | Technical Plan |
| Accepted architecture decision with lasting trade-offs | ADR |
| Coding, testing, or implementation convention | `docs/engineering.md` |
| Contribution workflow or review policy | `CONTRIBUTING.md` |

Feature Specs and Technical Plans follow the repository's Spec Kit layout under
`specs/<feature>/`, using `spec.md` and `plan.md`. When the first ADR is accepted,
create `docs/adr/`; do not create it earlier as a placeholder.

Exploratory discussion may happen in issues, chat, notes, or outside the
repository. Once a product or architecture decision is accepted, update the
owning repository artifact before treating the decision as an implementation or
review baseline. Do not create speculative ADRs or empty documentation areas.

## Contribution Flow

1. Identify the owning artifact and confirm the change fits the Constitution and
   product boundary.
2. For a user-visible feature or material behavioral change, create or update a
   Feature Spec with user scenarios, acceptance conditions, edge cases,
   assumptions, and non-goals.
3. Derive implementation planning from the approved Spec. Record a lasting
   architecture choice in an ADR only after it is accepted.
4. Implement the smallest complete vertical slice and add evidence proportional
   to its risk, following `docs/engineering.md`.
5. Review the diff for product terminology, document links, generated ownership,
   compatibility, and checks that were not run.

Keep unrelated refactors separate. A contribution must not silently broaden the
approved feature, adopt a new source of truth, or expand a Managed Surface.

Routine maintenance, internal refactoring, dependency updates, documentation
fixes, and defect corrections that preserve approved behavior may proceed
without a Feature Spec. A defect correction that intentionally changes public
behavior or a data contract is a material behavioral change and requires one.

## Review Gates

Maintainer approval is required before accepting:

- ratification or amendment of the Constitution;
- changes to cross-feature product boundaries or meanings;
- a Feature Spec's user-visible scope, acceptance conditions, or non-goals;
- a new public machine contract or incompatible persisted-data change;
- destructive migration, overwrite, security-default, or permission changes;
- a technology decision that materially limits supported platforms or future
  Agent Clients.

Internal naming, refactoring, test organization, and implementation details may
proceed within an approved Spec and Plan when they preserve public behavior and
satisfy the Constitution and engineering guide.

A review submission must state what changed, why it belongs in the approved
scope, what evidence was collected, and what remains unverified. A change is not
ready to merge while required acceptance evidence is missing or an owning
decision artifact contradicts the implementation.
