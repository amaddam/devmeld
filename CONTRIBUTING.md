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

## Spec Kit Customizations

The behavioral development rules live in `docs/engineering.md`. Project-owned
Spec Kit sources live in `.specify/templates/overrides/`:

- `tasks-template.md` defines behavior-slice task structure.
- `tasks.md` and `implement.md` replace the corresponding command instructions.
  They retain the existing Spec Kit governance checks and hooks and use
  Codex-compatible skill frontmatter. Their upstream baseline is the installed
  Spec Kit 1.0.1 command output; review upstream changes when upgrading.

Edit these sources, not only `.agents/skills/speckit-{tasks,implement}/SKILL.md`.
The latter are checked-in materialized copies, not separate rule authorities.
With an existing Python 3 interpreter, synchronize only these two copies:

```text
python tools/speckit/sync_skills.py --check
python tools/speckit/sync_skills.py --write
python tools/speckit/sync_skills.py --check
python -m unittest discover -s tools/speckit -p "test_*.py"
```

`--check` is read-only and fails on drift. Review differences before `--write`,
which replaces only those two outputs. This helper neither installs/upgrades
Spec Kit nor implements its general renderer, hooks or template resolution.
The existing template resolver reads `tasks-template.md` from the override
stack; command overrides must be materialized before the agent uses them.

Before a Spec Kit refresh, preserve local changes and review the upstream
command changes against these full replacement sources. Do not use `--force`
merely to bypass modified-file protection. After an approved refresh, reconcile
the override sources, synchronize the two skills and run the checks above.
Keep `.specify/integrations/*.manifest.json` as the installer-recorded baseline;
do not rewrite its hashes to disguise customized output as untouched upstream
files. Keep the Python workflow helpers; do not restore a parallel shell tree.

See upstream [upgrade guidance](https://github.com/github/spec-kit/blob/main/docs/upgrade.md)
and [override resolution](https://github.com/github/spec-kit/blob/main/docs/reference/presets.md).
No independent TDD skill is required for this convention.

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
