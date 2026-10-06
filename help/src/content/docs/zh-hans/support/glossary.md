---
title: "Moonpool 术语表：应用、状态、文件和设置"
description: "Moonpool 帮助中用来称呼各个部分、应用状态、文件和设置的词语的通俗定义，帮助你读懂其余文档。"
---

## 应用

| 术语 | 含义 |
| --- | --- |
| 应用（app） | Moonpool 管理的一个对象：开发服务器、桌面应用、页面或命令。 |
| 条目（entry） | 应用在 `apps.json` 中的记录。只在谈到 JSON 时使用。 |
| 应用行（app row） | 应用在侧边栏中的一行，带有状态圆点和控件。 |
| 分组（group） | 应用在侧边栏中所归属的标题，来自它的 `group` 字段。 |
| 类型（type） | `web`、`desktop`、`static` 或 `cli`。决定哪些字段有意义。参见[应用类型](/zh-hans/apps/types/)。 |
| id | 应用的永久键，用于文件名、命令和代理工具。参见 [id](/zh-hans/apps/apps-json/#id)。 |

## 应用状态

| 状态 | 含义 |
| --- | --- |
| 启动中（starting） | Moonpool 已启动应用，但还没看到它就绪。圆点闪动。 |
| 运行中（Running） | 它的 `port` 有应答、它的 `processName` 存在，或者两者都没设置时，Moonpool 启动的终端仍然存活。圆点为实心。参见[如何判定“运行中”](/zh-hans/apps/types/#如何判定运行中)。 |
| 已停止（stopped） | 以上都不是。圆点为灰色。 |
| 受管理（managed） | 由 Moonpool 在本次会话中启动。运行中但不受管理的应用是以其他方式启动的，退出 Moonpool 时不会动它。 |

终端标签页和运行中的应用是两回事。点击应用的名称只会打开它的终端标签页，绝不会启动应用。关闭标签页也绝不会停止应用。

## 窗口与组成部分

| 术语 | 含义 |
| --- | --- |
| hub | 常驻的 Moonpool 进程及其主窗口。工具名称把它称为 “launcher”（启动器）。 |
| 主窗口（hub window） | 主窗口：左侧是侧边栏，右侧是 CLI 面板。 |
| 托盘（tray） | 系统托盘图标及其菜单（**显示 Moonpool**、**退出**）。 |
| 侧边栏（sidebar） | 主窗口的左侧：筛选框、**...** 菜单和应用行。 |
| CLI 面板（CLI pane） | 主窗口的右侧，容纳终端标签页。 |
| 终端标签页（terminal tab） | CLI 面板中某个应用的终端。 |
| MCP 子行（MCP sub-row） | 应用下方一个变暗的行，显示它自己的 `<exe> mcp` 辅助进程。 |
| 应用编辑器（app editor） | “添加应用”和“编辑应用”对话框。 |

## 文件和文件夹

| 术语 | 含义 |
| --- | --- |
| 配置文件夹（config folder） | 存放 `apps.json` 和 Moonpool 其他文件的文件夹。即 `{MP_DATA}` 令牌。参见[配置位于何处](/zh-hans/apps/apps-json/#配置位于何处)。 |
| `{MP_HOME}` | Moonpool 文件夹：安装版是 `%USERPROFILE%\.moonpool`，便携副本是它的 `.moonpool\` 文件夹，Linux 上是配置文件夹。 |
| 会话（session） | hub 从启动到退出的一次运行。 |
| 会话日志（session log） | 保存应用在一次会话中打印的全部内容的文件，位于 `cli-output\` 下。参见[日志](/zh-hans/data/logs/)。 |
| `moonpool.log` | Moonpool 自己的调试日志，只有在 **把调试信息写入文件** 开启时才会写入。 |
| 转储（dump） | 由 `dump` 动词生成的会话日志的纯文本副本。 |
| 快照（snapshot） | `apps.json.history\` 中一份良好 `apps.json` 的副本。参见[备份与恢复](/zh-hans/data/backup-and-recovery/)。 |

## 模式

| 术语 | 含义 |
| --- | --- |
| 安装版（installed） | 位于 `%USERPROFILE%\.moonpool` 的 Moonpool，带有开始菜单快捷方式和“添加/删除程序”条目。仅限 Windows。 |
| 便携版（portable） | 位于你所选 `.moonpool\` 文件夹中的 Moonpool，以一个 `moonpool.portable` 文件为标记。参见[便携模式](/zh-hans/data/portable-mode/)。 |
| 副本（copy） | 一个 Moonpool 文件夹，安装版或便携版均可。每个副本各自运行。 |

## 停止与自动化

| 术语 | 含义 |
| --- | --- |
| `killMode` | “停止”在结束应用终端之后所做的额外步骤。参见[停止与重启](/zh-hans/apps/stop-and-restart/)。 |
| `stopCommand` | `killMode` 为 `command` 时“停止”所运行的命令。 |
| `processName` | Moonpool 监视的进程名，也是 `processName` 模式下要结束的进程。 |
| 控制通道（control channel） | hub 应答所用的命名管道（Windows）或 Unix 套接字（Linux、macOS）。参见[控制动词](/zh-hans/automation/control-verbs/)。 |
| 动词（verb） | 在命令行或控制通道上给出的命令词，例如 `launch` 或 `reload`。 |
| ticket | 用 `--ticket` 附加的键，用来从 `state.json` 读取某条命令的结果。 |
| 令牌（token） | `apps.json` 的版本标记，配置写入必须带上它。 |
| MCP 辅助进程（shim） | AI 宿主为访问应用自己的工具而启动的 `<exe> mcp` 进程。 |
