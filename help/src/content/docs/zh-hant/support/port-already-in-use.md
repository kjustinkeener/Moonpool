---
title: "解決 Error: listen EADDRINUSE: address already in use :::3000 與 Vite 的 Port 5173 is in use"
description: "解決 Node 的 EADDRINUSE 與 Vite 的 Port 5173 is in use：找出佔用連接埠的處理程序，釋放它，並用 Moonpool 的 port 與 killMode 欄位避免再次發生。"
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

這個 Node.js 錯誤代表已經有另一個處理程序在監聽 3000 連接埠（`:::` 是「所有位址」的 IPv6 寫法；你也可能看到 `127.0.0.1:3000`）。常見的原因是你先前啟動過同一個伺服器，卻一直沒有停止它。

Vite 對同樣的情況處理方式不同。預設情況下它會印出：

```text
Port 5173 is in use, trying another one...
```

（連接埠 5173 已被佔用，正在嘗試另一個連接埠……）然後在下一個空閒的連接埠上啟動，所以伺服器是起來了，但不在你預期的位置。如果使用 `--strictPort`（或 `server.strictPort: true`），Vite 會改為結束，並顯示 `Error: Port 5173 is already in use`。

## 自己解決

1. 找到佔用該連接埠的處理程序並結束它。在 Windows 上：

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   附 PowerShell 版本的逐步說明請見[找出並結束佔用連接埠的處理程序](/zh-hant/guides/find-and-kill-process-using-port-windows/)。
2. 或是讓你的伺服器改用另一個連接埠，例如對許多 Node 伺服器用 `PORT=3001`，對 Vite 用 `--port 5174`。

## Moonpool 如何幫忙

如果你透過 Moonpool 執行伺服器，請在它的項目上設定 `port`。這樣 Moonpool 會：

- 只要有東西在該連接埠上回應，就把應用程式顯示為執行中，所以佔著連接埠的殘留伺服器會顯示為執行中，但不是「由 Moonpool 管理」；
- 在 **停止** 與 **重新啟動** 時，當 `killMode` 為 `port`（這是 `web` 應用程式的預設值）時，結束仍在監聽 `port` 的任何處理程序，這樣下一次啟動時連接埠是空閒的；
- 標示出兩個設定了相同 `port` 的應用程式。

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool 在啟動之前不會檢查連接埠。如果連接埠仍被佔用，命令會在應用程式的終端機分頁中印出上面的錯誤。請按 **停止**（它會釋放連接埠），然後再 **啟動**。

對於 Vite，請傳入 `--strictPort`，並讓 `port` 等於你所要求的連接埠：

```text title="command"
npm run dev -- --port 5173 --strictPort
```

如果不這樣做，Vite 可能會換到 5174，而 Moonpool 仍在盯著 5173，狀態圓點就永遠不會變成實心。

`killMode` 為 `port` 會結束該連接埠上的任何處理程序，所以請只對沒有其他東西需要的連接埠使用它。對 Windows 上的 Docker 應用程式，絕不要使用它。請參閱[停止與重新啟動](/zh-hant/apps/stop-and-restart/#windows-上的-docker-應用程式)。

## 另請參閱

- [應用程式欄位](/zh-hant/apps/fields/)：`port`、`killMode`。
- [停止與重新啟動](/zh-hant/apps/stop-and-restart/)
- [疑難排解](/zh-hant/support/troubleshooting/#兩個應用程式使用同一個連接埠)
- [在 Windows 上讓 npm 開發伺服器在背景執行](/zh-hant/guides/run-npm-dev-server-in-background-windows/)
