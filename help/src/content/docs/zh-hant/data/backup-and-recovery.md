---
title: "備份 Moonpool、回復 apps.json 並復原設定"
description: "了解該備份什麼，如何回復出錯的 apps.json、重設為範例應用程式、把已安裝的設定搬到可攜副本，以及解除安裝會移除什麼。"
---

Moonpool 保存的所有內容都在兩個地方：設定資料夾與儀表板資料夾。各模式對應的路徑請見[設定位於何處](/zh-hant/apps/apps-json/#設定位於何處)。

## 設定資料夾

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

儀表板資料夾是 `{MP_HOME}\dashboards`：安裝版為 `%USERPROFILE%\.moonpool\dashboards`，可攜版為 `<your .moonpool folder>\dashboards`，Linux 上則是設定資料夾內的 `dashboards/`。請備份其中你自己的任何內容。它的 `examples` 資料夾屬於 Moonpool，更新時會被重寫。

佈景主題儲存在視窗的瀏覽器儲存空間中，而不是你可以複製的檔案裡。它不會隨備份一起搬動；還原後請重新選擇一次。

## 備份

1. 結束 Moonpool，避免有檔案只寫了一半。
2. 從設定資料夾複製 `apps.json`、`settings.json` 與 `icons\`，並從 `dashboards\` 複製你自己的檔案。

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

要還原，先結束 Moonpool，把檔案複製回去，然後啟動它。

## 回復 apps.json

每次成功的儲存、代理寫入與還原，以及每次發現內容有變動的重新載入，都會把驗證通過的 `apps.json` 複製到 `apps.json.history\` 中，保留最新的 10 份。每個檔案以產生時間命名，例如 `1767225600000.json`。沒有 `apps.json.bak`。

- **手動。** 把某個快照複製覆蓋 `apps.json`，然後選擇 **重新載入**。

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **透過指令碼。** `moonpool.exe restore-config` 會列出快照；`moonpool.exe restore-config 1` 會還原最新的那個。請參閱[命令列](/zh-hant/automation/command-line/)。
- **透過代理。** `moonpool_restore_config`。請參閱 [MCP 工具](/zh-hant/automation/mcp-tools/#設定)。

不會自動還原任何東西。

## 檔案損毀

- **apps.json。** Moonpool 絕不會覆寫損毀的檔案。請參閱[如果檔案有錯誤](/zh-hant/apps/apps-json/#如果檔案有錯誤)。
- **settings.json。** 修復它，或刪除它以重設所有設定，然後重新啟動 Moonpool。請參閱 [settings.json](/zh-hant/data/settings-json/#讀取與修復)。

## 重設為範例

Moonpool 只在沒有 `apps.json` 時才寫入範例應用程式。要重新開始，請結束 Moonpool（或讓它繼續執行），把 `apps.json` 重新命名或刪除，然後啟動 Moonpool 或選擇 **重新載入**。會寫入一份帶有範例的全新 `apps.json`。

## 從安裝版搬到可攜版

新的可攜副本一開始帶的是範例應用程式。要把你自己的應用程式帶過去，請參閱[可攜模式](/zh-hant/data/portable-mode/#從安裝程式選擇可攜模式)。如果需要，用同樣的方式複製 `icons\` 與 `settings.json`。

## 解除安裝

解除安裝已安裝的 Moonpool 會刪除整個 `%USERPROFILE%\.moonpool` 資料夾，包括設定資料夾與儀表板。請先備份。請參閱[解除安裝](/zh-hant/getting-started/install/#解除安裝)。可攜副本只需刪除它的 `.moonpool\` 資料夾即可移除。
