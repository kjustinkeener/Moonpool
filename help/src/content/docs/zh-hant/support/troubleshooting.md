---
title: "Moonpool 疑難排解：系統匣、無法啟動的應用程式、更新"
description: "依你看到的現象解決 Moonpool 的常見問題：系統匣圖示不見、應用程式無法啟動或停止、狀態圓點不對、更新失敗與 MCP 錯誤。"
---

先找到對應的現象，再依說明解決。引用的文字是 Moonpool 顯示的內容。要查詢某則確切的訊息，請參閱[錯誤訊息詳解](/zh-hant/support/error-messages/)。

## 我看不到系統匣圖示

- **Windows。** 圖示可能在隱藏圖示的區域。請按一下工作列右側的 **^** 箭頭。把圖示拖曳到工作列上可讓它一直顯示。
- **原生 GNOME 上的 Linux。** 沒有 AppIndicator 擴充功能時，GNOME 不會顯示系統匣圖示。請參閱 [Linux](/zh-hant/platforms/linux/#gnome-上的系統匣)。
- **設定。** **在系統匣顯示** 可能被關閉了。從工作列或「開始」功能表開啟主視窗，在[設定](/zh-hant/using/settings/)中把它重新開啟。

## 安裝程式顯示錯誤

| 訊息 | 該怎麼做 |
| --- | --- |
| `Install failed: <error>` | 冒號後面的文字指出了失敗的步驟，例如 `copy exe: ...`。如果某個檔案正被使用，請結束所有從 `%USERPROFILE%\.moonpool` 執行的 Moonpool，然後重試。 |
| `target folder does not exist` | 你為可攜副本選的資料夾已經不存在。請選一個已存在的資料夾。 |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | 請選一個空資料夾，或是先刪除那個 `.moonpool` 資料夾。 |

## 執行安裝程式時出現「Windows 已保護您的電腦」

這是 Windows SmartScreen，因為 `moonpool.exe` 沒有程式碼簽章。請按一下 **其他資訊**，然後按一下 **仍要執行**。請參閱 [Windows 已保護您的電腦](/zh-hant/support/windows-protected-your-pc/)。

## Moonpool 視窗在 Windows 上空白或始終開不起來

可能是缺少 Microsoft Edge WebView2 執行階段。請參閱[缺少 WebView2 執行階段](/zh-hant/support/webview2-runtime-missing/)。

## 應用程式無法啟動

1. 按一下應用程式的名稱，開啟它的終端機分頁並查看輸出。代理可以用 `moonpool_app_output` 讀取同樣的文字。
2. 檢查 `cwd`。資料夾不存在，或是使用了不帶 `./` 的相對路徑，是常見的原因。請參閱[路徑與環境](/zh-hant/apps/paths-and-environment/)。
3. 檢查 `command`。在 `cwd` 下的終端機裡手動執行一遍。在 Windows 上請避免巢狀雙引號；`cmd /c` 會把它們弄亂。
4. 在設定中開啟 **將除錯資訊寫入檔案**，然後再次啟動。`moonpool.log` 會記錄確切的命令與資料夾。請參閱[記錄](/zh-hant/data/logs/)。

| 訊息 | 意義 |
| --- | --- |
| `already running` | Moonpool 已經為這個應用程式持有一個終端機。請先停止它，或使用重新啟動。 |
| `stopped during launch` | 啟動仍在進行時按下了停止。 |
| `did not reach running in time` | 來自指令碼或代理：應用程式在 25 秒內沒有顯示為執行中。請檢查它的 `port` 或 `processName`，以及它的輸出。 |

## 狀態圓點不對

Moonpool 先依 `port`，再依 `processName`，最後依它自己的終端機是否還活著，來判斷是否執行中。請參閱[如何判定「執行中」](/zh-hant/apps/types/#如何判定執行中)。

- **一直不變成實心。** `web` 應用程式的 `port` 沒有回應，或是 `desktop` 應用程式的 `processName` 不符。在 Linux 上，`processName` 不得超過 15 個字元。
- **啟動後立刻變灰。** `cli` 應用程式在它的命令結束時就不再是執行中。如果想讓它保持開啟，請使用帶 `-NoExit` 的 shell。
- **`static` 應用程式從不顯示執行中。** 對只有 `url` 的項目，這是正常的。
- **你沒有啟動它卻顯示執行中。** 有別的東西在使用那個連接埠或處理程序名稱。Moonpool 把它顯示為執行中，但不是「由 Moonpool 管理」。

## 錯誤：listen EADDRINUSE 或 "Port 5173 is in use"

已經有別的東西在監聽你的伺服器想用的連接埠。請找到並結束它，或是替應用程式設定 `port`，讓「停止」來釋放它。請參閱[解決 EADDRINUSE 與 "Port 5173 is in use"](/zh-hant/support/port-already-in-use/)與[找出並結束佔用連接埠的處理程序](/zh-hant/guides/find-and-kill-process-using-port-windows/)。

## 兩個應用程式使用同一個連接埠

**...** 選單底部會出現一列警告，例如 `連接埠 3000：App A / App B`。請修改其中一個應用程式的 `port`（如果它讀取 `PORT`，也要改它的 `env`）。請參閱[連接埠衝突警告](/zh-hant/using/hub-window/#連接埠衝突警告)。

## 按下停止後應用程式仍在執行

來自指令碼或代理時，錯誤是 `still running after stop`（15 秒後）。

- 應用程式比它的終端機存活得更久。請把 `killMode` 設為 `port` 或 `processName`。請參閱[停止與重新啟動](/zh-hant/apps/stop-and-restart/)。
- Windows 上的 Docker 應用程式：請使用 `killMode` 為 `command`，並配一個 `stopCommand`，例如 `docker compose stop app`。絕不要用 `port`。

## apps.json 有錯誤

側邊欄會顯示一條橫幅，「apps.json 有錯誤，目前顯示的是上次成功載入的清單。」；或是在啟動時顯示「apps.json 有錯誤，因此沒有載入任何應用程式。」在檔案重新載入成功之前，來自 Moonpool 的儲存會被暫停。

典型的錯誤：

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. 在橫幅中選擇 **編輯 apps.json**，修正該項目，儲存，然後 **重新載入**（F5）。
2. 或是回復到最近的良好副本。請參閱[備份與復原](/zh-hant/data/backup-and-recovery/#回復-appsjson)。

完整的規則清單請見[驗證](/zh-hant/apps/apps-json/#驗證)。

如果某項設定無法變更，且訊息以 `Repair settings.json and restart Moonpool before changing settings` 結尾，請修復或刪除設定資料夾中的 `settings.json`，然後重新啟動 Moonpool。刪除它會把所有設定重設為預設值。

## 我的修改沒有生效

- 手動修改需要 **重新載入**（或 F5）。Moonpool 不會監看這個檔案。
- 重新載入不會重新啟動執行中的應用程式。要使用修改後的 `command`、`cwd` 或 `env`，請重新啟動該應用程式。
- 代理可能在編輯另一份 `apps.json`。請讓它呼叫 `moonpool_launcher_paths`，把 hub 的資料夾與它自己的作比較。如果有多個 Moonpool 副本，請確認你在編輯的是哪一個副本。

## 範例應用程式不見了

範例只在沒有 `apps.json` 時才會寫入。要找回它們，請參閱[重設為範例](/zh-hant/data/backup-and-recovery/#重設為範例)，或是從[範例儀表板](/zh-hant/getting-started/example-dashboards/#範例應用程式只在第一次執行時出現)複製這些項目。

## 更新失敗了

橫幅會顯示 `更新失敗：<error>`。請參閱[更新失敗時](/zh-hant/data/updating/#更新失敗時)。

## 網頁連結打不開

`refusing to open non-web url: <url>` 代表 `url` 不是 `http://`、`https://`、`mailto:` 或 `file://`。請修正 `url`。

## MCP 與指令碼錯誤

| 訊息 | 該怎麼做 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | 啟動 Moonpool，或是讓代理呼叫 `moonpool_bootup_launcher`。 |
| `frontend not loaded` | 主視窗還沒有載入完。請稍等片刻再重試。 |
| `stale token: ...` | 自代理讀取後，`apps.json` 已經變動。請重新讀取，然後再寫入。 |
| `rejected invalid manifest: ...` | 新的 `apps.json` 未通過驗證。檔案沒有被更動。 |
| `... A Moonpool process may be hung ...` | 有東西佔著控制通道卻不回應。從系統匣結束 Moonpool，或結束該處理程序，然後重新啟動。 |

更多內容請見 [MCP 設定](/zh-hant/automation/mcp-setup/#附註)與 [MCP 工具](/zh-hant/automation/mcp-tools/)。

## 視窗問題

- **跑到螢幕之外。** Moonpool 會忽略不在任何已連接顯示器上的已儲存位置。如果視窗仍然找不到，請結束 Moonpool，並刪除設定資料夾中的 `window-state.json`。
- **縮放卡在過大或過小。** 在主視窗上按 Ctrl + 滾輪可以變更。請參閱[快速鍵與縮放](/zh-hant/using/keyboard-shortcuts/#縮放)。
- **設定視窗開啟在主視窗後面。** 在設定中把 **永遠顯示在最上層** 關掉，或開啟。它作用於每個 Moonpool 視窗，所以它們會處在同一層。

## 記錄在哪裡？

請參閱[記錄](/zh-hant/data/logs/)。

## 備份、重設或解除安裝

請參閱[備份與復原](/zh-hant/data/backup-and-recovery/)與[解除安裝](/zh-hant/getting-started/install/#解除安裝)。

## 常見問題

**關閉視窗會停止我的應用程式嗎？**
預設情況下，關閉會結束 Moonpool，而在 Windows 上結束會停止它啟動的應用程式。開啟 **關閉時縮到系統匣**，就可以在關閉視窗時讓 Moonpool 繼續執行。請參閱[系統匣、關閉與最小化](/zh-hant/using/tray-and-closing/)。

**可以同時執行兩個 Moonpool 嗎？**
每個資料夾一個。再次啟動同一個副本，會把它的視窗叫回來。已安裝的副本與可攜副本可以並排執行。請參閱[可攜模式](/zh-hant/data/portable-mode/#同時執行多個副本)。

**Moonpool 會「回傳」資料嗎？**
只用於檢查更新：它會在啟動時（如果 **啟動時檢查更新** 開啟）以及你按下 **檢查更新** 時，從 GitHub 取得發行檔案（`update.json`）。每個下載在使用前都會用 Moonpool 的簽署金鑰驗證。

**我的命令由哪個 shell 執行？**
Windows 上是 `cmd /c`，Linux 上是 `$SHELL -c`。

**機密資料該放在哪裡？**
`env` 的值會以純文字儲存在 `apps.json` 中。請優先使用應用程式自己會讀取的檔案，或是你的使用者環境中已經設定好的變數，被啟動的應用程式會繼承它們。

**控制通道受到保護嗎？**
它沒有登入或權杖。任何以你的身分執行的處理程序都可以對它傳送命令。在 Linux 上，通訊端只有你的使用者可以讀取。請參閱[安全特性](/zh-hant/automation/overview/#安全特性)。
