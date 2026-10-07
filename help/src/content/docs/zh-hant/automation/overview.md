---
title: "用指令碼與 AI 代理自動化 Moonpool"
description: "從指令碼與 AI 代理驅動執行中的 Moonpool 的三種方式（MCP、命令列與控制動詞），它們之間的關係，以及各自能更動什麼。"
---

不碰它的視窗也能驅動 Moonpool。共有三種介面，全部由同一個常駐的 Moonpool（系統匣執行個體，這裡稱為 hub）提供服務。

每個 Moonpool 副本都是自己的 hub：已安裝的副本與每個可攜副本各自獨立執行，各有自己的控制通道。一種介面總是連到它所用的 `moonpool.exe` 所對應的那個副本。請參閱[可攜模式](/zh-hant/data/portable-mode/#同時執行多個副本)。

| 介面 | 它是什麼 | 參考 |
| --- | --- | --- |
| MCP 伺服器 | `moonpool.exe mcp`，由 AI 主機程式啟動的 stdio [MCP](https://modelcontextprotocol.io) 伺服器。 | [MCP 設定](/zh-hant/automation/mcp-setup/)、[MCP 工具](/zh-hant/automation/mcp-tools/) |
| 命令列 | `moonpool.exe <verb> [args]`。同一副本的第二次執行會透過控制通道把動詞交給它的 hub，然後結束。 | [命令列](/zh-hant/automation/command-line/) |
| 控制通道 | Windows 上是具名管道 `\\.\pipe\moonpool`（可攜副本為 `\\.\pipe\moonpool-<id>`），Linux 上是 Unix 通訊端，每行一個 JSON 要求。 | [控制動詞](/zh-hant/automation/control-verbs/) |

## 它們之間的關係

- hub 擁有一切：啟動應用程式、工作階段記錄、`apps.json`。
- MCP 伺服器是 hub 的用戶端，而不是它的第二個副本。大多數工具呼叫會透過控制通道轉送給 hub，回覆則作為工具結果傳回。例外：`moonpool_bootup_launcher` 會自己啟動 `moonpool.exe`；`moonpool_app_output` 與設定類工具會讓 hub 寫出一個檔案再讀取它；`moonpool_launcher_paths` 會把 MCP 處理程序自己的路徑附加到 hub 的路徑之後。
- 是否有 hub 在執行，是透過對該通道發出 ping 來判斷的，而不是去找某個處理程序。能回應的 hub 就是在執行；管道或通訊端不存在就代表沒有執行。
- 每種介面都執行與視窗相同的處理常式，所以一個動詞做的事就和對應的點選操作一樣。
- 如果沒有 hub 在執行，對它起作用的工具（包括 `moonpool_list_apps`）會以「Moonpool is not running」（Moonpool 沒有執行）拒絕。不會有過期的清單。`moonpool_bootup_launcher` 會啟動它。如果有東西佔著通道但幾秒內不回應，錯誤訊息會說明某個 Moonpool 處理程序可能已經停止回應。
- MCP 伺服器不再退回去驅動早於控制通道的 hub 版本。請更新那個副本，或是結束它再重新啟動。

## 哪些介面可以更動東西

| 可以更動 | 介面 |
| --- | --- |
| 啟動、停止或重新啟動應用程式 | MCP、命令列、管道 |
| 重寫 `apps.json` | MCP（`moonpool_write_config`、`moonpool_restore_config`）、命令列、管道 |
| 結束 Moonpool | MCP（`moonpool_shutdown_launcher`）、命令列（`quit`）、管道 |
| 結束應用程式的 MCP 輔助處理程序 | MCP（`moonpool_stop_mcp_server`）、管道（`stop-mcp`） |
| 重新載入 `apps.json`、重新取得圖示、顯示視窗 | MCP（`moonpool_reload_config`、`moonpool_refresh_app_icons`、`moonpool_raise_launcher`）、命令列（`reload`、`refresh-icons`、`show`）、管道 |
| 開啟視窗或終端機分頁 | 管道（`open-window`） |
| 清除已記住的 MCP 輔助處理程序記錄 | MCP（`moonpool_reset_mcp_seen`）、管道（`reset-mcp-seen`） |

唯讀工具：`moonpool_list_apps`、`moonpool_app_output`、`moonpool_read_config`、`moonpool_launcher_paths`、`moonpool_window_state`、`moonpool_screenshot`。

## 安全特性

- **設定寫入受到保護。** 寫入必須帶上上次讀取得到的版本權杖，過期的權杖會被拒絕，而且在寫入任何內容之前會先驗證新的 `apps.json`。被拒絕的寫入不會更動 `apps.json`。請參閱 [MCP 工具](/zh-hant/automation/mcp-tools/#設定)。
- **應用程式 id 受到限制。** MCP 伺服器只接受字母、數字、`.`、`_` 與 `-`，且絕不能以 `-` 開頭，這樣 id 就不會被當成命令列旗標。
- **螢幕擷取只限 Moonpool。** `moonpool_screenshot` 只會擷取 Moonpool 自己的某個視窗（`main`、`settings`、`about`、`installer`、`editor`、`help`、`themes`），絕不會擷取整個螢幕或其他應用程式。PNG 在記憶體中產生並以內嵌方式傳回；Moonpool 不會把它儲存為檔案。
- **通道沒有身分驗證。** Moonpool 不會替控制管道或通訊端加上登入或權杖。任何能開啟它的處理程序都可以傳送動詞。在 Linux 上，通訊端檔案以 `0600` 權限建立，所以只有你自己的使用者可以開啟。
- **會偵測沙箱主機。** 如果 MCP 伺服器發現自己執行在封裝（Store/MSIX）沙箱中，在那裡它看到的是 Moonpool 檔案的一份私有副本，那麼讀寫檔案的工具（`moonpool_app_output`、`moonpool_read_config`、`moonpool_write_config`、`moonpool_restore_config`）會傳回一則說明原因的錯誤，而不是過期的資料。只使用控制通道的工具不會被攔截。請參閱 [MCP 設定](/zh-hant/automation/mcp-setup/#沙箱主機)。

## 平台

控制通道在每個平台上都存在：Windows 上是具名管道，Linux 上是 Unix 通訊端（位置請見[控制動詞](/zh-hant/automation/control-verbs/#它在哪裡監聽)）。只有 `screenshot`（因此還有 `moonpool_screenshot`）僅限 Windows；在 Linux 上它會傳回「not supported on this platform」（此平台不支援）。命令列動詞在每個平台上都能用。

## 另請參閱

- [AI 代理：快速開始](/zh-hant/automation/quick-start/)
- [MCP 設定](/zh-hant/automation/mcp-setup/)
