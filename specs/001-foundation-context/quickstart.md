# Foundation Acceptance Guide

**Feature**: [Domain Foundation](spec.md)  
**Last Updated**: 2026-09-04
**Status**: Proposed tool/dependency-guidance Product change prepared for
Maintainer review; verification becomes runnable after implementation
**Audience**: Maintainers and contributors reviewing the first code foundation

## What This Guide Proves

The design review proves that DevMeld has a coherent five-domain map and an
honest three-domain implementation boundary. After implementation is approved,
the runnable checks prove that the three core domains enforce their invariants
and dependency direction.

This guide does not prove real Git or Vault access, production persistence,
public context operations, Managed Materialization, Capability Integration,
capability selection, generated Agent artifacts, a CLI/Desktop application, or
benchmark improvement.

## 1. Confirm Accepted Decisions and Deferred Scope

Review these owning artifacts:

1. [Domain Foundation specification](spec.md)
2. [Implementation plan](plan.md)
3. [Research decisions and proposals](research.md)
4. [Conceptual domain model](data-model.md)
5. [Context semantic sketch](context-semantics.md)
6. [Materialization semantic sketch](materialization-semantics.md)

Gate 1 and Gate 2 were explicitly accepted on 2026-09-02. Before implementing
[the task list](tasks.md), confirm the authoritative records:

1. [ADR-0001](../../docs/adr/0001-domain-oriented-modular-monolith.md):
   domain-oriented modular monolith and package-by-domain organization;
2. [ADR-0002](../../docs/adr/0002-initial-python-runtime.md): Python 3.14.x,
   standard-library-first core, and Ruff/mypy/pytest;
