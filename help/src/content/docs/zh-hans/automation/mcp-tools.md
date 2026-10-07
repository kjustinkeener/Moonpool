---
title: "Moonpool MCP 工具参考：参数与结果"
description: "Moonpool MCP 服务器向代理提供的每个工具，包括它的参数、返回内容，以及你可能遇到的错误情况。"
---

所有工具都返回文本，只有 `moonpool_screenshot` 返回 PNG 图像。失败会以标记为错误的工具结果返回，原因以文本形式给出。设置方法参见 [MCP 设置](/zh-hans/automation/mcp-setup/)。

接受 `app_id` 的工具，需要的是 `apps.json` 中该应用的 `id`。它只能使用字母、数字、`.`、`_` 和 `-`，并且不能以 `-` 开头，否则调用会以 "invalid app_id" 失败。

大多数作用于 hub 的工具，在它没有运行时都会以下面这条信息失败。`moonpool_bootup_launcher`、`moonpool_shutdown_launcher`、`moonpool_raise_launcher` 和 `moonpool_launcher_paths` 会自行处理这种情况（见它们各自的行）。对便携副本，这条信息会写出该副本的名称，例如 `Moonpool (<folder>)`。

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

等待结果的调用在 45 秒后超时。

## 启动器与应用

`moonpool_list_apps` 的结果示例：

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| 工具 | 参数 | 行为 |
| --- | --- | --- |
| `moonpool_list_apps` | 无 | 每个应用一行：`id  [running]` 或 `[stopped]`，适用时带 `(managed by Moonpool)`，见过 MCP 辅助进程时带 `[mcp: running]` 或 `[mcp: stopped]`，最后是名称。它通过控制通道（`list` 动词）向运行中的 hub 查询，所以是实时的。如果 Moonpool 没有运行，它会以 "Moonpool is not running" 失败，而不是显示过期的列表。刚启动 Moonpool 后、第一次状态检查之前，应用显示 `[status pending]`。`apps.json` 有错误期间，结果以 `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` 开头。如果文件在 Moonpool 启动时就已损坏，它会说明没有加载任何应用，并同时建议使用 `moonpool_restore_config`。 |
| `moonpool_bootup_launcher` | 无 | 启动 Moonpool 本身，并最多等待 30 秒让它的控制通道应答。返回 "Moonpool started" 或 "Moonpool is already running"。如果新进程立即退出（它把任务交给了一个仍在关闭的 Moonpool），则再启动一次。如果有东西占着通道却不应答，它会报告某个 Moonpool 进程可能已经挂起。 |
| `moonpool_shutdown_launcher` | 无 | 与托盘菜单中的“退出”相同。最多等待 30 秒让控制通道消失。返回 "Moonpool shut down" 或 "Moonpool is not running"。 |
| `moonpool_raise_launcher` | 无 | 把 Moonpool 窗口带到最前。返回 "window shown"。如果 Moonpool 没有运行，它会启动它并返回 "Moonpool was not running; started it"。 |
| `moonpool_start_app` | `app_id`（必填） | 启动应用并打开它的终端标签页。应用进入运行状态后返回 "launched"，否则返回未能启动的原因（`unknown app id: <id>`，25 秒后为 `did not reach running in time`）。对只有 `url` 的 `static` 条目，它会打开该页面，同样返回 "launched"。 |
| `moonpool_stop_app` | `app_id`（必填） | 停止应用。返回 "stopped"，或者返回诸如 `still running after stop`（15 秒后）之类的错误。 |
| `moonpool_restart_app` | `app_id`（必填） | 停止，等待端口和进程释放，再启动。返回 "restarted"。 |
| `moonpool_app_output` | `app_id`（必填），`tail_lines`（整数，默认 200，最小 1） | 应用在当前 Moonpool 会话中的终端输出，已去掉 ANSI 代码。当日志比 `tail_lines` 更长时，文本以一行完整日志的路径开头。如果应用还没有运行过，会以 `no console output recorded for '<id>' (not launched this session)` 失败。如果日志存在但为空，则返回 `(no output recorded for '<id>')`。 |
| `moonpool_stop_mcp_server` | `app_id`（必填） | 结束该应用接入的 MCP 辅助进程，而让应用继续运行。返回 "stopped"。如果应用既没有 `processName` 也没有 `mcpProcessName`，则什么都不做。 |
| `moonpool_refresh_app_icons` | 无 | 重新获取每个应用的图标。返回 "icons refreshed"。 |

