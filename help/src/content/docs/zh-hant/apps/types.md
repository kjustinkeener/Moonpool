---
title: "選擇應用程式類型：web、desktop、static 或 cli"
description: "了解 web、desktop、static 與 cli 應用程式在 Moonpool 中如何啟動，各自如何偵測「執行中」，以及「停止」按鈕預設會做什麼。"
---

`type` 決定哪些欄位有意義，以及「停止」預設做什麼。

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| 需要 | `command` | `command` | `url` | `command` |
| 通常還有 | `port`、`url` | `processName` | `command` 與 `port`（如果它自己提供服務） | `cwd` |
| 啟動 | 在終端機分頁中執行 `command` | 在終端機分頁中執行 `command` | 沒有 `command`：在瀏覽器中開啟 `url`。有 `command`：在終端機分頁中執行它 | 在終端機分頁中執行 `command` |
| 預設 `killMode` | `port` | `processName` | `none` | `none` |

![web 應用程式的「編輯應用程式」對話方塊：type 設為 web 並帶一行說明，port 欄位已填寫](../../../../assets/screenshots/edit-app-type-and-port.png)

1. `type` 下拉選單。它的提示行描述了該類型的作用。
2. `port` 欄位。對 `web` 應用程式，「執行中」取決於這個連接埠是否有回應。

## 如何判定「執行中」

Moonpool 每隔幾秒檢查一次。無論哪種類型，只要符合下列任一條，應用程式就是「執行中」：

- 設定了 `processName`，且存在使用該名稱的處理程序。Moonpool 自己的 `<exe> mcp` 輔助處理程序不計在內。
- 設定了 `port`，且 localhost 上該連接埠有回應。
- 應用程式由 Moonpool 啟動，既沒有 `port` 也沒有 `processName`，且終端機的處理程序仍然存活。

所以 `cli` 應用程式在它的命令執行期間是「執行中」，沒有 `port` 的 `web` 應用程式也是同樣的表現。只有 `url` 的 `static` 項目沒有可追蹤的東西，永遠不會顯示「執行中」。

## web

本機伺服器。設定 `port`，讓「執行中」反映伺服器是否有回應；再設定 `url` 加 `openBrowser`，讓它就緒時自動開啟。

## desktop

原生應用程式。把 `processName` 設為執行檔名稱，這樣即使視窗與啟動它的命令分離，「執行中」依然有效。預設的「停止」會結束所有同名的處理程序。

## static

一個頁面。只有 `url` 時，啟動與重新啟動會在瀏覽器中開啟它，而停止什麼都不做。`http://`、`https://`、`mailto:` 與 `file://` 網址都會被開啟，所以本機頁面也可以：

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

需要伺服器的頁面（PHP，或任何要讀取本機檔案的頁面）需要一個啟動伺服器的 `command`，以及一個用來追蹤它的 `port`。請參閱[範例](/zh-hant/apps/examples/)。

## cli

一個工具。`command` 會在 `cwd` 下的終端機分頁中執行，命令結束後該應用程式就不再是「執行中」。如果想讓 shell 保持開啟，就把命令寫成一個 shell，例如下面這個 `command`：

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

請避免在 `command` 中使用巢狀雙引號：它們會被 `cmd /c` 包裝弄亂。

![一個 cli 應用程式的終端機分頁：上方是 PowerShell 命令的輸出，下方是一個開啟的提示字元](../../../../assets/screenshots/terminal-cli-output.png)

## 按一下會發生什麼

按一下應用程式的名稱只會開啟它的終端機分頁。請使用「啟動」、「停止」與「重新啟動」控制項來執行它。請參閱[應用程式狀態](/zh-hant/support/glossary/#應用程式狀態)。
