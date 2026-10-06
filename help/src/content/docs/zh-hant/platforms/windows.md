---
title: "在 Windows 上使用 Moonpool"
description: "Windows 是 Moonpool 的主要平台：如何安裝，以及一張 Windows 與 Linux 的差異對照表，讓你心裡有個底。"
---

Windows 是 Moonpool 的主要平台。請依照[安裝](/zh-hant/getting-started/install/)中的說明進行安裝。

## 執行之前

- **SmartScreen。** `moonpool.exe` 沒有程式碼簽章，所以第一次執行時 Windows 可能會顯示「Windows 已保護您的電腦」。請選擇 **其他資訊**，然後選擇 **仍要執行**。
- **防毒軟體。** 一個會自我複製、更新時又取代自身的全新未簽署執行檔，可能會觸發防毒軟體。如果你的防毒軟體封鎖或隔離了 `moonpool.exe`，請針對 `.moonpool` 資料夾加以放行。
- **WebView2。** Moonpool 的視窗使用 Microsoft Edge WebView2，它隨 Windows 11 與較新的 Windows 10 一同提供。如果視窗一直空白或始終開不起來，請從 Microsoft 安裝 Evergreen WebView2 執行階段。

更多資訊：[Windows 已保護您的電腦](/zh-hant/support/windows-protected-your-pc/)、[缺少 WebView2 執行階段](/zh-hant/support/webview2-runtime-missing/)，以及[在 Windows 登入時自動啟動指令碼或開發伺服器](/zh-hant/guides/start-app-at-windows-login/)。

## 系統匣

在 Windows 11 上，新的系統匣圖示常會被放進隱藏圖示的區域。請按一下工作列右側的 **^** 箭頭找到它，再把它拖曳到工作列上，就能一直顯示。

## 命令

- 命令會透過 `cmd /c` 執行。請避免在 `command` 中使用巢狀雙引號，`cmd /c` 會把它們弄亂。如果希望指令碼結束後保留一個 shell，請使用 `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>`，且指令碼的部分不要加引號。
- `processName` 比對時有沒有 `.exe` 都可以，而且不分大小寫。
- 停止會結束 Moonpool 啟動的整個處理程序樹，包括已經與它分離的處理程序。
- Docker Desktop 應用程式需要把 `killMode` 設為 `none` 或 `command`，絕不能用 `port`。請參閱 [Windows 上的 Docker 應用程式](/zh-hant/apps/stop-and-restart/#windows-上的-docker-應用程式)。

## 各平台的差異

| | Windows | Linux |
| --- | --- | --- |
| 安裝 | 可自行安裝的 `moonpool.exe`，或可攜版 | AppImage、`.deb` 或 RPM；沒有安裝卡片 |
| 自動更新 | 支援，安裝版與可攜版皆可 | 僅 AppImage |
| 設定資料夾 | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| 命令使用的 shell | `cmd /c` | `$SHELL -c` |
| `processName` | 長度不限，`.exe` 可省略，不分大小寫 | 不超過 15 個字元，區分大小寫 |
| 依 `processName` 停止 | 結束該處理程序及其子處理程序 | 只結束名稱完全相符的處理程序 |
| 控制通道 | 具名管道 | Unix 通訊端 |
| 視窗螢幕擷取（測試用） | 支援 | 不支援 |
| 從程式檔擷取圖示 | 支援 | 不支援 |
| 系統匣 | 開箱即用 | 需要 AppIndicator；原生 GNOME 需要擴充功能 |

Linux 的詳細資訊請見 [Linux](/zh-hant/platforms/linux/) 頁面。
