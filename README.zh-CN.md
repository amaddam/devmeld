# DevMeld

[English](README.md) | 简体中文

DevMeld 是一个本地优先的项目上下文工具。它将团队维护的文档、资源描述和接入指引
组织成可直接阅读的 Markdown 索引。Agent 从项目中的一个小入口找到相关来源，
读取时不需要保持 DevMeld 运行。

登记资源 → 生成索引 → 接入项目入口 → Agent 读取原始资料

**开发状态：**尚未定版的 v0，适合本地试用，格式可能继续调整。

## 可以做什么

- 整理已有文档、服务和工具描述，无需搬移原始文件。
- 将资源与接入说明、工具文档关联起来；可使用本地 JSON Schema 校验描述属性。
- 生成普通 Markdown 入口，或在指定的项目说明文件中维护一小段入口。
- 生成英文或简体中文导航，支持本机 Windows 跨盘链接。

DevMeld 根据明确的输入生成上下文，不通过 AI 改写知识，也不代替 Agent 回答问题、
连接服务或执行列出的工具。

## 快速开始

### 1. 从源码构建

安装 Rust 后，在仓库根目录执行。工具链由
[rust-toolchain.toml](rust-toolchain.toml) 指定，构建还需要本机链接器。

```text
cargo build --release --locked -p devmeld
```

Linux/macOS 的可执行文件为 `target/release/devmeld`，Windows 为
`target/release/devmeld.exe`。下文用 `devmeld` 代指它：执行时使用实际路径，
或将其所在目录加入 PATH。不需要安装 Agent Client 或启动后台服务。

### 2. 登记文档并生成上下文

在需要保存上下文的项目目录中执行，无需先运行 `init`。
选择一份已有的本地文档，将下面的 `<文档绝对路径>` 替换为实际位置，例如
`C:/knowledge/notes.md` 或 `/home/me/knowledge/notes.md`。

```text
devmeld resource add "<文档绝对路径>" --as knowledge/notes
devmeld sync
```

登记命令直接保存配置；`sync` 显示发布预览，再询问一次 `Apply these changes? [y/N]`。
输入 `y` 发布，直接回车则取消。命令末尾加 `--dry-run` 只预览、不写入。
脚本中使用 `sync --yes` 明确确认；它不会绕过冲突检查或扩大维护权限。

打开 `.devmeld/output/index.md`，沿资源页找到原始文档。
也可以将这个索引文件指定给有本地读取权限的 Agent。
生成完成后 DevMeld 即可退出，读取时不再调用它。

命令会显示实际使用的上下文位置：优先使用 `--context PATH`；未指定时，
查找当前目录及其上层最近的 `.devmeld`；没有时，首次成功登记才在当前目录创建。
发现损坏或不完整的记录会报错，不会跳过它另建一份。
如需独立试用，可在两条命令中都加上 `--context "<新的上下文目录>"`；
该位置只在有效操作保存时创建。也可以用 `devmeld init PATH` 显式创建空上下文，
但登记资源前不需要单独初始化。

源文件、Schema、入口、输出的相对路径以执行命令时的目录为基准。
`knowledge/notes` 是逻辑分类地址，不是源文件夹或内部 ID；缺少的父组会自动建立。
原始文档保留在原处，不会被修改。

## 日常使用

### 查看命令用法

无需初始化或选择上下文，就可以逐层查看帮助；这些命令不会创建文件：

```text
devmeld --help
devmeld resource --help
devmeld resource add --help
devmeld entry attach --help
```

也可以使用简写 `-h`。帮助只展示当前已经实现的命令。

### 查看和整理资源

完成快速开始、登记了 `knowledge/notes` 后，可以继续：

```text
devmeld group list
devmeld group show knowledge
devmeld resource list knowledge
devmeld resource show knowledge/notes
devmeld group add database
devmeld resource move knowledge/notes database/notes
devmeld group move database reference/database
devmeld group remove knowledge
devmeld sync
```

`list`/`show` 只读，不需要确认。按组执行 list 会列出整个子树，group show 只显示直属内容。
resource show 展示登记的来源和接入关联，不代表来源当前可用或 Agent 已读取。
移动目标是完整逻辑地址：同名目标会报错，不会自动合并。
移动不搬移源文件，也不改变内部 ID 或关联；空的原分组会保留，需显式删除。
非空组不能直接删除。最后执行 `sync` 才更新生成的导航。
命令末尾加 `--dry-run` 可只看修改预览、不保存。

### 补充说明、标签和字段

三者是分开的上下文标注。例如，完成快速开始后、移动 `knowledge/notes` 之前：

```text
devmeld group update knowledge --description "团队文档" --tag backend --environment test --shared
devmeld resource update knowledge/notes --description "项目约定" --field "attention=修改配置前确认原始说明"
devmeld group show knowledge
devmeld resource show knowledge/notes
devmeld sync
```

