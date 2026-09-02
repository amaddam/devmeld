# ADR-0001: Domain-Oriented Modular Monolith

- Status: Accepted
- Decision date: 2026-09-02
- Scope: DevMeld application architecture; initial realization in Domain Foundation
- Supersedes: none

## Decision Record

The Maintainer approved this architecture decision (Gate 1) on 2026-09-02.
The runtime decision is recorded separately in
[ADR-0002](0002-initial-python-runtime.md).

This decision does not ratify the Constitution, approve Capability Product
semantics, or authorize supporting-domain implementation.

## Context

DevMeld must preserve distinct ownership, authority, and lifecycles for portable
project identity, local Checkout facts, and explainable context. Organizing the
core around individual workflows or global technical layers would make those
rules easier to duplicate or bypass as features grow.

The [foundation design](../../specs/001-foundation-context/data-model.md) maps
five domains. Its [implementation scope](../../specs/001-foundation-context/spec.md)
contains only Project Catalog, Local Context Resolution, and Context Knowledge.
Managed Materialization and Capability Integration remain design only.

## Decision

Use a domain-oriented modular monolith: one application/deployment boundary with
code organized by domain. Domain boundaries are language and ownership
boundaries, not separate services, databases, or processes.

1. Each implemented core domain owns its rules, immutable facts, and applicable
   value objects or aggregates. DDD does not require a class for every noun.
2. Domain code depends on its own domain and a minimal shared kernel. Only
   genuinely identical cross-domain values belong in that kernel; it is not a
   shared utilities, DTO, status, or entity-base package.
3. Cross-domain coordination uses stable identities and explicitly exposed
   immutable facts, never another domain's mutable internals or persistence
   adapter. Internal fact boundaries do not freeze a public application protocol.
4. An `application/` package is created only for actual cross-object/domain
   coordination that belongs outside domain objects. Do not create an empty
   application package, generic application service, or pass-through wrapper
   merely to mirror a diagram. Domain policies remain in the domain even when
   they examine several values.
5. A `ports/` package is created only for an external fact required by an
   implemented core rule. A prospective adapter alone is not justification.
6. Adapters and entrypoints arise only with approved external integration and
   consumer requirements. Neither is implemented by this foundation.
7. No microservices, event bus, event sourcing, Generic Repository, generic CRUD
   service, or global `models/` / `services/` layer is introduced by default.

The dependency directions are authoritative; the number of folders is not.
The detailed first-code layout and conditional packages remain in the
[Technical Plan](../../specs/001-foundation-context/plan.md).

## Alternatives Considered

- **Use-case-first transaction scripts**: straightforward for an isolated flow,
  but do not make cross-feature invariant ownership sufficiently explicit for
  this foundation.
- **Global horizontal layers**: group similar technical shapes while obscuring
  the language and lifecycle that owns each rule.
- **Microservices per domain**: add deployment and distributed-consistency
  obligations without a demonstrated requirement.
- **All five domains and all layers immediately**: create speculative packages
  and interfaces. Complete design does not imply complete implementation.

## Consequences

- Contributors must place each rule with its domain owner and justify new
  coordination or integration boundaries with actual implemented needs.
- Pure invariant tests can run without production sources or adapters.
- Package-by-domain does not by itself enforce isolation. After real core
  packages exist, introduce the minimum Import Linter dependency contracts and
  demonstrate them with controlled violations.
- Some small values or mappings may remain domain-local instead of sharing a
  misleading abstraction. A boundary may evolve when real feature evidence
  warrants an explicit design change.
- This accepts application architecture, not an API protocol, storage engine,
  complete product model, or all future package names.

## Revisit When

Revisit this ADR if measured deployment, scaling, isolation, or integration
requirements cannot be met within the modular monolith. A new noun, interface,
or supporting-domain feature alone is not evidence that separate services or
mandatory layers are needed.
