---
title: "在 Windows 上查找并结束占用端口的进程（3000、5173、8080）"
description: "用 netstat 或 PowerShell 找出 Windows 上占用 3000 或 5173 端口的进程，用 taskkill 结束它，并在你停止应用时让 Moonpool 释放端口。"
---

当开发服务器因端口已被占用而启动失败时，说明有别的东西正在监听那个端口。在命令提示符中，列出监听者及其所属进程 ID，然后结束它：

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

`LISTENING` 行的最后一列就是 PID（`findstr :3000` 也会匹配 `:30001`，所以请看本地地址）。`tasklist /FI "PID eq 12345"` 会显示它是哪个程序。在 PowerShell 中，同样的查找是：

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

给 `taskkill` 加上 `/T` 可以连同该进程的子进程一起结束。属于其他用户或系统的进程可能需要提升权限（管理员）的窗口。

## Moonpool 的做法

对通过 Moonpool 运行的应用，你不用去查 PID。给应用设置一个 `port`，“停止”就会释放它。对 `web` 应用，这就是默认的 `killMode`，这里把它写了出来：

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

“停止”先结束 Moonpool 启动的终端，然后强制结束仍在监听 `port` 的任何进程。在 Windows 上，这与上面是同样的查找（`Get-NetTCPConnection -LocalPort <port> -State Listen`），再对每个所有者执行 `taskkill /PID <pid> /T /F`。

- 如果有你没启动的东西占着端口，Moonpool 会把该应用显示为运行中，但不是“由 Moonpool 管理”。对它按 **停止**：`port` 这一步仍会执行。
- Moonpool 拒绝按端口结束一份固定清单里的 Windows 共享进程，例如 Docker Desktop 的后端、`svchost` 和 WSL 宿主。对 Docker 应用请使用 `killMode` 为 `command` 或 `none`，绝不要用 `port`。参见 [Windows 上的 Docker 应用](/zh-hans/apps/stop-and-restart/#windows-上的-docker-应用)。
- 这只对列在 `apps.json` 中的应用的端口有效。对其他任何端口，请使用开头的那些命令。
- `port` 模式会结束任何正在监听的进程，包括你手动启动的副本，所以只对机器上没有其他东西需要的端口使用它。

## 另请参阅

- [解决 EADDRINUSE 与 "Port 5173 is in use"](/zh-hans/support/port-already-in-use/)
- [停止与重启](/zh-hans/apps/stop-and-restart/)
- [应用字段](/zh-hans/apps/fields/)：`port` 和 `killMode`。
- [两个应用使用同一个端口](/zh-hans/support/troubleshooting/#两个应用使用同一个端口)
