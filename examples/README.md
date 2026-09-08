# 看一遍实际效果

从仓库根目录执行，首次使用已有 Rust 工具链运行 `cargo fetch --locked`。
下面每条带 `--apply` 的命令先显示预览，需要输入 `apply` 才会写入。
去掉 `--apply` 就只看预览。

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
当前版本不会自动修改 AGENTS.md、安装 Skill 或声称 Agent 已自动发现入口。

修改 `resources/service.json` 的 endpoint 或自定义属性后，再运行 `sync`。
原始资料可由人/AI 修改；登记、关联通过命令维护；生成文件不要手改。
无变化同步会显示 `0 changed target(s)`，不会重写文件。

这里只会创建示例自己的 `.devmeld` 和 `sample-project/CONTEXT.md`，不会连接
虚构服务、运行工具或修改依赖。生成内容和内部状态已被示例专用忽略规则排除；
提交的是原始示例资料与复现命令，不是假冒执行结果的手写输出。

## 当前限制

- 首次初始化创建新上下文，不接管已有同名文件；丢失/损坏的所有权记录不会被
  自动猜测修复。迁移或接管已有状态不是这期功能。
- JSON Schema 仅支持本地 Draft 2020-12 和本文件内引用，`format` 不做可用性验证。
- 描述属性为字符串；原始引用必须是存在的普通文件。重定向路径暂不支持。
- 文件链接使用相对路径，目标需要能从同一文件系统根表示；跨 Windows 盘符
  的链接会明确报错，不伪造可用链接。
- 输出不是跨文件原子快照。进程中断后的未完成操作使用 `recover`；恢复遇到
  外部改动会保留现场并报错，不提供 `--force` 覆盖。
- 写入前准备阶段失败或被终止可能留下唯一命名的临时副本/空目录，但不改目标文件；
  不通过递归删除或猜测归属来清理。机器断电保证、后台更新和自动加载不在本期。
