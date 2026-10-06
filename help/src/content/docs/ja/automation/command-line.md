---
title: "コマンドラインから Moonpool を操作する"
description: "ターミナルやスクリプトから moonpool.exe の動詞で実行中の Moonpool を操作し、チケットを付けて state.json から結果を読み取る方法です。"
---

同じ Moonpool がすでに実行中のときに `moonpool.exe` をもう一度実行しても、2 つ目のウィンドウは開きません。2 つ目のプロセスは引数を[制御チャネル](/ja/automation/control-verbs/)経由で実行中の Moonpool に渡し、そのまま終了します。Moonpool が先に実行されている必要があります。常駐しているものがない状態で同じコマンドを実行すると、新しい Moonpool が起動し、動詞は実行されません。

「同じ Moonpool」とは同じフォルダーのことです。インストール版とすべてのポータブル版はそれぞれ独立して動作するため、コマンドは実行した `moonpool.exe` のコピーにだけ届き、ほかのコピーには届きません。[ポータブルモード](/ja/data/portable-mode/#複数のコピーを同時に)を参照してください。

対象のコピーのパスを使ってください。インストール版の場合:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

複数のコピーが実行中の場合、`Get-Process moonpool` はそのすべてを一覧表示するので、先頭のものを選ぶのではなく `Path` で選んでください。MCP ホストが起動した待機中の `moonpool.exe mcp` ヘルパーも一覧に含まれるため、`moonpool` プロセスがあるだけではハブが実行中であることの証明にはなりません。確認するには、代わりに `ping` で制御チャネルに問い合わせます([制御動詞](/ja/automation/control-verbs/))。

## 動詞

動詞は大文字と小文字を区別しません。`<id>` は `apps.json` にあるアプリの `id` です。

| コマンド | 動作 |
| --- | --- |
| `moonpool.exe` | 動詞なし: ウィンドウを前面に表示します。 |
| `moonpool.exe show` | ウィンドウを前面に表示します。 |
| `moonpool.exe launch <id>` | アプリを起動し、そのターミナルタブを開きます。 |
| `moonpool.exe stop <id>` | アプリを停止します。 |
| `moonpool.exe restart <id>` | 停止し、ポートとプロセスが解放されるのを待ってから起動します。 |
| `moonpool.exe reload` | `apps.json` を再読み込みします。 |
| `moonpool.exe refresh-icons` | すべてのアイコンを再取得します。 |
| `moonpool.exe help` | ヘルプウィンドウを開きます。 |
| `moonpool.exe quit` | トレイメニューと同じく Moonpool を終了します。 |
| `moonpool.exe dump <id> [out-path]` | `out-path` を省略すると、このセッションにおけるアプリのログのパスを報告します。指定すると、ANSI コードを取り除いたプレーンテキストとしてログをそこにコピーします。 |
| `moonpool.exe paths` | 実行中の Moonpool が使う設定フォルダー、`apps.json`、`state.json`、ログ、ダンプフォルダー、アイコンフォルダー、ポータブルフラグ、exe のパスを報告します。 |
| `moonpool.exe read-config` | 設定フォルダーに `dumps\read-config.json` を書き出します。内容は `token`、`valid`、`error`、`path`、`manifest_text`(`apps.json` の内容そのまま)です。 |
| `moonpool.exe write-config <file> [token]` | `<file>` のマニフェストが有効で、`token` を指定した場合は `apps.json` がそれと一致していれば、`apps.json` をそのマニフェストで置き換えます。 |
| `moonpool.exe restore-config [index or filename]` | 引数なしでは、スナップショットの一覧を `dumps\restore-config.json` に書き出します。引数を 1 つ指定すると、そのスナップショットが有効であれば復元します。 |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

未知の動詞は無視されます。このプログラムには独自の起動引数もあります。`moonpool.exe mcp`([MCP のセットアップ](/ja/automation/mcp-setup/))、`--uninstall`(「アプリと機能」から使われます)、`--wait-pid <pid>`(Moonpool が自分自身を再起動するときに使われます)です。これらは最初の引数のときにだけ有効になるため、`--uninstall` のようなアプリ ID で誤って動作することはありません。

## 結果を読み取る

コマンドラインは何も出力しません。そのため、コマンドに `--ticket <key>`(重複しない任意のキー、位置は自由)を付け、設定フォルダーの `state.json` から結果を読み取ります。設定フォルダーは、インストール版では `%USERPROFILE%\.moonpool\moonpool-config\`、ポータブル版では `<your .moonpool folder>\moonpool-config\`、Linux では `~/.config/Moonpool/` です([設定の概要](/ja/apps/apps-json/#設定の保存場所)を参照)。`show` と `quit` はチケットを書き込みません。

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` には `apps`、`statuses`(アプリごとの `id`、`running`、`managed`、`mcpRunning`、`mcpSeen`)、`tickets` があります。実行中の Moonpool は数秒ごと、および各コマンドの後にこれを書き直し、終了してもファイルを削除しません。したがって、ファイルが残っていても Moonpool が実行中とは限りません。実行中かどうかの確認や、最新のアプリ一覧の取得には、制御チャネルの `ping` と `list` 動詞([制御動詞](/ja/automation/control-verbs/))または MCP ツールを使ってください。`status` が `pending` 以外になるまで、自分のチケットをポーリングします。

| `status` | 意味 |
| --- | --- |
| `pending` | 受信済みです。Moonpool がまだ処理中です。 |
| `ok` | 完了です。`dump`、`read-config`、`write-config`、`restore-config`、`paths` では、`detail` にパス、トークン、またはレポートが入ります。 |
| `error` | 失敗です。`detail` に理由が入ります。例: `unknown app id: x`、`did not reach running in time`、`unknown command`。 |

各チケットは `{ ticket, action, arg, status, detail, ts }` の形式で、`ts` は Unix ミリ秒です。

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

完了したチケットは 24 時間後に破棄され、完了から 5 分以上たったチケットが増えると、一覧は 50 件程度に切り詰められます。

MCP に対応したエージェントなら、ポーリングは不要です。[MCP のセットアップ](/ja/automation/mcp-setup/)を参照してください。

## 関連項目

- [AI エージェント: クイックスタート](/ja/automation/quick-start/)
- [制御動詞](/ja/automation/control-verbs/)
