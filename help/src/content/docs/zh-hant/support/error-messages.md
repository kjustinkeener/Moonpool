---
title: "Moonpool 錯誤訊息詳解：already running、requires a command 等"
description: "查詢 Moonpool 錯誤訊息的確切文字，例如 already running、requires a command、stale token 與 Update failed，以及各自的意義與解決方法。"
---

把你看到的訊息貼到頁面搜尋中，或是瀏覽下面的表格。訊息按 Moonpool 顯示的原樣引用。`<angle brackets>` 中的文字會被某個值取代（應用程式 id、路徑或來自系統的錯誤）。不屬於錯誤訊息的症狀請見[疑難排解與常見問題](/zh-hant/support/troubleshooting/)。

## 啟動與停止應用程式

| 訊息 | 意義與解決方法 |
| --- | --- |
| `already running` | Moonpool 已經為這個應用程式持有一個終端機。請先停止它，或使用重新啟動。 |
| `stopped during launch` | 啟動仍在進行時按下了停止。請再次啟動。 |
| `app has no launch command` | 該項目沒有 `command`。請在應用程式編輯器或 `apps.json` 中加入。只有帶 `url` 的 `static` 項目可以沒有。 |
| `unknown app: <id>` | 沒有載入具有該 `id` 的應用程式。請檢查 id；如果手動編輯過 `apps.json`，請重新載入。 |
| `unknown app id: <id>` | 同樣的問題，只是回報給指令碼或代理。請用 `moonpool_list_apps` 列出應用程式。 |
| `did not reach running in time` | 來自指令碼或代理：應用程式在 25 秒內沒有顯示為執行中。請檢查 `port` 或 `processName`，並查看輸出。請參閱[狀態圓點不對](/zh-hant/support/troubleshooting/#狀態圓點不對)。 |
| `still running after stop` | 15 秒後應用程式仍顯示為執行中。請設定 `killMode`。請參閱[停止與重新啟動](/zh-hant/apps/stop-and-restart/)。 |
| `refusing to open non-web url: <url>` | `url` 不是 `http://`、`https://`、`mailto:` 或 `file://`。請修正 `url`。 |
| `[process exited]` | 不是錯誤：應用程式的命令已結束。顯示在終端機分頁中。 |

## apps.json 驗證

Moonpool 會拒絕違反某條規則的 `apps.json`，並保留上一次成功載入的清單。`<n>` 是該項目在檔案中的位置，從 1 開始計數。

| 訊息 | 解決方法 |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | 重新命名 `id`。 |
| `duplicate app id "<id>"` | 兩個項目使用了同一個 `id`。請讓每個都是唯一的。 |
| `apps.json entry <n> (<id>) has an empty name` | 填寫 `name`。 |
| `apps.json entry <n> (<id>) has an empty group` | 填寫 `group`。 |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` 必須是 `web`、`desktop`、`static` 或 `cli`。 |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` 必須在 1 到 65535 之間。 |
| `apps.json entry <n> (<id>) requires a url` | `static` 項目需要 `url`。 |
| `apps.json entry <n> (<id>) requires a command` | 其他每種類型都需要 `command`。 |

在應用程式編輯器中，沒填名稱就儲存會顯示「name 為必填欄位。」。橫幅文字「apps.json 有錯誤，目前顯示的是上次成功載入的清單。」或「apps.json 有錯誤，因此沒有載入任何應用程式。」，以及如何復原，請見 [apps.json 有錯誤](/zh-hant/support/troubleshooting/#appsjson-有錯誤)。如果橫幅說儲存已暫停，訊息會以 `Repair apps.json and reload it before saving from Moonpool` 結尾。完整的規則清單請見[驗證](/zh-hant/apps/apps-json/#驗證)。

## 設定、更新與安裝程式

| 訊息 | 意義與解決方法 |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` 格式有誤。請修復或刪除它，然後重新啟動。請參閱 [settings.json](/zh-hant/data/settings-json/#讀取與修復)。 |
| `Update failed: <error>` | 更新的下載或安裝失敗。請參閱[更新失敗時](/zh-hant/data/updating/#更新失敗時)。 |
| `Update check failed: <error>` | 「關於」中的更新檢查失敗。冒號後面的文字說明了原因。請稍後再試。 |
| `Install failed: <error>` | 安裝程式在冒號後面所指的步驟停止了，例如 `copy exe: ...`。請結束所有從 `%USERPROFILE%\.moonpool` 執行的 Moonpool，然後重試。 |
| `target folder does not exist` | 為可攜副本選的資料夾已經不存在。請選一個已存在的資料夾。 |

## MCP 與指令碼

| 訊息 | 意義與解決方法 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | 啟動 Moonpool，或是讓代理呼叫那個工具。對可攜副本，這則訊息會寫出該副本的名稱。 |
| `frontend not loaded` | 主視窗還沒有載入完。請稍等後重試。 |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | 代理傳入了 MCP 伺服器不接受的 id。請使用 `moonpool_list_apps` 給出的 id。 |
| `stale token: apps.json changed since it was read ...` | 重新讀取 `apps.json`，重新套用修改，然後再寫入。 |
| `rejected invalid manifest: ...` | 新的 `apps.json` 未通過驗證（見上文）。檔案沒有被更動。 |
| `no console output recorded for '<id>' (not launched this session)` | `moonpool_app_output` 被用來查詢一個自 Moonpool 啟動以來沒有執行過的應用程式。 |

更多內容請見 [MCP 工具](/zh-hant/automation/mcp-tools/)與 [MCP 設定](/zh-hant/automation/mcp-setup/#如果這些工具沒有作用)。

## 來自其他程式的錯誤

- [`Error: listen EADDRINUSE: address already in use :::3000` 與 `Port 5173 is in use`](/zh-hant/support/port-already-in-use/)
- [`Windows 已保護您的電腦`](/zh-hant/support/windows-protected-your-pc/)
- [缺少 WebView2 執行階段](/zh-hant/support/webview2-runtime-missing/)
