"""Run with: python3 -m unittest discover -s examples/shop -p test_app.py -v"""

from concurrent.futures import ThreadPoolExecutor
from contextlib import contextmanager
import http.client
import json
from pathlib import Path
import tempfile
from threading import Thread
import unittest

from app import create_server


@contextmanager
def running_shop(data_dir):
    server = create_server(data_dir, port=0)
    thread = Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        yield server.server_port
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)


def request(port, method, path, body=None):
    connection = http.client.HTTPConnection("127.0.0.1", port, timeout=5)
    try:
        headers = {"Content-Type": "application/json"}
        connection.request(method, path, json.dumps(body) if body is not None else None, headers)
        response = connection.getresponse()
        return response.status, json.loads(response.read())
    finally:
        connection.close()


class ShopTest(unittest.TestCase):
    def test_page_assets_are_served_but_source_and_database_are_not(self):
        with tempfile.TemporaryDirectory(prefix="devmeld-shop-page-") as root:
            with running_shop(Path(root)) as port:
                for path, media_type in (("/", "text/html"), ("/styles.css", "text/css"), ("/app.js", "text/javascript")):
                    with self.subTest(path=path):
                        connection = http.client.HTTPConnection("127.0.0.1", port, timeout=5)
                        try:
                            connection.request("GET", path)
                            response = connection.getresponse()
                            self.assertEqual(200, response.status)
                            self.assertIn(media_type, response.getheader("Content-Type"))
                            self.assertTrue(response.read())
                        finally:
                            connection.close()
                for path in ("/data/shop.sqlite", "/app.py", "/../app.py", "/web/../data/shop.sqlite"):
                    self.assertEqual(404, request(port, "GET", path)[0])

    def test_startup_generates_products_and_restarting_keeps_the_remaining_stock(self):
        with tempfile.TemporaryDirectory(prefix="devmeld-shop-test-") as root:
            data = Path(root) / "data"
            self.assertFalse(data.exists())
            with running_shop(data) as port:
                self.assertTrue((data / "shop.sqlite").is_file())
                status, products = request(port, "GET", "/products")
                self.assertEqual(200, status)
                self.assertEqual([
                    {"id": "keyboard", "name": "Keyboard", "stock": 10},
                    {"id": "monitor", "name": "Monitor", "stock": 5},
                    {"id": "mouse", "name": "Mouse", "stock": 20},
                ], products["products"])
                status, bought = request(port, "POST", "/purchases", {"product_id": "keyboard", "quantity": 2})
                self.assertEqual(200, status)
                self.assertEqual({"product_id": "keyboard", "purchased": 2, "stock": 8}, bought)
            with running_shop(data) as port:
                status, product = request(port, "GET", "/products/keyboard")
                self.assertEqual(200, status)
                self.assertEqual(8, product["stock"])

    def test_invalid_or_unavailable_purchases_do_not_change_inventory(self):
        with tempfile.TemporaryDirectory(prefix="devmeld-shop-test-") as root:
            with running_shop(Path(root)) as port:
                for quantity in (0, -1, True, 1.5, "2", None, 2**63):
                    with self.subTest(quantity=quantity):
                        status, _ = request(port, "POST", "/purchases", {"product_id": "keyboard", "quantity": quantity})
                        self.assertEqual(400, status)
                self.assertEqual((409, {"error": "insufficient_stock"}), request(
                    port, "POST", "/purchases", {"product_id": "keyboard", "quantity": 11}))
                self.assertEqual((404, {"error": "product_not_found"}), request(
                    port, "POST", "/purchases", {"product_id": "missing", "quantity": 1}))
                self.assertEqual(404, request(port, "GET", "/products/missing")[0])
                self.assertEqual(400, request(port, "POST", "/purchases", ["not", "an", "object"])[0])
                self.assertEqual(10, request(port, "GET", "/products/keyboard")[1]["stock"])

    def test_competing_purchases_cannot_oversell_or_reseed_an_empty_stock(self):
        with tempfile.TemporaryDirectory(prefix="devmeld-shop-test-") as root:
            data = Path(root)
            with running_shop(data) as port:
                def buy(_):
                    return request(port, "POST", "/purchases", {"product_id": "monitor", "quantity": 1})[0]
                with ThreadPoolExecutor(max_workers=4) as pool:
                    statuses = list(pool.map(buy, range(8)))
                self.assertEqual(5, statuses.count(200))
                self.assertEqual(3, statuses.count(409))
                self.assertEqual(0, request(port, "GET", "/products/monitor")[1]["stock"])
            with running_shop(data) as port:
                self.assertEqual(0, request(port, "GET", "/products/monitor")[1]["stock"])

    def test_help_does_not_initialize_a_database(self):
        import subprocess
        import sys

        with tempfile.TemporaryDirectory(prefix="devmeld-shop-help-") as root:
            data = Path(root) / "not-created"
            result = subprocess.run(
                [sys.executable, str(Path(__file__).with_name("app.py")), "--data-dir", str(data), "--help"],
                capture_output=True, text=True, timeout=10,
            )
            self.assertEqual(0, result.returncode, result.stderr)
            self.assertFalse(data.exists())


if __name__ == "__main__":
    unittest.main()
