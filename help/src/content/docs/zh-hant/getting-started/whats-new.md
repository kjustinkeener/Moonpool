---
title: "Moonpool 版本資訊與近期變更"
description: "了解 Moonpool 近幾個版本的變更、執行所需的環境需求，以及在 GitHub 上哪裡可以看到完整的版本說明。"
---

每個版本的完整說明都在專案的 [Releases 頁面](https://github.com/kjustinkeener/Moonpool/releases)上。這份說明隨 Moonpool 一起發行，所以它描述的永遠是你正在執行的版本。Moonpool 會自動更新；請參閱[更新](/zh-hant/data/updating/)。

## 0.3.17

- **14 種語言的說明。** 說明會以 Moonpool 的介面語言開啟：英文、德文、西班牙文、法文、義大利文、荷蘭文、波蘭文、巴西葡萄牙文、俄文、土耳其文、日文、韓文，以及簡體中文和繁體中文。
- **新的指南和支援頁面**，涵蓋開發伺服器、連接埠、登入時啟動、MCP 代理程式、Python 指令碼和常見錯誤訊息。
- **`mcpProcessName`。** 用於應用程式 MCP 伺服器處理程序名稱的萬用字元模式，適用於以其他名稱執行的伺服器。請參閱 [mcpProcessName](/zh-hant/apps/fields/#mcpprocessname)。
- **不再支援 macOS。** 沒有 macOS 版本。Windows 和 Linux 不受影響。

## 0.3.16

- **同時執行多個 Moonpool。** 已安裝的 Moonpool 與任意數量的可攜副本可以並排執行，每個資料夾一個，各自擁有自己的應用程式、系統匣圖示與控制通道。請參閱[可攜模式](/zh-hant/data/portable-mode/#同時執行多個副本)。
- **佈景主題瀏覽器。** 共 68 個佈景主題，每個都以自己的配色預覽。請參閱[佈景主題、語言與透明度](/zh-hant/using/themes-and-language/)。
- **可直接執行的範例。** 全新的 `apps.json` 中所含的範例應用程式都能直接執行。範例儀表板現在位於由程式管理的 `dashboards/examples` 資料夾中，會隨 Moonpool 一起更新。請參閱[範例儀表板](/zh-hant/getting-started/example-dashboards/)。
- **會顯示 apps.json 的錯誤。** 側邊欄上方的橫幅會顯示錯誤，重新載入失敗時會保留上次成功載入的清單。請參閱[當 apps.json 有錯誤時](/zh-hant/using/hub-window/#當-appsjson-有錯誤時)。
- **Linux 上的控制通道**，透過 Unix 通訊端實作，另外新增了 `list` 動詞。請參閱[控制動詞](/zh-hant/automation/control-verbs/)。
- 「關於」視窗與應用程式編輯器會即時跟隨佈景主題與語言的變更。**安裝 Moonpool...** 選單項目在非 Windows 系統上會隱藏。

## 0.3.15

- 重新啟動後的應用程式會保留先前的輸出，並加上一條帶日期的「restarted」分隔線。請參閱[終端機分頁](/zh-hant/using/terminal-tabs/#重新啟動)。
- 每個應用程式都有自己的 `cli-output` 資料夾，因此清理記錄時絕不會動到其他應用程式的記錄。
- 應用程式編輯器中新增了 `killMode` 與 `stopCommand`。請參閱[停止與重新啟動](/zh-hant/apps/stop-and-restart/)。
- 被啟動的應用程式不再繼承 Moonpool 自己的 WebView2 設定檔。

## 0.3.14

- 工作階段記錄可以在工作階段之間保留，並可為每個應用程式設定大小上限。請參閱[記錄](/zh-hant/data/logs/)。
- 設定中為記錄資料夾新增了 Reveal 與 Copy 按鈕。
- 修正了說明視窗標題列的問題。

## 需求

- 具備 WebView2 的 Windows 10 或 11（請參閱 [Windows](/zh-hant/platforms/windows/)）。
- 具備 WebKitGTK 4.1 與 AppIndicator 函式庫的 Linux（請參閱 [Linux](/zh-hant/platforms/linux/)）。
