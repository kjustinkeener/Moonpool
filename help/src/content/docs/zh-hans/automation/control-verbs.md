---
title: "Moonpool 控制通道与动词参考"
description: "Moonpool 控制通道（命名管道或 Unix 套接字）如何工作，它的协议，以及运行中的应用所应答的每个动词，包括参数和回复。"
---

## 它在哪里监听

每个 Moonpool 副本都有自己的通道，所以已安装的 Moonpool 和任意便携副本可以并排运行，而不会替彼此应答。在 Windows 上，已安装的 Moonpool 监听命名管道 `\\.\pipe\moonpool`。便携副本会加上由其文件夹生成的 id：`\\.\pipe\moonpool-<id>`。

`<id>` 是由该副本的 `moonpool-config` 文件夹路径派生出的 8 位十六进制数，所以对同一个文件夹，它在重启和更新之间保持不变，而如果你移动了文件夹，它就会改变。副本的 `moonpool.exe`（包括 `moonpool.exe mcp`）总是能找到它自己副本的通道。

在 Linux 和 macOS 上，它改为监听 Unix 域套接字，权限为 `0600`：

| 情形 | 套接字路径 |
| --- | --- |
| 普通 | 设置了 `$XDG_RUNTIME_DIR` 时为 `$XDG_RUNTIME_DIR/moonpool.sock`，否则为 Moonpool 配置文件夹中的 `moonpool.sock` |
| 便携模式 | 便携副本配置文件夹中的 `moonpool.sock`，所以便携副本绝不会与已安装的副本冲突 |
| 路径对套接字来说太长（约 100 个字符） | `/tmp/moonpool-<uid>/moonpool.sock`，位于只有你能打开的目录中（便携副本为 `moonpool-<id>.sock`） |

崩溃遗留下来的套接字文件会在下次启动时被检测到并替换。仍有东西在应答的套接字绝不会被接管。Moonpool 正常退出时会删除该文件。

[MCP 服务器](/zh-hans/automation/mcp-setup/)也是靠这个通道得知 Moonpool 是否在运行的：如果 `ping` 得到了应答，它就在运行；管道或套接字不存在，它就没有运行。除下面的诊断动词外，同样的动词也可以从[命令行](/zh-hans/automation/command-line/)使用。

## 协议

每行输入一个 JSON 对象，输出一行 JSON，按顺序进行。一个连接可以承载多个请求。

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

在 PowerShell 中发送请求并读取回复：

