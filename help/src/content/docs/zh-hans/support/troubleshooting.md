---
title: "Moonpool 故障排除：托盘、无法启动的应用、更新"
description: "按你看到的现象解决 Moonpool 的常见问题：托盘图标消失、应用无法启动或停止、状态圆点不对、更新失败和 MCP 错误。"
---

先找到对应的现象，再按说明解决。引用的文字是 Moonpool 显示的内容。要查询某条确切的信息，参见[错误信息详解](/zh-hans/support/error-messages/)。

## 我看不到托盘图标

- **Windows。** 图标可能在隐藏图标区域。点击任务栏右侧的 **^** 箭头。把图标拖到任务栏上可让它保持可见。
- **原生 GNOME 上的 Linux。** 没有 AppIndicator 扩展时，GNOME 不显示托盘图标。参见 [Linux](/zh-hans/platforms/linux/#gnome-上的托盘)。
- **设置。** **在托盘中显示** 可能被关闭了。从任务栏或开始菜单打开主窗口，在[设置](/zh-hans/using/settings/)中把它重新打开。

## 安装程序显示错误

| 信息 | 该怎么做 |
| --- | --- |
| `Install failed: <error>` | 冒号后面的文字指出了失败的步骤，例如 `copy exe: ...`。如果某个文件正被占用，请退出所有从 `%USERPROFILE%\.moonpool` 运行的 Moonpool，然后重试。 |
| `target folder does not exist` | 你为便携副本选的文件夹已经不存在。请选一个已存在的文件夹。 |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | 请选一个空文件夹，或者先删除那个 `.moonpool` 文件夹。 |

## 运行安装程序时出现“Windows 已保护你的电脑”

这是 Windows SmartScreen，因为 `moonpool.exe` 没有代码签名。点击 **更多信息**，然后点击 **仍要运行**。参见 [Windows 已保护你的电脑](/zh-hans/support/windows-protected-your-pc/)。

## Moonpool 窗口在 Windows 上空白或始终打不开

可能是缺少 Microsoft Edge WebView2 运行时。参见[缺少 WebView2 运行时](/zh-hans/support/webview2-runtime-missing/)。

## 应用无法启动

1. 点击应用的名称，打开它的终端标签页并查看输出。代理可以用 `moonpool_app_output` 读取同样的文字。
2. 检查 `cwd`。文件夹不存在，或者使用了不带 `./` 的相对路径，是常见的原因。参见[路径与环境](/zh-hans/apps/paths-and-environment/)。
3. 检查 `command`。在 `cwd` 下的终端里手动运行一遍。在 Windows 上避免嵌套双引号；`cmd /c` 会把它们弄乱。
4. 在设置中打开 **把调试信息写入文件**，然后再次启动。`moonpool.log` 会记录确切的命令和文件夹。参见[日志](/zh-hans/data/logs/)。

| 信息 | 含义 |
| --- | --- |
| `already running` | Moonpool 已经为这个应用持有一个终端。请先停止它，或使用重启。 |
| `stopped during launch` | 启动仍在进行时按下了停止。 |
| `did not reach running in time` | 来自脚本或代理：应用在 25 秒内没有显示为运行中。请检查它的 `port` 或 `processName`，以及它的输出。 |

## 状态圆点不对

Moonpool 先根据 `port`，再根据 `processName`，最后根据它自己的终端是否还活着，来判断是否运行中。参见[如何判定“运行中”](/zh-hans/apps/types/#如何判定运行中)。

- **一直不变成实心。** `web` 应用的 `port` 没有响应，或者 `desktop` 应用的 `processName` 不匹配。在 Linux 上，`processName` 不能超过 15 个字符。
- **启动后立刻变灰。** `cli` 应用在它的命令退出时就不再是运行中。如果想让它保持打开，请使用带 `-NoExit` 的 shell。
- **`static` 应用从不显示运行中。** 对只有 `url` 的条目，这是正常的。
- **你没有启动它却显示运行中。** 有别的东西在使用那个端口或进程名。Moonpool 把它显示为运行中，但不是“由 Moonpool 管理”。

## 错误：listen EADDRINUSE 或 "Port 5173 is in use"

已经有别的东西在监听你的服务器想用的端口。请找到并结束它，或者给应用设置 `port`，让“停止”来释放它。参见[解决 EADDRINUSE 与 "Port 5173 is in use"](/zh-hans/support/port-already-in-use/)和[查找并结束占用端口的进程](/zh-hans/guides/find-and-kill-process-using-port-windows/)。

## 两个应用使用同一个端口

**...** 菜单底部会出现一行警告，例如 `端口 3000：App A / App B`。请修改其中一个应用的 `port`（如果它读取 `PORT`，也要改它的 `env`）。参见[端口冲突警告](/zh-hans/using/hub-window/#端口冲突警告)。

## 点了停止后应用仍在运行

来自脚本或代理时，错误是 `still running after stop`（15 秒后）。

- 应用比它的终端存活得更久。请把 `killMode` 设为 `port` 或 `processName`。参见[停止与重启](/zh-hans/apps/stop-and-restart/)。
- Windows 上的 Docker 应用：请使用 `killMode` 为 `command`，并配一个 `stopCommand`，例如 `docker compose stop app`。绝不要用 `port`。

## apps.json 有错误

侧边栏会显示一条横幅，“apps.json 有错误，当前显示的是上次成功加载的列表。”；或者在启动时显示“apps.json 有错误，因此没有加载任何应用。”在文件重新加载成功之前，来自 Moonpool 的保存会被暂停。

典型的错误：

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. 在横幅中选择 **编辑 apps.json**，修正该条目，保存，然后 **重新加载**（F5）。
2. 或者回滚到最近的良好副本。参见[备份与恢复](/zh-hans/data/backup-and-recovery/#回滚-appsjson)。

完整的规则列表见[校验](/zh-hans/apps/apps-json/#校验)。

如果某项设置无法更改，且信息以 `Repair settings.json and restart Moonpool before changing settings` 结尾，请修复或删除配置文件夹中的 `settings.json`，然后重新启动 Moonpool。删除它会把所有设置重置为默认值。

## 我的修改没有生效

- 手动修改需要 **重新加载**（或 F5）。Moonpool 不会监视这个文件。
- 重新加载不会重启运行中的应用。要使用修改后的 `command`、`cwd` 或 `env`，请重启该应用。
- 代理可能在编辑另一份 `apps.json`。请让它调用 `moonpool_launcher_paths`，把 hub 的文件夹与它自己的作比较。如果有多个 Moonpool 副本，请确认你在编辑的是哪一个副本。

## 示例应用不见了

示例只在没有 `apps.json` 时才会写入。要找回它们，参见[重置为示例](/zh-hans/data/backup-and-recovery/#重置为示例)，或者从[示例仪表盘](/zh-hans/getting-started/example-dashboards/#示例应用只在首次运行时出现)复制这些条目。

## 更新失败了

横幅会显示 `更新失败：<error>`。参见[更新失败时](/zh-hans/data/updating/#更新失败时)。

## 网页链接打不开

`refusing to open non-web url: <url>` 表示 `url` 不是 `http://`、`https://`、`mailto:` 或 `file://`。请修正 `url`。

## MCP 与脚本错误

| 信息 | 该怎么做 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | 启动 Moonpool，或者让代理调用 `moonpool_bootup_launcher`。 |
| `frontend not loaded` | 主窗口还没有加载完。稍等片刻再重试。 |
| `stale token: ...` | 自代理读取后，`apps.json` 已经变化。请重新读取，然后再写入。 |
| `rejected invalid manifest: ...` | 新的 `apps.json` 未通过校验。文件没有被改动。 |
| `... A Moonpool process may be hung ...` | 有东西占着控制通道却不应答。从托盘退出 Moonpool，或结束该进程，然后重新启动。 |

更多内容见 [MCP 设置](/zh-hans/automation/mcp-setup/#说明)和 [MCP 工具](/zh-hans/automation/mcp-tools/)。

## 窗口问题

- **跑到屏幕之外。** Moonpool 会忽略不在任何已连接显示器上的已保存位置。如果窗口仍然找不到，请退出 Moonpool，并删除配置文件夹中的 `window-state.json`。
- **缩放卡在过大或过小。** 在主窗口上按 Ctrl + 滚轮可以更改。参见[快捷键与缩放](/zh-hans/using/keyboard-shortcuts/#缩放)。
- **设置窗口打开在主窗口后面。** 在设置中把 **总在最前** 关掉，或打开。它作用于每个 Moonpool 窗口，所以它们会处在同一层。

## 日志在哪里？

参见[日志](/zh-hans/data/logs/)。

## 备份、重置或卸载

参见[备份与恢复](/zh-hans/data/backup-and-recovery/)和[卸载](/zh-hans/getting-started/install/#卸载)。

## 常见问题

**关闭窗口会停止我的应用吗？**
默认情况下，关闭会退出 Moonpool，而在 Windows 上退出会停止它启动的应用。打开 **关闭时收进托盘**，就可以在关闭窗口时让 Moonpool 继续运行。参见[托盘、关闭与最小化](/zh-hans/using/tray-and-closing/)。

**可以同时运行两个 Moonpool 吗？**
每个文件夹一个。再次启动同一个副本，会把它的窗口带回来。已安装的副本和便携副本可以并排运行。参见[便携模式](/zh-hans/data/portable-mode/#同时运行多个副本)。

**Moonpool 会“回传”数据吗？**
只用于检查更新：它会在启动时（如果 **启动时检查更新** 开启）以及你按下 **检查更新** 时，从 GitHub 获取发布文件（`update.json`）。每个下载在使用前都会用 Moonpool 的签名密钥校验。

**我的命令由哪个 shell 运行？**
Windows 上是 `cmd /c`，Linux 上是 `$SHELL -c`。

**密钥该放在哪里？**
`env` 的值以明文保存在 `apps.json` 中。请优先使用应用自己会读取的文件，或者你的用户环境中已经设置好的变量，被启动的应用会继承它们。

**控制通道受保护吗？**
它没有登录或令牌。任何以你的身份运行的进程都可以向它发送命令。在 Linux 上，套接字只有你的用户可以读取。参见[安全特性](/zh-hans/automation/overview/#安全特性)。
