---
title: "从命令行控制 Moonpool"
description: "在终端或脚本中用 moonpool.exe 动词驱动运行中的 Moonpool，用 ticket 标记命令，并从 state.json 读取结果。"
---

当同一个 Moonpool 已在运行时，再次运行 `moonpool.exe` 不会打开第二个窗口。第二个进程会通过[控制通道](/zh-hans/automation/control-verbs/)把它的参数传给运行中的那个，然后退出。Moonpool 必须已经在运行：如果没有常驻的实例，同样的命令会启动一个新的 Moonpool，而该动词不会被执行。

“同一个 Moonpool”指的是同一个文件夹。已安装的 Moonpool 和每个便携副本各自独立运行，所以一条命令只会到达你运行的那个 `moonpool.exe` 所对应的副本，绝不会到达别的副本。参见[便携模式](/zh-hans/data/portable-mode/#同时运行多个副本)。

请使用你所指副本的路径。对已安装的副本：

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

有多个副本在运行时，`Get-Process moonpool` 会把它们全部列出，所以请按 `Path` 挑选，而不要取第一个。它还会列出 MCP 宿主启动的空闲 `moonpool.exe mcp` 辅助进程，所以存在 `moonpool` 进程并不能证明有 hub 在运行。请改用 `ping` 询问控制通道（[控制动词](/zh-hans/automation/control-verbs/)）。

## 动词

动词不区分大小写。`<id>` 是 `apps.json` 中某个应用的 `id`。

| 命令 | 效果 |
| --- | --- |
| `moonpool.exe` | 不带动词：把窗口带到最前。 |
| `moonpool.exe show` | 把窗口带到最前。 |
| `moonpool.exe launch <id>` | 启动应用并打开它的终端标签页。 |
| `moonpool.exe stop <id>` | 停止应用。 |
| `moonpool.exe restart <id>` | 停止，等待端口和进程释放，再启动。 |
| `moonpool.exe reload` | 重新读取 `apps.json`。 |
| `moonpool.exe refresh-icons` | 重新获取每个图标。 |
| `moonpool.exe help` | 打开帮助窗口。 |
| `moonpool.exe quit` | 退出 Moonpool，与托盘菜单相同。 |
| `moonpool.exe dump <id> [out-path]` | 不带 `out-path` 时，报告该应用在本次会话中的日志路径。带上它时，把日志以去掉 ANSI 代码的纯文本形式复制到那里。 |
| `moonpool.exe paths` | 报告运行中的 Moonpool 所使用的配置文件夹、`apps.json`、`state.json`、日志、转储文件夹、图标文件夹、便携标志和 exe 路径。 |
| `moonpool.exe read-config` | 在配置文件夹中写出 `dumps\read-config.json`，其中包含 `token`、`valid`、`error`、`path` 和 `manifest_text`（`apps.json` 的原样内容）。 |
| `moonpool.exe write-config <file> [token]` | 用 `<file>` 中的清单替换 `apps.json`，前提是该清单有效，并且在给出 `token` 时 `apps.json` 仍与它一致。 |
| `moonpool.exe restore-config [index or filename]` | 不带参数时，把快照列表写入 `dumps\restore-config.json`。带参数时，如果该快照有效就恢复它。 |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

未知的动词会被忽略。该程序还有它自己的启动参数：`moonpool.exe mcp`（[MCP 设置](/zh-hans/automation/mcp-setup/)）、`--uninstall`（由“添加/删除程序”使用）和 `--wait-pid <pid>`（Moonpool 重新启动自身时使用）。这些参数只有作为第一个参数时才会生效，所以像 `--uninstall` 这样的应用 id 无法触发它们。

## 读取结果

命令行不会打印任何内容，所以请用 `--ticket <key>`（任意唯一的键，位置不限）标记一条命令，然后从配置文件夹中的 `state.json` 读取结果。该文件夹在安装版是 `%USERPROFILE%\.moonpool\moonpool-config\`，便携副本是 `<your .moonpool folder>\moonpool-config\`，Linux 上是 `~/.config/Moonpool/`（参见[配置概览](/zh-hans/apps/apps-json/#配置位于何处)）。`show` 和 `quit` 不写入 ticket。

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` 包含 `apps`、`statuses`（每个应用的 `id`、`running`、`managed`、`mcpRunning`、`mcpSeen`）和 `tickets`。运行中的 Moonpool 每隔几秒以及每条命令之后都会重写它，并且在退出时不会删除它，所以遗留下来的文件并不代表 Moonpool 在运行。要询问它是否在运行，或获取实时的应用列表，请使用控制通道的 `ping` 和 `list` 动词（[控制动词](/zh-hans/automation/control-verbs/)）或 MCP 工具。请轮询你的 ticket，直到 `status` 不再是 `pending`：

| `status` | 含义 |
| --- | --- |
| `pending` | 已收到；Moonpool 仍在处理。 |
| `ok` | 完成。对 `dump`、`read-config`、`write-config`、`restore-config` 和 `paths`，`detail` 里是路径、令牌或报告。 |
| `error` | 失败；`detail` 说明原因，例如 `unknown app id: x`、`did not reach running in time`、`unknown command`。 |

每个 ticket 是 `{ ticket, action, arg, status, detail, ts }`，其中 `ts` 为 Unix 毫秒时间戳：

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

已完成的 ticket 会在 24 小时后被丢弃；一旦已完成的 ticket 至少有 5 分钟之久，列表就会向 50 条的数量修剪。

支持 MCP 的代理可以省去轮询：参见 [MCP 设置](/zh-hans/automation/mcp-setup/)。

## 另请参阅

- [AI 代理：快速开始](/zh-hans/automation/quick-start/)
- [控制动词](/zh-hans/automation/control-verbs/)
