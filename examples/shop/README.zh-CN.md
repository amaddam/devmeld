# Shop 购买系统示例

简体中文 | [English](README.md)

只有商品、库存和购买扣库存。前端页面与 HTTP API 由同一个 Python 标准库服务提供。
没有账户、订单、支付，也不提供单独的 CLI 购买工具。不需要安装框架或依赖。

## 启动

在仓库根目录执行：

```text
python3 examples/shop/app.py
```

打开 **http://127.0.0.1:8765/**，选择数量并点击“购买”，页面显示剩余库存。
“刷新库存”重新读取当前数据。Ctrl+C 停止服务。仅监听本机，不用于生产部署。

首次启动生成 `examples/shop/data/shop.sqlite`：Keyboard 10 件、Mouse 20 件、Monitor 5 件。
再次启动保留剩余库存，包括售罄状态。导入模块或查看帮助不生成数据。
需要独立的一份新数据时，指定新的空目录和端口：

```text
python3 examples/shop/app.py --port 8766 --data-dir <新的空目录>
```

## HTTP 接口

| 方法 | 路径 | 用途 |
| --- | --- | --- |
| GET | `/products` | 商品及库存列表 |
| GET | `/products/keyboard` | 单个商品及库存 |
| POST | `/purchases` | 购买，JSON 请求见下方 |

```http
POST /purchases
Content-Type: application/json

{"product_id":"keyboard","quantity":2}
```

直接使用前端页面或已有的 HTTP 工具即可。数量必须是正整数；商品不存在返回 404，
库存不足返回 409，不部分扣减。并发购买不能超卖；超时后不要直接重复提交购买。
详细约定见 [HTTP 说明](docs/http-api.md)。

## 文件与 DevMeld

Git 只保存代码、页面、测试、[原始服务描述](resources/shop.json)和文档。
`data/`、`.devmeld/`、`CONTEXT.md` 和 Python 缓存均排除。

这个 demo 不附带生成上下文快照，不提供专用 CLI 客户端，运行也不依赖 DevMeld。
需要演练上下文整理时，可以另外将原始服务描述与代码登记到 DevMeld；
登记或生成索引不会替你启动服务或执行购买。

当前 DevMeld 将注册配置保存为 `.devmeld/context.toml`，维护记录保存为
`.devmeld/state/owned.toml`，阅读入口和索引仍为 Markdown。
`resources/shop.json` 是原始服务描述，不是旧版 DevMeld 配置，因此保留 JSON。

## 用 DevMeld 整理这个项目

下面演示组自己的说明、标签和多维字段，以及资源继承公共信息、覆盖个别字段。
这些步骤只登记和生成上下文，不启动 HTTP 服务、不执行购买，也不读取实时库存。

### 1. 准备 DevMeld

在仓库根目录构建：

```text
cargo build --release --locked -p devmeld
```

下文的 `devmeld` 指本次构建的可执行文件。Windows 使用
`target/release/devmeld.exe`，Linux/macOS 使用 `target/release/devmeld`；
将其目录加入当前终端的 PATH，或用完整路径替换命令中的 `devmeld`。
以下每行都是一条完整命令，不依赖 PowerShell/Bash 的续行语法。

### 2. 首次登记

从仓库根目录进入示例目录；后续命令都在这里执行：

```text
cd examples/shop
```

**如果已经有本例的 3 个组、5 份资源和 `CONTEXT.md` 入口，跳过本节的登记命令，直接执行第 3 节。**
`add` 不是覆盖操作，重复登记会报错；不要删除已有 `.devmeld` 重来。
首条命令用 `--context .` 明确选择 Shop，之后自动找到这里的 `.devmeld`，无需反复指定。

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

这些命令保存注册配置和维护记录，尚未生成索引，也不会复制源文件。
`--kind description` 表示读取结构化资源描述文件，与下一节的文字说明 `--description` 不同。
如需接入已有 `AGENTS.md`，可用 `devmeld entry attach AGENTS.md` **替代**上面的
`entry create`；它只登记一个受管片段，实际写入仍发生在 sync。这个示例默认使用独立入口。

