---
title: "在 Windows 上讓 Python 指令碼在背景持續執行"
description: "在 Windows 上讓長時間執行的 Python 指令碼或小型網頁應用程式在背景執行，查看它的輸出並乾淨地停止它，分別用 pythonw 與 Moonpool 實現。"
---

從主控台視窗執行的 Python 指令碼，會在你關閉那個視窗時停止。Windows 上常見的做法有：`pythonw.exe`（同一個直譯器，但沒有主控台視窗，輸出無處可去）、用 `Start-Process pythonw -ArgumentList worker.py` 以分離方式啟動，或是對需要在登入時或依排程執行的東西使用排定的工作。這些做法都會讓你在想讓它消失時，不得不去工作管理員裡找那個處理程序。

## Moonpool 的做法

Moonpool 在它自己的終端機分頁中執行命令，所以你不需要自己的主控台視窗，也能保留輸出與一個停止按鈕。對於執行到你停止為止的指令碼，請使用 `cli` 應用程式。`-u` 讓 Python 立即排清輸出，這樣分頁能即時顯示：

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

啟動它，然後按一下應用程式的名稱查看輸出。`cli` 應用程式在命令執行期間算「執行中」，指令碼結束後變成灰色，分頁中會留下 `[處理程序已結束]`。**停止** 會結束指令碼與它啟動的所有東西。直接使用虛擬環境的 `python.exe` 路徑，就不需要啟用步驟。

如果指令碼提供 HTTP 服務（Flask、FastAPI、`python -m http.server`），請把它做成 `web` 應用程式，讓「執行中」跟隨它的連接埠：

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

- 要讓 Moonpool 保持執行。預設情況下關閉它的視窗就會結束它，而在 Windows 上結束會停止它啟動的每個應用程式。開啟 **關閉時縮到系統匣** 可改為只隱藏視窗，請參閱[系統匣、關閉與最小化](/zh-hant/using/tray-and-closing/)。
- Moonpool 不會重新啟動當掉的指令碼，也不會在 Windows 登入時自行啟動它。請參閱[在 Windows 登入時自動啟動指令碼或開發伺服器](/zh-hant/guides/start-app-at-windows-login/)。
- 避免在 `command` 中使用巢狀雙引號：`cmd /c` 包裝會把它們弄亂。

## 另請參閱

- [應用程式類型](/zh-hant/apps/types/#cli)：`cli` 與 `web` 應用程式如何被追蹤。
- [停止與重新啟動](/zh-hant/apps/stop-and-restart/)
- [範例](/zh-hant/apps/examples/)
- [記錄](/zh-hant/data/logs/)：工作階段輸出保存在哪裡。
