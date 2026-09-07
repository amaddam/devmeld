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
Spec Kit sources remain in `.specify/templates/overrides/`:

- `tasks-template.md` defines behavior-slice task structure.
- `tasks.md` and `implement.md` contain command instructions, retaining the
  existing governance checks and hooks. Their upstream baseline is Spec Kit
  1.0.1; review upstream changes when upgrading.
- `preset.yml` registers these three files as the local `devmeld-workflow`
  preset. Command sources have command frontmatter; the official generator
  supplies skill names, provenance metadata and titles.

Edit these sources, not `.specify/presets/devmeld-workflow/` (the installed
copy) or `.agents/skills/speckit-{tasks,implement}/SKILL.md` (generated output).
The installed copy, `.specify/presets/.registry` and generated skills are
checked in so a fresh checkout retains the registered workflow. They are not
additional authorities; include regenerated changes in the same review.

Use the installed Specify CLI, tested here at 1.0.1 on native Windows. These
read-only commands inspect registration and template selection; they are NOT
a source-versus-generated drift check:

```text
specify preset list
specify preset info devmeld-workflow
specify preset resolve tasks-template
```

When the preset is not registered, install it from the project-owned source:

```text
specify preset add --dev .specify/templates/overrides
```

After editing an already registered preset, first review and preserve the
complete source directory, including `preset.yml`. In Spec Kit 1.0.1, refresh
using these separate official commands, stopping if either fails:

```text
specify preset remove devmeld-workflow
specify preset add --dev .specify/templates/overrides
```

Removal deletes the installed copy, not the override source directory. Never
use `.specify/presets/devmeld-workflow` as the reinstall source. Do not run
workflow skills between removal and successful reinstallation; if installation
fails, repair the source and rerun `preset add` before continuing. The `--dev`
option copies files; it does not create a live link or watch source edits.

After refresh, inspect the generated skill diff and confirm the required rule
body is preserved, run `specify preset list` and `specify preset resolve
tasks-template`, and run `git diff --check`. The official generator may change
frontmatter formatting, provenance and the skill title; those are not behavior
changes. Reinstallation also updates the registry's installation timestamp.

Do not use `specify integration use codex` as a narrow preset-refresh shortcut:
the tested 1.0.1 command also restores the removed PowerShell helper tree and
updates the shared infrastructure manifest, even with `script: py`. Likewise,
do not run integration upgrade, init or `--force` merely to regenerate skills.
Review shared-infrastructure changes separately during an approved upgrade.
Keep `.specify/integrations/*.manifest.json` as the installer-recorded baseline;
do not rewrite its hashes to disguise customized output as untouched upstream.

The migration was verified in isolated repository copies: official install,
remove/reinstall, edited-source propagation and repeated rendering preserved
both command bodies and unrelated files, with no PowerShell tree restored.
The project no longer maintains a separate skill synchronization script.

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
