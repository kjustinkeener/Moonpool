---
title: "让 Moonpool 留在托盘：关闭、最小化和退出的行为"
description: "控制托盘图标、关闭按钮、最小化和“退出”各自的作用，让窗口总在最前，并避免同时隐藏托盘和任务栏。"
---

## 托盘图标

| 操作 | 结果 |
| --- | --- |
| 左键单击 | 显示主窗口（如果已最小化或隐藏，则将其恢复）。 |
| 右键单击 | 菜单中只有 **显示 Moonpool** 和 **退出**（使用你的语言）。 |

同时运行多个 Moonpool 副本时，每个副本都有自己的托盘图标。悬停提示会说明它是哪个副本。参见[便携模式](/zh-hans/data/portable-mode/#同时运行多个副本)。

## 退出

**退出** 会退出 Moonpool，并且在 Windows 上会停止 Moonpool 启动的每个应用，包括它们的子进程。在 Moonpool 发现它们之前就已在运行的应用（显示为运行中但没有“由 Moonpool 管理”）不会受影响。在 Linux 和 macOS 上，退出并不能可靠地停止已启动的应用。

## 关闭与最小化

关闭按钮默认会退出 Moonpool（`closeToTray` 为 `false`）。在设置中打开 **关闭时收进托盘**，关闭窗口就会改为把它隐藏到托盘。Moonpool 继续运行，通过托盘图标或 **显示 Moonpool** 可以把它找回来。

**最小化时收进托盘**（`minimizeToTray`，默认开启）会在窗口最小化时把它隐藏到托盘，并从任务栏消失。关闭它则像平常一样最小化到任务栏。

![设置：关闭时收进托盘和最小化时收进托盘 (1)，以及背景透明度滑块 (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **关闭时收进托盘** 和 **最小化时收进托盘**。
2. **背景透明度**。参见[主题、语言与透明度](/zh-hans/using/themes-and-language/#透明度)。

## 托盘与任务栏的防锁定

**在托盘中显示** 和 **在任务栏中显示** 控制托盘图标和任务栏按钮是否可见。至少要保留一个开启，否则被隐藏的窗口就没有办法再打开了。只剩一个开启时，它的复选框会被禁用，直到你把另一个重新打开。

## 总在最前

设置中的 **总在最前** 会让 Moonpool 的每个窗口（主窗口、设置、关于、应用编辑器、主题浏览器、安装程序和帮助）都显示在其他窗口之上。默认关闭。

## 另请参阅

- [在 Windows 上让 npm 开发服务器在后台运行](/zh-hans/guides/run-npm-dev-server-in-background-windows/)
- [在 Windows 登录时自动启动脚本或开发服务器](/zh-hans/guides/start-app-at-windows-login/)
- [设置窗口](/zh-hans/using/settings/)
- [主窗口](/zh-hans/using/hub-window/)
