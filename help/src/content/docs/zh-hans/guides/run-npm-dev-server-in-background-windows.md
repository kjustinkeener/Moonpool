---
title: "在 Windows 上让 npm 开发服务器在后台运行，无需终端窗口"
description: "在 Windows 上让 npm run dev、Vite 或其他开发服务器持续运行，不必守着控制台窗口，并直接从托盘启动、停止它和查看输出。"
---

用 `npm run dev` 启动的开发服务器运行在启动它的终端里，所以关闭那个窗口就会让它结束。Windows 上让它继续运行的朴素办法是隐藏进程，例如在 PowerShell 中用 `Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden`，但这样你就没有输出可读，停止它也得去找出正确的 `node.exe`（参见[查找并结束占用端口的进程](/zh-hans/guides/find-and-kill-process-using-port-windows/)）。

## Moonpool 的做法

Moonpool 在主窗口内它自己内嵌的终端标签页中运行命令，所以不需要单独保持打开一个控制台窗口。把主窗口隐藏到托盘，服务器依然在运行。只需添加一次应用：

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

点击应用的 **启动** 控件。一旦 `port` 有了响应，状态圆点就变为实心，并且因为设置了 `openBrowser`，浏览器会打开 `url`。点击应用的名称，可在它自己的标签页中查看输出。**停止** 会结束终端及其启动的所有东西，并释放端口（`killMode` 为 `port` 是 `web` 的默认值）。

## 关闭窗口后继续运行

默认情况下，关闭按钮会退出 Moonpool，而在 Windows 上退出会停止它启动的每个应用。在[设置](/zh-hans/using/settings/)中打开 **关闭时收进托盘**，关闭窗口就只是把它隐藏。通过托盘图标（或 **显示 Moonpool**）可以把它找回来。详情见[托盘、关闭与最小化](/zh-hans/using/tray-and-closing/)。

## 让端口保持可预期

Moonpool 根据 `port` 判断是否在运行。Vite 在它的端口被占用时会改用下一个空闲端口，这会让 Moonpool 盯着错误的端口。请传入 `--strictPort`，让 Vite 直接退出，并把 `port` 设成一致的值：

```text title="command"
npm run dev -- --port 5173 --strictPort
```

如果端口已经被占用，参见[解决 EADDRINUSE 与 "Port 5173 is in use"](/zh-hans/support/port-already-in-use/)。

## 限制

- Moonpool 不会重启崩溃的服务器。它会把应用显示为已停止，标签页中会打印 `[进程已退出]`。
- Moonpool 不会在 Windows 登录时自行启动。参见[在 Windows 登录时自动启动脚本或开发服务器](/zh-hans/guides/start-app-at-windows-login/)。

## 另请参阅

- [应用字段](/zh-hans/apps/fields/)：`port`、`openBrowser`、`killMode`。
- [应用类型](/zh-hans/apps/types/)：如何判定 `web` 是否在运行。
- [停止与重启](/zh-hans/apps/stop-and-restart/)
- [示例](/zh-hans/apps/examples/#web-开发服务器)
