# Technical decisions for 003

Feature-local choices, subordinate to Product and the accepted Rust/architecture
ADRs. Research does not expand scope or approve public protocols.

## Declarations

Serde JSON DTOs stay in the application. Reject unknown control fields; domain
values do not depend on serialization. Use `jsonschema` 0.55 with default features
disabled and `.offline()`. Draft 2020-12 local schemas only; no external retrieval.
Format remains an annotation, not proof of connectivity. Inspect schema controls
in schema positions, not arbitrary `const` or example data. A handwritten partial
validator is rejected. Existing Serde is cached; the schema library needs Cargo
resolution, not a new Python environment or validation executable.

Sources: [Serde](https://serde.rs/derive.html),
[JSON maps](https://docs.rs/serde_json/latest/serde_json/map/index.html),
[schema options](https://docs.rs/jsonschema/0.55.0/jsonschema/struct.ValidationOptions.html),
[schema standard](https://json-schema.org/draft/2020-12/json-schema-core).

## Managed writes

Use `File::try_lock`, a versioned journal, sibling staging, native replacement
rename and hard-link no-clobber creation. Never delete a destination to make
rename work. Persist before bytes and stage identity before mutation. Recovery
checks bytes and identity: equal bytes alone do not prove who created a file.
`file-id` supplies safe portable IDs without project-authored unsafe FFI.

Include ownership state in the same operation. Pending operations block writes;
external recovery conflicts preserve the journal and affected data. Missing
state never authorizes adopting existing output. No worker/sandbox/executor is
needed. This covers process interruption, not hostile races or power-loss atomicity.

Sources: [locks](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock),
[rename](https://doc.rust-lang.org/std/fs/fn.rename.html),
[hard links](https://doc.rust-lang.org/std/fs/fn.hard_link.html),
[file-id](https://docs.rs/file-id/0.2.3/file_id/).

## Confirmation

Commands preview by default. `--apply` prints the preview and asks for literal
`apply`; recheck inputs and targets afterwards. No opaque token or manually
maintained hash. Consumers read files without DevMeld running.
