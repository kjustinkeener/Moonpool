---
title: "為 AI 代理（Claude Code、Codex、Cursor）提供啟動與停止本機應用程式的 MCP 伺服器"
description: "把 Moonpool 登錄為 MCP 伺服器，讓 Claude Code、Codex 或 Cursor 可以啟動、停止、重新啟動開發伺服器並讀取其輸出，而不會多出重複的處理程序。"
---

AI 程式設計代理通常是在它自己的 shell 裡輸入 `npm run dev` 來執行你的開發伺服器。這可能會卡住代理，留下佔著連接埠的孤兒處理程序，或是把你已經在執行的東西又啟動第二份。MCP 伺服器讓代理呼叫工具去啟動與停止你已經設定好的應用程式，而不是自己去拼湊它的命令列。

## Moonpool 的做法

Moonpool 的執行檔本身就是它的 MCP 伺服器：把 `moonpool.exe` 加上唯一的引數 `mcp`，登錄為 stdio 伺服器。應用程式寫進 `apps.json` 之後，代理就可以依 id 啟動它。

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

登錄伺服器。在 Claude Code 中，一條命令即可（已安裝的 Moonpool；請使用你自己執行檔的完整路徑）：

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

讀取 MCP 伺服器 JSON 檔案的主機程式（例如 Cursor 的 `mcp.json`）使用相同的結構（反斜線要寫成兩個）：

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

對於 Codex，在它的設定檔（`~/.codex/config.toml`）中加入一個使用相同命令與 `mcp` 引數的伺服器：

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

具體的檔案名稱與索引鍵名稱由各個主機程式決定，如果你的版本不同，請查閱它的 MCP 文件。Moonpool 只需要 `moonpool.exe` 的完整路徑，以及作為引數的 `mcp`。之後請重新啟動主機程式。

## 代理能做什麼

這些工具會以 `moonpool_*` 的名稱出現。日常工作用到的有：

| 工具 | 用途 |
| --- | --- |
| `moonpool_list_apps` | 找到應用程式的 id，並查看它是否在執行。 |
| `moonpool_start_app` | 依 id 啟動應用程式並開啟它的終端機分頁。 |
| `moonpool_stop_app` | 停止它，包括它的子處理程序。 |
| `moonpool_restart_app` | 停止，等待連接埠空出，再啟動。修改程式碼後使用。 |
| `moonpool_app_output` | 讀取應用程式印出的內容，可用 `tail_lines` 限制行數。 |
| `moonpool_bootup_launcher` | 在 Moonpool 沒有執行時啟動 Moonpool 本身。 |

典型的迴圈是先 `moonpool_restart_app`，再 `moonpool_app_output`。其餘工具（讀寫 `apps.json`、螢幕擷取）請見 [MCP 工具](/zh-hant/automation/mcp-tools/)。

## 如果沒有作用

如果每個工具都回傳 `Moonpool is not running - call moonpool_bootup_launcher first`（Moonpool 沒有執行，請先呼叫 moonpool_bootup_launcher），代表 Moonpool 還沒有啟動。修改沒有生效，通常是因為代理看的是另一份 `apps.json`：請呼叫 `moonpool_launcher_paths`。請參閱[如果這些工具沒有作用](/zh-hant/automation/mcp-setup/#如果這些工具沒有作用)。

## 另請參閱

- [MCP 設定](/zh-hant/automation/mcp-setup/)
- [MCP 工具](/zh-hant/automation/mcp-tools/)
- [AI 代理：快速開始](/zh-hant/automation/quick-start/)
- [在 Windows 上讓 npm 開發伺服器在背景執行](/zh-hant/guides/run-npm-dev-server-in-background-windows/)
