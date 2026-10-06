---
title: "在 Windows 上找出並結束佔用連接埠的處理程序（3000、5173、8080）"
description: "用 netstat 或 PowerShell 找出 Windows 上佔用 3000 或 5173 連接埠的處理程序，用 taskkill 結束它，並在你停止應用程式時讓 Moonpool 釋放連接埠。"
---

當開發伺服器因連接埠已被佔用而啟動失敗時，代表有別的東西正在監聽那個連接埠。在命令提示字元中，列出監聽者及其所屬的處理程序 ID，然後結束它：

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

`LISTENING` 那一列的最後一欄就是 PID（`findstr :3000` 也會符合 `:30001`，所以請看本機位址）。`tasklist /FI "PID eq 12345"` 會顯示它是哪個程式。在 PowerShell 中，同樣的查詢是：

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

在 `taskkill` 後面加上 `/T` 可以連同該處理程序的子處理程序一起結束。屬於其他使用者或系統的處理程序，可能需要提高權限（系統管理員）的視窗。

## Moonpool 的做法

對透過 Moonpool 執行的應用程式，你不用去查 PID。替應用程式設定一個 `port`，「停止」就會釋放它。對 `web` 應用程式，這就是預設的 `killMode`，這裡把它寫了出來：

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

「停止」會先結束 Moonpool 啟動的終端機，然後強制結束仍在監聽 `port` 的任何處理程序。在 Windows 上，這與上面是同樣的查詢（`Get-NetTCPConnection -LocalPort <port> -State Listen`），再對每個擁有者執行 `taskkill /PID <pid> /T /F`。

- 如果有你沒啟動的東西佔著連接埠，Moonpool 會把該應用程式顯示為執行中，但不是「由 Moonpool 管理」。對它按 **停止**：`port` 這一步仍會執行。
- Moonpool 拒絕依連接埠結束一份固定清單中的 Windows 共用處理程序，例如 Docker Desktop 的後端、`svchost` 與 WSL 主機。對 Docker 應用程式請使用 `killMode` 為 `command` 或 `none`，絕不要用 `port`。請參閱 [Windows 上的 Docker 應用程式](/zh-hant/apps/stop-and-restart/#windows-上的-docker-應用程式)。
- 這只對列在 `apps.json` 中的應用程式的連接埠有效。對其他任何連接埠，請使用開頭的那些命令。
- `port` 模式會結束任何正在監聽的處理程序，包括你手動啟動的副本，所以請只對電腦上沒有其他東西需要的連接埠使用它。

## 另請參閱

- [解決 EADDRINUSE 與 "Port 5173 is in use"](/zh-hant/support/port-already-in-use/)
- [停止與重新啟動](/zh-hant/apps/stop-and-restart/)
- [應用程式欄位](/zh-hant/apps/fields/)：`port` 與 `killMode`。
- [兩個應用程式使用同一個連接埠](/zh-hant/support/troubleshooting/#兩個應用程式使用同一個連接埠)
