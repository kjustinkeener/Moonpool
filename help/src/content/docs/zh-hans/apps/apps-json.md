---
title: "编辑 apps.json：它在哪里，如何重新加载与恢复"
description: "找到 Moonpool 为所有受管应用读取的 apps.json 文件，用应用编辑器或手动编辑它，重新加载它，并从错误的修改中恢复。"
---

Moonpool 管理的每个应用都是 `apps.json` 中的一个条目。你可以通过应用编辑器（“添加应用”和“编辑应用”对话框）编辑，也可以手动编辑。两者写入的是同一个文件。某些工具结果和消息会把这个文件称为清单（manifest）。

## 配置位于何处

| 模式 | 配置文件夹 |
| --- | --- |
| 已安装（Windows） | `%USERPROFILE%\.moonpool\moonpool-config\` |
| 便携 | `moonpool.exe` 旁边的 `moonpool-config\`（位于 `.moonpool\` 文件夹内） |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`，否则为 `~/.config/Moonpool/` |

`apps.json` 就在该文件夹中，旁边还有这些：

| 项目 | 用途 |
| --- | --- |
| `apps.json.history\` | 最近 10 个有效 `apps.json` 文件的回滚环。 |
| `settings.json` | 应用设置。参见 [settings.json](/zh-hans/data/settings-json/)。 |
| `cli-output\<id>\` | 每个应用的会话日志。参见[日志](/zh-hans/data/logs/)。 |
| `moonpool.log` | 调试日志，在 **把调试信息写入文件** 开启时生成。 |
| `icons\` | 可选的 `<id>.png`（也可以是 `.ico`、`.svg`、`.jpg`、`.jpeg`、`.webp`）图标覆盖文件。 |
| `state.json` | 实时状态快照，每隔几秒刷新一次。 |
| `dumps\` | 由 `dump`、`read-config` 和 `restore-config` 动词写入的文件。 |
| `mcp_seen.json` | 记录哪些应用曾经用过 MCP 辅助程序。 |
| `window-state.json` | 主窗口的大小和位置。 |
| `AI-README.md` | 给 AI 代理的指南，每次启动时重写。 |

其中哪些需要备份，参见[备份与恢复](/zh-hans/data/backup-and-recovery/#配置文件夹)。

首次运行时，Moonpool 会用示例条目生成 `apps.json`。已存在的文件绝不会被覆盖。

## 编辑

- **对话框。** 使用侧边栏顶部 **...** 菜单中的 **添加应用**。要修改某个应用，使用它所在行的铅笔图标，或右键单击它并选择 **编辑**。对话框会立即校验并保存。
- **手动。** 同一菜单中的 **编辑 apps.json** 会用你的默认编辑器打开该文件。保存后，在菜单中选择 **重新加载**（或按 F5 或 Ctrl+R）。

手动修改要等你重新加载后才会生效。重新加载只读取文件，不会重写它。

从对话框保存会以规范化、带缩进的形式重写整个文件。Moonpool 不认识的键会被丢弃，而且 JSON 没有注释，所以请把备注写在 `note` 字段中。

## 结构

该文件是一个由对象组成的 JSON 数组。每个条目必须有四个键：`id`、`name`、`group`、`type`，其余都是可选的。参见[应用字段](/zh-hans/apps/fields/)。

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

分组在侧边栏中的顺序，就是它们在文件中首次出现的顺序。

## 重新加载做了什么

重新加载会用文件的内容替换 Moonpool 内存中的列表。启动、停止和重启会在你点击时读取该条目，所以修改后的 `command`、`cwd`、`env` 或结束设置，会在你下次启动或重启该应用时生效。重新加载绝不会重启任何东西：已经在运行的应用会继续沿用它启动时的设置。

## 校验

Moonpool 在加载时、每次保存时以及每次代理写入时，都会校验整个文件。只要有一个条目有问题，整个文件就会被拒绝。

| 规则 | 错误信息包含 |
| --- | --- |
| 不是合法的 JSON、缺少必需的键，或某个值的类型不对 | JSON 解析器给出的信息 |
| `id` 为空、以 `-` 开头，或含有字母、数字、`.`、`_`、`-` 之外的字符 | `invalid id` |
| 两个条目使用了同一个 `id` | `duplicate app id` |
| `name` 为空白 | `has an empty name` |
| `group` 为空白 | `has an empty group` |
| `type` 不是 `desktop`、`web`、`static` 或 `cli` | `unknown type` |
| `port` 为 `0`（大于 65535 的 `port` 会解析失败） | `invalid port 0` |
| `static` 条目没有 `url` | `requires a url` |
| 其他类型没有 `command` | `requires a command` |

错误信息会按位置指明是哪个条目，例如：

```text
apps.json entry 2 (site) requires a command
```

### id

`id` 是条目的永久键。它决定日志文件夹和图标文件的名称，也是你传给 `moonpool.exe launch <id>` 和代理的值。添加应用时，对话框会根据名称生成它：先把名称转为小写，把每一段由 `a` 到 `z` 和 `0` 到 `9` 之外的字符组成的连续字符替换成一个 `-`，再去掉两端的 `-`。结果为空时用 `app`。如果该 id 已被占用，就依次加上 `-2`、`-3` 等。之后它绝不会再改动 id，所以重命名应用时 id 保持不变。名称 `Habit Tracker` 得到的 id 是 `habit-tracker`。

## 如果文件有错误

- **重新加载时**，校验失败的文件保持原样，Moonpool 继续使用上一次成功加载的列表。侧边栏上方会出现一条横幅显示错误，并带一个打开该文件的按钮；列表仍可使用，但会变暗。参见[当 apps.json 有错误时](/zh-hans/using/hub-window/#当-appsjson-有错误时)。
- **启动时**，文件损坏意味着没有可保留的列表，所以 Moonpool 会在没有任何应用的情况下启动，横幅也会这样说明。请修正文件并选择 **重新加载**，或恢复一个快照（见下文，或使用 `moonpool_restore_config` 工具）。
- 无论哪种情况，在文件重新加载成功之前，来自对话框的保存（以及重命名、删除、设置图标）都会被拒绝，这样损坏的文件绝不会被覆盖。请修正文件并选择 **重新加载**。
- **通过对话框、代理或恢复** 进行的无效更改会被拒绝，磁盘上的文件保持原样。

Moonpool 在 `apps.json.history\` 中保留 `apps.json` 最近 10 个良好版本。如何回滚见[备份与恢复](/zh-hans/data/backup-and-recovery/#回滚-appsjson)。症状和解决办法见[故障排除](/zh-hans/support/troubleshooting/#appsjson-有错误)。

## 代理

AI 代理应当通过 Moonpool 的 MCP 工具而不是直接改文件来修改 `apps.json`，这样过期或无效的写入会被拒绝，沙盒中的代理也不会去编辑一份私有副本。参见 [MCP 工具](/zh-hans/automation/mcp-tools/#配置)。
