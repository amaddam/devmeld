# DevMeld CLI Style

- 状态：2026-09-10 分层帮助已实现；本轮平台与回归证据见 [005 Acceptance](../specs/005-context-cli/acceptance.md)。
- 归属：[Engineering Guide](engineering.md) 的专项约定。具体命令、默认值、保存与确认行为仍由 [005 CLI Contract](../specs/005-context-cli/contracts/cli.md) 持有。
- 目标：命令帮助是速查卡，不是使用手册；先找到操作，再看清当前参数。

完整使用手册将由专门网页承载，包括首次使用流程、概念解释、继承规则、资源格式、场景示例和排障。CLI 不复制这些教程；目前仍以现有 README 和契约文档保留详细说明。网站尚未上线前，不添加占位链接或另建一套长帮助；实际地址确定后再提供简短文档链接。

## 参考与取舍

Kubernetes 的 `kubectl` 提供了几个有用的对照，但不作为 DevMeld 的行为规范：

| 参考 | DevMeld 的选择 |
| --- | --- |
| `kubectl` 的常见结构是操作、资源类型、名称 | 保留对象在前的 `resource add`、`group list`；整体操作保留 `sync`、`status`，不为形式统一额外套一层 |
| 命令参考把 Examples、Options、继承的选项分开 | 帮助分层展示，参数逐项解释，示例只针对当前命令 |
| `kubectl options` 集中展示继承选项 | 当前全局选项很少，顶层解释即可；子命令简短列出可用项，不新增 `options` 命令 |
| 官方脚本建议显式指定上下文并采用机器输出 | 脚本示例显式选择 DevMeld 上下文；不把 human-readable 输出当稳定机器协议，也不顺带新增 JSON/YAML 输出 |

