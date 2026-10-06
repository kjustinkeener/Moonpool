---
title: "Moonpool MCP 工具參考：參數與結果"
description: "Moonpool MCP 伺服器向代理提供的每個工具，包括它的參數、傳回內容，以及你可能遇到的錯誤情況。"
---

所有工具都傳回文字，只有 `moonpool_screenshot` 傳回 PNG 影像。失敗會以標記為錯誤的工具結果傳回，原因以文字形式給出。設定方法請見 [MCP 設定](/zh-hant/automation/mcp-setup/)。

接受 `app_id` 的工具，需要的是 `apps.json` 中該應用程式的 `id`。它只能使用字母、數字、`.`、`_` 與 `-`，而且不能以 `-` 開頭，否則呼叫會以 "invalid app_id" 失敗。

大多數作用於 hub 的工具，在它沒有執行時都會以下面這則訊息失敗。`moonpool_bootup_launcher`、`moonpool_shutdown_launcher`、`moonpool_raise_launcher` 與 `moonpool_launcher_paths` 會自行處理這種情況（見它們各自的列）。對可攜副本，這則訊息會寫出該副本的名稱，例如 `Moonpool (<folder>)`。

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

等待結果的呼叫在 45 秒後逾時。

## 啟動器與應用程式

`moonpool_list_apps` 的結果範例：

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| 工具 | 參數 | 行為 |
| --- | --- | --- |
| `moonpool_list_apps` | 無 | 每個應用程式一行：`id  [running]` 或 `[stopped]`，適用時帶 `(managed by Moonpool)`，看過 MCP 輔助程式時帶 `[mcp: running]` 或 `[mcp: stopped]`，最後是名稱。它透過控制通道（`list` 動詞）向執行中的 hub 查詢，所以是即時的。如果 Moonpool 沒有執行，它會以 "Moonpool is not running" 失敗，而不是顯示過期的清單。剛啟動 Moonpool 後、第一次狀態檢查之前，應用程式顯示 `[status pending]`。`apps.json` 有錯誤期間，結果會以 `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` 開頭。如果檔案在 Moonpool 啟動時就已損毀，它會說明沒有載入任何應用程式，並同時建議使用 `moonpool_restore_config`。 |
| `moonpool_bootup_launcher` | 無 | 啟動 Moonpool 本身，並最多等待 30 秒讓它的控制通道回應。傳回 "Moonpool started" 或 "Moonpool is already running"。如果新處理程序立即結束（它把工作交給了一個仍在關閉的 Moonpool），則再啟動一次。如果有東西佔著通道卻不回應，它會回報某個 Moonpool 處理程序可能已經停止回應。 |
| `moonpool_shutdown_launcher` | 無 | 與系統匣選單中的「結束」相同。最多等待 30 秒讓控制通道消失。傳回 "Moonpool shut down" 或 "Moonpool is not running"。 |
| `moonpool_raise_launcher` | 無 | 把 Moonpool 視窗帶到最前面。傳回 "window shown"。如果 Moonpool 沒有執行，它會啟動它並傳回 "Moonpool was not running; started it"。 |
| `moonpool_start_app` | `app_id`（必填） | 啟動應用程式並開啟它的終端機分頁。應用程式進入執行狀態後傳回 "launched"，否則傳回未能啟動的原因（`unknown app id: <id>`，25 秒後為 `did not reach running in time`）。對只有 `url` 的 `static` 項目，它會開啟該頁面，同樣傳回 "launched"。 |
| `moonpool_stop_app` | `app_id`（必填） | 停止應用程式。傳回 "stopped"，或是傳回諸如 `still running after stop`（15 秒後）之類的錯誤。 |
| `moonpool_restart_app` | `app_id`（必填） | 停止，等待連接埠與處理程序釋放，再啟動。傳回 "restarted"。 |
| `moonpool_app_output` | `app_id`（必填），`tail_lines`（整數，預設 200，最小 1） | 應用程式在目前 Moonpool 工作階段中的終端機輸出，已去掉 ANSI 代碼。當記錄比 `tail_lines` 更長時，文字會以一行完整記錄的路徑開頭。如果應用程式還沒有執行過，會以 `no console output recorded for '<id>' (not launched this session)` 失敗。如果記錄存在但是空的，則傳回 `(no output recorded for '<id>')`。 |
| `moonpool_stop_mcp_server` | `app_id`（必填） | 結束該應用程式接入的 MCP 輔助處理程序，而讓應用程式繼續執行。傳回 "stopped"。如果應用程式既沒有 `processName` 也沒有 `mcpProcessName`，則什麼都不做。 |
| `moonpool_refresh_app_icons` | 無 | 重新取得每個應用程式的圖示。傳回 "icons refreshed"。 |

