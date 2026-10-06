---
title: "在 Windows 上使用 Moonpool"
description: "Windows 是 Moonpool 的主要平台：如何安装，以及一张 Windows 与 Linux 差异对照表，让你心中有数。"
---

Windows 是 Moonpool 的主要平台。请按[安装](/zh-hans/getting-started/install/)中的说明进行安装。

## 运行之前

- **SmartScreen。** `moonpool.exe` 没有代码签名，所以第一次运行时 Windows 可能会显示“Windows 已保护你的电脑”。选择 **更多信息**，然后选择 **仍要运行**。
- **杀毒软件。** 一个会自我复制、更新时又替换自身的全新未签名 exe，可能会触发杀毒软件。如果你的杀毒软件拦截或隔离了 `moonpool.exe`，请为 `.moonpool` 文件夹放行。
- **WebView2。** Moonpool 的窗口使用 Microsoft Edge WebView2，它随 Windows 11 和较新的 Windows 10 一同提供。如果窗口一直空白或始终打不开，请从 Microsoft 安装 Evergreen WebView2 运行时。

更多内容：[Windows 已保护你的电脑](/zh-hans/support/windows-protected-your-pc/)、[缺少 WebView2 运行时](/zh-hans/support/webview2-runtime-missing/)，以及[在 Windows 登录时自动启动脚本或开发服务器](/zh-hans/guides/start-app-at-windows-login/)。

## 托盘

在 Windows 11 上，新的托盘图标常常会进入隐藏图标区域。点击任务栏右侧的 **^** 箭头即可找到它，并把它拖到任务栏上以保持可见。

## 命令

- 命令通过 `cmd /c` 运行。请避免在 `command` 中嵌套双引号，`cmd /c` 会把它们弄乱。如果希望脚本结束后保留一个 shell，请使用 `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>`，且脚本部分不要加引号。
- `processName` 匹配时带不带 `.exe` 都可以，并且不区分大小写。
- 停止会结束 Moonpool 启动的整个进程树，包括已经与它脱离的进程。
- Docker Desktop 应用需要把 `killMode` 设为 `none` 或 `command`，绝不能用 `port`。参见 [Windows 上的 Docker 应用](/zh-hans/apps/stop-and-restart/#windows-上的-docker-应用)。

## 各平台的差异

| | Windows | Linux |
| --- | --- | --- |
| 安装 | 自安装的 `moonpool.exe`，或便携版 | AppImage、`.deb` 或 RPM；没有安装卡片 |
| 自动更新 | 支持，安装版和便携版均可 | 仅 AppImage |
| 配置文件夹 | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| 命令使用的 shell | `cmd /c` | `$SHELL -c` |
| `processName` | 长度不限，`.exe` 可省略，不区分大小写 | 不超过 15 个字符，区分大小写 |
| 按 `processName` 停止 | 结束该进程及其子进程 | 只结束名称完全一致的进程 |
| 控制通道 | 命名管道 | Unix 套接字 |
| 窗口截图（测试用） | 支持 | 不支持 |
| 从程序文件提取图标 | 支持 | 不支持 |
| 托盘 | 开箱即用 | 需要 AppIndicator；原生 GNOME 需要扩展 |

Linux 的细节见 [Linux](/zh-hans/platforms/linux/) 页面。
