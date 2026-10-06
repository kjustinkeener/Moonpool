---
title: "編輯 apps.json：它在哪裡，如何重新載入與復原"
description: "找到 Moonpool 為所有受管理應用程式讀取的 apps.json 檔案，用應用程式編輯器或手動編輯它，重新載入它，並從錯誤的修改中復原。"
---

Moonpool 管理的每個應用程式都是 `apps.json` 中的一個項目。你可以透過應用程式編輯器（「新增應用程式」與「編輯應用程式」對話方塊）編輯，也可以手動編輯。兩者寫入的是同一個檔案。某些工具結果與訊息會把這個檔案稱為資訊清單（manifest）。

## 設定位於何處

| 模式 | 設定資料夾 |
| --- | --- |
| 已安裝（Windows） | `%USERPROFILE%\.moonpool\moonpool-config\` |
| 可攜 | `moonpool.exe` 旁邊的 `moonpool-config\`（位於 `.moonpool\` 資料夾內） |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`，否則為 `~/.config/Moonpool/` |

`apps.json` 就在該資料夾中，旁邊還有這些：

| 項目 | 用途 |
| --- | --- |
| `apps.json.history\` | 最近 10 個有效 `apps.json` 檔案的復原環。 |
| `settings.json` | 應用程式設定。請參閱 [settings.json](/zh-hant/data/settings-json/)。 |
| `cli-output\<id>\` | 每個應用程式的工作階段記錄。請參閱[記錄](/zh-hant/data/logs/)。 |
| `moonpool.log` | 偵錯記錄，在 **將除錯資訊寫入檔案** 開啟時產生。 |
| `icons\` | 選用的 `<id>.png`（也可以是 `.ico`、`.svg`、`.jpg`、`.jpeg`、`.webp`）圖示覆寫檔。 |
| `state.json` | 即時狀態快照，每隔幾秒更新一次。 |
| `dumps\` | 由 `dump`、`read-config` 與 `restore-config` 動詞寫入的檔案。 |
| `mcp_seen.json` | 記錄哪些應用程式曾經用過 MCP 輔助程式。 |
| `window-state.json` | 主視窗的大小與位置。 |
| `AI-README.md` | 給 AI 代理的指南，每次啟動時重寫。 |

其中哪些需要備份，請見[備份與復原](/zh-hant/data/backup-and-recovery/#設定資料夾)。

第一次執行時，Moonpool 會用範例項目產生 `apps.json`。已存在的檔案絕不會被覆寫。

## 編輯

- **對話方塊。** 使用側邊欄頂端 **...** 選單中的 **新增應用程式**。要修改某個應用程式，請使用它所在列的鉛筆圖示，或在它上面按右鍵並選擇 **編輯**。對話方塊會立即驗證並儲存。
- **手動。** 同一選單中的 **編輯 apps.json** 會用你的預設編輯器開啟該檔案。儲存後，在選單中選擇 **重新載入**（或按 F5 或 Ctrl+R）。

手動修改要等你重新載入後才會生效。重新載入只會讀取檔案，不會重寫它。

從對話方塊儲存會以標準化、含縮排的形式重寫整個檔案。Moonpool 不認得的索引鍵會被捨棄，而且 JSON 沒有註解，所以請把備註寫在 `note` 欄位中。

## 結構

該檔案是一個由物件組成的 JSON 陣列。每個項目必須有四個索引鍵：`id`、`name`、`group`、`type`，其餘都是選用的。請參閱[應用程式欄位](/zh-hant/apps/fields/)。

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

群組在側邊欄中的順序，就是它們在檔案中第一次出現的順序。

## 重新載入做了什麼

重新載入會用檔案的內容取代 Moonpool 記憶體中的清單。啟動、停止與重新啟動會在你按下時讀取該項目，所以修改後的 `command`、`cwd`、`env` 或結束設定，會在你下次啟動或重新啟動該應用程式時生效。重新載入絕不會重新啟動任何東西：已經在執行的應用程式會繼續沿用它啟動時的設定。

## 驗證

Moonpool 在載入時、每次儲存時以及每次代理寫入時，都會驗證整個檔案。只要有一個項目有問題，整個檔案就會被拒絕。

| 規則 | 錯誤訊息包含 |
| --- | --- |
| 不是合法的 JSON、缺少必要的索引鍵，或某個值的類型不對 | JSON 剖析器給出的訊息 |
| `id` 為空、以 `-` 開頭，或含有字母、數字、`.`、`_`、`-` 之外的字元 | `invalid id` |
| 兩個項目使用了同一個 `id` | `duplicate app id` |
| `name` 為空白 | `has an empty name` |
| `group` 為空白 | `has an empty group` |
| `type` 不是 `desktop`、`web`、`static` 或 `cli` | `unknown type` |
| `port` 為 `0`（大於 65535 的 `port` 會剖析失敗） | `invalid port 0` |
| `static` 項目沒有 `url` | `requires a url` |
| 其他類型沒有 `command` | `requires a command` |

錯誤訊息會依位置指出是哪個項目，例如：

```text
apps.json entry 2 (site) requires a command
```

### id

`id` 是項目的永久索引鍵。它決定記錄資料夾與圖示檔案的名稱，也是你傳給 `moonpool.exe launch <id>` 與代理的值。新增應用程式時，對話方塊會根據名稱產生它：先把名稱轉為小寫，把每一段由 `a` 到 `z` 與 `0` 到 `9` 之外的字元組成的連續字元換成一個 `-`，再去掉兩端的 `-`。結果為空時使用 `app`。如果該 id 已被使用，就依序加上 `-2`、`-3` 等。之後它絕不會再更動 id，所以重新命名應用程式時 id 維持不變。名稱 `Habit Tracker` 得到的 id 是 `habit-tracker`。

## 如果檔案有錯誤

- **重新載入時**，驗證失敗的檔案會維持原樣，Moonpool 繼續使用上一次成功載入的清單。側邊欄上方會出現一條橫幅顯示錯誤，並附一個開啟該檔案的按鈕；清單仍可使用，但會變暗。請參閱[當 apps.json 有錯誤時](/zh-hant/using/hub-window/#當-appsjson-有錯誤時)。
- **啟動時**，檔案損毀代表沒有可保留的清單，所以 Moonpool 會在沒有任何應用程式的情況下啟動，橫幅也會這樣說明。請修正檔案並選擇 **重新載入**，或還原一個快照（見下文，或使用 `moonpool_restore_config` 工具）。
- 無論哪種情況，在檔案重新載入成功之前，來自對話方塊的儲存（以及重新命名、刪除、設定圖示）都會被拒絕，這樣損毀的檔案絕不會被覆寫。請修正檔案並選擇 **重新載入**。
- **透過對話方塊、代理或還原** 進行的無效變更會被拒絕，磁碟上的檔案維持原樣。

Moonpool 在 `apps.json.history\` 中保留 `apps.json` 最近 10 個良好版本。如何回復請見[備份與復原](/zh-hant/data/backup-and-recovery/#回復-appsjson)。症狀與解決方法請見[疑難排解](/zh-hant/support/troubleshooting/#appsjson-有錯誤)。

## 代理

AI 代理應該透過 Moonpool 的 MCP 工具而不是直接改檔案來修改 `apps.json`，這樣過期或無效的寫入會被拒絕，沙箱中的代理也不會去編輯一份私有副本。請參閱 [MCP 工具](/zh-hant/automation/mcp-tools/#設定)。