### 3. 填写组和资源信息

新建和已经登记的示例都执行本节。`update` 只修改列出的说明、标签、字段或继承选项，
未列出的信息保留；它不改变资源 ID、来源或关联，也不直接更新生成文件。

```text
devmeld config set language zh-CN
devmeld config set defaults.inherit true
devmeld config set defaults.propagate true
```

这里将**本例以后新建节点**的默认值设为“接收父组信息、组允许向下传递”；
不是修改 DevMeld 的内置默认值。已有节点不会自动改变，所以下面的更新仍显式写出选择。
三个顶层组不接收父组信息，但允许向下传递：

```text
devmeld group update code --description "Shop 的 HTTP 服务、浏览器交互和行为测试。修改功能时从这里定位实现，再对照 HTTP 约定验证。" --tag shop --tag source --field scope=implementation --attention "修改实现后运行 test_app.py；不要提交运行时数据库。" --no-inherit --propagate
devmeld group update services --description "本机运行的 Shop HTTP 服务及其接入资料。登记的是服务说明，不是存活状态或实时库存。" --tag shop --tag service --environment local-demo --field boundary=loopback-only --attention "先确认服务已启动；索引不代表当前连接可用。" --no-inherit --propagate
devmeld group update knowledge --description "HTTP 请求、错误响应和数据生命周期约定。调用服务或修改 API 前，先阅读这里的资料。" --tag shop --tag documentation --field scope=usage-guidance --attention "文档描述接口约定，不保存实时库存；修改接口时同步核对 app.py。" --no-inherit --propagate
```

每份资源再填写自己的说明和字段，接收所在组的标签、字段：

```text
devmeld resource update services/shop --description "定位本机演示服务的 endpoint、运行条件和调用资料。查询商品与库存使用 GET，购买使用 POST /purchases；服务启动和请求执行由使用者另行决定。" --tag http --field "use_when=需要找到服务地址或了解如何查询商品、执行购买时" --attention "购买会真实扣减演示库存；POST 结果不确定时不要自动重试。" --inherit
devmeld resource update code/backend --description "app.py 实现商品查询、库存扣减和静态页面服务；启动时初始化 SQLite，重启保留已有库存。修改 HTTP 行为或排查购买失败时，从这里进入。" --tag python --tag http --field responsibility=inventory-and-http --field "use_when=修改请求校验、库存扣减、错误响应或启动初始化时" --attention "购买扣减使用带库存条件的 UPDATE；不要在请求处理时补建或重置数据库。" --inherit
devmeld resource update code/frontend --description "web/app.js 负责读取商品、展示库存、提交购买并处理 HTTP 错误；不保存权威库存，页面结构和样式分别见 web/index.html、web/styles.css。" --tag javascript --tag browser --field responsibility=browser-interaction --field "use_when=调整购买交互、库存刷新或请求失败提示时" --attention "购买结果不确定时先刷新并核对，不要自动重试 POST。" --inherit
devmeld resource update code/tests --description "test_app.py 使用临时 SQLite 数据库和真实 loopback HTTP，验证初始化、重启保留库存、非法请求、并发购买与静态资源边界。" --tag python --tag test --field responsibility=behavior-verification --field "use_when=验证后端行为变更或定位库存相关回归时" --attention "在 Shop 项目目录运行 python3 -m unittest discover -s . -p test_app.py -v；测试不得依赖默认 data/shop.sqlite。" --inherit
devmeld resource update knowledge/http-api --description "HTTP 方法、请求体、响应、错误码与数据生命周期的使用说明，也是 Shop 服务关联的接入指引。" --tag http --tag reference --field "use_when=调用服务、理解错误响应或核对接口变更时" --inherit
```

