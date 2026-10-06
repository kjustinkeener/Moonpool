---
title: "Moonpool 错误信息详解：already running、requires a command 等"
description: "查询 Moonpool 错误信息的确切文字，例如 already running、requires a command、stale token 和 Update failed，以及各自的含义和解决办法。"
---

把你看到的信息粘贴到页面搜索中，或者浏览下面的表格。信息按 Moonpool 显示的原样引用。`<angle brackets>` 中的文字会被某个值替换（应用 id、路径或来自系统的错误）。不属于错误信息的症状见[故障排除与常见问题](/zh-hans/support/troubleshooting/)。

## 启动和停止应用

| 信息 | 含义与解决办法 |
| --- | --- |
| `already running` | Moonpool 已经为这个应用持有一个终端。请先停止它，或使用重启。 |
| `stopped during launch` | 启动仍在进行时按下了停止。请再次启动。 |
| `app has no launch command` | 该条目没有 `command`。请在应用编辑器或 `apps.json` 中添加。只有带 `url` 的 `static` 条目可以没有。 |
| `unknown app: <id>` | 没有加载具有该 `id` 的应用。请检查 id；如果手动编辑过 `apps.json`，请重新加载。 |
| `unknown app id: <id>` | 同样的问题，只是报告给脚本或代理。请用 `moonpool_list_apps` 列出应用。 |
| `did not reach running in time` | 来自脚本或代理：应用在 25 秒内没有显示为运行中。请检查 `port` 或 `processName`，并查看输出。参见[状态圆点不对](/zh-hans/support/troubleshooting/#状态圆点不对)。 |
| `still running after stop` | 15 秒后应用仍显示为运行中。请设置 `killMode`。参见[停止与重启](/zh-hans/apps/stop-and-restart/)。 |
| `refusing to open non-web url: <url>` | `url` 不是 `http://`、`https://`、`mailto:` 或 `file://`。请修正 `url`。 |
| `[process exited]` | 不是错误：应用的命令已结束。显示在终端标签页中。 |

## apps.json 校验

Moonpool 会拒绝违反某条规则的 `apps.json`，并保留上一次成功加载的列表。`<n>` 是该条目在文件中的位置，从 1 开始计数。

| 信息 | 解决办法 |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | 重命名 `id`。 |
| `duplicate app id "<id>"` | 两个条目使用了同一个 `id`。请让每个都唯一。 |
| `apps.json entry <n> (<id>) has an empty name` | 填写 `name`。 |
| `apps.json entry <n> (<id>) has an empty group` | 填写 `group`。 |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` 必须是 `web`、`desktop`、`static` 或 `cli`。 |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` 必须在 1 到 65535 之间。 |
| `apps.json entry <n> (<id>) requires a url` | `static` 条目需要 `url`。 |
| `apps.json entry <n> (<id>) requires a command` | 其他每种类型都需要 `command`。 |

在应用编辑器中，不填名称就保存会显示“name 为必填项。”。横幅文字“apps.json 有错误，当前显示的是上次成功加载的列表。”或“apps.json 有错误，因此没有加载任何应用。”，以及如何恢复，见 [apps.json 有错误](/zh-hans/support/troubleshooting/#appsjson-有错误)。如果横幅说保存已暂停，信息会以 `Repair apps.json and reload it before saving from Moonpool` 结尾。完整的规则列表见[校验](/zh-hans/apps/apps-json/#校验)。

## 设置、更新和安装程序

| 信息 | 含义与解决办法 |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` 格式有误。请修复或删除它，然后重启。参见 [settings.json](/zh-hans/data/settings-json/#读取与修复)。 |
| `Update failed: <error>` | 更新的下载或安装失败。参见[更新失败时](/zh-hans/data/updating/#更新失败时)。 |
| `Update check failed: <error>` | “关于”中的更新检查失败。冒号后面的文字说明了原因。请稍后再试。 |
| `Install failed: <error>` | 安装程序在冒号后面所指的步骤停止了，例如 `copy exe: ...`。请退出所有从 `%USERPROFILE%\.moonpool` 运行的 Moonpool，然后重试。 |
| `target folder does not exist` | 为便携副本选的文件夹已经不存在。请选一个已存在的文件夹。 |

## MCP 与脚本

| 信息 | 含义与解决办法 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | 启动 Moonpool，或者让代理调用那个工具。对便携副本，这条信息会写出该副本的名称。 |
| `frontend not loaded` | 主窗口还没有加载完。请稍等后重试。 |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | 代理传入了 MCP 服务器不接受的 id。请使用 `moonpool_list_apps` 给出的 id。 |
| `stale token: apps.json changed since it was read ...` | 重新读取 `apps.json`，重新应用修改，然后再写入。 |
| `rejected invalid manifest: ...` | 新的 `apps.json` 未通过校验（见上文）。文件没有被改动。 |
| `no console output recorded for '<id>' (not launched this session)` | `moonpool_app_output` 被用来查询一个自 Moonpool 启动以来没有运行过的应用。 |

更多内容见 [MCP 工具](/zh-hans/automation/mcp-tools/)和 [MCP 设置](/zh-hans/automation/mcp-setup/#如果这些工具不起作用)。

## 来自其他程序的错误

- [`Error: listen EADDRINUSE: address already in use :::3000` 和 `Port 5173 is in use`](/zh-hans/support/port-already-in-use/)
- [`Windows 已保护你的电脑`](/zh-hans/support/windows-protected-your-pc/)
- [缺少 WebView2 运行时](/zh-hans/support/webview2-runtime-missing/)
