---
title: "apps.json の全フィールド: 型、既定値、動作"
description: "apps.json のエントリーの全キーを、型、既定値、使われるアプリの種類とともに調べられます。アプリを編集するダイアログの名前と一致しています。"
---

アプリを編集するダイアログには、同じフィールドが同じ名前で表示されます。選択した type に当てはまらないフィールドはダイアログで薄く表示されますが、それでも保存されます。例外は `stopCommand` で、`killMode` が `command` のときだけ保存されます。

![アプリを編集するダイアログの name から stopCommand まで。killMode の選択が枠線で強調され、processName や stopCommand などの未使用のフィールドは薄く表示されている](../../../../assets/screenshots/edit-app-dialog.png)

1. `killMode` の選択。それが使わないフィールドは、薄く表示されたままになります。

| フィールド | 型 | 必須 | 使う type | 動作 |
| --- | --- | --- | --- | --- |
| `id` | 文字列 | はい | すべて | 一意のキー。英数字、`.`、`_`、`-`。`-` で始めることはできません。[概要](/ja/apps/apps-json/#id)を参照してください。 |
| `name` | 文字列 | はい | すべて | サイドバーのラベル。空白は不可。 |
| `group` | 文字列 | はい | すべて | アプリが一覧表示されるサイドバーの見出し。手作業の編集では空白は不可で、ダイアログは空のグループを `Apps` として保存します。任意のテキストを使え、新しい名前は新しいグループを作ります。 |
| `type` | 文字列 | はい | すべて | `web`、`desktop`、`static`、`cli` のいずれか。[アプリの種類](/ja/apps/types/)を参照してください。 |
| `command` | 文字列 | `static` 以外は必須 | すべて | アプリを起動するためにターミナルで実行します。Windows では `cmd /c`、それ以外では `$SHELL -c` を通します (`SHELL` が未設定なら `/bin/sh`)。`static` では省略可能です。 |
| `cwd` | 文字列 | いいえ | `command` のあるすべて | コマンドを実行するフォルダー。既定は Moonpool 自身の作業フォルダーです。トークンと `./` を使えます。[パスと環境変数](/ja/apps/paths-and-environment/)を参照してください。 |
| `port` | 整数、1 から 65535 | いいえ | 任意 | ローカルホストのこのポート (IPv4 または IPv6) で何かが応答している間は実行中になります。`killMode` の `port` が読み取ります。 |
| `processName` | 文字列 | いいえ | 任意、主に `desktop` | この名前のプロセスが存在する間は実行中になります。大文字小文字を区別せず、`.exe` の有無を問わないので、`my-app` は `my-app.exe` に一致します。Linux では 15 文字以内です。`killMode` の `processName` が読み取ります。 |
| `mcpProcessName` | 文字列 | いいえ | `processName` のある任意の type | このアプリの MCP サーバーのプロセス名のワイルドカードパターン。`*` は任意の文字列、`?` は 1 文字に一致します。大文字小文字を区別せず、名前全体に一致し、`.exe` は省略できます。一致するプロセスは、そのアプリの MCP サーバー (サイドバーの MCP の子項目) として扱われ、最初の引数が `mcp` である必要はありません。[mcpProcessName](#mcpprocessname)を参照してください。 |
| `url` | 文字列 | `static` のみ | `web`、`static` | 開くページ。開かれるのは、`http://`、`https://`、`mailto:`、`file://` の URL だけです。 |
| `openBrowser` | 真偽値、既定は `false` | いいえ | `url` のある任意の type (ダイアログでは `desktop` と `cli` は薄く表示) | Moonpool がアプリの立ち上がりを検出したら、`url` を自動的に開きます (下記参照)。 |
| `killMode` | 文字列 | いいえ | すべて | 停止と再起動時の追加の後始末: `processName`、`port`、`command`、`none`。[停止と再起動](/ja/apps/stop-and-restart/)を参照してください。 |
| `stopCommand` | 文字列 | いいえ | `killMode` が `command` | 停止時に実行するコマンド。ほかのモードでは無視されます。 |
| `env` | 文字列のオブジェクト | いいえ | すべて | 追加の環境変数。ダイアログでは、1 行に 1 つの `KEY=VALUE` として編集します。 |
| `icon` | 文字列 | いいえ | すべて | サイドバーの画像: ファイルパス、`http(s)` の URL、または `data:` URI。アプリのコンテキストメニューの **アイコンを設定...** か、手作業で設定します。 |
| `note` | 文字列 | いいえ | すべて | サイドバーでアプリにカーソルを合わせたときのツールチップ。 |

`env` と `killMode` を使ったエントリーの例です。

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

`port` と `killMode` の連携については、[ポートを使っているプロセスを見つけて終了する](/ja/guides/find-and-kill-process-using-port-windows/)を参照してください。

## mcpProcessName

既定では、Moonpool は、プロセスの名前が `processName` に一致し、最初の引数が `mcp` であるとき (たとえば `notes-app.exe mcp`)、そのプロセスをアプリの MCP サーバーとみなします。サーバーが別の名前で動く場合は、`mcpProcessName` を設定します。たとえば、アプリは 1 つの exe を監視し、MCP サーバーは別の exe (`mog.exe mcp`) である場合や、サーバーの名前を変えたコピーの場合です。

値はワイルドカードパターンです。`*` は任意の文字列 (空も含む)、`?` はちょうど 1 文字に一致します。プロセス名の全体に対して大文字小文字を区別せず比較され、`.exe` のないパターンは `.exe` 付きの名前にも一致します。空の値は未設定として扱われます。

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

これは、`destiny-mcp-2706210170.exe` のような、名前を変えたコピーに一致します。`mcpProcessName` に一致するプロセスは、`mcp` 付きで起動されたかどうかにかかわらずサーバーとみなされ、アプリ自体が実行中であるとは数えられません。パターンが `processName` 自体にも一致する場合 (たとえば `destiny*`)、Moonpool は引き続き `mcp` 引数を要求するので、本物のアプリが MCP サーバーと間違われることはありません。[MCP のセットアップ](/ja/automation/mcp-setup/#独自の-mcp-サーバーを持つアプリ)を参照してください。

## openBrowser

Moonpool が起動したアプリが初めて「実行中」と判定されたときに、`url` を 1 回だけ開きます。それを検出するには、`port` か `processName` が必要です。どちらもない場合、「実行中」は単にターミナルのプロセスが生きていることを意味するだけで、ブラウザーは自動では開きません。コマンド自身がブラウザーを開く場合は、`openBrowser` をオフにしてください。コマンドのない `static` エントリーは、`openBrowser` にかかわらず、起動を押すたびに `url` を開きます。

同じ `port` が設定された 2 つのアプリは、サイドバーで警告されます。

## アイコン

アプリのアイコンは、次のうち最初に存在するものです。

1. `icon` フィールド。
2. 設定フォルダーの `icons\<id>.<ext>`。たとえば `icons\site.png`。
3. アプリ自身のフォルダー (`cwd`、または `file:///` の `url` のフォルダー) にあるアイコンファイル。
4. `desktop` では、ビルド済みまたは実行中の `.exe` のアイコン。
5. `web` と `static` では、サーバーが立ち上がったあとのサイトの `/favicon.ico`。
6. type を表す図形。

ほとんどのアプリでは、アイコンの設定は不要です。
