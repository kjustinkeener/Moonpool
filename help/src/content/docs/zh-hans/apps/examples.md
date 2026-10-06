---
title: "可直接复制的 apps.json 示例：常见的应用配置"
description: "开发服务器、桌面应用、静态页面、CLI 工具、Docker Compose 和便携应用的完整有效 apps.json 条目，可复制后改用。"
---

每个代码片段是一个条目。把它们放进 `apps.json` 顶层数组中，用逗号分隔。请把 id、名称和路径改成你自己的配置。

## Web 开发服务器

只要端口 5173 有响应就算运行中。响应后会打开浏览器。停止时也会释放该端口，这是 `web` 的默认行为。

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

## 从环境变量读取端口的 Web 应用

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

## 桌面应用

只要存在名为 `notes-app` 的进程就算运行中。停止时按名称结束该进程。

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

## 已经托管好的静态页面

没有终端。启动会打开该页面。

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## 由命令提供服务的静态文件夹

Moonpool 在终端中运行该服务器，按端口跟踪它，并在它有响应时打开页面。

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

在终端标签页中运行。`-NoExit` 会让脚本结束后 shell 保持打开。

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

`command` 本身就会重新创建容器然后退出，所以“运行中”由端口决定。停止时运行 `stopCommand`，而不是结束占用端口的进程（在 Windows 上那会是 Docker Desktop）。参见 [Windows 上的 Docker 应用](/zh-hans/apps/stop-and-restart/#windows-上的-docker-应用)。

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

如果你希望停止应用时让容器继续运行，请使用 `"killMode": "none"` 并去掉 `stopCommand`。

## 便携应用

路径以便携文件夹为基准，所以文件夹被搬动后，这个条目仍然有效。

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

## 另请参阅

- [示例仪表盘](/zh-hans/getting-started/example-dashboards/)：随 Moonpool 一起提供的仪表盘。
- [应用字段](/zh-hans/apps/fields/)
- [在 Windows 上让 npm 开发服务器在后台运行](/zh-hans/guides/run-npm-dev-server-in-background-windows/)
- [在 Windows 上让 Python 脚本在后台持续运行](/zh-hans/guides/keep-python-script-running-background-windows/)
