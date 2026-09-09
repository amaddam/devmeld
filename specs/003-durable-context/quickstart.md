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
9. Initialize a fresh fixture with `--language zh-CN`, then publish a document,
   service/tool description and multiple file entries. Check Chinese fixed text,
   unchanged ssh/http/curl/JSON names, authored fields and valid links. A source
   summary literally equal to `Original document` must remain authored text.
10. For an existing English context, preview `language zh-CN`, confirm it, then
    separately preview/confirm `sync`. Configuration changes must not publish
    implicitly. Switching back with `language en` and sync restores the original
    English bytes; repeats change nothing. Missing language defaults to English
    even under a Chinese process locale. Invalid codes/types, stale previews,
    external edits and interrupted changes must preserve the existing boundaries.

11. On Windows, put sources on a second real local drive, with Chinese, spaces,
    `#`, `%` and parentheses in names. Publish on the first drive with an entry
    on the second; follow the file URIs and compare their decoded targets with
    the originals. Move output to the second drive, add an entry on the first,
    and verify configuration/source links in both languages. Check unchanged
    no-op, external conflicts and recovery at every relocation write boundary.
    Remote address attributes remain literal data, never fetched index sources.

The two-drive tests are opt-in, so normal checks do not require another disk.
Set `DEVMELD_TEST_OTHER_ROOT` to an existing directory on a **different local
Windows drive** from the process temporary directory, then run:

```text
cargo test -p devmeld cross_drive --locked --offline -- --ignored
```

The tests create and clean their own uniquely named children there. An unset or
same-drive setting fails, not a silent skip. Path-string unit tests also run in
the default gate; they alone do not establish actual two-drive IO support.

Record actual RED/GREEN commands, dependency/boundary checks, toolchain and hosts
in `acceptance.md`. Required executed hosts: Windows and Linux (WSL accepted).
macOS and automatic Agent loading stay unverified unless actually run. Passing
tests does not grant Maintainer acceptance or approve future integrations.