对便携副本，请用它的管道名称（`moonpool-<id>`，由 `paths` 动词显示）代替 `moonpool`。

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` 是字符串列表，可以省略。其他字段会被忽略。
- `result` 是字符串或 null。返回结构化数据的动词会把它作为 JSON 字符串返回。
- 不是合法 JSON 的行会得到 `{"ok": false, "error": "bad request: ..."}`。
- 未知的 `cmd` 会得到 `unknown cmd: <name>`。
- 经由窗口处理的动词（`launch`、`stop`、`restart`、`reload`、`refresh-icons`、`help`、`open-window`）会在操作完成时应答，或在 45 秒后以超时错误应答。如果主窗口的界面还没有加载，它会立即以 `frontend not loaded` 失败。
- 在前一个 Moonpool 仍在退出时启动的 Moonpool，会重试绑定通道约 8 秒。如果仍然不行，它会记录这一点，并在没有通道的情况下继续运行。

## 动词

| 动词 | 参数 | 结果 |
| --- | --- | --- |
| `ping` | 无 | `pong`。仅限通道。 |
| `list` | 无 | JSON 字符串 `{"apps": [...], "statuses": [...]}`，读取自运行中 hub 的内存，`apps` 和 `statuses` 的结构与 `state.json` 相同。当已登记应用但还没进行第一次状态检查时，会加上 `"statusNotReady": true`。当 `apps.json` 加载失败时，会加上 `"manifestError": "<message>"`（此时应用是上一次成功加载的列表），并且如果自启动以来还没有加载过任何列表，再加上 `"manifestLoaded": false`。仅限通道。 |
| `show` | 无 | null。把窗口带到最前。 |
| `quit` | 无 | null。退出 Moonpool。 |
| `launch` | `<id>` | 成功时为 null，对只有 `url` 的 `static` 条目为 `opened`。错误：`unknown app id: <id>`、`did not reach running in time`。 |
| `stop` | `<id>` | 成功时为 null，对只有 `url` 的 `static` 条目为 `stopped`。错误：`still running after stop`。 |
| `restart` | `<id>` | 结果和错误与 `launch` 相同。 |
| `reload` | 无 | 成功时为 null。 |
| `refresh-icons` | 无 | 成功时为 null。 |
| `help` | 无 | null。打开帮助窗口。 |
| `dump` | `<id>` [`out-path`] | 该应用会话日志的路径，或位于 `out-path` 的纯文本副本的路径。 |
| `paths` | 无 | hub 所使用的文件夹和 exe 的多行报告。 |
| `read-config` | 无 | `dumps\read-config.json` 的路径，其中包含 `token`、`valid`、`error`、`path`、`manifest_text`。 |
| `write-config` | `<source-file>` [`token`] | 新的版本令牌。错误：`stale token: ...`、`rejected invalid manifest: ...`、`cannot read source ...`。 |
| `restore-config` | [`index` 或 `filename`] | 不带参数：`dumps\restore-config.json` 的路径（`count`、`snapshots`）。带参数时：`restored <file> (<n> apps); new version token <token>`。 |
| `argv` | 命令行参数 | 立即返回 null。会像该副本的第二个 `moonpool.exe <args>` 那样原样运行它们，包括 `--ticket`。第二次启动正是靠它在退出前把自己的参数交过来。 |

交互示例：

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` 和 `restore-config` 会立即加载新的清单，在 `apps.json.history\` 中记录一份快照，并刷新窗口。

## 诊断动词（测试用）

仅限通道：命令行不接受这些动词。除 `screenshot` 外，在 Windows、Linux 和 macOS 上都能用；`screenshot` 仅限 Windows，在其他系统上会回答 `screenshot is not supported on this platform (Windows only)`。

| 动词 | 参数 | 结果 |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | 仅限 Windows。该 Moonpool 窗口（默认 `main`）的 PNG 的 Base64。可选的 `max_dim` 限制较长一边的像素数（限定在 320-2400，默认 320；MCP 工具始终使用默认值）。非整数的 `max_dim` 是错误。允许的窗口：`main`、`settings`、`about`、`installer`、`editor`、`help`、`themes`。错误：`unknown window '<name>'`、`window '<name>' is not open`。不会写入磁盘。 |
| `open-window` | `<kind>` [`<id>`] | null。像它的菜单项那样打开一个窗口。`kind`：`settings`、`about`、`installer`、`help`、`themes`、`editor`（可选的 `<id>` 会打开该应用的“编辑应用”对话框，不带则打开“添加应用”）、`terminal`（必须带 `<id>`：选中该应用的终端标签页，并加宽主窗口以显示 CLI 面板；不会启动它）、`cli`（只加宽主窗口）。错误：`unknown window kind '<kind>'`、`terminal needs an app id`、`unknown app id: <id>`。与 `launch` 一样经由主窗口应答。 |
| `window-state` | [`window`] | JSON 字符串：`{"open":false}`，或包含 `open`、`visible`、`minimized`、`maximized`、`x`、`y`、`width`、`height`。 |
| `stop-mcp` | `<id>` | `stopped`。结束该应用的 `<processName> mcp` 辅助进程，而不是应用本身。错误：`missing app id`、`unknown app id: <id>`。 |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` 或 `<id>: was not marked seen`；不带 id 时为 `cleared <n> entries`。清除已记住的 MCP 辅助进程记录。 |

命令行的 `--ticket` 和 `state.json` 结果记录属于另一个通道；参见[命令行](/zh-hans/automation/command-line/#读取结果)。通道请求的答案直接在回复中给出。

## 另请参阅

- [命令行](/zh-hans/automation/command-line/)
- [AI 代理：快速开始](/zh-hans/automation/quick-start/#同一个操作的三种写法)
