---
title: "apps.json 的每个字段：类型、默认值及作用"
description: "查阅 apps.json 条目的每个键，包括它的类型、默认值和适用的应用类型，名称与“编辑应用”对话框中的一致。"
---

“编辑应用”对话框以相同的名称显示相同的字段。不适用于所选类型的字段在对话框中会变暗，但仍会被保存，只有一个例外：`stopCommand` 仅在 `killMode` 为 `command` 时才会保存。

![“编辑应用”对话框，从 name 到 stopCommand，killMode 下拉框被框出；processName 和 stopCommand 等未使用的字段呈暗色](../../../../assets/screenshots/edit-app-dialog.png)

1. `killMode` 下拉框。它用不到的字段会保持变暗。

| 字段 | 类型 | 必填 | 适用于 | 作用 |
| --- | --- | --- | --- | --- |
| `id` | string | 是 | 全部 | 唯一键。可用字母、数字、`.`、`_`、`-`，不能以 `-` 开头。参见[概览](/zh-hans/apps/apps-json/#id)。 |
| `name` | string | 是 | 全部 | 侧边栏中的标签。不能为空白。 |
| `group` | string | 是 | 全部 | 应用在侧边栏中所归属的标题。手动编辑时不能为空白；对话框会把空白分组保存为 `Apps`。可以是任意文字，新名称会创建新分组。 |
| `type` | string | 是 | 全部 | `web`、`desktop`、`static` 或 `cli`。参见[应用类型](/zh-hans/apps/types/)。 |
| `command` | string | 除 `static` 外都必填 | 全部 | 在终端中运行以启动应用，Windows 上通过 `cmd /c`，其他系统通过 `$SHELL -c`（未设置 `SHELL` 时用 `/bin/sh`）。对 `static` 是可选的。 |
| `cwd` | string | 否 | 所有带 `command` 的类型 | 命令运行所在的文件夹。默认是 Moonpool 自己的工作文件夹。支持令牌和 `./`。参见[路径与环境](/zh-hans/apps/paths-and-environment/)。 |
| `port` | integer，1 到 65535 | 否 | 任意 | 只要 localhost 上这个端口有响应（IPv4 或 IPv6）就算运行中。由 `killMode` 为 `port` 时读取。 |
| `processName` | string | 否 | 任意，主要是 `desktop` | 只要存在使用此名称的进程就算运行中。不区分大小写，带不带 `.exe` 均可，所以 `my-app` 能匹配 `my-app.exe`。在 Linux 上不超过 15 个字符。由 `killMode` 为 `processName` 时读取。 |
| `mcpProcessName` | string | 否 | 任何带 `processName` 的类型 | 此应用的 MCP 服务器进程名所用的通配符模式。`*` 匹配任意一段字符，`?` 匹配单个字符。不区分大小写，与完整名称比较，`.exe` 可省略。匹配的进程会被视为该应用的 MCP 服务器（侧边栏中的 MCP 子行），并且不要求 `mcp` 作为它的第一个参数。参见 [mcpProcessName](#mcpprocessname)。 |
| `url` | string | 仅 `static` 必填 | `web`、`static` | 要打开的页面。只会打开 `http://`、`https://`、`mailto:` 和 `file://` 网址。 |
| `openBrowser` | boolean，默认 `false` | 否 | 任何带 `url` 的类型（对话框会对 `desktop` 和 `cli` 将其变暗） | 一旦 Moonpool 检测到应用已就绪，就自动打开 `url`（见下文）。 |
| `killMode` | string | 否 | 全部 | 停止和重启时的额外清理：`processName`、`port`、`command` 或 `none`。参见[停止与重启](/zh-hans/apps/stop-and-restart/)。 |
| `stopCommand` | string | 否 | `killMode` 为 `command` | 停止时运行的命令。在其他任何模式下都会被忽略。 |
| `env` | object of strings | 否 | 全部 | 额外的环境变量。对话框以每行一个 `KEY=VALUE` 的形式编辑它。 |
| `icon` | string | 否 | 全部 | 侧边栏图像：文件路径、`http(s)` 网址或 `data:` URI。可通过应用右键菜单中的 **设置图标...** 设置，也可以手动设置。 |
| `note` | string | 否 | 全部 | 在侧边栏中把鼠标悬停在应用上时显示的提示。 |

一个使用了 `env` 和 `killMode` 的条目：

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

`port` 和 `killMode` 如何配合，参见[查找并结束占用端口的进程](/zh-hans/guides/find-and-kill-process-using-port-windows/)。

## mcpProcessName

默认情况下，当某个进程的名称与 `processName` 匹配、且它的第一个参数是 `mcp`（例如 `notes-app.exe mcp`）时，Moonpool 会把它视为该应用的 MCP 服务器。当服务器使用不同的名称运行时，请设置 `mcpProcessName`：比如应用监视的是一个 exe，而它的 MCP 服务器是另一个（`mog.exe mcp`），或者是服务器的一个改过名的副本。

该值是一个通配符模式。`*` 匹配任意一段字符（包括空），`?` 恰好匹配一个字符。它与完整进程名不区分大小写地比较，不带 `.exe` 的模式也能匹配带 `.exe` 的名称。空值视为未设置。

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

这能匹配改过名的副本，例如 `destiny-mcp-2706210170.exe`。匹配 `mcpProcessName` 的进程无论是否带 `mcp` 参数启动，都算服务器，并且绝不会被当作应用本身在运行。如果该模式同时也匹配 `processName` 本身（例如 `destiny*`），Moonpool 仍然要求带 `mcp` 参数，这样真正的应用就绝不会被误认为它的 MCP 服务器。参见 [MCP 设置](/zh-hans/automation/mcp-setup/#有自己-mcp-服务器的应用)。

## openBrowser

当一个由 Moonpool 启动的应用第一次显示为“运行中”时，Moonpool 会打开一次 `url`。这需要有 `port` 或 `processName` 才能检测到。如果两者都没有，“运行中”只表示终端进程还活着，浏览器不会自动打开。如果你的命令自己会打开浏览器，请关闭 `openBrowser`。没有命令的 `static` 条目，无论 `openBrowser` 如何，每次按下启动都会打开 `url`。

两个配置了相同 `port` 的应用会在侧边栏中被标出。

## 图标

应用的图标取下面第一个存在的：

1. `icon` 字段。
2. 配置文件夹中的 `icons\<id>.<ext>`，例如 `icons\site.png`。
3. 应用自己文件夹（它的 `cwd`，或 `file:///` `url` 所在的文件夹）中的图标文件。
4. 对 `desktop`，取它已构建或正在运行的 `.exe` 的图标。
5. 对 `web` 和 `static`，在服务器就绪后取网站的 `/favicon.ico`。
6. 该类型对应的字形。

大多数应用都不需要设置图标。
