---
title: "Moonpool の制御チャネルと動詞のリファレンス"
description: "Moonpool の制御チャネル(名前付きパイプまたは Unix ソケット)の仕組みとプロトコル、実行中のアプリが応答するすべての動詞の引数と応答を説明します。"
---

## 待ち受け先

Moonpool のコピーはそれぞれ独自のチャネルを持つため、インストール版とポータブル版を並べて実行しても、互いに応答を取り違えることはありません。Windows では、インストール版の Moonpool は名前付きパイプ `\\.\pipe\moonpool` で待ち受けます。ポータブル版では、そのフォルダーから作られた ID が付き、`\\.\pipe\moonpool-<id>` になります。

`<id>` は、そのコピーの `moonpool-config` フォルダーのパスから求めた 16 進数 8 桁です。同じフォルダーであれば再起動や更新をしても変わらず、フォルダーを移動すると変わります。`moonpool.exe mcp` を含め、あるコピーの `moonpool.exe` は、常に自分のコピーのチャネルを見つけます。

Linux と macOS では、代わりにモード `0600` の Unix ドメインソケットで待ち受けます。

| 場合 | ソケットのパス |
| --- | --- |
| 通常 | 変数 `$XDG_RUNTIME_DIR` が設定されていれば `$XDG_RUNTIME_DIR/moonpool.sock`、なければ Moonpool の設定フォルダー内の `moonpool.sock` |
| ポータブルモード | ポータブル版の設定フォルダー内の `moonpool.sock`。ポータブル版がインストール版と衝突することはありません。 |
| ソケットとしてはパスが長すぎる場合(約 100 文字) | `/tmp/moonpool-<uid>/moonpool.sock`。自分だけが開けるディレクトリに置かれます(ポータブル版は `moonpool-<id>.sock`)。 |

クラッシュで残ったソケットファイルは、次回起動時に検出されて置き換えられます。まだ何かが応答しているソケットが奪われることはありません。Moonpool が正常に終了すると、ファイルは削除されます。

このチャネルは、[MCP サーバー](/ja/automation/mcp-setup/)が Moonpool の実行状況を知る手段でもあります。`ping` に応答があれば実行中で、パイプまたはソケットがなければ実行中ではありません。後述の診断用動詞を除き、同じ動詞は[コマンドライン](/ja/automation/command-line/)からも使えます。

## プロトコル

入力は 1 行に 1 つの JSON オブジェクト、出力は 1 行の JSON で、順番どおりに処理されます。1 つの接続で多数のリクエストを送れます。

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

PowerShell からのリクエストと応答の例:

