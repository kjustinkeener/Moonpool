---
title: "选择应用类型：web、desktop、static 或 cli"
description: "了解 web、desktop、static 和 cli 应用在 Moonpool 中如何启动，各自如何检测“运行中”，以及“停止”按钮默认会做什么。"
---

`type` 决定哪些字段有意义，以及“停止”默认做什么。

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| 需要 | `command` | `command` | `url` | `command` |
| 通常还有 | `port`、`url` | `processName` | `command` 和 `port`（如果它自己提供服务） | `cwd` |
| 启动 | 在终端标签页中运行 `command` | 在终端标签页中运行 `command` | 没有 `command`：在浏览器中打开 `url`。有 `command`：在终端标签页中运行它 | 在终端标签页中运行 `command` |
| 默认 `killMode` | `port` | `processName` | `none` | `none` |

![web 应用的“编辑应用”对话框：type 设为 web 并带一行说明，port 字段已填写](../../../../assets/screenshots/edit-app-type-and-port.png)

1. `type` 下拉框。它的提示行描述了该类型的作用。
2. `port` 字段。对 `web` 应用，“运行中”取决于这个端口是否有响应。

## 如何判定“运行中”

Moonpool 每隔几秒检查一次。无论哪种类型，只要满足下面任意一条，应用就是“运行中”：

- 设置了 `processName`，并且存在使用该名称的进程。Moonpool 自己的 `<exe> mcp` 辅助进程不计在内。
- 设置了 `port`，并且 localhost 上该端口有响应。
- 应用由 Moonpool 启动，既没有 `port` 也没有 `processName`，并且终端的进程仍然存活。

所以 `cli` 应用在它的命令运行期间是“运行中”，没有 `port` 的 `web` 应用也是同样的表现。只有 `url` 的 `static` 条目没有可跟踪的东西，永远不会显示“运行中”。

## web

本地服务器。设置 `port`，让“运行中”反映服务器是否有响应；再设置 `url` 加 `openBrowser`，让它就绪时自动打开。

## desktop

原生应用。把 `processName` 设为可执行文件名，这样即使窗口与启动它的命令脱离，“运行中”依然有效。默认的“停止”会结束所有同名进程。

## static

一个页面。只有 `url` 时，启动和重启会在浏览器中打开它，而停止什么都不做。`http://`、`https://`、`mailto:` 和 `file://` 网址都会被打开，所以本地页面也可以：

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

需要服务器的页面（PHP，或任何要读取本地文件的页面）需要一个启动服务器的 `command`，以及一个用来跟踪它的 `port`。参见[示例](/zh-hans/apps/examples/)。

## cli

一个工具。`command` 在 `cwd` 下的终端标签页中运行，命令退出后该应用就不再是“运行中”。如果想让 shell 保持打开，就把命令写成一个 shell，例如下面这个 `command`：

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

避免在 `command` 中嵌套双引号：它们会被 `cmd /c` 包装弄乱。

![一个 cli 应用的终端标签页：上方是 PowerShell 命令的输出，下方是一个打开的提示符](../../../../assets/screenshots/terminal-cli-output.png)

## 点击会发生什么

点击应用的名称只会打开它的终端标签页。请使用“启动”“停止”和“重启”控件来运行它。参见[应用状态](/zh-hans/support/glossary/#应用状态)。
