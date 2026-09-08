# 003 implementation evidence

Status: implementation in progress; not Maintainer acceptance.

## Native Windows behavior cycles (2026-09-08)

Rust/Cargo 1.98.1, native MSVC host. Focused commands use
`cargo test -p devmeld --test workflow <name> --locked --offline`.

| Cycle | Observed RED | Observed GREEN |
| --- | --- | --- |
| initialization | command unavailable | preview has zero writes; confirmation creates configuration; repeat refused |
| documents_publish | entry/registration command unavailable | real binary, Unicode/spaced links, offline files, unchanged bytes/mtime |
| publication_cannot | sync overwrote registered generated source | source overlap refused, original output preserved |
| reserved_surfaces | init accepted state as output | reserved paths and duplicate entries refused |
| interrupted_create (library test) | missing recovery API (intended E0599) | pending recovery detected; external bytes preserved; successful rollback |
| registration_entry | change commands unavailable | owned output/entry migration; source and earlier config preserved |
| descriptions_validate | description command unavailable | custom attributes rendered; invalid required attributes leave old output |
| access_guidance | access command unavailable | source/tool/declaration links, no execution, explicit association removal |

Additional storage tests pass for every complete mutation boundary in a
replace/delete/receipt operation, identical external creation and cooperative lock.
These were added against the implemented journal as risk verification, not claimed
as separately observed RED cycles. Stale-preview/external-edit tests also pass.
Syntax/type errors encountered while implementing were corrected; they are not
counted as behavioral evidence.

## Dependencies

One Cargo workspace/lockfile. Domain crates remain standard-library-only.
Application: serde 1.0.229, serde_json 1.0.151, jsonschema 0.55.0 with default
features disabled, file-id 0.2.3. Cargo resolved 90 registry packages including
platform-specific dependencies. The inspected native graph has no reqwest,
tokio or HTTP retrieval feature. Schema interpretation has a significant normal
transitive dependency cost; this is not represented as a zero-dependency feature.

## Outstanding verification

Final dependency gate/probes, additional negative/recovery tests and final full
Windows/Linux acceptance are not yet recorded. macOS and automatic Agent discovery
remain unverified. No comparison benchmark or broad filesystem security guarantee.
