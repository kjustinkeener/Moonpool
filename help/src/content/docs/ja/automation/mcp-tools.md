---
title: "Moonpool MCP ツールのリファレンス: パラメーターと結果"
description: "Moonpool の MCP サーバーがエージェント向けに公開するすべてのツールについて、パラメーター、戻り値、遭遇しうるエラーを説明します。"
---

すべてのツールはテキストを返します。ただし `moonpool_screenshot` は PNG 画像を返します。失敗した場合は、エラーとして印が付いたツール結果として、理由がテキストで返ります。セットアップについては [MCP のセットアップ](/ja/automation/mcp-setup/)を参照してください。

`app_id` を取るツールには、`apps.json` にあるアプリの `id` が必要です。使えるのは英字、数字、`.`、`_`、`-` だけで、先頭を `-` にしてはいけません。そうでない場合、呼び出しは "invalid app_id" で失敗します。

ハブに対して動作するツールの多くは、ハブが実行中でないと次のメッセージで失敗します。`moonpool_bootup_launcher`、`moonpool_shutdown_launcher`、`moonpool_raise_launcher`、`moonpool_launcher_paths` は、この場合を自分で処理します(それぞれの行を参照)。ポータブル版では、メッセージにコピーの名前が入ります。例: `Moonpool (<folder>)`。

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

結果を待つ呼び出しは、45 秒でタイムアウトします。

## ランチャーとアプリ

`moonpool_list_apps` の結果の例:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| ツール | パラメーター | 動作 |
| --- | --- | --- |
| `moonpool_list_apps` | なし | アプリごとに 1 行。`id  [running]` または `[stopped]`、該当すれば `(managed by Moonpool)`、MCP ヘルパーが検出されていれば `[mcp: running]` または `[mcp: stopped]`、続けて名前が出ます。制御チャネル(`list` 動詞)を通じて実行中のハブに問い合わせるため、常に最新です。Moonpool が実行中でなければ、古い一覧は表示せず "Moonpool is not running" で失敗します。Moonpool の起動直後で最初の状態確認の前は、アプリは `[status pending]` と表示されます。`apps.json` にエラーがある間、結果は `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` で始まります。Moonpool の起動時点ですでにファイルが壊れていた場合は、アプリが読み込まれていないことが示され、`moonpool_restore_config` も提案されます。 |
| `moonpool_bootup_launcher` | なし | Moonpool 自体を起動し、制御チャネルが応答するまで最大 30 秒待ちます。"Moonpool started"、または "Moonpool is already running" を返します。新しいプロセスがすぐに終了した場合(まだ終了処理中の Moonpool に処理を引き渡した場合)は、もう一度起動します。何かがチャネルを保持しているのに応答しない場合は、Moonpool プロセスがハングしている可能性があると報告します。 |
| `moonpool_shutdown_launcher` | なし | トレイメニューの「終了」と同じです。制御チャネルがなくなるまで最大 30 秒待ちます。"Moonpool shut down"、または "Moonpool is not running" を返します。 |
| `moonpool_raise_launcher` | なし | Moonpool のウィンドウを前面に表示します。"window shown" を返します。Moonpool が実行中でなければ、起動して "Moonpool was not running; started it" を返します。 |
| `moonpool_start_app` | `app_id`(必須) | アプリを起動し、そのターミナルタブを開きます。実行中になった時点で "launched" を返し、そうならなかった場合はその理由を返します(`unknown app id: <id>`、25 秒後の `did not reach running in time`)。`url` だけを持つ `static` エントリでは、ページを開き、同じく "launched" を返します。 |
| `moonpool_stop_app` | `app_id`(必須) | アプリを停止します。"stopped" を返すか、`still running after stop`(15 秒後)のようなエラーを返します。 |
| `moonpool_restart_app` | `app_id`(必須) | 停止し、ポートとプロセスが解放されるのを待ってから起動します。"restarted" を返します。 |
| `moonpool_app_output` | `app_id`(必須)、`tail_lines`(整数、既定値 200、最小 1) | 現在の Moonpool セッションにおけるアプリのターミナル出力で、ANSI コードは取り除かれています。ログが `tail_lines` より長い場合、テキストは完全なログのパスを示す 1 行で始まります。アプリが実行されていなければ、`no console output recorded for '<id>' (not launched this session)` で失敗します。ログはあるが空の場合は、`(no output recorded for '<id>')` を返します。 |
| `moonpool_stop_mcp_server` | `app_id`(必須) | アプリに接続されている MCP ヘルパープロセスを終了させ、アプリは実行したままにします。"stopped" を返します。アプリに `processName` も `mcpProcessName` もない場合は何もしません。 |
| `moonpool_refresh_app_icons` | なし | すべてのアプリアイコンを再取得します。"icons refreshed" を返します。 |