3. [Engineering Guide](../../docs/engineering.md#runtime-and-automated-quality-gates):
   the concrete progressive check policy.

This resolves the two general prerequisites. It does not mean the code,
`pyproject.toml`, tool environment, or acceptance evidence already exists.

Gate 3, Capability Product meaning, may remain undecided. It blocks only work
that depends on that meaning in any domain. Such work would require a Maintainer
decision recorded in `docs/product.md` and an approved Feature scope. The current
foundation excludes that work, so neither task generation nor core acceptance
requires the Capability decision. Resolving Gate 3 does not automatically add
capability selection or Capability Integration to this Feature.

## 2. Confirm the Design Boundary

The domain map must assign clear ownership and invariants to all five domains:

| Domain | Design | First implementation |
| --- | --- | --- |
| Project Catalog | Required | Required; Context Profile limited to the Capability-independent subset |
| Local Context Resolution | Required | Required |
| Context Knowledge | Required | Required |
| Managed Materialization | Required | Not allowed |
| Capability Integration | Required | Not allowed |

Reject the foundation plan if either supporting domain gains a package, port,
test double, fixture, or public protocol merely to mirror the design diagram.
Also reject generic entity, CRUD, provider, storage, or metadata abstractions
that erase the three core domains' language.

An `application/` package is allowed only when an implemented scenario needs
real cross-object/domain coordination outside domain objects. Reject empty
application packages and services that merely forward calls. A domain with only
pure rules is complete for its approved scope without an application layer.

The Context Profile model retains its full Product design, but this foundation
implements only identity, naming, Repository/Resource selections, and portable
rules independent of Capability semantics. Do not introduce Capability-related
placeholder fields, default-empty lists, no-op behavior, or test fixtures.

### Review Tool and Dependency Guidance

Review the proposed Product behavior in the
[semantic sketch](tool-use-semantics.md) and its representation in the
[Domain Model](data-model.md#tool-and-dependency-guidance) using at least these
cases:

1. an explicitly selected option is verified and eligible in the current scope;
2. an explicitly selected option is unavailable or conflicts with a project
   constraint;
3. no option is selected and a maintained script or declared, verified
   dependency already satisfies the need;
4. no adequate existing option exists and a bounded dependency/environment
   change is proposed;
5. the only observation is global, from another project, or stale.

For every case, identify the authoritative project declaration, local
observation and freshness, project/task scope, selection authority, separate
change authority, and execution owner. Reject silent substitution, duplicate
dependency registries, implicit installation permission, and claims that
DevMeld itself executed or enforced the choice.

This is a design review only. Do not discover tools, inspect unrelated machine
locations, create a Capability Integration package, or introduce a placeholder
provider/port/fixture to perform it.

## 3. Review Task Context Semantics

Confirm the model and semantic sketch preserve this order:

```text
raw Task Context
    -> validation
        -> InvalidTaskContext: stop, explain, never fall back
        -> ValidTaskContext
            -> Active Checkout resolution
                -> Resolved | Ambiguous | Unavailable
```

Wrong-Repository, conflicting, unknown, and ineligible explicit Checkout
selections are validation failures. Ambiguous and Unavailable describe only
valid input evaluated against local facts.

## 4. Review Deferred Protocol Choices

The context and materialization sketches may state semantic facts and
invariants, but they must not define a v1 contract, JSON envelope, operation
catalog, pagination/cursor format, ranking representation, public error-code
catalog, approval payload, or compatibility-version policy.

Those decisions belong to the first approved Feature with a real consumer.

## 5. Verification Environment After Implementation

Use the accepted Python 3.14.x baseline and record the interpreter and quality-
tool versions selected during setup. If a runtime or baseline change becomes
necessary, update the owning ADR/Engineering artifacts before relying on it.

The local/CI sequence is:

```powershell
python -m ruff format --check src tests
python -m ruff check src tests
python -m mypy src/devmeld
python -m pytest
```

The accepted configuration keeps Ruff conservative (`E`, `F`, `I`, `UP`, `B`,
with `E501` and initial complexity/size gates disabled), gives mypy ownership of
core static typing, and uses pytest for invariant and failure-case evidence. No
database, network, Git fixture repository, Vault, Codex installation, or hosted
service is required.

## 6. Exercise Project Catalog

Using pure values and only core-required substitutes, verify:

1. a Workspace requires exactly one primary Vault reference;
2. portable Workspace, Repository, Context Profile, and Resource-registration
   values reject machine paths and current-machine choices;
3. Repository identity survives display-name and alias changes;
4. aliases cannot collide across logical Repositories;
5. the implemented Context Profile subset selects stable Repository and
   Resource references, enforces Capability-independent portable rules, and
   cannot persist an Active Checkout;
6. removing a referenced registration is rejected until dependent selections
   are revised explicitly.

Review evidence must also confirm that capability-selection fields, behavior,
and placeholders are absent. This is subset acceptance, not a claim that the
complete Product Context Profile is implemented.

## 7. Exercise Local Context Resolution

Validate raw Task Context before invoking resolution:

| Fixture | Expected semantic result |
| --- | --- |
| Explicit selection belongs to another Repository | InvalidTaskContext; no resolution or fallback |
| Explicit selections conflict | InvalidTaskContext; no resolution or fallback |
| Explicit observation is unknown or ineligible | InvalidTaskContext; no resolution or fallback |
| Valid explicit task selection | Resolved with explicit-selection basis |
| No task selection, valid workspace selection | Resolved with workspace-selection basis |
| No stronger selection, valid local default | Resolved with local-default basis |
| Exactly one eligible candidate | Resolved with sole-candidate basis |
| Two equally eligible candidates | Ambiguous with both candidates retained |
| No eligible candidate | Unavailable with an explanation |

Candidate order must not change an outcome, and a later Checkout observation
must not mutate an earlier observation.

## 8. Exercise Context Knowledge

Create representative Resource, Relation, Evidence, and Scope facts without a
real parser, index, or public query API. Verify:

1. Source Type, Review Status, Validity Status, and Scope Match can disagree and
   remain independently visible;
2. Scope Match is recomputed for a new query without mutating stored review,
   validity, Evidence, or Scope facts;
3. reversing a directed Relation changes its meaning unless the relation type
   explicitly declares symmetry;
4. an explainable context result preserves stable identity, source revision,
   Evidence, Scope, Scope Match, and applicable Checkout-resolution basis;
5. relevance cannot turn out-of-scope content into an ordinary match;
6. an unavailable relation target remains explainable rather than being
   substituted or erased.

The evidence must demonstrate [Context Semantics](context-semantics.md), not a
preselected wire representation.

## 9. Activate Dependency Enforcement Only After Packages Exist

Do not create Import Linter configuration while the domain packages are only a
plan. Once the three real core packages and their accepted dependencies exist,
add only the minimum contracts needed to reject representative violations such
as:

```text
core domain -> adapter                 REJECT
core domain -> entrypoint              REJECT
core domain A -> core domain B internals REJECT
application -> concrete adapter        REJECT
adapter -> core-owned port              ALLOW
entrypoint -> application API           ALLOW
```

Run `lint-imports` only after configuration exists. Seed controlled violations
or equivalent rule probes so an empty or ineffective scan cannot be presented
as architecture evidence.

Only the architecture probes may use isolated temporary package files or invoke
local verification tools to test these rules. They must not create fake
production layers, access real user targets, or require a hosted service.

## 10. Confirm Honest Scope

The first code foundation passes only when:

- production code exists only for the minimal shared kernel and three core
  domains;
- each port and substitute is justified by an implemented core rule;
- each application package is justified by actual coordination outside domain
  objects; absent application layers need no placeholder or substitute service;
- Context Profile covers only its declared subset; no Capability-dependent
  values, selection behavior, or placeholders have entered any core domain or
  the shared kernel;
- no `materialization/`, `capabilities/`, production `adapters/`, or
  `entrypoints/` package exists as empty architecture;
- no tool/environment discovery, dependency installation, command execution,
  enforcement integration, duplicate dependency registry, or tool-guidance
  placeholder is implemented;
- no public machine contract or real Git, Vault, filesystem, database, Codex,
  CLI, Desktop, or network behavior is claimed;
- no benchmark improvement or user-visible workflow is claimed;
- later Features must traverse accepted core rules rather than bypass them for
  delivery speed.

## Acceptance Record

When implementation is reviewed, record:

```text
Foundation revision:
Spec/Plan revision:
Architecture ADR revision (required):
Runtime ADR revision (required):
Context Profile acceptance scope: Capability-independent subset only
Capability-dependent implementation: excluded; Gate 3 not required for this acceptance
Tool/dependency-guidance Product proposal: accept / revise
Tool/dependency-guidance implementation: excluded
Verification command:
Passing tests:
Seeded dependency violations detected:
Maintainer:
Decision: ACCEPT / REVISE
Notes:
```

An ACCEPT decision means the three-domain code foundation is ready to support a
later adapter-backed Feature. It does not approve either supporting-domain
implementation, capability-selection or tool-guidance behavior, the Capability
Product meaning, or any future application protocol.