依据：[kubectl syntax and output](https://kubernetes.io/docs/reference/kubectl/)、[get reference](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_get/)、[options reference](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_options/)、[usage conventions](https://kubernetes.io/docs/reference/kubectl/conventions/)。

对象在前适合 DevMeld 当前按资源、组、入口组织操作的使用方式；这是本项目的设计选择，不代表动词在前的 CLI 不合理。也不照搬 Kubernetes 的资源缩写、服务端 apply、watch、插件发现或集群运行模型。

## 命令与参数

- 保留 `devmeld <对象> <操作>`；独立整体操作不强行改造成对象操作。命令不是领域目录或类名的映射。
- 一个操作保留一个主要写法：`list` 列表、`show` 详情、`add/update/move/remove` 管理登记。`entry attach/create` 保留其真实的片段/整文件区别，不机械重命名为 `add`。
- 不新增 `resource-add`、`ls`、`get` 等等价别名或单复数变体。`-h` 与 `--help` 继续等价；暂不增加长短两套帮助。
- 主要操作对象使用位置参数，附加设置使用具名选项。可重复参数、默认值、互斥条件在对应选项旁说明。快捷选项指向相同语义，不维护另一套规则。
- 保留已澄清的 `CONTEXT_DIR`、`SOURCE_FILE`、`RESOURCE_PATH`、`GROUP_PATH`、`ENTRY_FILE` 等名称。当前命令只解释自己使用的名称，不粘贴全局参数词典。
- `<VALUE>` 必填、`[VALUE]` 可选、`[--option <VALUE>]` 可选选项。括号只出现在语法说明，实际示例使用具体值；完整符号规则集中在顶层。
- Usage 可以用 `[OPTIONS]` 收纳较多的已支持选项，但必须在本页逐项列出。标注、继承等开关合并在 Options 中，以一行简注说明；不再拆成多个带规则讲解的分区。
- 不借展示调整改变参数顺序或解析能力：当前 `--context` 位于命令前，`--dry-run` / `--yes` 位于命令末尾。若以后放宽位置、增加 `--` 分隔符等能力，应明确设计并验证，不能只在帮助里宣称支持。

## 帮助层级

1. **顶层**：一句用途、Usage、直接子命令及各自一句说明、全局选项、一个帮助示例和简短符号说明。不展开 `resource add/update/...` 的全部参数，不塞入存储格式、恢复算法或完整产品边界。
2. **命令组**：一句用途、Usage、直接操作及各自一句说明、可用全局选项、下一级帮助提示。不展开每个操作的选项全集。
3. **具体命令**：一句用途、Usage、当前 Arguments、Options / Global options，必要时给一个最小 Examples。参数就地标明默认值、可重复或必需组合；只保留当前操作的关键风险，删除机制讲解、实现细节、完整流程和重复说明。没有内容的分区不输出。

简单命令自然应短，选项多的命令可以更长；不设置机械行数限制。`group list` 不介绍工具资源、schema、注解或继承。`resource add` 的 schema、默认逻辑地址及不修改源文件，则属于必须就地说明的内容。

`--context` 可在各层的 Global options 中简短出现；完整选择优先级在顶层或使用指南解释。与当前操作有关的风险不能全部藏到顶层：例如 `group remove` 只移除空组、不删除源目录；`sync --yes` 不覆盖冲突检查。

帮助必须不读取、创建或修复上下文；无参数仍展示根帮助，不启动会话或隐式执行 sync。错误主题不得伪装成成功的根帮助。

### 顶层目标示例

下列说明已实现的展示布局，使用现有命令；不是逐字输出快照。

```text
DevMeld - organize resources and publish local context files.

Usage:
  devmeld [--context <CONTEXT_DIR>] <COMMAND>

Commands:
  resource   Manage registered resources
  group      Organize resources into logical groups
  access     Associate resources with access guidance
  entry      Manage project reading entries
  output     Set the generated output directory
  config     Change context settings
  status     Inspect configuration and publication status
  sync       Publish generated context files
  recover    Recover an interrupted managed operation
  init       Explicitly initialize a context (optional setup)

Global options:
  --context <CONTEXT_DIR>  Select a local context directory
  -h, --help               Show help

Examples:
  devmeld resource add --help

Context: use --context or the nearest .devmeld in this directory or its ancestors.
Without a marker, a valid first resource/group add can initialize here.
<VALUE> is required; [VALUE] is optional; a|b means choose one.
Replace placeholders with values; do not type the brackets.
Use devmeld <COMMAND> --help for details.
```

已有但无效的上下文仍须报错，不能悄悄另选位置；此类异常细节放在相关错误及指南中。顶层不把 `--dry-run` 或 `--yes` 展示成所有命令都可用的全局开关。

### 具体命令目标示例

```text
List registered logical groups (read-only).

Usage:
  devmeld [--context <CONTEXT_DIR>] group list [GROUP_PATH]

Arguments:
  [GROUP_PATH]  Logical scope: descendants only, excluding this group; omit for all groups.

Examples:
  devmeld group list

Global options:
  --context <CONTEXT_DIR>  Context directory (before command)
  -h, --help              Show help
```

示例采用已有资源或明确标注的示例路径，不承诺 `database` 在用户上下文中存在。通用示例优先使用相对路径、单行命令；Windows 跨盘等专门场景单独标明，避免把 PowerShell/Bash 换行符当通用语法。

## 反馈与安全语义

- 普通结果直说完成了什么、是否还需 sync。配置已保存、产物已发布、Agent 已读取是不同状态，不相互代替。
- `list` 保持简洁列表，`show` 展示详情，`status` 展示状态。成功时不重复整份安全声明，失败时解释相关限制。
- 错误说明对象、原因及可行的下一步；语法错误指向最具体的帮助，不自动换目标、修改输入或执行建议。正常结果/帮助走 stdout，错误走 stderr，失败使用非零退出码；本轮不新增退出码分类或机器格式。
- 保留 005 的保存与确认行为：登记/配置直接保存；`--dry-run` 不写；sync/recover 有变化时确认一次，脚本显式 `--yes`。`--yes` 只跳过询问，不代表强制覆盖或扩权；无变化仍重检。
- 自动化示例显式 `--context`，但不因它而改变其他本地相对路径相对调用目录解析的规则。不要依赖终端人类输出字段/措辞构建稳定机器客户端。
- 当前 CLI 帮助仍为英文；生成内容的 en/zh-CN 设置不作为终端帮助语言开关。命令、选项、路径和 SSH/HTTP 等技术标识不翻译。中英文 README 保留相同命令语义。

## 实施边界与验收

本轮已实现根/组的直接子项摘要、操作页自己的参数/选项/示例，以及公共词典只在需要的层级解释。`group list` 不再附带工具、Schema 或继承说明。具体命令语义、参数顺序及保存/确认方式保持不变；平台与测试结果单独记录，不借用此前验收。

CLI 同时收敛为适配器：解析类型化应用请求，展示结构化查询结果及变更预览，再按交互策略调用应用能力。应用库不接收 argv、不要求未来 GUI 拼命令或解析终端文字。领域校验、原生路径解析及受管写入仍走同一应用路径；生成 Markdown 的产品能力不等同于终端 renderer。

回归测试核对帮助层级、当前参数说明、真实可执行示例、无上下文帮助及无写入。直接应用测试不经过 CLI parser，验证非终端调用与相同的所有权/过期检查。测试验证信息归属和有效用法，不冻结无关空格或强制每页包含所有公共术语。

不因确定风格而增加 parser/TUI/颜色依赖、命令别名、`options`/`explain` 命令、自动补全、JSON/YAML 输出、后台进程或数据迁移。工具选型可以在出现实际需求时单独评估；本约定不是永久禁止库或限制业务实现。
