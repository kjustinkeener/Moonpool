---
title: "为 AI 代理（Claude Code、Codex、Cursor）提供启动和停止本地应用的 MCP 服务器"
description: "把 Moonpool 注册为 MCP 服务器，让 Claude Code、Codex 或 Cursor 可以启动、停止、重启开发服务器并读取其输出，而不会多出重复的进程。"
---

AI 编程代理通常是在它自己的 shell 里输入 `npm run dev` 来运行你的开发服务器。这可能会卡住代理，留下占着端口的孤儿进程，或者把你已经在运行的东西又启动第二份。MCP 服务器让代理调用工具去启动和停止你已经配置好的应用，而不是自己去拼凑它的命令行。

## Moonpool 的做法

Moonpool 的可执行文件本身就是它的 MCP 服务器：把 `moonpool.exe` 加上唯一的参数 `mcp`，注册为 stdio 服务器。应用写进 `apps.json` 之后，代理就可以按 id 启动它。

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

注册服务器。在 Claude Code 中，一条命令即可（已安装的 Moonpool；请使用你自己 exe 的完整路径）：

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

读取 MCP 服务器 JSON 文件的宿主（例如 Cursor 的 `mcp.json`）使用相同的结构（反斜杠要写成双份）：

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

对于 Codex，在它的配置（`~/.codex/config.toml`）中添加一个使用相同命令和 `mcp` 参数的服务器：

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

具体的文件名和键名由各个宿主决定，如果你的版本不同，请查看它的 MCP 文档。Moonpool 只需要 `moonpool.exe` 的完整路径和作为参数的 `mcp`。之后请重启宿主。

## 代理能做什么

这些工具以 `moonpool_*` 的名称出现。日常工作用到的有：

| 工具 | 用途 |
| --- | --- |
| `moonpool_list_apps` | 找到应用的 id，并查看它是否在运行。 |
| `moonpool_start_app` | 按 id 启动应用并打开它的终端标签页。 |
| `moonpool_stop_app` | 停止它，包括它的子进程。 |
| `moonpool_restart_app` | 停止，等待端口空出，再启动。修改代码后使用。 |
| `moonpool_app_output` | 读取应用打印的内容，可用 `tail_lines` 限制行数。 |
| `moonpool_bootup_launcher` | 在 Moonpool 没有运行时启动 Moonpool 本身。 |

典型的循环是先 `moonpool_restart_app`，再 `moonpool_app_output`。其余工具（读写 `apps.json`、截图）见 [MCP 工具](/zh-hans/automation/mcp-tools/)。

## 如果不起作用

如果每个工具都返回 `Moonpool is not running - call moonpool_bootup_launcher first`（Moonpool 没有运行，请先调用 moonpool_bootup_launcher），说明 Moonpool 还没有启动。修改没有生效，通常是因为代理看的是另一份 `apps.json`：请调用 `moonpool_launcher_paths`。参见[如果这些工具不起作用](/zh-hans/automation/mcp-setup/#如果这些工具不起作用)。

## 另请参阅

- [MCP 设置](/zh-hans/automation/mcp-setup/)
- [MCP 工具](/zh-hans/automation/mcp-tools/)
- [AI 代理：快速开始](/zh-hans/automation/quick-start/)
- [在 Windows 上让 npm 开发服务器在后台运行](/zh-hans/guides/run-npm-dev-server-in-background-windows/)
