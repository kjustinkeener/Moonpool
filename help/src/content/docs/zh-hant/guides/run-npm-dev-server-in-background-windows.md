---
title: "在 Windows 上讓 npm 開發伺服器在背景執行，不需終端機視窗"
description: "在 Windows 上讓 npm run dev、Vite 或其他開發伺服器持續執行，不必守著主控台視窗，並直接從系統匣啟動、停止它與查看輸出。"
---

用 `npm run dev` 啟動的開發伺服器會在啟動它的終端機裡執行，所以關閉那個視窗就會讓它結束。Windows 上讓它繼續執行的樸素做法是隱藏處理程序，例如在 PowerShell 中用 `Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden`，但這樣你就沒有輸出可讀，要停止它也得去找出正確的 `node.exe`（請參閱[找出並結束佔用連接埠的處理程序](/zh-hant/guides/find-and-kill-process-using-port-windows/)）。

## Moonpool 的做法

Moonpool 在主視窗內它自己內嵌的終端機分頁中執行命令，所以不需要另外保持開啟一個主控台視窗。把主視窗隱藏到系統匣，伺服器依然在執行。只需新增一次應用程式：

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

按一下應用程式的 **啟動** 控制項。一旦 `port` 有了回應，狀態圓點就變為實心，而且因為設定了 `openBrowser`，瀏覽器會開啟 `url`。按一下應用程式的名稱，可在它自己的分頁中查看輸出。**停止** 會結束終端機與它啟動的所有東西，並釋放連接埠（`killMode` 為 `port` 是 `web` 的預設值）。

## 關閉視窗後繼續執行

預設情況下，關閉按鈕會結束 Moonpool，而在 Windows 上結束會停止它啟動的每個應用程式。在[設定](/zh-hant/using/settings/)中開啟 **關閉時縮到系統匣**，關閉視窗就只是把它隱藏。透過系統匣圖示（或 **顯示 Moonpool**）可以把它叫回來。詳情請見[系統匣、關閉與最小化](/zh-hant/using/tray-and-closing/)。

## 讓連接埠保持可預期

Moonpool 依據 `port` 判斷是否在執行。Vite 在它的連接埠被佔用時會改用下一個空閒的連接埠，這會讓 Moonpool 盯著錯誤的連接埠。請傳入 `--strictPort`，讓 Vite 直接結束，並把 `port` 設成一致的值：

```text title="command"
npm run dev -- --port 5173 --strictPort
```

如果連接埠已經被佔用，請參閱[解決 EADDRINUSE 與 "Port 5173 is in use"](/zh-hant/support/port-already-in-use/)。

## 限制

- Moonpool 不會重新啟動當掉的伺服器。它會把應用程式顯示為已停止，分頁中會印出 `[處理程序已結束]`。
- Moonpool 不會在 Windows 登入時自行啟動。請參閱[在 Windows 登入時自動啟動指令碼或開發伺服器](/zh-hant/guides/start-app-at-windows-login/)。

## 另請參閱

- [應用程式欄位](/zh-hant/apps/fields/)：`port`、`openBrowser`、`killMode`。
- [應用程式類型](/zh-hant/apps/types/)：如何判定 `web` 是否在執行。
- [停止與重新啟動](/zh-hant/apps/stop-and-restart/)
- [範例](/zh-hant/apps/examples/#網頁開發伺服器)