`group add`、`resource add` 也支持这些参数。自定义字段不需要新增插件：
`--environment test` 等价于 `--field environment=test`；`--shared`、`--no-shared`
分别等价于 `--field shared=true`、`--field shared=false`。
字段值均为描述文本，不代表权限或执行配置；`--tag shared` 仍是独立的标签。

更新时，未指定的信息保留。需要删除时，在 update 中使用 `--clear-description`、
`--remove-tag 标签` 或 `--remove-field 字段名`。标签去重；同一命令重复赋值同一字段会报错。
查看和生成的导航会区分上下文标注与源文件属性，不改写原始来源。
本级声明与继承得到的值保持区分。

### 控制继承

继承需要两端同时允许：父组向下传递（`propagate`），子项接收（`inherit`）。
初始默认值是父组允许传递、子项不继承。完成快速开始后、移动 `knowledge/notes` 之前：

```text
devmeld group update knowledge --tag backend --environment test --propagate
devmeld resource update knowledge/notes --inherit
devmeld resource show knowledge/notes
devmeld sync
```

查看和生成的导航会区分本级标注与生效的继承信息，并标明来源。
标签合并去重，同名字段由最近的本级声明覆盖；整体说明、ID、源路径和权限不继承。
父组使用 `--no-propagate`，或子项使用 `--no-inherit`，都会切断这一层继承，
更远祖先的信息也不能绕过。删除本级字段覆盖后，可能重新显现父组的值。

在已有上下文中，可以调整两个创建默认值：

```text
devmeld config set defaults.inherit true
devmeld config set defaults.propagate false
devmeld group add tools --no-inherit --propagate
devmeld group show tools
```

默认值只影响新节点，包括自动建立的父组。创建时的显式参数只覆盖目标节点，
更新时未指定的选项保留。修改默认值或移动节点，都不会改写已有节点保存的开关。
父组标注发生变化后，下一次 `sync` 才更新生成文件；继承值不会复制进子项配置。

### 查看发布状态

```text
devmeld status
devmeld sync --dry-run
devmeld sync
```

`status` 只读区分配置已保存、发布待更新或已更新，以及入口已登记或已发布。
来源缺失、文件冲突、待恢复操作会阻止验证，不会自动修复。
它不代表 Agent 已读取上下文。无变化的同步仍检查输入和所有权，但不再询问或重写文件。

### 更新上下文

修改原始文档或资源描述后，再同步：

```text
devmeld sync
```

通过 DevMeld 命令维护资源登记和入口，不要直接编辑受管配置或生成文件。
输入和产物没有变化时，同步不会重写文件。

### 为项目接入入口

明确选择一个普通 UTF-8 项目说明文件，例如项目的 `AGENTS.md`：

```text
devmeld entry attach "<项目绝对路径>/AGENTS.md"
devmeld sync
```

DevMeld 只维护其中的生成片段，保留周围的作者内容。
新 Agent 会话能否自动读到该文件，取决于具体客户端配置；
也可以通过 `devmeld entry create CONTEXT.md` 登记普通入口，再执行同步。
初始化上下文本身不会隐式登记或修改项目入口。

移除入口时，在同一上下文中执行（也可显式指定 `--context`）
`entry remove "<项目绝对路径>/AGENTS.md"`，再执行 `sync`。
这只移除片段，不删除项目说明文件或原始资源。

### 切换生成语言

```text
devmeld config set language zh-CN
devmeld sync
```

使用 `en` 可切回英文；未指定时默认英文。
只切换生成的说明文字，作者内容、ssh/http/curl、命令、ID 和路径保持原样。
CLI 帮助和错误提示目前仍为英文。

## 当前限制

- 仅处理本机上下文，源文件可位于不同本地磁盘。远程服务地址可保存在描述中，
  但不会被自动连接或展开成远程索引。
- 当前需要手动同步，没有自动发现项目、定时更新、安装或执行工具的能力。
- `status` 根据当前输入比较应生成的内容和受管文件，不是历史来源快照，
  也不代表 Agent 已经读取。
- 不兼容的旧开发记录会原样保留并拒绝维护，不自动迁移。首次试用请选择新路径。
  当前操作意外中断时，先用 `devmeld --context PATH recover --dry-run` 查看恢复预览，
  再运行 `recover` 确认一次；脚本使用 `recover --yes`。
- 各平台的实际结果见[当前 CLI 验证记录](specs/005-context-cli/acceptance.md)。
  macOS 及其他 Agent Client 配置尚未验证。

## 更多资料

- [使用示例](examples/README.md)：服务和工具描述、接入关联、Schema 校验及跨盘路径。
- [产品说明](docs/product.md)：产品概念与内容归属。
- [贡献指南](CONTRIBUTING.md)与[工程规范](docs/engineering.md)：开发实践；
  项目检查命令为 `cargo xtask check`。
