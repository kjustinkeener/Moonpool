---
title: "從命令列控制 Moonpool"
description: "在終端機或指令碼中用 moonpool.exe 動詞驅動執行中的 Moonpool，用 ticket 標記命令，並從 state.json 讀取結果。"
---

當同一個 Moonpool 已在執行時，再次執行 `moonpool.exe` 不會開啟第二個視窗。第二個處理程序會透過[控制通道](/zh-hant/automation/control-verbs/)把它的引數傳給執行中的那個，然後結束。Moonpool 必須已經在執行：如果沒有常駐的執行個體，同樣的命令會啟動一個新的 Moonpool，而該動詞不會被執行。

「同一個 Moonpool」指的是同一個資料夾。已安裝的 Moonpool 與每個可攜副本各自獨立執行，所以一條命令只會到達你執行的那個 `moonpool.exe` 所對應的副本，絕不會到達別的副本。請參閱[可攜模式](/zh-hant/data/portable-mode/#同時執行多個副本)。

請使用你所指副本的路徑。對已安裝的副本：

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

有多個副本在執行時，`Get-Process moonpool` 會把它們全部列出，所以請依 `Path` 挑選，而不要取第一個。它還會列出 MCP 主機程式啟動的閒置 `moonpool.exe mcp` 輔助程式，所以存在 `moonpool` 處理程序並不能證明有 hub 在執行。請改用 `ping` 詢問控制通道（[控制動詞](/zh-hant/automation/control-verbs/)）。

## 動詞

動詞不分大小寫。`<id>` 是 `apps.json` 中某個應用程式的 `id`。

| 命令 | 效果 |
| --- | --- |
| `moonpool.exe` | 不帶動詞：把視窗帶到最前面。 |
| `moonpool.exe show` | 把視窗帶到最前面。 |
| `moonpool.exe launch <id>` | 啟動應用程式並開啟它的終端機分頁。 |
| `moonpool.exe stop <id>` | 停止應用程式。 |
| `moonpool.exe restart <id>` | 停止，等待連接埠與處理程序釋放，再啟動。 |
| `moonpool.exe reload` | 重新讀取 `apps.json`。 |
| `moonpool.exe refresh-icons` | 重新取得每個圖示。 |
| `moonpool.exe help` | 開啟說明視窗。 |
| `moonpool.exe quit` | 結束 Moonpool，與系統匣選單相同。 |
| `moonpool.exe dump <id> [out-path]` | 不帶 `out-path` 時，回報該應用程式在本次工作階段中的記錄路徑。帶上它時，把記錄以去掉 ANSI 代碼的純文字形式複製到那裡。 |
| `moonpool.exe paths` | 回報執行中的 Moonpool 所使用的設定資料夾、`apps.json`、`state.json`、記錄、傾印資料夾、圖示資料夾、可攜旗標與執行檔路徑。 |
| `moonpool.exe read-config` | 在設定資料夾中寫出 `dumps\read-config.json`，其中包含 `token`、`valid`、`error`、`path` 與 `manifest_text`（`apps.json` 的原樣內容）。 |
| `moonpool.exe write-config <file> [token]` | 用 `<file>` 中的資訊清單取代 `apps.json`，前提是該資訊清單有效，而且在給出 `token` 時 `apps.json` 仍與它相符。 |
| `moonpool.exe restore-config [index or filename]` | 不帶引數時，把快照清單寫入 `dumps\restore-config.json`。帶引數時，如果該快照有效就還原它。 |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

未知的動詞會被忽略。該程式還有它自己的啟動引數：`moonpool.exe mcp`（[MCP 設定](/zh-hant/automation/mcp-setup/)）、`--uninstall`（由「新增或移除程式」使用）與 `--wait-pid <pid>`（Moonpool 重新啟動自身時使用）。這些引數只有作為第一個引數時才會生效，所以像 `--uninstall` 這樣的應用程式 id 無法觸發它們。

## 讀取結果

命令列不會印出任何內容，所以請用 `--ticket <key>`（任意唯一的索引鍵，位置不限）標記一條命令，然後從設定資料夾中的 `state.json` 讀取結果。該資料夾在安裝版是 `%USERPROFILE%\.moonpool\moonpool-config\`，可攜副本是 `<your .moonpool folder>\moonpool-config\`，Linux 上是 `~/.config/Moonpool/`（請參閱[設定概觀](/zh-hant/apps/apps-json/#設定位於何處)）。`show` 與 `quit` 不會寫入 ticket。

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` 包含 `apps`、`statuses`（每個應用程式的 `id`、`running`、`managed`、`mcpRunning`、`mcpSeen`）與 `tickets`。執行中的 Moonpool 每隔幾秒以及每條命令之後都會重寫它，而且在結束時不會刪除它，所以遺留下來的檔案並不代表 Moonpool 在執行。要詢問它是否在執行，或取得即時的應用程式清單，請使用控制通道的 `ping` 與 `list` 動詞（[控制動詞](/zh-hant/automation/control-verbs/)）或 MCP 工具。請輪詢你的 ticket，直到 `status` 不再是 `pending`：

| `status` | 意義 |
| --- | --- |
| `pending` | 已收到；Moonpool 仍在處理。 |
| `ok` | 完成。對 `dump`、`read-config`、`write-config`、`restore-config` 與 `paths`，`detail` 裡是路徑、權杖或報告。 |
| `error` | 失敗；`detail` 說明原因，例如 `unknown app id: x`、`did not reach running in time`、`unknown command`。 |

每個 ticket 是 `{ ticket, action, arg, status, detail, ts }`，其中 `ts` 為 Unix 毫秒時間戳記：

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

已完成的 ticket 會在 24 小時後被捨棄；一旦已完成的 ticket 至少有 5 分鐘之久，清單就會朝 50 筆的數量修剪。

支援 MCP 的代理可以省去輪詢：請參閱 [MCP 設定](/zh-hant/automation/mcp-setup/)。

## 另請參閱

- [AI 代理：快速開始](/zh-hant/automation/quick-start/)
- [控制動詞](/zh-hant/automation/control-verbs/)
