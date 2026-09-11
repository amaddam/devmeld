"use strict";

const grid = document.querySelector("#products");
const message = document.querySelector("#message");
const refresh = document.querySelector("#refresh");
const template = document.querySelector("#product-template");
const cards = new Map();
let busy = false;
let fresh = false;

function notify(text, state = "info") {
  message.textContent = text;
  message.dataset.state = state;
}

function controls() {
  grid.setAttribute("aria-busy", String(busy));
  refresh.disabled = busy;
  for (const { element, product } of cards.values()) {
    element.querySelector(".buy").disabled = busy || !fresh || product.stock === 0;
    element.querySelector("input").disabled = busy || !fresh || product.stock === 0;
  }
}

async function api(path, options = {}) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 5000);
  try {
    const response = await fetch(path, { ...options, signal: controller.signal, cache: "no-store" });
    const body = await response.json();
    if (!response.ok) {
      const error = new Error(body.error || "request_failed");
      error.status = response.status;
      throw error;
    }
    return body;
  } finally {
    clearTimeout(timer);
  }
}

function showStock(card, stock) {
  card.product.stock = stock;
  card.element.dataset.empty = String(stock === 0);
  card.element.querySelector(".stock").textContent = stock;
  card.element.querySelector(".availability").textContent = stock ? "有货" : "已售罄";
  const input = card.element.querySelector("input");
  input.max = Math.max(stock, 1);
  if (Number(input.value) > stock) input.value = Math.max(stock, 1);
}

function render(products) {
  grid.replaceChildren();
  cards.clear();
  for (const product of products) {
    const element = template.content.firstElementChild.cloneNode(true);
    element.querySelector(".product-id").textContent = product.id;
    element.querySelector("h2").textContent = product.name;
    const input = element.querySelector("input");
    input.setAttribute("aria-label", `${product.name} 购买数量`);
    const card = { element, product };
    cards.set(product.id, card);
    showStock(card, product.stock);
    element.querySelector("form").addEventListener("submit", (event) => {
      event.preventDefault();
      void purchase(product.id, Number(input.value));
    });
    grid.append(element);
  }
}

async function load() {
  if (busy) return;
  busy = true;
  controls();
  notify("正在读取库存…");
  try {
    const result = await api("/products");
    render(result.products);
    fresh = true;
    notify(result.products.length ? "库存已更新。购买会实际扣减演示数据。" : "暂无商品。");
    return result;
  } catch {
    fresh = false;
    notify("无法读取库存，请确认本地服务仍在运行，再刷新重试。", "error");
  } finally {
    busy = false;
    controls();
  }
}

async function purchase(productId, quantity) {
  if (busy || !fresh) return;
  if (!Number.isSafeInteger(quantity) || quantity <= 0) return;
  const card = cards.get(productId);
  if (!card || quantity > card.product.stock) return;
  busy = true;
  controls();
  notify(`正在购买 ${card.product.name}…`);
  try {
    const result = await api("/purchases", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ product_id: productId, quantity }),
    });
    showStock(card, result.stock);
    notify(`已购买 ${quantity} 件 ${card.product.name}，剩余 ${result.stock} 件。`, "success");
    return result;
  } catch (error) {
    fresh = false;
    const text = error.status === 409 ? "库存不足，请刷新库存后调整数量。"
      : error.status === 404 ? "商品不存在，请刷新库存。"
      : error.status === 400 ? "购买数量无效，请刷新后重新填写。"
      : "购买结果未确认。请刷新库存后核对，不要直接重复购买。";
    notify(text, "error");
  } finally {
    busy = false;
    controls();
  }
}

refresh.addEventListener("click", () => { void load(); });
void load();
