# 003 data and ownership

| Value | Owner | Source / lifetime |
| --- | --- | --- |
| Registration and ID | Resource Organization | Managed configuration; unique portable slug |
| Document / description / schema | Author | Original file; never rewritten by publication |
| Access association | Resource Organization | Pair of registered IDs; not authority or availability |
| Validated facts | Resource Organization | Loaded description/document reference; transient |
| Desired file / managed change | Context Publication | Derived bytes and before/after comparison |
| Ownership receipt | Publication adapter | Last committed bytes and physical identity; local evidence |
| Operation journal | Publication adapter | Exact targets, before/after intent and stages; recovery evidence |

Removal fails while associations reference a resource; remove them explicitly
first. Registration removal never deletes a source. All entries share one index.

Configuration and publication are separate committed operations. Failed sync
cannot undo successful registration changes or author edits. Recovery reverses
only its unfinished operation. Internal state is unnecessary for offline reading.

Domain constructors/methods enforce invariants. JSON, schema engines and file IO
belong to adapters. No aggregate is created merely to mirror a file or directory.
