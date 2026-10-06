---
title: "在 Windows 上让 Python 脚本在后台持续运行"
description: "在 Windows 上让长期运行的 Python 脚本或小型 Web 应用在后台运行，查看它的输出并干净地停止它，分别用 pythonw 和 Moonpool 实现。"
---

从控制台窗口运行的 Python 脚本，会在你关闭那个窗口时停止。Windows 上常见的办法有：`pythonw.exe`（同一个解释器，但没有控制台窗口，输出无处可去）、用 `Start-Process pythonw -ArgumentList worker.py` 以分离方式启动，或者对需要在登录时或按定时运行的东西使用计划任务。这些办法都会让你在想让它消失时，不得不去任务管理器里找那个进程。

## Moonpool 的做法

Moonpool 在它自己的终端标签页中运行命令，所以你不需要自己的控制台窗口，也能保留输出和一个停止按钮。对于运行到你停止为止的脚本，使用 `cli` 应用。`-u` 让 Python 立即刷新输出，这样标签页能实时显示：

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

启动它，然后点击应用的名称查看输出。`cli` 应用在命令运行期间算“运行中”，脚本退出后变灰，标签页中会留下 `[进程已退出]`。**停止** 会结束脚本及其启动的所有东西。直接用虚拟环境的 `python.exe` 路径，就不需要激活步骤。

如果脚本提供 HTTP 服务（Flask、FastAPI、`python -m http.server`），请把它做成 `web` 应用，让“运行中”跟随它的端口：

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## 限制

- 要让 Moonpool 保持运行。默认情况下关闭它的窗口就会退出它，而在 Windows 上退出会停止它启动的每个应用。打开 **关闭时收进托盘** 可改为只隐藏窗口，参见[托盘、关闭与最小化](/zh-hans/using/tray-and-closing/)。
- Moonpool 不会重启崩溃的脚本，也不会在 Windows 登录时自行启动它。参见[在 Windows 登录时自动启动脚本或开发服务器](/zh-hans/guides/start-app-at-windows-login/)。
- 避免在 `command` 中嵌套双引号：`cmd /c` 包装会把它们弄乱。

## 另请参阅

- [应用类型](/zh-hans/apps/types/#cli)：`cli` 和 `web` 应用如何被跟踪。
- [停止与重启](/zh-hans/apps/stop-and-restart/)
- [示例](/zh-hans/apps/examples/)
- [日志](/zh-hans/data/logs/)：会话输出保存在哪里。
