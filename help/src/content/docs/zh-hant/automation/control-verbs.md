---
title: "Moonpool 控制通道與動詞參考"
description: "Moonpool 控制通道（具名管道或 Unix 通訊端）如何運作，它的通訊協定，以及執行中的應用程式所回應的每個動詞，包括引數與回覆。"
---

## 它在哪裡監聽

每個 Moonpool 副本都有自己的通道，所以已安裝的 Moonpool 與任意可攜副本可以並排執行，而不會替彼此回應。在 Windows 上，已安裝的 Moonpool 監聽具名管道 `\\.\pipe\moonpool`。可攜副本會加上由其資料夾產生的 id：`\\.\pipe\moonpool-<id>`。

`<id>` 是由該副本的 `moonpool-config` 資料夾路徑衍生出的 8 位十六進位數，所以對同一個資料夾，它在重新啟動與更新之間維持不變，而如果你搬動了資料夾，它就會改變。副本的 `moonpool.exe`（包括 `moonpool.exe mcp`）總是能找到它自己副本的通道。

在 Linux 上，它改為監聽 Unix 網域通訊端，權限為 `0600`：

| 情形 | 通訊端路徑 |
| --- | --- |
| 一般 | 設定了 `$XDG_RUNTIME_DIR` 時為 `$XDG_RUNTIME_DIR/moonpool.sock`，否則為 Moonpool 設定資料夾中的 `moonpool.sock` |
| 可攜模式 | 可攜副本設定資料夾中的 `moonpool.sock`，所以可攜副本絕不會與已安裝的副本衝突 |
| 路徑對通訊端來說太長（約 100 個字元） | `/tmp/moonpool-<uid>/moonpool.sock`，位於只有你能開啟的目錄中（可攜副本為 `moonpool-<id>.sock`） |

當機遺留下來的通訊端檔案會在下次啟動時被偵測到並取代。仍有東西在回應的通訊端絕不會被接管。Moonpool 正常結束時會刪除該檔案。

[MCP 伺服器](/zh-hant/automation/mcp-setup/)也是靠這個通道得知 Moonpool 是否在執行的：如果 `ping` 得到了回應，它就在執行；管道或通訊端不存在，它就沒有執行。除下面的診斷動詞外，同樣的動詞也可以從[命令列](/zh-hant/automation/command-line/)使用。

## 通訊協定

每行輸入一個 JSON 物件，輸出一行 JSON，依序進行。一個連線可以承載多個要求。

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

在 PowerShell 中傳送要求並讀取回覆：

