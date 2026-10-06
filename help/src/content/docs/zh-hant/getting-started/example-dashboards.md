---
title: "試用 Moonpool 隨附的範例儀表板"
description: "開啟隨程式附帶的離線範例儀表板，了解它們存放的位置、範例應用程式如何參照它們，以及如何把它們加入既有的設定中。"
---

Moonpool 在程式內附帶了一組獨立的儀表板。它們完全離線執行，不需要伺服器，也不依賴 CDN。

| 儀表板 | 說明 |
| --- | --- |
| CSV explorer | 拖入 CSV 或 TSV 檔案，它會分析各欄並顯示資料。 |
| JSON explorer | 拖入 JSON（陣列、巢狀物件或對應表）。 |
| Excel explorer | 拖入 `.xlsx` 或 `.xls` 檔案，離線剖析。 |
| Moonpool Docs | 離線的 Markdown 文件瀏覽器。 |

## 它們存放在哪裡

Moonpool 會在啟動時把這些儀表板寫入 `{MP_HOME}\dashboards\examples`：

| 模式 | 資料夾 |
| --- | --- |
| 已安裝（Windows） | `%USERPROFILE%\.moonpool\dashboards\examples` |
| 可攜 | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples`（或 `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`） |

`examples` 資料夾歸 Moonpool 所有：每次 Moonpool 更新時它都會被取代，因此你在裡面做的修改會遺失。若要自訂某個儀表板，請把它的資料夾與共用的 `_lib` 資料夾複製到上一層的 `dashboards` 中，再讓你的應用程式指向這份副本。Moonpool 絕不會更動 `dashboards` 中的其他任何內容。

0.3.16 之前的版本會把範例直接寫入 `dashboards`。這些副本會留在原處，不再取得更新；指向它們的應用程式仍可繼續使用。若想取得更新後的版本，請把它們的 `url` 改成下面的 `dashboards/examples/...` 路徑。

## 應用程式如何參照它們

每個都是一個 `static` 應用程式，它的 `url` 是以 `{MP_HOME}` 為基準的 `file:///` 網址：

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` 會解析為安裝資料夾；在可攜模式下則是整個可攜包所在的資料夾，所以這個項目在可攜包被搬動之後依然有效。`file://` 網址是允許使用的。請參閱[路徑與環境](/zh-hant/apps/paths-and-environment/)。

## 範例應用程式只在第一次執行時出現

只有在尚未存在設定檔時，範例項目才會被寫入 `apps.json`。如果你已經有 `apps.json`，請自行加入這些儀表板項目（在「...」選單中選擇 **編輯 apps.json**，然後選擇 **重新載入**）。請把下面這四項加到最外層的陣列中，並用逗號與其他項目隔開：

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

各欄位的意義請見[應用程式欄位](/zh-hant/apps/fields/)。

## 另請參閱

- [範例](/zh-hant/apps/examples/)：更多可直接複製的完整項目。
- [應用程式類型](/zh-hant/apps/types/#static)
