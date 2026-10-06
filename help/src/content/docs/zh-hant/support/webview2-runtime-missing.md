---
title: "缺少 WebView2 執行階段：修復 Windows 上 Moonpool 視窗空白或不出現"
description: "如果 Moonpool 的視窗在 Windows 上始終開不起來或一直空白，可能是缺少 Microsoft Edge WebView2 執行階段。本文說明如何檢查並安裝。"
---

如果 Moonpool 的視窗在 Windows 上始終開不起來，或是開啟後一直空白，很可能是缺少 Microsoft Edge WebView2 執行階段。Moonpool 是一個 Tauri 應用程式，它的視窗都是由 WebView2 繪製的網頁。

WebView2 隨 Windows 11 與較新的 Windows 10 一同提供，所以大多數電腦已經具備。在較舊或被精簡過的 Windows 10，或是被移除過它的電腦上，它可能不存在。Moonpool 自己的原始碼中沒有針對這種情況的專用訊息，因此這裡不引用任何錯誤文字：症狀就是視窗沒有出現或內容是空的。

## 檢查是否已安裝

在 PowerShell 中，從登錄檔查詢該執行階段的版本（第一個路徑是系統層級的安裝，第二個是目前使用者層級的安裝）：

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

出現 `120.0.2210.91` 這樣的版本號碼，代表已經安裝。兩條命令都回報錯誤，代表尚未安裝。

## 安裝

從 Microsoft 的 WebView2 頁面下載 **Evergreen** WebView2 執行階段（搜尋「WebView2 Runtime download」），執行安裝程式，然後重新啟動 Moonpool。Evergreen 執行階段會自動更新。

## 已安裝但視窗仍然空白

- 從系統匣結束所有的 Moonpool（或在工作管理員中結束 `moonpool.exe`），然後重新啟動。
- 如果你能開啟[設定](/zh-hant/using/settings/)，就開啟 **將除錯資訊寫入檔案**，然後查看 `moonpool.log`。請參閱[記錄](/zh-hant/data/logs/)。
- 如果視窗能開啟但跑到了螢幕之外，請參閱[視窗問題](/zh-hant/support/troubleshooting/#視窗問題)。

## 另請參閱

- [Windows](/zh-hant/platforms/windows/#執行之前)
- [安裝](/zh-hant/getting-started/install/)
- [疑難排解](/zh-hant/support/troubleshooting/)