## 配置

这些工具通过 hub 读取和修改 `apps.json`，而不是磁盘上的文件。写入必须带上上次读取得到的令牌，过期的令牌会被拒绝，并且在写入任何内容之前会先校验新文件。通过 hub 很重要，因为沙盒宿主中的代理看到的可能是配置文件夹的私有副本，而不是真正的那份。

| 工具 | 参数 | 行为 |
| --- | --- | --- |
| `moonpool_read_config` | 无 | JSON 文本，包含 `manifest_text`（文件的原样内容）、`token`、`valid`、`error`（有效时为 null）和 `path`。文件缺失或为空时，`token` 为 `none`。 |
| `moonpool_write_config` | `manifest`（必填，新的完整 `apps.json` 文本），`expected_token`（必填，来自上次读取） | 校验该清单并替换 `apps.json`，然后加载它。返回 `apps.json updated; new version token <token>`。过期的令牌会以 `stale token: apps.json changed since it was read ...` 失败。无效的清单会以 `rejected invalid manifest: ...` 失败。无论哪种情况，文件都不会被改动。空的 `expected_token` 会被拒绝。 |
| `moonpool_restore_config` | `snapshot`（可选） | 不带值时，返回按从新到旧列出已保存快照的 JSON 文本（`index`、`filename`、`millis`、`app_count`、`valid`）。带索引（1 为最新）或文件名时，会校验该快照并恢复它。返回 `restored <file> (<n> apps); new version token <token>`。不需要令牌：恢复是有意覆盖当前文件的。 |
| `moonpool_reload_config` | 无 | 重新读取 `apps.json`。返回 "apps.json reloaded"。如果文件无法解析或校验，它会以 `apps.json has an error: ...` 失败，Moonpool 继续使用上一次成功加载的列表。 |
| `moonpool_launcher_paths` | 无 | 列出 hub 的配置文件夹、`apps.json`、`state.json`、日志、转储文件夹、图标文件夹、便携标志和 exe 路径，然后列出 MCP 进程的配置文件夹、`apps.json`、`state.json`、转储文件夹、便携标志和 exe 路径（没有日志和图标）。如果 hub 没有运行，它那一半会显示为 `hub paths unavailable: ...`，MCP 那一半仍会显示。当修改没有生效时使用它。 |

## 进阶：测试工具

`moonpool_screenshot` 仅限 Windows；在 Linux 上它会以 "screenshot is not supported on this platform" 失败。`moonpool_window_state` 和 `moonpool_reset_mcp_seen` 在所有平台上都可用。

`window` 是 `main`、`settings`、`about`、`installer`、`editor`、`help` 或 `themes` 之一，默认为 `main`。未知名称会以 `unknown window '<name>'` 失败。

| 工具 | 参数 | 行为 |
| --- | --- | --- |
| `moonpool_screenshot` | `window`（可选） | 以内联 PNG 的形式截取该 Moonpool 窗口自己的内容，较长的一边最多 320 像素。无法通过 MCP 提高这个尺寸。如果窗口没有显示，会以 `window '<name>' is not open` 失败。它无法截取任何其他应用。 |
| `moonpool_window_state` | `window`（可选） | JSON 文本：窗口未打开时为 `{"open":false}`，否则包含 `open`、`visible`、`minimized`、`maximized`、`x`、`y`、`width`、`height`。用于测试。 |
| `moonpool_reset_mcp_seen` | `app_id`（可选） | 仅供测试。清除某个应用（省略时则为所有应用）已记住的“见过 MCP 辅助进程”记录，这样侧边栏的 MCP 子行会再次隐藏，直到再次见到辅助进程。 |

## 另请参阅

- [MCP 设置](/zh-hans/automation/mcp-setup/)
- [命令行](/zh-hans/automation/command-line/)