對可攜副本，請用它的管道名稱（`moonpool-<id>`，由 `paths` 動詞顯示）取代 `moonpool`。

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` 是字串清單，可以省略。其他欄位會被忽略。
- `result` 是字串或 null。傳回結構化資料的動詞會把它作為 JSON 字串傳回。
- 不是合法 JSON 的行會得到 `{"ok": false, "error": "bad request: ..."}`。
- 未知的 `cmd` 會得到 `unknown cmd: <name>`。
- 經由視窗處理的動詞（`launch`、`stop`、`restart`、`reload`、`refresh-icons`、`help`、`open-window`）會在動作完成時回應，或在 45 秒後以逾時錯誤回應。如果主視窗的介面還沒有載入，它會立即以 `frontend not loaded` 失敗。
- 在前一個 Moonpool 仍在結束時啟動的 Moonpool，會重試繫結通道約 8 秒。如果仍然不行，它會記錄這一點，並在沒有通道的情況下繼續執行。

## 動詞

| 動詞 | 引數 | 結果 |
| --- | --- | --- |
| `ping` | 無 | `pong`。僅限通道。 |
| `list` | 無 | JSON 字串 `{"apps": [...], "statuses": [...]}`，讀取自執行中 hub 的記憶體，`apps` 與 `statuses` 的結構與 `state.json` 相同。當已登錄應用程式但還沒進行第一次狀態檢查時，會加上 `"statusNotReady": true`。當 `apps.json` 載入失敗時，會加上 `"manifestError": "<message>"`（此時應用程式是上一次成功載入的清單），而且如果自啟動以來還沒有載入過任何清單，再加上 `"manifestLoaded": false`。僅限通道。 |
| `show` | 無 | null。把視窗帶到最前面。 |
| `quit` | 無 | null。結束 Moonpool。 |
| `launch` | `<id>` | 成功時為 null，對只有 `url` 的 `static` 項目為 `opened`。錯誤：`unknown app id: <id>`、`did not reach running in time`。 |
| `stop` | `<id>` | 成功時為 null，對只有 `url` 的 `static` 項目為 `stopped`。錯誤：`still running after stop`。 |
| `restart` | `<id>` | 結果與錯誤和 `launch` 相同。 |
| `reload` | 無 | 成功時為 null。 |
| `refresh-icons` | 無 | 成功時為 null。 |
| `help` | 無 | null。開啟說明視窗。 |
| `dump` | `<id>` [`out-path`] | 該應用程式工作階段記錄的路徑，或位於 `out-path` 的純文字副本的路徑。 |
| `paths` | 無 | hub 所使用的資料夾與執行檔的多行報告。 |
| `read-config` | 無 | `dumps\read-config.json` 的路徑，其中包含 `token`、`valid`、`error`、`path`、`manifest_text`。 |
| `write-config` | `<source-file>` [`token`] | 新的版本權杖。錯誤：`stale token: ...`、`rejected invalid manifest: ...`、`cannot read source ...`。 |
| `restore-config` | [`index` 或 `filename`] | 不帶引數：`dumps\restore-config.json` 的路徑（`count`、`snapshots`）。帶引數時：`restored <file> (<n> apps); new version token <token>`。 |
| `argv` | 命令列引數 | 立即傳回 null。會像該副本的第二個 `moonpool.exe <args>` 那樣原樣執行它們，包括 `--ticket`。第二次啟動正是靠它在結束前把自己的引數交過來。 |

互動範例：

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` 與 `restore-config` 會立即載入新的資訊清單，在 `apps.json.history\` 中記錄一份快照，並重新整理視窗。

## 診斷動詞（測試用）

僅限通道：命令列不接受這些動詞。除 `screenshot` 外，在 Windows 與 Linux 上都能用；`screenshot` 僅限 Windows，在其他系統上會回應 `screenshot is not supported on this platform (Windows only)`。

| 動詞 | 引數 | 結果 |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | 僅限 Windows。該 Moonpool 視窗（預設 `main`）的 PNG 的 Base64。選用的 `max_dim` 限制較長一邊的像素數（限定在 320-2400，預設 320；MCP 工具一律使用預設值）。非整數的 `max_dim` 是錯誤。允許的視窗：`main`、`settings`、`about`、`installer`、`editor`、`help`、`themes`。錯誤：`unknown window '<name>'`、`window '<name>' is not open`。不會寫入磁碟。 |
| `open-window` | `<kind>` [`<id>`] | null。像它的選單項目那樣開啟一個視窗。`kind`：`settings`、`about`、`installer`、`help`、`themes`、`editor`（選用的 `<id>` 會開啟該應用程式的「編輯應用程式」對話方塊，不帶則開啟「新增應用程式」）、`terminal`（必須帶 `<id>`：選取該應用程式的終端機分頁，並加寬主視窗以顯示 CLI 面板；不會啟動它）、`cli`（只加寬主視窗）。錯誤：`unknown window kind '<kind>'`、`terminal needs an app id`、`unknown app id: <id>`。與 `launch` 一樣經由主視窗回應。 |
| `window-state` | [`window`] | JSON 字串：`{"open":false}`，或包含 `open`、`visible`、`minimized`、`maximized`、`x`、`y`、`width`、`height`。 |
| `stop-mcp` | `<id>` | `stopped`。結束該應用程式的 `<processName> mcp` 輔助程式，而不是應用程式本身。錯誤：`missing app id`、`unknown app id: <id>`。 |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` 或 `<id>: was not marked seen`；不帶 id 時為 `cleared <n> entries`。清除已記住的 MCP 輔助程式記錄。 |

命令列的 `--ticket` 與 `state.json` 結果記錄屬於另一個通道；請參閱[命令列](/zh-hant/automation/command-line/#讀取結果)。通道要求的答案直接在回覆中給出。

## 另請參閱

- [命令列](/zh-hant/automation/command-line/)
- [AI 代理：快速開始](/zh-hant/automation/quick-start/#同一個操作的三種寫法)