## 設定

這些工具透過 hub 讀取與修改 `apps.json`，而不是磁碟上的檔案。寫入必須帶上上次讀取得到的權杖，過期的權杖會被拒絕，而且在寫入任何內容之前會先驗證新檔案。透過 hub 很重要，因為沙箱主機中的代理看到的可能是設定資料夾的私有副本，而不是真正的那份。

| 工具 | 參數 | 行為 |
| --- | --- | --- |
| `moonpool_read_config` | 無 | JSON 文字，包含 `manifest_text`（檔案的原樣內容）、`token`、`valid`、`error`（有效時為 null）與 `path`。檔案不存在或是空的時，`token` 為 `none`。 |
| `moonpool_write_config` | `manifest`（必填，新的完整 `apps.json` 文字），`expected_token`（必填，來自上次讀取） | 驗證該資訊清單並取代 `apps.json`，然後載入它。傳回 `apps.json updated; new version token <token>`。過期的權杖會以 `stale token: apps.json changed since it was read ...` 失敗。無效的資訊清單會以 `rejected invalid manifest: ...` 失敗。無論哪種情況，檔案都不會被更動。空的 `expected_token` 會被拒絕。 |
| `moonpool_restore_config` | `snapshot`（選用） | 不帶值時，傳回由新到舊列出已儲存快照的 JSON 文字（`index`、`filename`、`millis`、`app_count`、`valid`）。帶索引（1 為最新）或檔名時，會驗證該快照並還原它。傳回 `restored <file> (<n> apps); new version token <token>`。不需要權杖：還原是刻意覆蓋目前檔案的。 |
| `moonpool_reload_config` | 無 | 重新讀取 `apps.json`。傳回 "apps.json reloaded"。如果檔案無法剖析或驗證，它會以 `apps.json has an error: ...` 失敗，Moonpool 繼續使用上一次成功載入的清單。 |
| `moonpool_launcher_paths` | 無 | 列出 hub 的設定資料夾、`apps.json`、`state.json`、記錄、傾印資料夾、圖示資料夾、可攜旗標與執行檔路徑，然後列出 MCP 處理程序的設定資料夾、`apps.json`、`state.json`、傾印資料夾、可攜旗標與執行檔路徑（沒有記錄與圖示）。如果 hub 沒有執行，它那一半會顯示為 `hub paths unavailable: ...`，MCP 那一半仍會顯示。當修改沒有生效時使用它。 |

## 進階：測試工具

`moonpool_screenshot` 僅限 Windows；在 Linux 與 macOS 上它會以 "screenshot is not supported on this platform" 失敗。`moonpool_window_state` 與 `moonpool_reset_mcp_seen` 在所有平台上都可用。

`window` 是 `main`、`settings`、`about`、`installer`、`editor`、`help` 或 `themes` 之一，預設為 `main`。未知的名稱會以 `unknown window '<name>'` 失敗。

| 工具 | 參數 | 行為 |
| --- | --- | --- |
| `moonpool_screenshot` | `window`（選用） | 以內嵌 PNG 的形式擷取該 Moonpool 視窗自己的內容，較長的一邊最多 320 像素。無法透過 MCP 提高這個尺寸。如果視窗沒有顯示，會以 `window '<name>' is not open` 失敗。它無法擷取任何其他應用程式。 |
| `moonpool_window_state` | `window`（選用） | JSON 文字：視窗未開啟時為 `{"open":false}`，否則包含 `open`、`visible`、`minimized`、`maximized`、`x`、`y`、`width`、`height`。供測試使用。 |
| `moonpool_reset_mcp_seen` | `app_id`（選用） | 僅供測試。清除某個應用程式（省略時則為所有應用程式）已記住的「看過 MCP 輔助程式」記錄，這樣側邊欄的 MCP 子列會再次隱藏，直到再次看到輔助程式。 |

## 另請參閱

- [MCP 設定](/zh-hant/automation/mcp-setup/)
- [命令列](/zh-hant/automation/command-line/)
