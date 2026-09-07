# DevMeld Product

- Status: early product baseline
- Audience: users, contributors, maintainers, and Agent Clients working on DevMeld
- Purpose: explain what DevMeld is, the value it provides, and the product boundaries implementations must preserve

## What DevMeld Is

DevMeld is a local-first project context layer for software development.

It connects local code checkouts, project knowledge, developer-specific context,
and external capabilities so that Coding Agents can retrieve relevant,
traceable context without loading an entire project into one prompt.

The central product boundary is:

> **DevMeld owns context, not workflow or execution.**

DevMeld helps answer what an Agent needs to know about the current project. It
does not prescribe how a feature must be developed, replace the Coding Agent,
or take over the execution of business-code changes.

## Who It Is For

DevMeld is intended for developers and teams who:

- work across one or more related repositories;
- have useful knowledge distributed across code, Markdown, tools, and people;
- want Coding Agents to locate the right project context with less manual prompting;
- need project knowledge to remain inspectable by people;
- want project context to survive changes of machine, editor, or Agent Client.

The first target user already has two or three local Git checkouts and wants to
connect them to a small project knowledge Vault without documenting the entire
system first.

## How It Fits with Other Development Tools

AI-assisted development is separated into three replaceable planes:

```text
Workflow Plane
Spec Kit / OpenSpec / team workflow
              │
              ▼
Context Plane
DevMeld
              │
              ▼
Execution Plane
Codex / Claude Code / IDE Agent
```

These planes describe replaceable responsibilities, not mandatory sequential
stages.

- Workflow tools decide how work moves through specification, planning, tasks,
  implementation, and review.
- DevMeld provides project identity, current code context, knowledge, evidence,
  relationships, and available capabilities.
- Agent Clients read and modify code, run commands and tests, and execute the
  development loop.

Using DevMeld must not require a project to adopt a particular workflow tool or
Agent Client.

## Core Product Concepts

| Concept | Meaning |
| --- | --- |
| Workspace | A logical DevMeld work area connecting a primary Vault, optional additional knowledge sources, Context Profiles, and project resources |
| Vault | A portable, human-readable project knowledge source maintained independently of DevMeld |
| Context Profile | A named selection of Repositories, Resources, and capabilities for a particular working context |
| Repository | The stable identity of a logical Git repository, independent of a machine path or branch |
| Checkout | A concrete clone or Git worktree on the current machine |
| Active Checkout | The single Checkout selected for a Repository in the current task context |
| Local Binding | Machine-specific mappings such as Vault, Checkout, developer, and client locations |
| Resource | A product term for project content that can be selected, related, and queried; it does not require a shared implementation base class |
| Relation | A typed, directed relationship between two stable project objects |
| Evidence | Traceable support for a piece of knowledge or a relation |
| Scope | The repository, checkout, environment, version, or working context in which knowledge applies or was observed |
| Capability Provider | An external system or package that provides a capability |
| Agent Client | A client that consumes context and performs development work |
| Managed Surface | Files, configuration nodes, or controlled regions that DevMeld or a Capability Provider is authorized to maintain |

Knowledge is a product term. This document does not require a standalone
Knowledge entity, a common inheritance tree, or a particular database schema.

## Repository and Checkout Rules

A Context Profile refers to logical Repositories. Before reading or indexing
code, DevMeld resolves each Repository to one Active Checkout for the current
task.

Resolution may use an explicit task selection, the current working directory or
client workspace, and machine-local bindings. If multiple candidates remain,
DevMeld must report the ambiguity instead of silently mixing results from
multiple Checkouts.

Code query results remain traceable to their Repository, Checkout, branch,
commit, and working-tree state. The actual Git Checkout is the source of truth
for current code and Git state.

## Data and Source-of-Truth Boundaries

DevMeld separates four kinds of data:

1. **Portable source data**: code, Vault content, project configuration, and
   connected knowledge or capability sources.
2. **Local Bindings**: paths and selections that are true only on the current
   machine.
3. **Rebuildable derived state**: search indexes, extracted relationships,
   cached observations, and other data that can be recreated from its sources.
4. **Generated artifacts**: lightweight instructions, Skills, manifests, and
   client-specific context entry points.

The Vault is the source of truth for DevMeld-managed project catalog entries,
explicit configuration, and human-maintained relationships. It does not replace:

- the current Checkout as the source of truth for code behavior;
- Git as the source of truth for branch, commit, and working-tree state;
- an external system as the source of truth for its own facts.

