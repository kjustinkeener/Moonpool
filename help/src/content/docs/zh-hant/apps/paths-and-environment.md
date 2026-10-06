---
title: "在應用程式中使用路徑、MP_HOME 權杖與環境變數"
description: "在應用程式項目中使用 {MP_HOME} 與 {MP_DATA} 權杖以及 ./ 相對路徑，了解哪些欄位會展開它們，並設定 env 與工作資料夾。"
---

## 權杖

| 權杖 | 展開為 |
| --- | --- |
| `{MP_HOME}` | 可攜模式：存放 `moonpool.exe` 的資料夾（即 `.moonpool\` 資料夾）。Windows 安裝版：`%USERPROFILE%\.moonpool`。Linux：`$XDG_CONFIG_HOME/Moonpool`，否則為 `~/.config/Moonpool`，與 `{MP_DATA}` 是同一個資料夾。 |
| `{MP_DATA}` | 設定資料夾，也就是存放 `apps.json` 的那個資料夾。 |

無法解析的權杖會原樣保留。

## 哪些欄位會展開

| 欄位 | 權杖 | 開頭的 `./` 或 `.\` |
| --- | --- | --- |
| `cwd` | 是 | 是，以 `{MP_HOME}` 為基準 |
| `command` | 是 | 否 |
| `stopCommand` | 是 | 否（它在 `cwd` 中執行，而 `cwd` 已有基準） |
| `url` | 是 | 否 |
| `icon` | 是 | 是，以 `{MP_HOME}` 為基準 |
| `env` 的值、`processName`、`note` | 否 | 否 |

不帶 `./` 的相對路徑（例如 `apps\tool`）不會被處理，而是相對於 Moonpool 自己的工作資料夾解析，這很少是你想要的。請優先使用 `./` 或權杖。

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

搬動可攜資料夾後，這兩種寫法都仍然有效。像 `C:\tools\notes` 這樣的固定路徑則無法跟著搬動。在可攜模式下，「編輯應用程式」對話方塊會替絕對路徑的 `cwd` 與 `url` 值加上「不可攜」標記。請參閱[可攜模式](/zh-hant/data/portable-mode/)。

## 環境

`env` 是一個由字串組成的物件。對話方塊以每行一個 `KEY=VALUE` 的形式編輯它；它在每行的第一個 `=` 處拆開，去掉兩側空白，並忽略不含 `=` 的行。

在對話方塊中：

```text
PORT=8091
NODE_ENV=development
```

在 `apps.json` 中，作為項目的 `env` 索引鍵：

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- 被啟動的命令會繼承 Moonpool 的環境，再加上 `env`。`env` 中的項目優先。
- `env` 同樣會套用到 `stopCommand`。
- 值會照原樣使用：Moonpool 不會展開 `{MP_HOME}`，也不會展開 `%VAR%`。
- Moonpool 透過 `WEBVIEW2_USER_DATA_FOLDER` 把它自己的 WebView2 指向一個私有的設定檔資料夾。被啟動的應用程式不會繼承它。如果你在啟動 Moonpool 之前自己設定過這個變數，它們取得的是你的值；否則它是未設定的。`env` 項目仍然可以覆寫它。

## 工作資料夾

命令與 `stopCommand` 都在 `cwd` 中執行。省略 `cwd` 時，命令會在 Moonpool 自己的工作資料夾中執行，所以只要涉及相對路徑，就請設定 `cwd`。
