# Shop demo

[简体中文](README.zh-CN.md) | English

Products, stock, and purchases that decrease stock. The browser page and HTTP API
share one Python standard-library server. No accounts, orders, payments, separate
CLI client, npm packages or third-party Python dependencies.

## Run

From the repository root, using an existing Python 3 with `sqlite3`:

```text
python3 examples/shop/app.py
```

Open **http://127.0.0.1:8765/**. Select a quantity and click Purchase (购买).
The page displays remaining stock; Refresh reads the current inventory.
Use Ctrl+C to stop. The server is local-only, not intended for production.

The first startup creates `examples/shop/data/shop.sqlite`: keyboard=10, mouse=20,
monitor=5. Subsequent starts preserve stock, including zero. Importing the module
or requesting help does not generate data. For a separate fresh dataset:

```text
python3 examples/shop/app.py --port 8766 --data-dir <NEW_EMPTY_DIRECTORY>
```

## HTTP API

| Method | Path | Behavior |
| --- | --- | --- |
| GET | `/products` | List products and stock |
| GET | `/products/keyboard` | Read one product |
| POST | `/purchases` | Buy using the JSON body below |

```http
POST /purchases
Content-Type: application/json

{"product_id":"keyboard","quantity":2}
```

Use the browser page or an existing HTTP tool. Quantity must be a positive integer;
unknown products return 404, insufficient stock returns 409 without a partial purchase.
Concurrent purchases cannot oversell. Do not automatically retry an uncertain POST.
See the [HTTP contract](docs/http-api.md) for details.

## Files and DevMeld

Git retains source, page assets, tests, [service description](resources/shop.json)
and documentation. `data/`, `.devmeld/`, `CONTEXT.md` and Python caches are ignored.
There is no bundled generated context, dedicated purchase CLI or DevMeld runtime
dependency. The service description is authored input you can register with DevMeld
separately; registration and context generation never start the shop or buy goods.

DevMeld now stores registration in `.devmeld/context.toml` and ownership records
in `.devmeld/state/owned.toml`; reading entries and indexes remain Markdown.
`resources/shop.json` is authored service data, not legacy DevMeld configuration,
so it remains JSON.

## Organize this project with DevMeld

This walkthrough demonstrates group-owned descriptions, tags and named fields,
plus resource inheritance and local overrides. It registers and publishes context;
it does not start HTTP services, purchase products or inspect live inventory.

### 1. Prepare DevMeld

Build from the repository root:

```text
cargo build --release --locked -p devmeld
```

Below, `devmeld` means this build: `target/release/devmeld.exe` on Windows or
`target/release/devmeld` on Linux/macOS. Add its directory to the current terminal's
PATH, or substitute its full path. Each line is one complete command, without
PowerShell/Bash-specific line continuations.

### 2. Register for the first time

From the repository root, enter the example; run subsequent commands here:

```text
cd examples/shop
```

**If this example already has its three groups, five resources and CONTEXT.md
entry registered, skip the registration commands and proceed to step 3.**
`add` does not overwrite existing registrations. Do not delete an existing
`.devmeld` to repeat the tutorial. Only the first command needs `--context .` to
select Shop explicitly; subsequent commands find its local `.devmeld` automatically.

```text
devmeld --context . group add code
devmeld group add services
devmeld group add knowledge
devmeld resource add resources/shop.json --as services/shop --kind description
devmeld resource add app.py --as code/backend
devmeld resource add web/app.js --as code/frontend
devmeld resource add test_app.py --as code/tests
devmeld resource add docs/http-api.md --as knowledge/http-api
devmeld access add services/shop knowledge/http-api
devmeld entry create CONTEXT.md
```

These commands save registration and maintenance records, not the published index;
source files are not copied. `--kind description` selects a structured description
file, unlike the textual `--description` annotation in the next step.
To use an existing `AGENTS.md`, substitute `devmeld entry attach AGENTS.md` for
`entry create`; it registers a managed insertion, written only on sync. This example
uses the separate file entry by default.

### 3. Describe groups and resources

Run this section for either a new or an already registered example. `update`
changes only the listed annotations/choices, preserving omitted information,
resource identity, source and associations. Generated files stay unchanged until sync.

```text
devmeld config set language en
devmeld config set defaults.inherit true
devmeld config set defaults.propagate true
```

These defaults apply to **new nodes in this example**, not DevMeld's built-in
defaults. They do not change existing nodes, so the updates below explicitly
set their choices. Root groups refuse incoming inheritance but allow propagation:

```text
devmeld group update code --description "Shop HTTP implementation, browser interaction and behavior tests. Locate implementation here, then verify changes against the HTTP contract." --tag shop --tag source --field scope=implementation --attention "Run test_app.py after implementation changes; do not commit the runtime database." --no-inherit --propagate
devmeld group update services --description "Locally running Shop HTTP services and access information. Registration describes a service, not its health or current stock." --tag shop --tag service --environment local-demo --field boundary=loopback-only --attention "Confirm the service is running; an index entry is not an availability check." --no-inherit --propagate
devmeld group update knowledge --description "HTTP requests, error responses and data lifecycle guidance. Read before calling a service or changing its API." --tag shop --tag documentation --field scope=usage-guidance --attention "Documentation describes contracts, not live stock; check app.py when changing the API." --no-inherit --propagate
```

Each resource adds its own description/fields and receives its group's tags/fields:

