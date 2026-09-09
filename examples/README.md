# 看一遍实际效果

从仓库根目录执行，首次使用已有 Rust 工具链运行 `cargo fetch --locked`。
下面每条带 `--apply` 的命令先显示预览，需要输入 `apply` 才会写入。
去掉 `--apply` 就只看预览。

当前是未定版的 v0 开发实现。以下初始化示例适用于尚未运行过的新 checkout。
如果 `team-context/.devmeld` 已有不兼容的开发记录，请保留它，不要清空或改格式字段。
可以在仓库外创建独立的新上下文目录，以原始资料的绝对路径重新注册，
并选择未占用的输出/入口路径。原始知识不必复制；旧产物仍可阅读。

```text
cargo run -p devmeld --locked --offline -- --context examples/team-context init --entry ../sample-project/CONTEXT.md --apply
cargo run -p devmeld --locked --offline -- --context examples/team-context resource add notes --document knowledge/notes.md --apply
cargo run -p devmeld --locked --offline -- --context examples/team-context resource add service --description resources/service.json --schema templates/service.schema.json --apply
cargo run -p devmeld --locked --offline -- --context examples/team-context resource add http-guide --description resources/http-guide.json --apply
cargo run -p devmeld --locked --offline -- --context examples/team-context access add service http-guide --apply
cargo run -p devmeld --locked --offline -- --context examples/team-context sync --apply
```

然后关闭命令，从 `examples/sample-project/CONTEXT.md` 开始看：

```text
项目入口 → 共同索引 → 服务 / 接入说明 / 原始知识
```

这条阅读路径不需要 DevMeld 运行。你可以在 Agent 会话里明确指定入口文件；
不会自动发现或修改 AGENTS.md、安装 Skill 或仅凭文件名声称 Agent 已发现入口。
如需在已有项目说明文件中添加小段入口，显式执行：

```text
devmeld --context PATH entry add /path/to/project/AGENTS.md --kind instructions --apply
devmeld --context PATH sync --apply
```

第一条只登记，第二条才发布；周围原文保留。移除时执行 `entry remove PATH` 后再同步，
只移除片段，不删除宿主文件。普通文件入口与片段入口可以并存，共享同一索引。
请勿手工复制受管片段；维护授权来自当前上下文的记录，不来自标记外观。
客户端验收步骤和实际结果见 [004](../specs/004-project-entry-integration/quickstart.md)
与[验证记录](../specs/004-project-entry-integration/acceptance.md)。

修改 `resources/service.json` 的 endpoint 或自定义属性后，再运行 `sync`。
原始资料可由人/AI 修改；登记、关联通过命令维护；生成文件不要手改。
无变化同步会显示 `0 changed target(s)`，不会重写文件。

这里只会创建示例自己的 `.devmeld` 和 `sample-project/CONTEXT.md`，不会连接
虚构服务、运行工具或修改依赖。生成内容和内部状态已被示例专用忽略规则排除；
提交的是原始示例资料与复现命令，不是假冒执行结果的手写输出。

## 选择生成语言

每套上下文使用一种生成语言，支持 `en`（默认）和 `zh-CN`。
首次初始化时，可在上述 `init` 命令中添加 `--language zh-CN`。
已有上下文按以下方式切换：

```text
cargo run -p devmeld --locked --offline -- --context examples/team-context language zh-CN --apply
cargo run -p devmeld --locked --offline -- --context examples/team-context sync --apply
```

分别检查预览并输入 `apply`。语言设置先保存到受管配置，下次 `sync` 才更新
索引、资源页和全部入口；使用 `language en` 并再次同步即可切回英文。
未配置语言时固定使用英文，不读取系统 locale。

中文产物使用“上下文索引”“原始来源”“接入指引”等正式表述。
ssh、http、curl、JSON 等技术术语，以及用户编写的标题、摘要、属性、引用名称、
命令、ID 和链接均保留原样。仅执行现有 Markdown 转义，不翻译或改写源资料。
两种语言共用文件名和资源集合，不生成两套索引，不控制 Agent 的回复语言。
CLI 帮助和错误信息暂不本地化。

## 引用本机其他目录或磁盘

不用把资料搬进上下文目录。以 Windows 的 D 盘资料为例，先自行准备真实文件，
再注册并同步（同样需要预览确认）：

```text
cargo run -p devmeld --locked --offline -- --context examples/team-context resource add shared-notes --document "D:/knowledge/团队 notes.md" --apply
cargo run -p devmeld --locked --offline -- --context examples/team-context sync --apply
```

命令中的路径相对于 `--context`；绝对路径则直接指定本机位置。
描述文件内 `references[].path` 的相对路径基准是该描述文件的目录，
也可填写本机绝对路径。Windows JSON 中建议写 `D:/knowledge/notes.md`，
若使用反斜杠，需要按 JSON 语法转义。

若输出在 C 盘，生成的资源页示意为：

```markdown
[原始来源](file:///D:/knowledge/%E5%9B%A2%E9%98%9F%20notes.md) (本地路径: D:/knowledge/团队 notes.md)
```

同盘继续使用相对于生成文件的链接。跨盘的入口、输出与受管配置链接
采用同一规则；通过已有 `entry add PATH`、`output PATH` 命令维护，随后同步。
源文件不移动、不复制，链接也不是 DevMeld 专用协议。
如果阅读器禁止 `file:` 链接，可使用附带的本地路径，仍需相应读取权限。
相对链接依赖目录关系不变，绝对链接依赖本机路径不变；都不是跨机器同步机制。

远程信息仅作为用户维护的资源数据，例如描述属性中的
`"endpoint": "https://example.invalid/api"` 或 `"host": "ssh.example.invalid"`。
这些文字不会被抓取、连接或展开成远程索引。路径字段不接受 URL、UNC、
Windows 设备路径或 `D:notes.md` 这类盘符相对路径；请使用本机明确位置。

## 当前限制

- 首次初始化创建新上下文，不接管已有同名文件；丢失/损坏的所有权记录不会被
  自动猜测修复。迁移或接管已有状态不是这期功能。
- JSON Schema 仅支持本地 Draft 2020-12 和本文件内引用，`format` 不做可用性验证。
- 描述属性为字符串；原始引用必须是存在的普通文件。重定向路径暂不支持。
- 本期面向本地磁盘；网络映射盘、远程挂载与跨机器索引不在支持范围。
  不检测所有底层挂载来源，也不提供文件系统沙箱。
- 输出不是跨文件原子快照。进程中断后的未完成操作使用 `recover`；恢复遇到
  外部改动会保留现场并报错，不提供 `--force` 覆盖。
- 写入前准备阶段失败或被终止可能留下唯一命名的临时副本/空目录，但不改目标文件；
  不通过递归删除或猜测归属来清理。机器断电保证、后台更新和自动加载不在本期。
