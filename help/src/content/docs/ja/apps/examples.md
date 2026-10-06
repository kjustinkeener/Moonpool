---
title: "よくあるアプリ構成の apps.json サンプル (コピーして使えます)"
description: "開発サーバー、デスクトップアプリ、静的ページ、CLI ツール、Docker Compose、ポータブルアプリの、完全で有効な apps.json エントリーを、コピーして調整できる形で紹介します。"
---

各スニペットは 1 つのエントリーです。`apps.json` の最上位の配列の中に、カンマで区切って入れてください。id、名前、パスは、ご自分の環境に合わせて変更してください。

## Web 開発サーバー

ポート 5173 が応答している間は実行中になります。応答したらブラウザーが開きます。停止するとポートも解放されます。これは `web` の既定の動作です。

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

## ポートを環境変数から読む Web アプリ

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

## デスクトップアプリ

`notes-app` という名前のプロセスがある間は実行中になります。停止すると、そのプロセスを名前で終了します。

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

## すでにホストされている静的ページ

ターミナルはありません。起動するとページが開きます。

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## コマンドで配信する静的フォルダー

Moonpool がターミナルでサーバーを実行し、ポートで追跡して、応答したらページを開きます。

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

## CLI ツール

ターミナルタブで実行します。`-NoExit` により、スクリプトが終わってもシェルは開いたままになります。

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

`command` はコンテナーを再作成してすぐに終了するので、「実行中」はポートで判断します。停止では、ポートの所有者を終了する代わりに `stopCommand` を実行します。Windows では、その所有者は Docker Desktop になってしまうためです。[Windows 上の Docker アプリ](/ja/apps/stop-and-restart/#windows-上の-docker-アプリ)を参照してください。

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

アプリを停止してもコンテナーを動かしたままにしたい場合は、`"killMode": "none"` を使い、`stopCommand` を削除してください。

## ポータブルアプリ

パスはポータブルフォルダーを基準にするので、フォルダーを移動してもエントリーは動作します。

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

## 関連ページ

- [サンプルダッシュボード](/ja/getting-started/example-dashboards/): Moonpool に同梱されているダッシュボード。
- [アプリのフィールド](/ja/apps/fields/)
- [Windows で npm の開発サーバーをバックグラウンドで実行する](/ja/guides/run-npm-dev-server-in-background-windows/)
- [Windows で Python スクリプトをバックグラウンドで動かし続ける](/ja/guides/keep-python-script-running-background-windows/)
