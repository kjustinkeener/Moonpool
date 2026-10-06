---
title: "アプリでパス、MP_HOME トークン、環境変数を使う"
description: "アプリのエントリーで {MP_HOME} と {MP_DATA} のトークンや相対パス ./ を使い、どのフィールドで展開されるかを確認します。env と作業フォルダーの設定も説明します。"
---

## トークン

| トークン | 展開される値 |
| --- | --- |
| `{MP_HOME}` | ポータブル: `moonpool.exe` があるフォルダー (`.moonpool\` フォルダー)。Windows のインストール版: `%USERPROFILE%\.moonpool`。Linux: `$XDG_CONFIG_HOME/Moonpool`、なければ `~/.config/Moonpool`。`{MP_DATA}` と同じフォルダーです。 |
| `{MP_DATA}` | 設定フォルダー。`apps.json` があるフォルダーです。 |

解決できないトークンは、書かれたままになります。

## 展開されるフィールド

| フィールド | トークン | 先頭の `./` または `.\` |
| --- | --- | --- |
| `cwd` | はい | はい。`{MP_HOME}` を基準にします |
| `command` | はい | いいえ |
| `stopCommand` | はい | いいえ (基準が決まっている `cwd` で実行されます) |
| `url` | はい | いいえ |
| `icon` | はい | はい。`{MP_HOME}` を基準にします |
| `env` の値、`processName`、`note` | いいえ | いいえ |

`./` のない相対パス (`apps\tool` など) は、そのまま扱われ、Moonpool 自身の作業フォルダーを基準に解決されます。これはほとんどの場合、意図したものではありません。`./` かトークンを使うのがお勧めです。

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

どちらの形式も、ポータブルフォルダーを移動しても動作し続けます。`C:\tools\notes` のような固定のパスは、持ち運べません。ポータブルモードでは、アプリを編集するダイアログが、絶対パスの `cwd` と `url` の値に「持ち運び不可」のバッジを付けます。[ポータブルモード](/ja/data/portable-mode/)を参照してください。

## 環境変数

`env` は、文字列のオブジェクトです。ダイアログでは、1 行に 1 つの `KEY=VALUE` として編集します。各行を最初の `=` で分割し、両側の空白を取り除き、`=` のない行は無視します。

ダイアログでは次のようになります。

```text
PORT=8091
NODE_ENV=development
```

`apps.json` では、エントリーの `env` キーとして次のようになります。

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- 起動されたコマンドは、Moonpool の環境変数に `env` を加えたものを継承します。`env` のエントリーが優先されます。
- `env` は `stopCommand` にも適用されます。
- 値は書かれたとおりに使われます。Moonpool が `{MP_HOME}` や `%VAR%` を展開することはありません。
- Moonpool 自身の WebView2 は、`WEBVIEW2_USER_DATA_FOLDER` を通して専用のプロファイルフォルダーを使います。起動されたアプリは、それを継承しません。Moonpool を起動する前に自分でこの変数を設定していた場合は、その値が渡され、そうでなければ未設定になります。`env` のエントリーで上書きすることもできます。

## 作業フォルダー

コマンドと `stopCommand` は、`cwd` で実行されます。`cwd` を省略すると、コマンドは Moonpool 自身の作業フォルダーで実行されるので、相対パスを使うものには `cwd` を設定してください。