## 設定

これらはハブを通じて `apps.json` を読み書きし、ディスク上のファイルを直接操作することはありません。書き込みには直前の読み取りで得たトークンが必要で、古いトークンは拒否され、何かを書き込む前に新しいファイルが検証されます。ハブを経由するのは、サンドボックス化されたホストのエージェントには、本物の設定フォルダーではなく非公開コピーが見えている場合があるためです。

| ツール | パラメーター | 動作 |
| --- | --- | --- |
| `moonpool_read_config` | なし | `manifest_text`(ファイルの内容そのまま)、`token`、`valid`、`error`(有効な場合は null)、`path` を含む JSON テキスト。ファイルがないか空の場合、`token` は `none` です。 |
| `moonpool_write_config` | `manifest`(必須、新しい `apps.json` の全文)、`expected_token`(必須、直前の読み取りで得たもの) | マニフェストを検証して `apps.json` を置き換え、読み込みます。`apps.json updated; new version token <token>` を返します。古いトークンは `stale token: apps.json changed since it was read ...` で失敗します。無効なマニフェストは `rejected invalid manifest: ...` で失敗します。どちらの場合もファイルは変更されません。空の `expected_token` は拒否されます。 |
| `moonpool_restore_config` | `snapshot`(省略可) | 値なしでは、保存されたスナップショットを新しい順に並べた JSON テキスト(`index`、`filename`、`millis`、`app_count`、`valid`)を返します。インデックス(1 が最新)またはファイル名を指定すると、そのスナップショットを検証して復元します。`restored <file> (<n> apps); new version token <token>` を返します。トークンは不要です。復元は意図的に現在のファイルを上書きします。 |
| `moonpool_reload_config` | なし | `apps.json` を再読み込みします。"apps.json reloaded" を返します。ファイルの解析または検証に失敗した場合は `apps.json has an error: ...` で失敗し、Moonpool は最後に読み込めた一覧を保持します。 |
| `moonpool_launcher_paths` | なし | ハブの設定フォルダー、`apps.json`、`state.json`、ログ、ダンプフォルダー、アイコンフォルダー、ポータブルフラグ、exe のパスを一覧表示し、続けて MCP プロセスの設定フォルダー、`apps.json`、`state.json`、ダンプフォルダー、ポータブルフラグ、exe のパス(ログとアイコンはなし)を表示します。ハブが実行中でなければ、その半分は `hub paths unavailable: ...` となり、MCP 側の半分は引き続き表示されます。編集が反映されないときに使います。 |

## 上級: テスト用ツール

`moonpool_screenshot` は Windows 専用で、Linux と macOS では "screenshot is not supported on this platform" で失敗します。`moonpool_window_state` と `moonpool_reset_mcp_seen` は、すべてのプラットフォームで動作します。

`window` は `main`、`settings`、`about`、`installer`、`editor`、`help`、`themes` のいずれかで、既定値は `main` です。未知の名前は `unknown window '<name>'` で失敗します。

| ツール | パラメーター | 動作 |
| --- | --- | --- |
| `moonpool_screenshot` | `window`(省略可) | その Moonpool ウィンドウ自身の内容を、長辺が最大 320 ピクセルのインライン PNG として取得します。MCP からサイズを大きくすることはできません。表示されていなければ `window '<name>' is not open` で失敗します。ほかのアプリは取得できません。 |
| `moonpool_window_state` | `window`(省略可) | JSON テキスト: ウィンドウが開いていなければ `{"open":false}`、開いていれば `open`、`visible`、`minimized`、`maximized`、`x`、`y`、`width`、`height`。テスト用です。 |
| `moonpool_reset_mcp_seen` | `app_id`(省略可) | テスト専用。1 つのアプリ(省略時はすべてのアプリ)について、記憶されている「MCP ヘルパーを検出した」記録を消去します。ヘルパーが再び検出されるまで、サイドバーの MCP の子行は再び非表示になります。 |

## 関連項目

- [MCP のセットアップ](/ja/automation/mcp-setup/)
- [コマンドライン](/ja/automation/command-line/)