ポータブル版では、`moonpool` の代わりにそのパイプ名(`moonpool-<id>`、`paths` 動詞で確認できます)を使います。

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` は文字列のリストで、省略できます。ほかのフィールドは無視されます。
- `result` は文字列または null です。構造化データを返す動詞は、それを JSON 文字列として返します。
- 有効な JSON でない行には `{"ok": false, "error": "bad request: ..."}` が返ります。
- 未知の `cmd` には `unknown cmd: <name>` が返ります。
- ウィンドウを経由する動詞(`launch`、`stop`、`restart`、`reload`、`refresh-icons`、`help`、`open-window`)は、動作が完了した時点で応答するか、45 秒後にタイムアウトのエラーを返します。ハブウィンドウの UI が読み込まれていない場合は、`frontend not loaded` ですぐに失敗します。
- 前の Moonpool がまだ終了処理中に起動した Moonpool は、約 8 秒間、チャネルのバインドを再試行します。それでもできなければ、その旨をログに記録し、チャネルなしで実行を続けます。

## 動詞

| 動詞 | 引数 | 結果 |
| --- | --- | --- |
| `ping` | なし | `pong`。チャネル専用です。 |
| `list` | なし | 実行中のハブのメモリから読み取った JSON 文字列 `{"apps": [...], "statuses": [...]}`。`apps` と `statuses` の形式は `state.json` と同じです。アプリが登録済みで最初の状態確認がまだ行われていない場合は、`"statusNotReady": true` が加わります。`apps.json` の読み込みに失敗している間は `"manifestError": "<message>"` が加わり(このときのアプリは最後に読み込めた一覧です)、起動後に一度も一覧が読み込まれていなければ `"manifestLoaded": false` も加わります。チャネル専用です。 |
| `show` | なし | null。ウィンドウを前面に表示します。 |
| `quit` | なし | null。Moonpool を終了します。 |
| `launch` | `<id>` | 成功すると null。`url` だけを持つ `static` エントリでは `opened`。エラー: `unknown app id: <id>`、`did not reach running in time`。 |
| `stop` | `<id>` | 成功すると null。`url` だけを持つ `static` エントリでは `stopped`。エラー: `still running after stop`。 |
| `restart` | `<id>` | `launch` と同じ結果とエラー。 |
| `reload` | なし | 成功すると null。 |
| `refresh-icons` | なし | 成功すると null。 |
| `help` | なし | null。ヘルプウィンドウを開きます。 |
| `dump` | `<id>` [`out-path`] | アプリのセッションログのパス、または `out-path` に作ったプレーンテキストのコピーのパス。 |
| `paths` | なし | ハブが使うフォルダーと exe の複数行レポート。 |
| `read-config` | なし | `dumps\read-config.json` のパス。内容は `token`、`valid`、`error`、`path`、`manifest_text` です。 |
| `write-config` | `<source-file>` [`token`] | 新しいバージョントークン。エラー: `stale token: ...`、`rejected invalid manifest: ...`、`cannot read source ...`。 |
| `restore-config` | [`index` または `filename`] | 引数なし: `dumps\restore-config.json` のパス(`count`、`snapshots`)。指定あり: `restored <file> (<n> apps); new version token <token>`。 |
| `argv` | コマンドライン引数 | 即座に null。このコピーの 2 つ目の `moonpool.exe <args>` とまったく同じように、`--ticket` を含めて実行します。2 つ目の起動が終了する前に引数を渡す仕組みがこれです。 |

やり取りの例:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` と `restore-config` は、新しいマニフェストをすぐに読み込み、`apps.json.history\` にスナップショットを記録して、ウィンドウを更新します。

## 診断用の動詞(テスト用)

チャネル専用です。コマンドラインからは使えません。`screenshot` を除き、すべて Windows、Linux、macOS で動作します。`screenshot` は Windows 専用で、それ以外では `screenshot is not supported on this platform (Windows only)` と応答します。

| 動詞 | 引数 | 結果 |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Windows 専用。その Moonpool ウィンドウ(既定は `main`)の PNG を Base64 で返します。省略可能な `max_dim` は長辺のピクセル数の上限です(320 から 2400 に丸められ、既定は 320。MCP ツールは常に既定値を使います)。整数でない `max_dim` はエラーです。指定できるウィンドウ: `main`、`settings`、`about`、`installer`、`editor`、`help`、`themes`。エラー: `unknown window '<name>'`、`window '<name>' is not open`。ディスクには書き込まれません。 |
| `open-window` | `<kind>` [`<id>`] | null。メニュー項目と同じ方法でウィンドウを開きます。`kind`: `settings`、`about`、`installer`、`help`、`themes`、`editor`(`<id>` を付けるとそのアプリの「アプリを編集」ダイアログ、なければ「アプリを追加」が開きます)、`terminal`(`<id>` が必須: そのアプリのターミナルタブを選択し、CLI ペインが見えるようにハブを広げます。アプリは起動しません)、`cli`(ハブを広げるだけ)。エラー: `unknown window kind '<kind>'`、`terminal needs an app id`、`unknown app id: <id>`。`launch` と同様にハブウィンドウを経由して応答します。 |
| `window-state` | [`window`] | JSON 文字列: `{"open":false}`、または `open`、`visible`、`minimized`、`maximized`、`x`、`y`、`width`、`height`。 |
| `stop-mcp` | `<id>` | `stopped`。アプリではなく、そのアプリの `<processName> mcp` ヘルパーを終了させます。エラー: `missing app id`、`unknown app id: <id>`。 |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` または `<id>: was not marked seen`。ID なしでは `cleared <n> entries`。記憶されている MCP ヘルパーの検出記録を消去します。 |

コマンドラインの `--ticket` と `state.json` の結果記録は、もう一方のチャネルのものです。[コマンドライン](/ja/automation/command-line/#結果を読み取る)を参照してください。チャネルのリクエストは、応答の中で結果を受け取ります。

## 関連項目

- [コマンドライン](/ja/automation/command-line/)
- [AI エージェント: クイックスタート](/ja/automation/quick-start/#同じ操作を-3-通りで)
