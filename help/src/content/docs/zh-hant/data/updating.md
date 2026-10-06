---
title: "更新 Moonpool 並解決更新失敗"
description: "了解 Moonpool 如何檢查、下載並套用更新，更新橫幅的作用，可攜副本與 Linux 如何更新，以及失敗時該怎麼辦。"
---

Moonpool 會自行更新。不需要另外下載安裝程式，也不需要點一堆精靈畫面。

## 更新如何到來

Moonpool 從專案的 GitHub Releases 取得 `update.json`（Linux 上是 `linux-update.json`），比較版本，並且只提供嚴格較新的版本。它會在以下時機檢查：

- 啟動時，除非在[設定](/zh-hant/using/settings/)中關閉了 **啟動時檢查更新**；
- 每當你在「關於」視窗中按下 **檢查更新** 時。這個按鈕會直接安裝較新的版本並重新啟動 Moonpool。否則它會告訴你已使用最新版本，或顯示錯誤。

「關於」視窗會在名稱下方顯示你正在執行的版本：

![「關於」視窗的頂部：標誌、名稱 (1)，以及它下方的版本列](../../../../assets/screenshots/about-header.png)

1. 名稱。它下面一行是版本號碼與建置日期。

每個下載在套用之前，都會用 Moonpool 的 minisign 簽署金鑰驗證，所以被竄改或損毀的下載會被拒絕。Moonpool 絕不會安裝較舊的版本。

## 更新橫幅

啟動時如果發現更新，會在主視窗的空白畫面上顯示為一條橫幅：

```text
Moonpool {version} 已推出（你目前是 {current}）。
```

橫幅只在沒有開啟任何應用程式分頁、且 CLI 面板處於展開狀態時顯示。面板摺疊時，改為篩選方塊旁邊的箭頭閃動。開啟了分頁時則完全沒有提示。要看到橫幅，請關閉所有分頁（並展開面板），或使用「關於」視窗中的 **檢查更新**。

按一下 **下載並安裝**，Moonpool 就會取代自己並重新啟動；也可以用 x 關閉橫幅。

## 可攜副本

可攜副本會以同樣的方式更新它自己 `.moonpool\` 資料夾中的 `moonpool.exe`。每個副本各自檢查與更新。該資料夾必須可寫入，所以位於唯讀 USB 隨身碟或共用資料夾上的副本無法自行更新；請手動把較新的 `moonpool.exe` 複製過去覆蓋。

## Linux

只有 AppImage 會自行更新。它會就地取代 AppImage 檔案，所以請把它放在你有寫入權限的資料夾中。`.deb` 或 RPM 安裝由你的套件管理員更新：在 Moonpool 中安裝會失敗，並提示

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

請參閱 [Linux](/zh-hant/platforms/linux/#更新)。

## 更新失敗時

橫幅會顯示原因，按鈕也會重新可用，讓你重試：

```text
更新失敗：<error>
```

| 錯誤包含 | 可能的原因 | 該怎麼做 |
| --- | --- | --- |
| `download failed` | 沒有網路連線、使用了 Proxy，或 GitHub 限制了要求 | 等一會兒再重試，或手動更新。 |
| `signature verification FAILED - refusing to install` | 下載的檔案已損毀或被更動 | 重試。如果一直失敗，請從 Releases 頁面手動更新。 |
| `rename self aside` 或 `write new exe` | 資料夾是唯讀的，或防毒軟體佔用了該檔案 | 讓資料夾可寫入，或在防毒軟體中放行 `moonpool.exe`，然後重試。 |
| `refusing to install ... not newer than current` | 提供的版本並不比目前的更新 | 無需處理。 |

### 手動更新

結束 Moonpool，從專案的 [Releases 頁面](https://github.com/kjustinkeener/Moonpool/releases)下載 `moonpool.exe`，並複製覆蓋舊的那個：安裝版是 `%USERPROFILE%\.moonpool\moonpool.exe`，可攜副本則是你 `.moonpool\` 資料夾中的那個。你的設定資料夾不會被動到。在 Linux 上，請取代 AppImage，或使用你的套件管理員。

## 說明也會一起更新

這份說明隨 Moonpool 一起發行，所以每次程式更新都會帶上與之相符的說明。離線副本永遠與你所執行的版本一致。

## 另請參閱

- [新增內容](/zh-hant/getting-started/whats-new/)
- [設定視窗](/zh-hant/using/settings/)