```text
devmeld resource update services/shop --description "Find the local demo endpoint, runtime requirements and calling guidance. Use GET for products and stock, POST /purchases to buy; service startup and request execution are separate user decisions." --tag http --field "use_when=Finding the service address or learning how to query products and purchase" --attention "Purchases decrement real demo stock; do not automatically retry an uncertain POST." --inherit
devmeld resource update code/backend --description "app.py implements product queries, stock decrements and static page serving. Startup initializes SQLite; restarts preserve stock. Start here for HTTP behavior changes or purchase failures." --tag python --tag http --field responsibility=inventory-and-http --field "use_when=Changing request validation, stock decrements, error responses or startup initialization" --attention "Keep stock decrements in a conditional UPDATE; do not recreate or reset the database during request handling." --inherit
devmeld resource update code/frontend --description "web/app.js loads products, displays stock, submits purchases and handles HTTP errors. It does not own stock; page structure and styling are in web/index.html and web/styles.css." --tag javascript --tag browser --field responsibility=browser-interaction --field "use_when=Changing purchase interaction, stock refresh or request failure messages" --attention "Refresh and reconcile an uncertain purchase; do not automatically retry POST." --inherit
devmeld resource update code/tests --description "test_app.py uses temporary SQLite databases and real loopback HTTP to verify initialization, restart persistence, invalid requests, concurrent purchases and static asset boundaries." --tag python --tag test --field responsibility=behavior-verification --field "use_when=Checking backend behavior changes or investigating inventory regressions" --attention "From the Shop project directory run python3 -m unittest discover -s . -p test_app.py -v; tests must not depend on the default data/shop.sqlite." --inherit
devmeld resource update knowledge/http-api --description "HTTP methods, request bodies, responses, error codes and data lifecycle guidance; also the access-guidance resource associated with Shop." --tag http --tag reference --field "use_when=Calling the service, interpreting errors or reviewing API changes" --inherit
```

`scope`, `responsibility` and `use_when` are example-defined fields, requiring no
DevMeld code changes. `--attention` and `--environment` are shortcuts for the
corresponding `--field attention=...` and `--field environment=...`. These values
are descriptions, not permissions, executable configuration or verified availability.

### 4. Inspect and publish

```text
devmeld group show code
devmeld group show services
devmeld resource show code/backend
devmeld status
devmeld sync --dry-run
devmeld sync --yes
devmeld status
```

Review local/effective information and origins in show, then inspect the publication
preview. `sync --yes` confirms publication but cannot override conflicts. Use
`sync` instead to answer one terminal `y/N` prompt. A subsequent
`devmeld sync --dry-run` should report `0 changed target(s)`.

The published reading path is:

```text
CONTEXT.md                              # generated project entry
└── .devmeld/output/index.md             # generated top-level navigation
    └── resources/code/code.md          # generated group description, tags, fields and child links
        └── backend.md                  # generated resource description and effective metadata
            └── app.py                  # original source, neither copied nor rewritten
```

This represents links, not nested disk folders. Point an Agent at `CONTEXT.md`
to follow the navigation to source. A plain file does not inject itself into all
Agent sessions. Reading requires neither a DevMeld process nor a running Shop.

On disk, `code.md`, `backend.md`, `frontend.md` and `tests.md` are siblings in
`.devmeld/output/resources/code/`. The other groups publish
`resources/services/services.md` and `resources/knowledge/knowledge.md` under
the same output directory. Subgroups have their own documents and direct-child
navigation too; the total index does not expand every descendant's metadata.

### Where group information lives and how it applies

A group is not just a folder name. Each group owns annotations (description,
tags and fields) in `[[groups]]` within managed `.devmeld/context.toml`, maintained
through `group add/update`. Saved inheritance controls also live there, not in
reading cards. Do not edit this managed configuration directly.

These commands are maintenance interfaces, not the Agent's reading interface.
After sync, an Agent finds the group's complete information in `code/code.md`
through the total index, without invoking DevMeld or reading configuration.

| Information | Group's own presentation | Child group / resource behavior |
| --- | --- | --- |
| description | In the group document; also a short navigation summary | Not inherited; each child describes itself |
| tags | Effective values in the group document | Union/deduplication when inheritance is enabled |
| fields | Effective values in the group document | Nearest child-local declaration overrides a matching key |
| inherit / propagate | Inspect with `group show` | Both parent propagation and child receipt must be enabled |

For example, `code/backend` has its own description, `python`/`http` tags,
`responsibility` and `use_when`. It also receives `shop`/`source` and
`scope=implementation` from `code`. Its local attention overrides the group's
attention; the card shows the final values without inheritance-origin labels.
Inspect origins through `resource show`. In contrast, `knowledge/http-api` has no local attention and
receives the knowledge group's guidance.

Subgroups use the same rules to receive and propagate information. Disabling a
group's `--propagate` does not remove its own information; disabling a resource's
`--inherit` does not remove its local information. Defaults affect creation only;
use `group update` / `resource update` to change existing choices, then sync.
Generated labels can switch languages, but authored descriptions, values and
technical terms are never translated automatically.

## Test

Run from the repository root, not the `examples/shop` directory used above:

```text
python3 -m unittest discover -s examples/shop -p test_app.py -v
```

Tests use temporary SQLite databases and real loopback HTTP. They do not depend
on generated context or change the default demo's stock. See [verification](verification.md).
