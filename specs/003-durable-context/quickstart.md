# 003 acceptance guide

After one reviewed dependency fetch, build with
`cargo build --package devmeld --locked --offline`. Full gate: `cargo xtask check`.
Use the native binary; no Python or shell-specific runtime helper.

The [checked-in example inputs and commands](../../examples/README.md) provide a
reproducible starting point. They are authored fixtures, not generated output.

In an isolated context, create a document, service/tool JSON descriptions, tool
instructions and dependency declaration. Include a Unicode/spaced path.

1. `init --entry ../project/CONTEXT.md` previews without writes. Add `--apply`,
   inspect and confirm `apply`. Register the document and descriptions similarly.
2. Add service-to-tool association, preview `sync`, explicitly apply.
3. Close DevMeld; follow entry → index → resource → original files. Original bytes
   remain unchanged; automatic Agent discovery is not claimed.
4. Repeat sync twice: zero changed targets, identical bytes and modification times.
5. Edit service attributes; valid custom fields update output, invalid required
   fields fail before writes. No connection or installation happens.
6. Removing an associated tool fails until its association is explicitly removed.
   Unregister/sync removes obsolete managed pages, never originals.
7. Generated-file edits/unowned targets fail. Alter a source during confirmation:
   reject stale preview. Configuration commits survive a failed later publication.
8. Storage tests interrupt each mutation boundary, then recover/retry; external
   edits remain intact. Test aliases, redirected paths, versions, remote schema
   references, cooperative locks and output/entry movement/removal.

Record actual RED/GREEN commands, dependency/boundary checks, toolchain and hosts
in `acceptance.md`. Required executed hosts: Windows and Linux (WSL accepted).
macOS and automatic Agent loading stay unverified unless actually run. Passing
tests does not grant Maintainer acceptance or approve future integrations.
