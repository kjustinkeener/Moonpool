---
title: "讓 AI 代理設定並操作 Moonpool：快速開始"
description: "讓 AI 代理或指令碼設定並操作 Moonpool 的三種方式，如何依你的代理選擇，以及同一個操作在每種方式下的寫法。"
---

有三種入口。請依你的代理能做什麼來選擇。

| 你想要 | 使用 | 從這裡開始 |
| --- | --- | --- |
| 讓代理找到你的應用程式並一次性加入它們 | 主視窗空白畫面上的 **複製提示詞** | 見下文 |
| 讓代理以工具呼叫的方式啟動、停止與讀取應用程式 | MCP 伺服器，`moonpool.exe mcp` | [MCP 設定](/zh-hant/automation/mcp-setup/) |
| 指令碼，或不支援 MCP 的代理 | 命令列動詞 | [命令列](/zh-hant/automation/command-line/) |

## 複製提示詞

沒有開啟任何分頁時，CLI 面板會顯示一段現成的提示詞（「第一次使用？把這段內容交給 AI 代理，讓它幫你設定應用程式：」）。**複製提示詞** 會把它放到剪貼簿。把它貼給你的代理。它會讓代理查看設定資料夾中的 `AI-README.md` 與 `apps.json`，並請它找到你的應用程式並登錄。完成後，選擇 **重新載入**。

Moonpool 每次啟動都會重寫 `apps.json` 旁邊的 `AI-README.md`，所以它永遠與你所執行的版本一致。請不要把你自己的修改保存在裡面。

## 同一個操作的三種寫法

| 操作 | 命令列 | 控制通道動詞 | MCP 工具 |
| --- | --- | --- | --- |
| 啟動應用程式 | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| 停止應用程式 | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| 重新啟動應用程式 | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| 讀取應用程式的輸出 | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| 列出應用程式與狀態 | 讀取 `state.json` | `list` | `moonpool_list_apps` |
| 重新讀取 `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| 讀取 `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| 取代 `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| 回復 `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| 顯示視窗 | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| 啟動 Moonpool | `moonpool.exe` | 無 | `moonpool_bootup_launcher` |
| 結束 Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| 顯示正在使用的資料夾 | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

命令列不會印出任何內容；請用 `--ticket` 讀取結果（請參閱[讀取結果](/zh-hant/automation/command-line/#讀取結果)）。控制通道與 MCP 則會直接回答。

## 當代理的工具失敗時

- `Moonpool is not running - call moonpool_bootup_launcher first`（Moonpool 沒有執行，請先呼叫 moonpool_bootup_launcher）：啟動 Moonpool，或是讓代理呼叫那個工具。
- 修改「沒有生效」：請代理呼叫 `moonpool_launcher_paths`。如果主視窗與 MCP 的資料夾不同，代表代理讀取的是另一份 `apps.json`。請參閱[沙箱主機](/zh-hant/automation/mcp-setup/#沙箱主機)。
- 有多個 Moonpool 副本：為每個副本用各自的名稱登錄。請參閱[多個 Moonpool](/zh-hant/automation/mcp-setup/#多個-moonpool)。

針對 Claude Code、Codex 與 Cursor 的完整範例請見[為 AI 代理提供啟動與停止本機應用程式的 MCP 伺服器](/zh-hant/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)。

更多症狀請見[疑難排解](/zh-hant/support/troubleshooting/#mcp-與指令碼錯誤)。