Observed or inferred context must retain enough provenance for a person or Agent
to understand where it came from.

## Knowledge and Query Semantics

Queryable knowledge can carry independent dimensions:

- **Source Type**: whether it was declared, observed, or inferred;
- **Evidence**: why the conclusion is supported;
- **Scope**: where and when the knowledge applies or was observed;
- **Review Status**: whether a maintainer or approved process has accepted it;
- **Validity Status**: whether its supporting basis is still current.

Scope Match is calculated for the current query. Knowledge can be accepted and
current while still being out of scope for one particular query. An out-of-scope
result does not become stale merely because it was queried from another context.

Review automation may evolve over time, but public context contracts must keep
these meanings distinct rather than collapsing them into one status.

## Tool and Dependency Guidance

**Decision**: Accepted by the Maintainer on 2026-09-07 as cross-feature product
behavior.

DevMeld provides evidence-backed context about tools, dependencies, scripts,
and runtimes applicable to a task. This helps Agent Clients reuse available
options while keeping their choices explainable.

An applicable explicit user or project selection controls the choice. If the
required option is unavailable, incompatible, or conflicts with an applicable
constraint, guidance reports the reason and must not silently substitute it.

Without an explicit selection, prefer an eligible existing option requiring no
new dependency, environment, or installation change. Among such options,
project-managed options have the strongest reuse preference. If no project
option is suitable, prefer an existing local/system option verified callable,
compatible, and authorized in the current project/task scope before proposing
a change. An option must not be excluded merely because it is system-managed.
Machine-wide presence alone does not establish current-task eligibility.

Project manifests, lockfiles, maintained scripts, and configuration remain the
sources of truth for project declarations. DevMeld must not create a competing
dependency registry. Declared, discovered, verified usable, compatible, and
authorized are distinct facts. Observations retain their source, scope,
freshness, and relevant compatibility evidence; missing or stale evidence
remains unknown. Local availability is not a portable project guarantee and
must be reverified when the machine or scope changes.

Selection Authority and Change Authority are separate. A request to use a tool
authorizes its selection, but does not by itself authorize installation,
dependency changes, environment creation, or system modification. Any required
change remains a separate proposal explaining the unmet need and affected
surface, with its own applicable authorization from an explicit instruction or
project policy. Existing authorization may cover the change within its stated
scope; a tool selection cannot supply missing change authority. An option is
not ready merely because a change is authorized: fresh evidence must establish
its availability after the change.

Agent Clients retain responsibility for invocation, installation, environment
changes, and enforcement. DevMeld supplies guidance and evidence, and must not
claim that guidance alone enforces client behavior. Guidance must not expose
credential values; it may identify a required credential kind or approved
reference. A command, library, or script does not become a Capability Provider
merely because it is a candidate tool option.

## Managed Writes

DevMeld writes only within an authorized Managed Surface.

Generated or managed changes must be inspectable before application and
traceable afterward. Existing human-maintained or externally managed content
must not be silently overwritten, adopted, or merged. Overlapping Managed
Surfaces must produce a visible conflict rather than an automatic guess.

Security and automation have useful defaults, but users and maintainers can
select policies appropriate to their context. Connected content and external
capabilities cannot grant themselves additional permissions.

## Non-Goals

DevMeld is not:

- an IDE or code editor;
- a replacement for Obsidian or Git;
- a general Agent runtime or chat client;
- a required hosted SaaS platform;
- a feature, task, or specification management system;
- a development workflow engine;
- a tool that copies all project knowledge into one prompt;
- a system that makes a project unusable when DevMeld is removed.

## Product Evolution

DevMeld should prove value through a small local-first, cross-repository context
scenario before expanding to additional Agent Clients, Capability Providers, or
automation.

This document contains the product boundary and shared vocabulary contributors
need in the repository. Feature-specific behavior and scope belong in Feature
Specs. Coding and delivery guidance belongs in the engineering and contribution
guides.

Exploratory discussion may happen outside the repository and does not need to be
copied verbatim. Once a product or architecture decision is accepted, its durable
outcome must be written back to the artifact that owns it:

- update this document for a cross-feature product boundary or product term;
- update the relevant Feature Spec for accepted feature behavior or scope;
- add an ADR for an accepted architecture decision with lasting trade-offs.

Technical Plans may record provisional implementation choices, but they do not
replace the product baseline, an approved Feature Spec, or an ADR for an accepted
cross-cutting architecture decision. The product boundary changes only when
implementation, real usage, or benchmark evidence shows that the current model
cannot express or usefully serve an approved scenario.
