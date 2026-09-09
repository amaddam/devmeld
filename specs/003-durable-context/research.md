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

## Local file references (2026-09-09)

Separate locating an original file from serializing its link at an output location.
[Zim's export linker](https://raw.githubusercontent.com/zim-desktop-wiki/zim-desktop-wiki/master/zim/export/linker.py)
uses output-relative paths in its applicable export scope and otherwise falls
back to the file URI. Adopt that separation, not Zim's notebook-specific rules.
[RFC 8089](https://www.rfc-editor.org/rfc/rfc8089) supplies local file-URI syntax
and filesystem-name encoding considerations. For this feature, relative links
remain preferred where representable; different Windows disk roots use an
empty-authority file URI and an independently readable absolute path.

This is a local rendering choice, not a generic URL resolver. Keep native path
resolution and sibling-volume recovery in existing application code; no URL
library, source relocation, private app scheme or always-running resolver is
needed. Reader support/permission and link correctness are separate concerns.
Cross-machine base directories, remote-index traversal and network share support
are excluded by the Maintainer's current scope, not deferred implementation tasks.
