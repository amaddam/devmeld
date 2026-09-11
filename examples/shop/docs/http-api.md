# Shop HTTP API

The demo has only products and stock. A purchase reduces stock; there are no
orders, users, prices, payments or refunds. All data is fictional.

Default base URL: `http://127.0.0.1:8765`. The server is local-only and has no
authentication. Do not deploy it as a public service.

| Request | Result |
| --- | --- |
| `GET /products` | Product list, sorted by ID |
| `GET /products/keyboard` | One product's ID, name and current stock |
| `POST /purchases` | Decrement stock and return the remaining quantity |

A purchase requires `Content-Type: application/json` and this body:

```json
{"product_id": "keyboard", "quantity": 2}
```

With an initial stock of 10, a successful response is HTTP 200:

```json
{"product_id": "keyboard", "purchased": 2, "stock": 8}
```

- Quantity must be a positive integer; malformed input returns HTTP 400.
- An unknown product returns HTTP 404.
- Insufficient stock returns HTTP 409 without changing stock. Concurrent
  purchases use a conditional SQL update and cannot reduce stock below zero.
- Purchases are **not idempotent**. Do not automatically retry an uncertain POST;
  it could purchase twice. Inspect stock before deciding what to do next.

## Startup data

`app.py` creates `data/shop.sqlite` on first startup, with keyboard=10, mouse=20
and monitor=5. Subsequent starts retain stock, including zero stock. Importing
the module or asking for `--help` creates no data.

The database and journal files are runtime state and are Git-ignored. DevMeld
indexes this description and source code, not the SQLite binary or live stock.
To know current stock, explicitly call the running shop's HTTP API. An index
entry is neither a health check nor a cached stock count.

For a separate fresh run, start with `--data-dir <NEW_EMPTY_DIRECTORY>`; do not
overwrite or reset an existing database to reproduce a sample response.
