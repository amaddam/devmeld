# Shop demo verification

Date: 2026-09-10. This is example delivery evidence, not a new DevMeld Feature,
an Agent-quality benchmark or a claim of production readiness.

## Current HTTP-only demo

- Native Windows, Python 3.14.6: **5 tests passed**.
- Isolated WSL Linux, Python 3.14.4: the same **5 tests passed** from a clean copy
  at `/tmp/devmeld-shop-http-v7j7gI`, without generated context, an existing
  database or a dedicated CLI client. No dependencies were installed.
- `python3 -m unittest discover -s examples/shop -p test_app.py -v` exercises
  real loopback HTTP and temporary SQLite data: startup/restart, valid and invalid
  purchases, insufficient stock, competing buyers, startup help and static routes.
- `node --check examples/shop/web/app.js`: passed. The live preview's GET `/`
  returned HTTP 200. Automated tests verify page assets are served while source
  files and the database are not exposed as static assets.
- The dedicated purchase/query CLI client and its guide were removed. The web
  page and HTTP API remain; service startup arguments are unchanged.
- Git ignores `data/`, `.devmeld/` and `CONTEXT.md` under this demo. Generated
  context is not shipped or required by its tests. The local-only context was
  regenerated separately after cleanup, as recorded below; existing stock data
  was retained.

## Local TOML context regeneration

After the user removed the obsolete example directories and Shop's old generated
context, the current release binary was rebuilt with
`cargo build --release --locked --offline -p devmeld` on native Windows.

- A fresh `zh-CN` context was created through DevMeld commands, not by editing
  generated files: service description, backend, frontend behavior, tests and
  HTTP guide, organized under `services`, `code` and `knowledge`.
- The service's access guidance points to the HTTP guide. No removed dedicated
  CLI client or guide is registered or referenced.
- The result contains `context.toml`, `state/owned.toml`, an index, five resource
  cards and `CONTEXT.md`. No legacy JSON files remain under `.devmeld`.
  The authored `resources/shop.json` is intentionally unchanged.
- All **23 local Markdown link occurrences** resolve to existing files.
  A second preview and sync report **0 changed targets**; configuration, generated
  files and ownership evidence remain byte-identical. `status` reports up to date.
- Generation preserved the existing source files and stock database. Native
  Windows HTTP tests were rerun afterward: **5 passed**, using temporary data.
- Generated context and runtime data remain Git-ignored. Obsolete example
  directories and their temporary ignore rules are gone. This regeneration is
  local Windows evidence, not a new Linux/macOS or Agent-consumption acceptance.

## Readable card paths

The existing TOML context was updated using the rebuilt native Windows release
binary and `sync --dry-run`, then `sync --yes`. This is normal managed publication,
not manually renaming files or resetting ownership/configuration.

```text
.devmeld/output/
├── index.md
└── resources/
    ├── services/shop.md
    ├── code/backend.md
    ├── code/frontend.md
    ├── code/tests.md
    └── knowledge/http-api.md
```

- The preview contained 11 targets: five new cards, five old `r-resource-*.md`
  removals and one index update. Only unchanged owned files were withdrawn.
- All 23 local Markdown link occurrences still resolve, including the service's
  access link to `../knowledge/http-api.md` and original source links.
- Configuration, `CONTEXT.md`, inspected source files and existing SQLite data
  stayed byte-identical. Repeated preview/sync report zero changes and preserve
  the ownership receipt; status is up to date.
- These paths remain ignored local outputs. No Shop runtime behavior changed;
  the earlier HTTP test results above are not a fresh HTTP/Agent acceptance for
  this filename-only publication. DevMeld's current Windows, Linux and actual
  C:/D: test evidence is recorded in the 005 T021 acceptance section.

## Reading-oriented cards

The rebuilt native Windows release refreshed the existing context through
`sync --dry-run` followed by `sync --yes`. Seven existing targets changed:
five cards, the index and `CONTEXT.md`. No files were renamed or removed.

- `code/backend.md` now contains a short generated-file comment, its title,
  `商品、库存和 HTTP 服务`, and a link to `app.py`.
