---
title: "用脚本和 AI 代理自动化 Moonpool"
description: "从脚本和 AI 代理驱动运行中的 Moonpool 的三种方式（MCP、命令行和控制动词），它们之间的关系，以及各自能改变什么。"
---

不碰它的窗口也能驱动 Moonpool。共有三种接口，全部由同一个常驻的 Moonpool（托盘实例，这里称为 hub）提供服务。

每个 Moonpool 副本都是自己的 hub：已安装的副本和每个便携副本各自独立运行，各有自己的控制通道。一种接口总是连到它所用的 `moonpool.exe` 所对应的那个副本。参见[便携模式](/zh-hans/data/portable-mode/#同时运行多个副本)。

| 接口 | 它是什么 | 参考 |
| --- | --- | --- |
| MCP 服务器 | `moonpool.exe mcp`，由 AI 宿主启动的 stdio [MCP](https://modelcontextprotocol.io) 服务器。 | [MCP 设置](/zh-hans/automation/mcp-setup/)、[MCP 工具](/zh-hans/automation/mcp-tools/) |
| 命令行 | `moonpool.exe <verb> [args]`。同一副本的第二次运行会通过控制通道把动词交给它的 hub，然后退出。 | [命令行](/zh-hans/automation/command-line/) |
| 控制通道 | Windows 上是命名管道 `\\.\pipe\moonpool`（便携副本为 `\\.\pipe\moonpool-<id>`），Linux 上是 Unix 套接字，每行一个 JSON 请求。 | [控制动词](/zh-hans/automation/control-verbs/) |

## 它们之间的关系

- hub 拥有一切：启动应用、会话日志、`apps.json`。
- MCP 服务器是 hub 的客户端，而不是它的第二个副本。大多数工具调用会通过控制通道转发给 hub，回复则作为工具结果返回。例外：`moonpool_bootup_launcher` 会自己启动 `moonpool.exe`；`moonpool_app_output` 和配置类工具会让 hub 写出一个文件再读取它；`moonpool_launcher_paths` 会把 MCP 进程自己的路径附加到 hub 的路径之后。
- 是否有 hub 在运行，是通过对该通道发起 ping 来判断的，而不是去找某个进程。能应答的 hub 就是在运行；管道或套接字不存在就说明没有运行。
- 每种接口都运行与窗口相同的处理程序，所以一个动词做的事就和对应的点击一样。
- 如果没有 hub 在运行，对它起作用的工具（包括 `moonpool_list_apps`）会以“Moonpool is not running”（Moonpool 没有运行）拒绝。不会有过期的列表。`moonpool_bootup_launcher` 会启动它。如果有东西占着通道但几秒内不应答，错误信息会说明某个 Moonpool 进程可能已经挂起。
- MCP 服务器不再回退去驱动早于控制通道的 hub 版本。请更新那个副本，或者退出它再重新启动。

## 哪些接口可以改动东西

| 可以改动 | 接口 |
| --- | --- |
| 启动、停止或重启应用 | MCP、命令行、管道 |
| 重写 `apps.json` | MCP（`moonpool_write_config`、`moonpool_restore_config`）、命令行、管道 |
| 退出 Moonpool | MCP（`moonpool_shutdown_launcher`）、命令行（`quit`）、管道 |
| 结束应用的 MCP 辅助进程 | MCP（`moonpool_stop_mcp_server`）、管道（`stop-mcp`） |
| 重新加载 `apps.json`、重新获取图标、显示窗口 | MCP（`moonpool_reload_config`、`moonpool_refresh_app_icons`、`moonpool_raise_launcher`）、命令行（`reload`、`refresh-icons`、`show`）、管道 |
| 打开窗口或终端标签页 | 管道（`open-window`） |
| 清除已记住的 MCP 辅助进程记录 | MCP（`moonpool_reset_mcp_seen`）、管道（`reset-mcp-seen`） |

只读工具：`moonpool_list_apps`、`moonpool_app_output`、`moonpool_read_config`、`moonpool_launcher_paths`、`moonpool_window_state`、`moonpool_screenshot`。

## 安全特性

- **配置写入受保护。** 写入必须带上上次读取得到的版本令牌，过期的令牌会被拒绝，并且在写入任何内容之前会先校验新的 `apps.json`。被拒绝的写入不会改动 `apps.json`。参见 [MCP 工具](/zh-hans/automation/mcp-tools/#配置)。
- **应用 id 受到限制。** MCP 服务器只接受字母、数字、`.`、`_` 和 `-`，且绝不能以 `-` 开头，这样 id 就不会被当作命令行标志。
- **截图只限 Moonpool。** `moonpool_screenshot` 只会截取 Moonpool 自己的某个窗口（`main`、`settings`、`about`、`installer`、`editor`、`help`、`themes`），绝不会截取屏幕或其他应用。PNG 在内存中生成并以内联方式返回；Moonpool 不会把它保存为文件。
- **通道没有身份验证。** Moonpool 不会给控制管道或套接字添加登录或令牌。任何能打开它的进程都可以发送动词。在 Linux 上，套接字文件以 `0600` 权限创建，所以只有你自己的用户可以打开。
- **会检测沙盒宿主。** 如果 MCP 服务器发现自己运行在打包（Store/MSIX）沙盒中，在那里它看到的是 Moonpool 文件的一份私有副本，那么读写文件的工具（`moonpool_app_output`、`moonpool_read_config`、`moonpool_write_config`、`moonpool_restore_config`）会返回一条说明原因的错误，而不是过期的数据。只使用控制通道的工具不会被拦截。参见 [MCP 设置](/zh-hans/automation/mcp-setup/#沙盒宿主)。

## 平台

控制通道在每个平台上都存在：Windows 上是命名管道，Linux 上是 Unix 套接字（位置见[控制动词](/zh-hans/automation/control-verbs/#它在哪里监听)）。只有 `screenshot`（因此还有 `moonpool_screenshot`）仅限 Windows；在 Linux 上它会返回“not supported on this platform”（此平台不支持）。命令行动词在每个平台上都能用。

## 另请参阅

- [AI 代理：快速开始](/zh-hans/automation/quick-start/)
- [MCP 设置](/zh-hans/automation/mcp-setup/)
