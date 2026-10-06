---
title: "从 U 盘或同步文件夹运行 Moonpool"
description: "把 Moonpool 及其所有数据放进一个可移动的文件夹，这样你可以放在 U 盘上随身携带或同步，并且可以并排运行多个副本。"
---

便携模式把 Moonpool 及它写入的所有内容都放在一个 `.moonpool\` 文件夹里，所以你可以把它放在 U 盘上随身携带，或放进同步文件夹，在任何电脑上运行。

## 工作原理

选择便携安装时，Moonpool 会在你选的位置内创建一个 `.moonpool\` 文件夹。这个文件夹里有程序本体、你的配置和它的帮助内容。不会往 Windows 的 AppData 写入任何东西，所以移动或复制这个文件夹，就等于把你的整套配置一起带走。

```text
<chosen location>\.moonpool\
```

## 与安装版有何不同

| | 安装版 | 便携版 |
| --- | --- | --- |
| 程序 | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| 配置文件夹 | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| 窗口的浏览器配置文件、窗口大小和位置 | 在配置文件夹中 | 在配置文件夹中，所以它们也会一起迁移 |
| 开始菜单、桌面快捷方式、“添加/删除程序”条目 | 有 | 无 |
| 更新 | 替换它自己的 exe | 同样，在 `.moonpool\` 文件夹内进行。参见[更新](/zh-hans/data/updating/#便携副本)。 |
| 移除 | “添加/删除程序”或 `--uninstall` | 删除该文件夹 |

两种模式都不会写入 Windows 的 AppData。

### 同步文件夹

你可以把便携副本放在同步文件夹（OneDrive、Dropbox 之类）里，但同一时间只能在一台电脑上运行它。Moonpool 每隔几秒就会写入 `state.json`，应用运行时也会写日志，所以两台电脑运行同一个文件夹会争用同样的文件，同步冲突还可能留下损坏的 `apps.json`。请先在一台电脑上退出它，再在另一台上启动。

## 同时运行多个副本

每个文件夹只运行一个 Moonpool。已安装的 Moonpool 和任意数量的便携副本（各自在自己的文件夹中）可以同时运行，而且彼此完全独立：各有自己的应用、托盘图标、窗口、设置、日志和[控制通道](/zh-hans/automation/control-verbs/)。

- 托盘悬停提示和任务栏名称会说明哪个是哪个副本：安装版是 `Moonpool`，便携版是 `Moonpool (<folder>)`，其中 `<folder>` 是你选的文件夹（包含 `.moonpool\` 的那个）。
- 再次启动同一个副本，会把它的窗口重新带出来，而不是再打开一个。启动另一个副本，则会打开那个副本。
- 要让 AI 代理使用多个副本，请用各自的名称分别注册；参见 [MCP 设置](/zh-hans/automation/mcp-setup/#多个-moonpool)。
- 移动或重命名便携文件夹会让它获得新的标识（新的控制通道名称）。移动之前请先退出它。
- 各副本并不知道彼此的应用。两个副本如果都在同一个端口上启动同一个服务器，仍会冲突；而按进程名或端口工作的停止，可能会结束另一个副本启动的东西，参见[停止与重启](/zh-hans/apps/stop-and-restart/#多个-moonpool或你自己的进程)。

## 让你的应用也能随身携带

在应用的路径中使用 `{MP_HOME}` 令牌，让它指向便携文件夹内部，而不是某台机器上的固定位置。在便携副本中，`{MP_HOME}` 是存放 `moonpool.exe` 的文件夹，也就是 `.moonpool\` 文件夹本身，而不是你选的那个文件夹：

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

这里的 `{MP_HOME}/my-app` 就是 `<chosen location>\.moonpool\my-app`。以 `./` 开头的路径也以同样方式定位。令牌和 `./` 路径在已安装的 Moonpool 中同样有效。路径如何解析，参见[路径与环境](/zh-hans/apps/paths-and-environment/)。

## 从安装程序选择便携模式

便携模式是从安装卡片中设置的，卡片上在 **安装 Moonpool** 旁边提供 **安装便携版**。

![安装卡片：“安装便携版”链接位于主按钮“安装 Moonpool”下方](../../../../assets/screenshots/installer-window.png)

选一个文件夹，Moonpool 就会在那里创建 `.moonpool\` 文件夹，把自己复制进去，并用全新的配置启动这个新副本。

该卡片也在“...”菜单中，名为 **安装 Moonpool...**，已安装模式和便携模式下都有。从那里使用 **安装便携版**，会让正在运行的 Moonpool 退出，并由新的便携副本取而代之启动。你启动它时用的那个 Moonpool 仍留在原处，所以之后可以再次启动它。

便携副本从全新状态开始，不会复制你已有的应用。要把它们带过去，请先退出便携副本，再手动复制 `apps.json`：

| | 路径 |
| --- | --- |
| 来源（安装版） | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| 目标（便携版） | `<chosen location>\.moonpool\moonpool-config\apps.json` |

使用绝对路径的条目在同一台电脑上仍然有效，但无法随身迁移。“编辑应用”对话框会把它们标为“不便携”。

## Moonpool 如何知道自己是便携版

只要 `moonpool.exe` 旁边有一个名为 `moonpool.portable` 的文件，这个副本就是便携版。没有别的标记，也没有任何东西注册到 Windows。

要移除便携副本，请先退出它，再删除它的 `.moonpool\` 文件夹。`--uninstall` 只会移除已安装的 Moonpool，绝不会移除便携副本。