`scope`、`responsibility`、`use_when` 是示例自定义的字段，不需要 DevMeld 增加代码。
`--attention`、`--environment` 是对应 `--field attention=...`、`--field environment=...`
的快捷写法；它们是描述信息，不是权限、执行指令或可用性验证。

### 4. 检查并生成

```text
devmeld group show code
devmeld group show services
devmeld resource show code/backend
devmeld status
devmeld sync --dry-run
devmeld sync --yes
devmeld status
```

先看 show 中的本级声明、继承后的信息和来源，再看发布预览。`sync --yes` 明确确认发布，
不会覆盖有冲突的文件。也可以改用 `sync`，在终端中回答一次 `y/N`。
再次运行 `devmeld sync --dry-run` 应显示 `0 changed target(s)`。

生成后的阅读路径：

```text
CONTEXT.md                              # DevMeld 生成的项目入口
└── .devmeld/output/index.md             # DevMeld 生成的顶层导航
    └── resources/code/code.md          # DevMeld 生成；组说明、标签、字段、直属子项链接
        └── backend.md                  # DevMeld 生成；资源说明及生效标签、字段
            └── app.py                  # 原始源码；DevMeld 不复制、不改写
```

这是链接关系，不是磁盘嵌套结构。把 `CONTEXT.md` 指定给 Agent，它可以逐层找到源码；
普通文件入口不会自动注入所有 Agent 会话。读取不需要 DevMeld 进程，也不需要启动 Shop。

磁盘上，`code.md`、`backend.md`、`frontend.md`、`tests.md` 是
`.devmeld/output/resources/code/` 中的同级文件。另外两个组在相同 output 目录下
生成 `resources/services/services.md` 和 `resources/knowledge/knowledge.md`。
子组也有自己的文档和直属子项导航；总索引不再展开所有后代的属性。

### 组的信息存在哪里、如何生效

组不是只有名字的目录。每个组都在受管 `.devmeld/context.toml` 的 `[[groups]]` 中
保存自己的 annotations：description、tags 和 fields；通过 `group add/update` 维护。
继承开关也保存在那里，不显示在阅读卡片中。不要手动编辑这份受管配置。

上述命令只是维护方式，不是 Agent 的阅读入口。同步后，Agent 从总索引进入
`code/code.md` 就能读到组的完整信息，无需执行命令或读取配置。

| 信息 | 组自己的展示 | 子组 / 资源的处理 |
| --- | --- | --- |
| description | 显示在组文档中，也作为导航的简短说明 | 不继承，子项写自己的整体说明 |
| tags | 组文档中显示生效标签 | 开启继承后合并去重 |
| fields | 组文档中显示生效字段 | 开启继承后，同名字段由最近的子项声明覆盖 |
| inherit / propagate | 通过 `group show` 检查 | 父组允许传递且子项接收，才形成继承 |

例如，`code/backend` 有自己的 description、`python`/`http` 标签、responsibility 和
`use_when`；同时得到 `code` 的 `shop`/`source` 标签和 `scope=implementation`。
它自己的 attention 覆盖组的 attention，生成卡片只展示最终值，不附加继承来源。
需要检查来源时，使用 `resource show`。
而 `knowledge/http-api` 没有本级 attention，所以继承 knowledge 的注意事项。

子组也按相同规则接收父组信息，再决定是否向下传递；组关闭 `--propagate` 不会删掉
自己的信息，资源关闭 `--inherit` 也不会删掉本级信息。默认值只影响新建，已有节点需要
`group update` / `resource update` 显式调整。修改后运行 sync，索引才会更新。
中英文切换只改变生成标签；用户填写的描述、字段值和技术术语不会自动翻译。

## 测试

以下命令在仓库根目录执行，不是在上面的 `examples/shop` 目录：

```text
python3 -m unittest discover -s examples/shop -p test_app.py -v
```

测试使用临时 SQLite 数据库和真实本机 HTTP 请求，不依赖生成产物，不改默认 demo 库存。
验证情况见[验证记录](verification.md)。
