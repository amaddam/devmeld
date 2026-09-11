# Examples

## Shop：可运行的购买系统

[简体中文](shop/README.zh-CN.md) | [English](shop/README.md)

当前只维护 Shop 示例：提供前端页面和 HTTP API，包含商品、库存和购买扣库存。
首次启动生成本地 SQLite 数据，不需要安装第三方依赖。

从仓库根目录启动：

```text
python3 examples/shop/app.py
```

打开 http://127.0.0.1:8765/。通过页面或 HTTP API 使用，不提供专用 CLI 客户端。

代码、页面、测试、[HTTP 说明](shop/docs/http-api.md)和
[原始服务描述](shop/resources/shop.json)保存在 Git 中。
业务数据、DevMeld 生成上下文和内部状态不入库。

需要体验 DevMeld 时，可按[项目使用说明](../README.zh-CN.md#快速开始)
注册这些真实文件。Shop 本身不依赖 DevMeld 运行，也不附带预生成索引。

运行测试：

```text
python3 -m unittest discover -s examples/shop -p test_app.py -v
```

测试使用临时数据，不修改默认 demo 的库存。详见[验证记录](shop/verification.md)。
