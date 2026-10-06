---
title: "從 USB 隨身碟或同步資料夾執行 Moonpool"
description: "把 Moonpool 與它所有的資料放進一個可搬動的資料夾，這樣你可以放在 USB 隨身碟上隨身攜帶或同步，並且可以並排執行多個副本。"
---

可攜模式把 Moonpool 與它寫入的所有內容都放在一個 `.moonpool\` 資料夾裡，所以你可以把它放在 USB 隨身碟上隨身攜帶，或放進同步資料夾，在任何電腦上執行。

## 運作方式

選擇可攜安裝時，Moonpool 會在你選的位置內建立一個 `.moonpool\` 資料夾。這個資料夾裡有程式本體、你的設定與它的說明內容。不會往 Windows 的 AppData 寫入任何東西，所以搬動或複製這個資料夾，就等於把你的整套設定一起帶走。

```text
<chosen location>\.moonpool\
```

## 與安裝版有何不同

| | 安裝版 | 可攜版 |
| --- | --- | --- |
| 程式 | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| 設定資料夾 | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| 視窗的瀏覽器設定檔、視窗大小與位置 | 在設定資料夾中 | 在設定資料夾中，所以它們也會一起搬動 |
| 「開始」功能表、桌面捷徑、「新增或移除程式」項目 | 有 | 無 |
| 更新 | 取代它自己的執行檔 | 同樣，在 `.moonpool\` 資料夾內進行。請參閱[更新](/zh-hant/data/updating/#可攜副本)。 |
| 移除 | 「新增或移除程式」或 `--uninstall` | 刪除該資料夾 |

兩種模式都不會寫入 Windows 的 AppData。

### 同步資料夾

你可以把可攜副本放在同步資料夾（OneDrive、Dropbox 之類）裡，但同一時間只能在一台電腦上執行它。Moonpool 每隔幾秒就會寫入 `state.json`，應用程式執行時也會寫記錄，所以兩台電腦執行同一個資料夾會爭用同樣的檔案，同步衝突還可能留下損毀的 `apps.json`。請先在一台電腦上結束它，再在另一台上啟動。

## 同時執行多個副本

每個資料夾只執行一個 Moonpool。已安裝的 Moonpool 與任意數量的可攜副本（各自在自己的資料夾中）可以同時執行，而且彼此完全獨立：各有自己的應用程式、系統匣圖示、視窗、設定、記錄與[控制通道](/zh-hant/automation/control-verbs/)。

- 系統匣提示文字與工作列名稱會說明哪個是哪個副本：安裝版是 `Moonpool`，可攜版是 `Moonpool (<folder>)`，其中 `<folder>` 是你選的資料夾（包含 `.moonpool\` 的那個）。
- 再次啟動同一個副本，會把它的視窗叫回來，而不是再開啟一個。啟動另一個副本，則會開啟那個副本。
- 要讓 AI 代理使用多個副本，請用各自的名稱分別登錄；請參閱 [MCP 設定](/zh-hant/automation/mcp-setup/#多個-moonpool)。
- 搬動或重新命名可攜資料夾會讓它取得新的識別（新的控制通道名稱）。搬動之前請先結束它。
- 各副本並不知道彼此的應用程式。兩個副本如果都在同一個連接埠上啟動同一個伺服器，仍會衝突；而依處理程序名稱或連接埠運作的停止，可能會結束另一個副本啟動的東西，請參閱[停止與重新啟動](/zh-hant/apps/stop-and-restart/#多個-moonpool或你自己的處理程序)。

## 讓你的應用程式也能隨身攜帶

在應用程式的路徑中使用 `{MP_HOME}` 權杖，讓它指向可攜資料夾內部，而不是某台電腦上的固定位置。在可攜副本中，`{MP_HOME}` 是存放 `moonpool.exe` 的資料夾，也就是 `.moonpool\` 資料夾本身，而不是你選的那個資料夾：

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

這裡的 `{MP_HOME}/my-app` 就是 `<chosen location>\.moonpool\my-app`。以 `./` 開頭的路徑也以同樣方式定位。權杖與 `./` 路徑在已安裝的 Moonpool 中同樣有效。路徑如何解析，請參閱[路徑與環境](/zh-hant/apps/paths-and-environment/)。

## 從安裝程式選擇可攜模式

可攜模式是從安裝卡片中設定的，卡片上在 **安裝 Moonpool** 旁邊提供 **安裝可攜版**。

![安裝卡片：「安裝可攜版」連結位於主按鈕「安裝 Moonpool」下方](../../../../assets/screenshots/installer-window.png)

選一個資料夾，Moonpool 就會在那裡建立 `.moonpool\` 資料夾，把自己複製進去，並用全新的設定啟動這個新副本。

該卡片也在「...」選單中，名為 **安裝 Moonpool...**，已安裝模式與可攜模式下都有。從那裡使用 **安裝可攜版**，會讓正在執行的 Moonpool 結束，並由新的可攜副本取而代之啟動。你啟動它時用的那個 Moonpool 仍留在原處，所以之後可以再次啟動它。

可攜副本從全新狀態開始，不會複製你既有的應用程式。要把它們帶過去，請先結束可攜副本，再手動複製 `apps.json`：

| | 路徑 |
| --- | --- |
| 來源（安裝版） | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| 目的地（可攜版） | `<chosen location>\.moonpool\moonpool-config\apps.json` |

使用絕對路徑的項目在同一台電腦上仍然有效，但無法跟著搬動。「編輯應用程式」對話方塊會把它們標示為「不可攜」。

## Moonpool 如何知道自己是可攜版

只要 `moonpool.exe` 旁邊有一個名為 `moonpool.portable` 的檔案，這個副本就是可攜版。沒有別的標記，也沒有任何東西登錄到 Windows。

要移除可攜副本，請先結束它，再刪除它的 `.moonpool\` 資料夾。`--uninstall` 只會移除已安裝的 Moonpool，絕不會移除可攜副本。