- Cards no longer contain generic original-document placeholders, repeated
  maintenance paragraphs, configuration links or saved inheritance controls.
  The entry/index carry central maintenance guidance; source attributes,
  references and access guidance remain present where applicable.
- All **17** local Markdown link occurrences resolve. This count supersedes
  the earlier 23 because the six configuration links were intentionally removed.
- Configuration, authored files and the existing SQLite database stayed
  byte-identical through regeneration. Repeated preview/sync report zero changes,
  preserve the ownership receipt, and status is up to date. Outputs remain ignored.
- Windows, isolated WSL Linux and actual C:/D: DevMeld checks passed; see the
  [005 T022 evidence](../../specs/005-context-cli/acceptance.md#t022--reading-oriented-publication-2026-09-10).
  This is not a fresh Shop HTTP, browser, Agent or macOS acceptance. No commit/push.

## Rich annotations and reproducible README commands

Both READMEs now document the complete 28-command setup and publication path,
including group-owned descriptions/tags/fields, creation defaults, resource
inheritance and local overrides. An already registered example uses the same
18-command metadata/update section without deleting or reinitializing its context.

- Actual commands were extracted from each README and replayed through the real
  CLI on fresh native Windows and isolated WSL Linux fixtures. Both languages
  passed, with three groups, five resources and all 17 local links resolving.
- Verified group information in the index, resource-local descriptions,
  inherited tags/scope, local attention overrides and the guide's inherited
  attention. Source attributes remain separate from inherited context fields.
  Repeating all update/inspection/publication commands preserved managed bytes.
- The current local Shop was updated through these commands. IDs, associations,
  source references, entry registration, seven source files and the existing
  stock database were preserved. Final preview is a no-op and status is up to date.
- The README HTTP test command passed five tests on native Windows and five in
  isolated WSL Linux, using temporary databases, not the running demo's stock.
  No HTTP implementation or authored service-description file changed.
- See [005 T023 evidence](../../specs/005-context-cli/acceptance.md#t023--rich-shop-context-and-executable-readme-2026-09-10).
  No new dependency, browser/Agent/macOS acceptance, commit or push.

## Hierarchical group documents

The latest publication separates the total index from group information:
`index.md` → `resources/code/code.md` → `backend.md` → original `app.py`.
The services and knowledge groups have equivalent generated documents. Each
contains its description, effective tags/fields and direct-child links.

- Both README recipes passed on clean Windows and isolated WSL Linux fixtures:
  28 commands per language, three groups, five resources, **23** valid links and
  unchanged source bytes. Repeated updates/sync were byte-identical.
- Normal preview/confirmed sync refreshed existing Shop: three new group pages
  and the index. Configuration, entry, all five resource cards, seven source
  files and the existing stock database were preserved. A repeated sync reports
  zero changes; status is up to date. No generated output is added to Git.
- Native Windows and isolated Linux full checks passed; four actual C:/D:
  cases passed. See [T024 evidence](../../specs/005-context-cli/acceptance.md#t024--generated-group-documents-2026-09-10).
  No new HTTP/browser/Agent/macOS acceptance, CLI installation, commit or push.

## Earlier implementation checks

Startup/purchase and page-route tests were observed failing before their respective
implementations and passing afterward. Additional edge checks passed; no unobserved
RED is claimed.

An earlier generated-Markdown check exposed spaces on empty annotation lines.
A publication regression reproduced it; the renderer now leaves those lines empty.
The exact-output test changed only for those removed spaces. At that stage, native
`RUSTUP_AUTO_INSTALL=0 cargo xtask check` passed **120 tests**, with 4 explicit
cross-drive skips. That result predates this demo-only cleanup and was not rerun
for it. Dependencies, persisted formats, link semantics and ownership behavior
were unchanged.

No browser interaction or visual QA, real Agent session, macOS run, new cross-drive
run, public hosting, commit or push was performed for this cleanup. A served page
and JavaScript syntax check are not browser interaction acceptance.
