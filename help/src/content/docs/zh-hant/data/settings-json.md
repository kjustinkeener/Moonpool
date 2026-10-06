---
title: "了解 settings.json 並修復損毀的檔案"
description: "了解 Moonpool 的 settings.json 的結構、Moonpool 會替你寫入哪些索引鍵，以及檔案損毀時如何讀取並修復它。"
---

應用程式範圍的設定儲存在設定資料夾中的 `settings.json` 裡（請參閱[設定位於何處](/zh-hant/apps/apps-json/#設定位於何處)）。請在[設定視窗](/zh-hant/using/settings/)中變更它們，那裡列出了每項設定及其 JSON 索引鍵與預設值。記錄及其保留規則請見[記錄](/zh-hant/data/logs/)頁面。

## 結構

一個 JSON 物件。你省略的索引鍵會取預設值：

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| 索引鍵 | 預設值 |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0`（0 到 90） |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10`（最小為 1） |

## 替你寫入的索引鍵

Moonpool 還會把介面縮放（`uiScale`，0.5 到 3.0）與解析出的語言（`localeResolved`）存放在這個檔案中。你不需要設定它們。佈景主題不在這裡：它儲存在 webview 的儲存空間中（請參閱[佈景主題、語言與透明度](/zh-hant/using/themes-and-language/)）。

## 讀取與修復

Moonpool 在啟動時讀取該檔案。執行期間所做的編輯不會被讀取；請先結束。

如果檔案格式有誤，Moonpool 會用預設值啟動，並拒絕變更設定。錯誤訊息會以 `Repair settings.json and restart Moonpool before changing settings`（請先修復 settings.json 並重新啟動 Moonpool，再變更設定）結尾。請修復檔案，或刪除它以重設所有設定，然後重新啟動 Moonpool。
