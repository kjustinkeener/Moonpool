---
title: "通过 MCP 把 AI 代理连接到 Moonpool"
description: "在你的宿主中把 moonpool.exe mcp 注册为 stdio MCP 服务器（安装版或便携版均可），并了解 Moonpool 如何跟踪应用自己的 MCP 辅助进程。"
---

Moonpool 的可执行文件本身就是它的 MCP 服务器。在宿主中把它注册为 stdio 服务器，运行 `moonpool.exe` 并只带一个参数 `mcp`。

## 注册服务器

安装版的程序是 `%USERPROFILE%\.moonpool\moonpool.exe`。便携版则是你 `.moonpool\` 文件夹里的 `moonpool.exe`。把这个完整路径用作 `command`。对于读取 `.mcp.json` 的宿主：

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

在 JSON 文件中，反斜杠必须写成双份，如上所示。对于提供命令行注册方式的宿主（例如 Claude Code），一步就能添加：

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

该服务器以 `moonpool` 作为自己的名称，使用 MCP 协议版本 `2025-06-18`，并且只提供工具（不列出资源或提示词）。这些工具在代理看来是 `moonpool_*`，参见 [MCP 工具](/zh-hans/automation/mcp-tools/)。

## 多个 Moonpool

已安装的 Moonpool 和每个便携副本都是各自独立的启动器，各有自己的应用，并且可以同时运行。某个副本的 `moonpool.exe mcp` 总是驱动那个副本。要让代理使用多个副本，请为每一个注册一个不同的名称，指向该副本的 exe：

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

在大多数宿主中，把两个副本注册为同一个名称会让一个替换掉另一个。每个副本的工具名称都相同，所以宿主靠你注册时用的名称来区分它们。便携副本还会把自己声明为 `moonpool (<folder>)`，它的服务器说明里也会写出文件夹，这样代理就能看出自己在和哪个副本通信。

## 说明

- `moonpool.exe mcp` 从不打开窗口，也从不启动安装程序。宿主关闭它的输入时，它就会退出。
- 它使用启动它的那个 exe 的配置文件夹和控制通道，所以便携版 exe 读取便携文件夹的数据，并驱动那个便携副本。只有 `moonpool.portable` 位于它旁边时，这个 exe 才算便携版。任何其他 `moonpool.exe`，无论放在哪里，都使用已安装的 Moonpool 的文件夹（`%USERPROFILE%\.moonpool\moonpool-config\`），并驱动已安装的 Moonpool。
- 大多数工具需要 Moonpool 正在运行。如果它没有运行，代理可以先调用 `moonpool_bootup_launcher`。
- `moonpool_launcher_paths` 会把 hub 使用的文件夹与 MCP 进程解析出的文件夹并排显示。两者不同，就说明代理看的是与 hub 不同的 `apps.json`。

## 沙盒宿主

有些宿主在打包（Store/MSIX）沙盒中运行它们的工具，沙盒会把 AppData 重定向到每个包各自私有的副本。当 Moonpool 的配置文件夹或 exe 解析到类似 `...\Packages\<package>\LocalCache\...` 的路径下时，它就会检测到这一点。

当控制通道有应答但无法读取 `state.json` 时，它也会检测到。这时读写文件的工具（`moonpool_app_output`、`moonpool_read_config`、`moonpool_write_config`、`moonpool_restore_config`）会返回一条指明原因的错误，而不是空数据或过期数据。只使用控制通道的工具（例如 `moonpool_list_apps`）在通道可达时不会被拦截。如果沙盒连通道也隐藏了，这些工具会报告沙盒的问题，而不是“Moonpool is not running”。请改在沙盒之外的 shell 中使用[命令行](/zh-hans/automation/command-line/)。

## 有自己 MCP 服务器的应用

Moonpool 中的许多应用，本身就是由 MCP 宿主通过一个 `<exe> mcp` 辅助进程来访问的。Moonpool 会查找名称与应用的 `processName` 匹配、且第一个参数为 `mcp` 的进程，例如 `notes-app.exe mcp`。如果服务器使用别的名称运行（比如改过名的副本），请设置应用的 `mcpProcessName` 通配符（参见[字段](/zh-hans/apps/fields/#mcpprocessname)）；匹配它的进程不需要 `mcp` 参数也会被计入。

- 有辅助进程接入时，应用的侧边栏会把一个 MCP 子行显示为运行中，`moonpool_list_apps` 也会在该应用那一行后附加 `[mcp: running]`。辅助进程不算作应用本身在运行。
- 一旦见到过某个辅助进程，Moonpool 就会记住它（保存在配置文件夹中的 `mcp_seen.json`），所以在辅助进程退出之后，MCP 子行仍会显示为已停止，`moonpool_list_apps` 则显示 `[mcp: stopped]`。
- MCP 子行由 `showMcpProcesses` 设置控制（[设置窗口](/zh-hans/using/settings/)）。
- `moonpool_stop_mcp_server` 会结束辅助进程，而不动应用。没有对应的启动操作：拥有该辅助进程的宿主会在它下一次工具调用时再次启动它。

## 如果这些工具不起作用

- **宿主没有显示任何 `moonpool_*` 工具。** 检查 `command` 是否是 `moonpool.exe` 的完整路径，`args` 是否为 `["mcp"]`，然后重启宿主。
- **每个工具都说 Moonpool 没有运行。** 启动 Moonpool，或调用 `moonpool_bootup_launcher`。确认注册的 exe 就是你正在运行的那个副本。
- **修改没有生效。** 调用 `moonpool_launcher_paths`，比较 hub 的文件夹和 MCP 进程的文件夹。参见[沙盒宿主](#沙盒宿主)。

更多内容见[故障排除](/zh-hans/support/troubleshooting/#mcp-与脚本错误)。

## 另请参阅

- [为 AI 代理（Claude Code、Codex、Cursor）提供启动和停止本地应用的 MCP 服务器](/zh-hans/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [MCP 工具](/zh-hans/automation/mcp-tools/)
- [AI 代理：快速开始](/zh-hans/automation/quick-start/)
