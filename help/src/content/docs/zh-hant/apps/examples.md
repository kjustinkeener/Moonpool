---
title: "可直接複製的 apps.json 範例：常見的應用程式設定"
description: "開發伺服器、桌面應用程式、靜態頁面、CLI 工具、Docker Compose 與可攜應用程式的完整有效 apps.json 項目，可複製後改用。"
---

每個程式碼片段是一個項目。請把它們放進 `apps.json` 最外層的陣列中，並用逗號分隔。請把 id、名稱與路徑改成你自己的設定。

## 網頁開發伺服器

只要連接埠 5173 有回應就算執行中。回應後會開啟瀏覽器。停止時也會釋放該連接埠，這是 `web` 的預設行為。

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

## 從環境變數讀取連接埠的網頁應用程式

```json title="apps.json"
{
  "id": "habit-tracker",
  "name": "Habit Tracker",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\habits",
  "command": "python app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "openBrowser": true,
  "env": { "PORT": "8091" },
  "note": "Moved off 8000 to avoid a clash"
}
```

## 桌面應用程式

只要存在名為 `notes-app` 的處理程序就算執行中。停止時依名稱結束該處理程序。

```json title="apps.json"
{
  "id": "notes-app",
  "name": "Notes App",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "C:\\code\\notes-app",
  "command": "npm run tauri dev",
  "processName": "notes-app"
}
```

## 已經代管好的靜態頁面

沒有終端機。啟動會開啟該頁面。

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## 由命令提供服務的靜態資料夾

Moonpool 在終端機中執行該伺服器，依連接埠追蹤它，並在它有回應時開啟頁面。

```json title="apps.json"
{
  "id": "docs-site",
  "name": "Docs",
  "group": "Docs",
  "type": "static",
  "cwd": "C:\\code\\docs\\public",
  "command": "python -m http.server 8090",
  "port": 8090,
  "url": "http://localhost:8090",
  "openBrowser": true,
  "killMode": "port"
}
```

## CLI 工具

在終端機分頁中執行。`-NoExit` 會讓指令碼結束後 shell 保持開啟。

```json title="apps.json"
{
  "id": "backup",
  "name": "Backup script",
  "group": "CLI tools",
  "type": "cli",
  "cwd": "C:\\code\\scripts",
  "command": "pwsh -NoLogo -NoProfile -NoExit -Command .\\backup.ps1 -Verbose"
}
```

## Docker Compose

`command` 本身就會重新建立容器然後結束，所以「執行中」由連接埠決定。停止時執行 `stopCommand`，而不是結束佔用連接埠的處理程序（在 Windows 上那會是 Docker Desktop）。請參閱 [Windows 上的 Docker 應用程式](/zh-hant/apps/stop-and-restart/#windows-上的-docker-應用程式)。

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "docker compose up -d --build",
  "port": 8080,
  "url": "http://localhost:8080",
  "killMode": "command",
  "stopCommand": "docker compose stop app"
}
```

如果你希望停止應用程式時讓容器繼續執行，請使用 `"killMode": "none"` 並移除 `stopCommand`。

## 可攜應用程式

路徑以可攜資料夾為基準，所以資料夾被搬動後，這個項目仍然有效。

```json title="apps.json"
{
  "id": "notes",
  "name": "Notes",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes",
  "icon": "{MP_HOME}\\icons\\notes.png"
}
```

## 另請參閱

- [範例儀表板](/zh-hant/getting-started/example-dashboards/)：隨 Moonpool 一起提供的儀表板。
- [應用程式欄位](/zh-hant/apps/fields/)
- [在 Windows 上讓 npm 開發伺服器在背景執行](/zh-hant/guides/run-npm-dev-server-in-background-windows/)
- [在 Windows 上讓 Python 指令碼在背景持續執行](/zh-hant/guides/keep-python-script-running-background-windows/)
