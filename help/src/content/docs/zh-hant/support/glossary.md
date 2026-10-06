---
title: "Moonpool 詞彙表：應用程式、狀態、檔案與設定"
description: "Moonpool 說明中用來稱呼各個部分、應用程式狀態、檔案與設定的詞彙的白話定義，幫助你讀懂其餘文件。"
---

## 應用程式

| 詞彙 | 意義 |
| --- | --- |
| 應用程式（app） | Moonpool 管理的一個對象：開發伺服器、桌面應用程式、頁面或命令。 |
| 項目（entry） | 應用程式在 `apps.json` 中的記錄。只在談到 JSON 時使用。 |
| 應用程式列（app row） | 應用程式在側邊欄中的一列，帶有狀態圓點與控制項。 |
| 群組（group） | 應用程式在側邊欄中所屬的標題，來自它的 `group` 欄位。 |
| 類型（type） | `web`、`desktop`、`static` 或 `cli`。決定哪些欄位有意義。請參閱[應用程式類型](/zh-hant/apps/types/)。 |
| id | 應用程式的永久索引鍵，用於檔案名稱、命令與代理工具。請參閱 [id](/zh-hant/apps/apps-json/#id)。 |

## 應用程式狀態

| 狀態 | 意義 |
| --- | --- |
| 啟動中（starting） | Moonpool 已啟動應用程式，但還沒看到它就緒。圓點閃動。 |
| 執行中（Running） | 它的 `port` 有回應、它的 `processName` 存在，或是兩者都沒設定時，Moonpool 啟動的終端機仍然存活。圓點為實心。請參閱[如何判定「執行中」](/zh-hant/apps/types/#如何判定執行中)。 |
| 已停止（stopped） | 以上都不是。圓點為灰色。 |
| 受管理（managed） | 由 Moonpool 在本次工作階段中啟動。執行中但不受管理的應用程式是以其他方式啟動的，結束 Moonpool 時不會動它。 |

終端機分頁與執行中的應用程式是兩回事。按一下應用程式的名稱只會開啟它的終端機分頁，絕不會啟動應用程式。關閉分頁也絕不會停止應用程式。

## 視窗與組成部分

| 詞彙 | 意義 |
| --- | --- |
| hub | 常駐的 Moonpool 處理程序及其主視窗。工具名稱把它稱為 "launcher"（啟動器）。 |
| 主視窗（hub window） | 主視窗：左側是側邊欄，右側是 CLI 面板。 |
| 系統匣（tray） | 系統匣圖示及其選單（**顯示 Moonpool**、**結束**）。 |
| 側邊欄（sidebar） | 主視窗的左側：篩選方塊、**...** 選單與應用程式列。 |
| CLI 面板（CLI pane） | 主視窗的右側，容納終端機分頁。 |
| 終端機分頁（terminal tab） | CLI 面板中某個應用程式的終端機。 |
| MCP 子列（MCP sub-row） | 應用程式下方一個變暗的列，顯示它自己的 `<exe> mcp` 輔助處理程序。 |
| 應用程式編輯器（app editor） | 「新增應用程式」與「編輯應用程式」對話方塊。 |

## 檔案與資料夾

| 詞彙 | 意義 |
| --- | --- |
| 設定資料夾（config folder） | 存放 `apps.json` 與 Moonpool 其他檔案的資料夾。即 `{MP_DATA}` 權杖。請參閱[設定位於何處](/zh-hant/apps/apps-json/#設定位於何處)。 |
| `{MP_HOME}` | Moonpool 資料夾：安裝版是 `%USERPROFILE%\.moonpool`，可攜副本是它的 `.moonpool\` 資料夾，Linux 上是設定資料夾。 |
| 工作階段（session） | hub 從啟動到結束的一次執行。 |
| 工作階段記錄（session log） | 保存應用程式在一次工作階段中印出的全部內容的檔案，位於 `cli-output\` 下。請參閱[記錄](/zh-hant/data/logs/)。 |
| `moonpool.log` | Moonpool 自己的偵錯記錄，只有在 **將除錯資訊寫入檔案** 開啟時才會寫入。 |
| 傾印檔（dump） | 由 `dump` 動詞產生的工作階段記錄的純文字副本。 |
| 快照（snapshot） | `apps.json.history\` 中一份良好 `apps.json` 的副本。請參閱[備份與復原](/zh-hant/data/backup-and-recovery/)。 |

## 模式

| 詞彙 | 意義 |
| --- | --- |
| 安裝版（installed） | 位於 `%USERPROFILE%\.moonpool` 的 Moonpool，帶有「開始」功能表捷徑與「新增或移除程式」項目。僅限 Windows。 |
| 可攜版（portable） | 位於你所選 `.moonpool\` 資料夾中的 Moonpool，以一個 `moonpool.portable` 檔案為標記。請參閱[可攜模式](/zh-hant/data/portable-mode/)。 |
| 副本（copy） | 一個 Moonpool 資料夾，安裝版或可攜版皆可。每個副本各自執行。 |

## 停止與自動化

| 詞彙 | 意義 |
| --- | --- |
| `killMode` | 「停止」在結束應用程式終端機之後所做的額外步驟。請參閱[停止與重新啟動](/zh-hant/apps/stop-and-restart/)。 |
| `stopCommand` | `killMode` 為 `command` 時「停止」所執行的命令。 |
| `processName` | Moonpool 監看的處理程序名稱，也是 `processName` 模式下要結束的處理程序。 |
| 控制通道（control channel） | hub 回應所用的具名管道（Windows）或 Unix 通訊端（Linux、macOS）。請參閱[控制動詞](/zh-hant/automation/control-verbs/)。 |
| 動詞（verb） | 在命令列或控制通道上給出的命令字，例如 `launch` 或 `reload`。 |
| ticket | 用 `--ticket` 附加的索引鍵，用來從 `state.json` 讀取某條命令的結果。 |
| 權杖（token） | `apps.json` 的版本標記，設定寫入必須帶上它。 |
| MCP 輔助程式（shim） | AI 主機程式為存取應用程式自己的工具而啟動的 `<exe> mcp` 處理程序。 |
