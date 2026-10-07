---
title: "Moonpool 发行说明与近期变化"
description: "了解 Moonpool 近几个版本的变化、运行所需的环境要求，以及在 GitHub 上哪里可以看到完整的发行说明。"
---

每个版本的完整说明都在项目的
[Releases 页面](https://github.com/kjustinkeener/Moonpool/releases)上。这份帮助随 Moonpool
一起发布，所以它描述的始终是你正在运行的版本。Moonpool 会自动更新；参见
[更新](/zh-hans/data/updating/)。

## 0.3.18

- **即使在上次更新之前启动的 Moonpool MCP 服务器仍在运行，更新也不会再因“拒绝访问”而失败。**

## 0.3.17

- **14 种语言的帮助。** 帮助以 Moonpool 的界面语言打开：英语、德语、西班牙语、法语、意大利语、荷兰语、波兰语、巴西葡萄牙语、俄语、土耳其语、日语、韩语，以及简体中文和繁体中文。
- **新的指南和支持页面**，涵盖开发服务器、端口、登录时启动、MCP 智能体、Python 脚本和常见错误消息。
- **`mcpProcessName`。** 用于应用 MCP 服务器进程名的通配符模式，适用于以其他名称运行的服务器。参见 [mcpProcessName](/zh-hans/apps/fields/#mcpprocessname)。
- **不再支持 macOS。** 没有 macOS 版本。Windows 和 Linux 不受影响。

## 0.3.16

- **同时运行多个 Moonpool。** 已安装的 Moonpool 和任意数量的便携副本可以并排运行，
  每个文件夹一个，各自拥有自己的应用、托盘图标和控制通道。
  参见[便携模式](/zh-hans/data/portable-mode/#同时运行多个副本)。
- **主题浏览器。** 共 68 个主题，每个都以自己的配色预览。参见
  [主题、语言与透明度](/zh-hans/using/themes-and-language/)。
- **可直接运行的示例。** 全新的 `apps.json` 中包含的示例应用都可以直接运行。
  示例仪表盘现在位于由程序管理的 `dashboards/examples` 文件夹中，会随 Moonpool 一起更新。
  参见[示例仪表盘](/zh-hans/getting-started/example-dashboards/)。
- **会显示 apps.json 的错误。** 侧边栏上方的提示条会显示错误，重新加载失败时会保留
  上次成功加载的列表。参见
  [apps.json 出错时](/zh-hans/using/hub-window/#当-appsjson-有错误时)。
- **Linux 上的控制通道**，通过 Unix 套接字实现，另外新增了 `list` 动词。参见
  [控制动词](/zh-hans/automation/control-verbs/)。
- “关于”窗口和应用编辑器会实时跟随主题和语言的变化。**安装 Moonpool…** 菜单项在
  非 Windows 系统上会隐藏。

## 0.3.15

- 重启后的应用会保留之前的输出，并加上一条带日期的“restarted”分隔线。参见
  [终端标签页](/zh-hans/using/terminal-tabs/#重启)。
- 每个应用都有自己的 `cli-output` 文件夹，因此清理日志时绝不会动到其他应用的日志。
- 应用编辑器中新增了 `killMode` 和 `stopCommand`。参见
  [停止与重启](/zh-hans/apps/stop-and-restart/)。
- 启动的应用不再继承 Moonpool 自己的 WebView2 配置文件。

## 0.3.14

- 会话日志可以在会话之间保留，并可为每个应用设置大小上限。参见
  [日志](/zh-hans/data/logs/)。
- 设置中为日志文件夹新增了“打开”和“复制”按钮。
- 修复了帮助窗口标题栏的问题。

## 环境要求

- 带有 WebView2 的 Windows 10 或 11（参见 [Windows](/zh-hans/platforms/windows/)）。
- 带有 WebKitGTK 4.1 和 AppIndicator 库的 Linux（参见 [Linux](/zh-hans/platforms/linux/)）。
