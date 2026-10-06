---
title: "让 AI 代理设置并操作 Moonpool：快速开始"
description: "让 AI 代理或脚本设置并操作 Moonpool 的三种方式，如何根据你的代理选择，以及同一个操作在每种方式下的写法。"
---

有三种入口。请根据你的代理能做什么来选择。

| 你想要 | 使用 | 从这里开始 |
| --- | --- | --- |
| 让代理找到你的应用并一次性添加它们 | 主窗口空白界面上的 **复制提示词** | 见下文 |
| 让代理以工具调用的方式启动、停止和读取应用 | MCP 服务器，`moonpool.exe mcp` | [MCP 设置](/zh-hans/automation/mcp-setup/) |
| 脚本，或不支持 MCP 的代理 | 命令行动词 | [命令行](/zh-hans/automation/command-line/) |

## 复制提示词

没有打开任何标签页时，CLI 面板会显示一段现成的提示词（“第一次使用？把这段内容交给 AI 助手，让它帮你配置应用：”）。**复制提示词** 会把它放到剪贴板。把它粘贴给你的代理。它会让代理查看配置文件夹中的 `AI-README.md` 和 `apps.json`，并请它找到你的应用并登记。完成后，选择 **重新加载**。

Moonpool 每次启动都会重写 `apps.json` 旁边的 `AI-README.md`，所以它始终与你运行的版本一致。请不要把你自己的修改保存在里面。

## 同一个操作的三种写法

| 操作 | 命令行 | 控制通道动词 | MCP 工具 |
| --- | --- | --- | --- |
| 启动应用 | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| 停止应用 | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| 重启应用 | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| 读取应用的输出 | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| 列出应用及状态 | 读取 `state.json` | `list` | `moonpool_list_apps` |
| 重新读取 `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| 读取 `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| 替换 `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| 回滚 `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| 显示窗口 | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| 启动 Moonpool | `moonpool.exe` | 无 | `moonpool_bootup_launcher` |
| 退出 Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| 显示正在使用的文件夹 | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

命令行不会打印任何内容；请用 `--ticket` 读取结果（参见[读取结果](/zh-hans/automation/command-line/#读取结果)）。控制通道和 MCP 则会直接回答。

## 当代理的工具失败时

- `Moonpool is not running - call moonpool_bootup_launcher first`（Moonpool 没有运行，请先调用 moonpool_bootup_launcher）：启动 Moonpool，或者让代理调用那个工具。
- 修改“没有生效”：让代理调用 `moonpool_launcher_paths`。如果主窗口和 MCP 的文件夹不同，说明代理读取的是另一份 `apps.json`。参见[沙盒宿主](/zh-hans/automation/mcp-setup/#沙盒宿主)。
- 有多个 Moonpool 副本：为每个副本用各自的名称注册。参见[多个 Moonpool](/zh-hans/automation/mcp-setup/#多个-moonpool)。

针对 Claude Code、Codex 和 Cursor 的完整示例见[为 AI 代理提供启动和停止本地应用的 MCP 服务器](/zh-hans/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)。

更多症状见[故障排除](/zh-hans/support/troubleshooting/#mcp-与脚本错误)。
