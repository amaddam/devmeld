# DevMeld

[English](README.md) | 简体中文

DevMeld 将资源和访问指引组织成持久化、可直接阅读的项目上下文。
Agent 从项目中的一个小入口出发，按需找到所需资源；
日常读取这些内容时，不需要保持 DevMeld 运行。

产品名称为 **DevMeld**，仓库名称为 **devmeld**。

## 当前状态

项目维护者（Maintainer）于 2026-09-08 批准重新设计。
旧的 001/002 设计文档、四个领域 crate 及其测试已移除。
旧的领域划分、API 和测试结果不再作为新设计必须遵循的要求。

重新设计后的首个实现位于 `003-durable-context`：通过原生 Rust 命令维护资源注册、
访问指引关联，并发布持久化 Markdown 文件。
两个领域 crate 均不依赖第三方库；文件系统、JSON 和 Schema 适配器位于应用层。
当前没有后台服务、资源连接器、工具执行器或运行时查询 API。

## 建议先读

1. [产品说明](docs/product.md)：已确认的、以持久化产物为核心的产品方向。
2. [领域模型](docs/domain-model.md)：已审查的两个领域及其职责边界；
   具体实现选择记录在当前 Feature 文档中。
3. [工程规范](docs/engineering.md)与[贡献指南](CONTRIBUTING.md)：
   实现实践与各类决策的归属。
4. [讨论记录](docs/notes/2026-09-08-context-generation-and-consumption.md)：
   已确认的意图、已否定的方向和仍待决定的事项。
5. [重置记录](docs/notes/2026-09-08-foundation-reset.md)：
   移除与保留的内容、验证情况和工作流程评估。

[Constitution（治理原则）](.specify/memory/constitution.md)仍为草案。
[ADR-0001](docs/adr/0001-domain-oriented-modular-monolith.md)保留通用的
面向领域的模块化单体原则，不再要求沿用已废弃的领域划分。
[ADR-0003](docs/adr/0003-rust-runtime.md)保留使用 Rust 的运行时决策；
其中旧的基础代码布局仅作为历史记录。

## 运行检查

准备好项目锁定版本的 Rust 工具链、rustfmt、Clippy 和本机链接器后，运行：

```text
cargo xtask check
```

该命令通过 Rust 原生进程 API 执行格式检查、编译检查、采用宽松规则的 Clippy 检查
和 Cargo 测试，不需要 PowerShell、Bash 或 Python。
应用依赖记录在工作区锁文件中。首次使用 `cargo fetch --locked` 获取依赖，
后续检查和构建使用本地缓存。

实际测试结果、待验证事项和平台限制见 [003 验证记录](specs/003-durable-context/acceptance.md)。
旧基础实现的测试结果仅作为历史记录。

## 试用基于文件的上下文

按照[可运行示例](examples/README.md)，完成初始化，注册文档与服务、工具描述，
建立关联，预览并显式同步。之后即可从生成的入口查阅上下文，
无需让 DevMeld 持续运行。

命令默认只进行只读预览；使用 `--apply` 时会要求确认。
源文件仍由人或 Agent 编写和维护，请勿直接修改生成产物。
操作中断后可通过显式的 `recover` 流程恢复。
当前 Feature 不执行工具、不连接网络，也不进行安装。

### 生成内容的语言

每套上下文可选择英文（`en`，默认）或简体中文（`zh-CN`）。
新上下文通过 `init --language zh-CN` 指定；已有上下文使用：

```text
devmeld --context PATH language zh-CN --apply
devmeld --context PATH sync --apply
```

每条命令均需检查预览并输入 `apply` 确认。第一条保存语言设置，
第二条更新索引、资源页和入口。通过 `language en` 可切回英文。
当前配置未设置语言时使用英文，不随系统语言变化。

仅 DevMeld 生成的固定说明文字参与本地化。用户内容、ssh、http、curl
等技术名称，以及命令、字段名、ID 和链接保持原样。
此设置不翻译源文件，也不规定 Agent 的回复语言；CLI 帮助和诊断仍使用英文。
直接从当前仓库运行的完整命令见[示例](examples/README.md)。

### 项目说明文件中的入口

明确指定一个普通 UTF-8 项目说明文件，不进行自动发现：

```text
devmeld --context PATH entry add /path/to/project/AGENTS.md --kind instructions --apply
devmeld --context PATH sync --apply
```

新上下文也可通过 `init --instruction-entry PATH` 登记。
登记命令只修改配置，确认 `sync` 后才插入小段导航；片段之外的作者内容保持原样。
使用 `entry remove PATH` 后再次同步，只移除片段，即使宿主文件变空也不会删除它。
原有 `--entry` 和默认的 `entry add` 仍表示整份文件由 DevMeld 生成。

当前处于**未定版的 v0 开发阶段**，本次是设计演进，不是第二个产品版本发布。
内部草稿标记不代表稳定格式承诺，索引和资源正文不加版本标签。
不兼容的早期开发记录会原样保留并拒绝维护，不自动迁移；请使用独立的新路径，
不要删除或修改旧记录的格式标记来绕过检查。
实际验证的平台和客户端配置见 [004 验证记录](specs/004-project-entry-integration/acceptance.md)；
文件名正确不等于 Agent 已自动发现入口。

### 不同目录与磁盘上的本地文件

源文件保留在原位置，可用本机绝对路径注册，例如
`resource add notes --document "D:/knowledge/notes.md"`。
资源、输出与普通入口文件可以位于不同的本地 Windows 磁盘。
同盘使用相对于当前 Markdown 文件的链接；跨盘使用 `file:///D:/...`，
并附可直接识别的本地绝对路径。空格、中文、`#`、`%` 等字符会正确编码。

当前只处理本机上下文。远程服务地址可保存在资源描述的属性中，
但不会作为索引被拉取或展开，也不做跨机器路径映射。
部分 Markdown 阅读器禁止打开文件链接，此时可通过有权限的本地读取工具
使用旁边的路径；不需要运行 DevMeld，也不承诺所有阅读器均可点击或跨机器通用。
完整用法见[示例](examples/README.md)。

## 重新设计期间的工作流程

当前开发路径是 [003 持久化上下文发布](specs/003-durable-context/spec.md)，
对应[实现计划](specs/003-durable-context/plan.md)与
[行为任务](specs/003-durable-context/tasks.md)。不要恢复执行旧的 001/002 任务。
维护者已授权继续推进并自行验证经过审查的文件式上下文方案；
实现进度与验收情况分别记录。

当前扩展为 [004 项目指引入口接入](specs/004-project-entry-integration/spec.md)：
在明确选定的项目指引文件内维护小段入口，保留用户编写的其他内容。
范围已于 2026-09-09 获得维护者确认；[Plan](specs/004-project-entry-integration/plan.md)
采用单一当前草稿维护模型，实现与验证进度记录在
[任务](specs/004-project-entry-integration/tasks.md)和[验证记录](specs/004-project-entry-integration/acceptance.md)中。
不会自动迁移已有数据。
Spec Kit 的当前 Feature 指针已指向 004。

Spec Kit 的可选开发辅助工具使用已配置的 Python 工作流程；
它们不是 DevMeld 或其生成上下文的运行时依赖。
是否继续使用完整的 Spec Kit 工作流程，仍是一个独立的待决事项。

重启与设计快照已保留在本地 Git 历史中。远程发布仍由维护者负责。
