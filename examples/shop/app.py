"""Local HTTP shop demo. Python standard library only; not a production server."""

import argparse
from contextlib import closing
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import sqlite3
from urllib.parse import urlsplit


class PurchaseError(Exception):
    def __init__(self, status, code):
        super().__init__(code)
        self.status = status
        self.code = code


class Inventory:
    def __init__(self, data_dir):
        self.database = Path(data_dir).resolve() / "shop.sqlite"

    def initialize(self):
        """Called at server startup only. Never reseed an existing product table."""
        self.database.parent.mkdir(parents=True, exist_ok=True)
        with closing(sqlite3.connect(self.database)) as connection, connection:
            connection.execute("BEGIN IMMEDIATE")
            exists = connection.execute(
                "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'products'"
            ).fetchone()
            if not exists:
                connection.execute(
                    "CREATE TABLE products (id TEXT PRIMARY KEY, name TEXT NOT NULL, "
                    "stock INTEGER NOT NULL CHECK(stock >= 0))"
                )
                connection.executemany(
                    "INSERT INTO products (id, name, stock) VALUES (?, ?, ?)",
                    [("keyboard", "Keyboard", 10), ("mouse", "Mouse", 20), ("monitor", "Monitor", 5)],
                )

    def connect(self):
        # A missing runtime database is an error, not permission to recreate data.
        connection = sqlite3.connect(self.database.as_uri() + "?mode=rw", uri=True)
        connection.row_factory = sqlite3.Row
        return connection

    def products(self, product_id=None):
        with closing(self.connect()) as connection:
            if product_id is None:
                rows = connection.execute("SELECT id, name, stock FROM products ORDER BY id")
                return {"products": [dict(row) for row in rows]}
            row = connection.execute(
                "SELECT id, name, stock FROM products WHERE id = ?", (product_id,)
            ).fetchone()
            if row is None:
                raise PurchaseError(404, "product_not_found")
            return dict(row)

    def purchase(self, product_id, quantity):
        if not isinstance(product_id, str) or not product_id:
            raise PurchaseError(400, "invalid_product_id")
        if type(quantity) is not int or not 1 <= quantity <= 2147483647:
            raise PurchaseError(400, "quantity_must_be_a_positive_integer")
        with closing(self.connect()) as connection, connection:
            # Check and decrement in one SQL statement so concurrent buyers cannot oversell.
            changed = connection.execute(
                "UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?",
                (quantity, product_id, quantity),
            ).rowcount
            row = connection.execute(
                "SELECT stock FROM products WHERE id = ?", (product_id,)
            ).fetchone()
            if row is None:
                raise PurchaseError(404, "product_not_found")
            if not changed:
                raise PurchaseError(409, "insufficient_stock")
            return {"product_id": product_id, "purchased": quantity, "stock": row["stock"]}


class ShopHandler(BaseHTTPRequestHandler):
    def setup(self):
        super().setup()
        self.connection.settimeout(5)

    def send_body(self, status, content_type, body):
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def send_json(self, status, value):
        self.send_body(status, "application/json; charset=utf-8", json.dumps(value, ensure_ascii=False).encode("utf-8"))

    def dispatch(self):
        path = urlsplit(self.path).path
        if self.command == "GET" and path == "/products":
            return self.server.inventory.products()
        if self.command == "GET" and path.startswith("/products/"):
            return self.server.inventory.products(path.removeprefix("/products/"))
        if self.command == "POST" and path == "/purchases":
            if self.headers.get_content_type() != "application/json":
                raise PurchaseError(415, "expected_application_json")
            length = int(self.headers.get("Content-Length", "0"))
            if not 0 < length <= 4096:
                raise PurchaseError(400, "invalid_body_size")
            body = json.loads(self.rfile.read(length))
            if not isinstance(body, dict) or set(body) != {"product_id", "quantity"}:
                raise PurchaseError(400, "expected_product_id_and_quantity")
            return self.server.inventory.purchase(body["product_id"], body["quantity"])
        raise PurchaseError(404, "route_not_found")

    def handle_request(self):
        assets = {"/": ("index.html", "text/html"), "/styles.css": ("styles.css", "text/css"),
                  "/app.js": ("app.js", "text/javascript")}
        asset = assets.get(urlsplit(self.path).path) if self.command == "GET" else None
        if asset:
            try:
                body = (Path(__file__).resolve().parent / "web" / asset[0]).read_bytes()
            except OSError:
                self.send_json(500, {"error": "page_unavailable"})
            else:
                self.send_body(200, asset[1] + "; charset=utf-8", body)
            return
        try:
            result = self.dispatch()
        except PurchaseError as error:
            self.send_json(error.status, {"error": error.code})
        except (ValueError, UnicodeError):
            self.send_json(400, {"error": "invalid_json_request"})
        except sqlite3.Error:
            self.send_json(500, {"error": "inventory_unavailable"})
        else:
            self.send_json(200, result)

    do_GET = handle_request
    do_POST = handle_request


def create_server(data_dir, port=8765):
    server = ThreadingHTTPServer(("127.0.0.1", port), ShopHandler)
    server.inventory = Inventory(data_dir)
    try:
        server.inventory.initialize()
    except Exception:
        server.server_close()
        raise
    return server


def main():
    parser = argparse.ArgumentParser(description="Run the local shop demo.")
    parser.add_argument("--port", type=int, default=8765)
    parser.add_argument("--data-dir", type=Path, default=Path(__file__).resolve().parent / "data")
    args = parser.parse_args()
    try:
        server = create_server(args.data_dir, args.port)
    except (OSError, sqlite3.Error, ValueError) as error:
        parser.exit(1, f"Cannot start shop: {error}\n")
    print(f"Shop: http://127.0.0.1:{server.server_port}", flush=True)
    print(f"Data: {server.inventory.database}", flush=True)
    print("Ctrl+C to stop. Existing stock is preserved on restart.", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
