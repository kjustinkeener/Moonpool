---
title: "透過 MCP 把 AI 代理連接到 Moonpool"
description: "在你的主機程式中把 moonpool.exe mcp 登錄為 stdio MCP 伺服器（安裝版或可攜版皆可），並了解 Moonpool 如何追蹤應用程式自己的 MCP 輔助程式。"
---

Moonpool 的執行檔本身就是它的 MCP 伺服器。在主機程式中把它登錄為 stdio 伺服器，執行 `moonpool.exe` 並只帶一個引數 `mcp`。

## 登錄伺服器

安裝版的程式是 `%USERPROFILE%\.moonpool\moonpool.exe`。可攜版則是你 `.moonpool\` 資料夾裡的 `moonpool.exe`。把這個完整路徑用作 `command`。對於讀取 `.mcp.json` 的主機程式：

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

在 JSON 檔案中，反斜線必須寫成兩個，如上所示。對於提供命令列登錄方式的主機程式（例如 Claude Code），一步就能加入：

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

該伺服器以 `moonpool` 作為自己的名稱，使用 MCP 通訊協定版本 `2025-06-18`，而且只提供工具（不列出資源或提示詞）。這些工具在代理看來是 `moonpool_*`，請參閱 [MCP 工具](/zh-hant/automation/mcp-tools/)。

## 多個 Moonpool

已安裝的 Moonpool 與每個可攜副本都是各自獨立的啟動器，各有自己的應用程式，而且可以同時執行。某個副本的 `moonpool.exe mcp` 總是驅動那個副本。要讓代理使用多個副本，請為每一個登錄一個不同的名稱，指向該副本的執行檔：

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

在大多數主機程式中，把兩個副本登錄為同一個名稱會讓一個取代另一個。每個副本的工具名稱都相同，所以主機程式靠你登錄時用的名稱來區分它們。可攜副本還會把自己宣告為 `moonpool (<folder>)`，它的伺服器說明裡也會寫出資料夾，這樣代理就能看出自己在和哪個副本通訊。

## 附註

- `moonpool.exe mcp` 從不開啟視窗，也從不啟動安裝程式。主機程式關閉它的輸入時，它就會結束。
- 它使用啟動它的那個執行檔的設定資料夾與控制通道，所以可攜版執行檔讀取可攜資料夾的資料，並驅動那個可攜副本。只有 `moonpool.portable` 位於它旁邊時，這個執行檔才算可攜版。任何其他 `moonpool.exe`，無論放在哪裡，都使用已安裝的 Moonpool 的資料夾（`%USERPROFILE%\.moonpool\moonpool-config\`），並驅動已安裝的 Moonpool。
- 大多數工具需要 Moonpool 正在執行。如果它沒有執行，代理可以先呼叫 `moonpool_bootup_launcher`。
- `moonpool_launcher_paths` 會把 hub 使用的資料夾與 MCP 處理程序解析出的資料夾並排顯示。兩者不同，就代表代理看的是與 hub 不同的 `apps.json`。

## 沙箱主機

有些主機程式在封裝（Store/MSIX）沙箱中執行它們的工具，沙箱會把 AppData 重新導向到每個套件各自私有的副本。當 Moonpool 的設定資料夾或執行檔解析到類似 `...\Packages\<package>\LocalCache\...` 的路徑下時，它就會偵測到這一點。

當控制通道有回應但無法讀取 `state.json` 時，它也會偵測到。這時讀寫檔案的工具（`moonpool_app_output`、`moonpool_read_config`、`moonpool_write_config`、`moonpool_restore_config`）會傳回一則指明原因的錯誤，而不是空資料或過期資料。只使用控制通道的工具（例如 `moonpool_list_apps`）在通道可連線時不會被攔截。如果沙箱連通道也隱藏了，這些工具會回報沙箱的問題，而不是「Moonpool is not running」。請改在沙箱之外的 shell 中使用[命令列](/zh-hant/automation/command-line/)。

## 有自己 MCP 伺服器的應用程式

Moonpool 中的許多應用程式，本身就是由 MCP 主機程式透過一個 `<exe> mcp` 輔助處理程序來存取的。Moonpool 會尋找名稱符合應用程式的 `processName`、且第一個引數為 `mcp` 的處理程序，例如 `notes-app.exe mcp`。如果伺服器以別的名稱執行（例如改過名稱的副本），請設定應用程式的 `mcpProcessName` 萬用字元（請參閱[欄位](/zh-hant/apps/fields/#mcpprocessname)）；符合它的處理程序不需要 `mcp` 引數也會被計入。

- 有輔助處理程序接入時，應用程式的側邊欄會把一個 MCP 子列顯示為執行中，`moonpool_list_apps` 也會在該應用程式那一行後附加 `[mcp: running]`。輔助處理程序不算作應用程式本身在執行。
- 一旦看過某個輔助處理程序，Moonpool 就會記住它（儲存在設定資料夾中的 `mcp_seen.json`），所以在輔助處理程序結束之後，MCP 子列仍會顯示為已停止，`moonpool_list_apps` 則顯示 `[mcp: stopped]`。
- MCP 子列由 `showMcpProcesses` 設定控制（[設定視窗](/zh-hant/using/settings/)）。
- `moonpool_stop_mcp_server` 會結束輔助處理程序，而不動應用程式。沒有對應的啟動操作：擁有該輔助處理程序的主機程式會在它下一次工具呼叫時再次啟動它。

## 如果這些工具沒有作用

- **主機程式沒有顯示任何 `moonpool_*` 工具。** 檢查 `command` 是否是 `moonpool.exe` 的完整路徑，`args` 是否為 `["mcp"]`，然後重新啟動主機程式。
- **每個工具都說 Moonpool 沒有執行。** 啟動 Moonpool，或呼叫 `moonpool_bootup_launcher`。確認登錄的執行檔就是你正在執行的那個副本。
- **修改沒有生效。** 呼叫 `moonpool_launcher_paths`，比較 hub 的資料夾與 MCP 處理程序的資料夾。請參閱[沙箱主機](#沙箱主機)。

更多內容請見[疑難排解](/zh-hant/support/troubleshooting/#mcp-與指令碼錯誤)。

## 另請參閱

- [為 AI 代理（Claude Code、Codex、Cursor）提供啟動與停止本機應用程式的 MCP 伺服器](/zh-hant/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [MCP 工具](/zh-hant/automation/mcp-tools/)
- [AI 代理：快速開始](/zh-hant/automation/quick-start/)
