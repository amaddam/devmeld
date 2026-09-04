# Tool and Dependency Guidance Semantics

**Feature**: [Domain Foundation](spec.md)

**Status**: Non-binding supporting-domain semantic sketch; design only; pending review
**Last Updated**: 2026-09-04

## Purpose and Authority

This sketch describes the facts and decisions that future tool/dependency
guidance must preserve. It does not define a public operation, wire format,
package-manager integration, installer, or execution protocol. It creates no
Capability Integration package, port, adapter, fixture, or placeholder in the
current foundation.

The cross-feature behavior in this sketch is a proposed Product change pending
Maintainer review. If accepted, its stable terms, boundaries, and semantics move
to [`docs/product.md`](../../docs/product.md); until then this sketch is not a
Product baseline. The standalone Product meaning of `Capability` remains
deferred; nothing here accepts it or adds capability selection to the current
implementation.

## Question Answered

For a task that may need a command, declared dependency, maintained project
script, runtime-provided solution, or other external tool, guidance answers:

> Which option is applicable and authorized in this project/task scope, what
> evidence supports that conclusion, and would using it require a separately
> authorized change?

Guidance does not execute the option. It also does not treat every executable,
library, or script as a Capability Provider.

## Input Facts

### Selection Requirements and Selection Authority

A selection requirement records its source and scope:

- an explicit user instruction for the current task;
- an authoritative project rule or maintained project choice;
- a Context Profile or other future accepted selection source;
- no explicit selection.

An explicit applicable selection grants selection authority for that option: it
controls the choice and prevents silent substitution. It does not grant change
authority. If the selected option cannot be used without installation,
dependency changes, environment creation, or system modification, guidance
reports the unmet prerequisite rather than assuming permission or choosing a
different option silently.

### Change Authority

Change authority is a separate fact identifying who or what permits a specific
change, the exact project/task or system surface covered, and any conditions.
It may come from an explicit instruction to perform the change or an applicable
project policy. A request only to use an option is not change authority.

No project/task-scoped change is implicitly authorized merely because it is the
smallest way to satisfy a selection. Global, system-wide, and unrelated-
environment changes remain outside task-scoped authority unless explicitly and
separately authorized.

### Project-Declared Facts

Project manifests, lockfiles, maintained scripts, and project configuration own
their declarations. Guidance references the source and revision it inspected;
it does not copy those declarations into a competing dependency registry.

Declared does not automatically mean installed, runnable, compatible, or
authorized for this task.

### Local Availability Observations

A local observation is immutable evidence about one scoped environment. It may
record:

- candidate kind and identity;
- version or other compatibility facts when relevant;
- the project, Checkout, runtime, environment, or machine scope inspected;
- whether the option was declared, discovered, or actually verified usable;
- observation time, observer/source, and an explanation.

Discovery must be bounded to the relevant project/task environment. It must not
scan unrelated user locations, execute arbitrary code, contact a remote target,
install anything, or expose credential values merely to decide availability.
No observation, or a stale observation, means unknown.

## Guidance Decision

A future guidance decision retains the considered candidates, governing
requirements, evidence, scope, and basis. Its semantic outcome is one of:

- an existing eligible option is ready, selected either by an explicit
  requirement or by evidenced reuse;
- a specific dependency, installation, or environment change is proposed and
  awaits its own applicable change authority;
- the required option is unavailable or conflicts with an applicable project
  constraint;
- available evidence is insufficient or stale, so the result is undetermined.

These descriptions are semantic categories, not accepted public enum names or
an error catalog.

When there is no explicit selection, an existing eligible project-managed
option is preferred. Convenience, familiarity, machine-wide presence, or a tool
found in another project does not establish eligibility. If no existing option
is adequate, a proposal explains the unmet need, the exact project-owned surface
that would change, and the expected effect before authorization is requested.

## Ownership Across Domains

- **Project Catalog** publishes stable project, Repository, Resource, and
  declared-source identities; it does not inspect installed tools.
- **Local Context Resolution** publishes the selected Checkout and task-local
  basis needed to scope observations; it remains a Checkout-resolution domain,
  not a general environment manager.
- **Capability Integration** owns tool/provider compatibility and the guidance
  decision semantics described here.
- **Context Knowledge** may carry the resulting evidence-backed guidance as
  context without becoming its decision owner.
- **Adapters** may inspect manifests, runtimes, executables, or supported
  environments for a later approved Feature. Adapter objects and native failures
  do not enter domain decisions.
- **Agent Clients** invoke tools and perform installation or dependency changes.
  Enforcement depends on the execution client honoring the decision and its
  authorization boundary.
- **Managed Materialization** applies only if a later Feature generates or
  manages a persistent artifact; ordinary tool invocation is not a managed
  write by default.

## Required Scenarios

1. **Explicit and ready**: a user requires one option and scoped evidence shows
   it is eligible and usable. Guidance selects it with the explicit basis.
2. **Explicit but unavailable**: the required option is missing, incompatible,
   or conflicts with a project constraint. Guidance explains the conflict and
   does not fall back to another option.
3. **No explicit selection, existing option**: a maintained project script or
   declared and verified dependency satisfies the need. Guidance reuses it
   instead of proposing a duplicate tool, dependency, or environment.
4. **No adequate existing option**: guidance proposes one bounded change with
   its rationale and target. The option is not represented as ready until the
   required change authority and a fresh post-change observation exist.
5. **Wrong scope or stale evidence**: a candidate exists globally, in another
   project, or only in an outdated observation. Guidance reports unknown or
   unavailable for the current scope rather than guessing.

## Invariants

1. An applicable explicit selection is never silently substituted.
2. Selection authority never implies change authority.
3. Project-owned declarations remain authoritative; DevMeld does not create a
   second dependency source of truth.
4. Declared, discovered, verified, compatible, and authorized remain distinct
   facts.
5. Availability is scoped; machine-wide presence does not imply project or task
   eligibility.
6. A dependency, installation, or environment change is never presented as an
   existing usable option before its own applicable authorization.
7. Observation never mutates the inspected environment or reveals credentials.
8. Unknown or stale evidence never becomes a guessed success.
9. Guidance never claims that it executed a tool or enforced an Agent Client's
   behavior.

## Deferred Choices

The first Feature that implements this supporting domain must decide, from real
consumer requirements:

- which project ecosystems and declaration sources are supported;
- how local availability is observed safely;
- how an Agent Client consumes guidance and reports whether it complied;
- how installation or dependency-change authorization is represented;
- whether any tool is also a Capability Provider under the future accepted
  Product meaning of `Capability`;
- public operations, transport, persistence, cache lifetime, and error forms.

None of these deferred choices blocks the Capability-independent three-domain
foundation or authorizes placeholder implementation.
