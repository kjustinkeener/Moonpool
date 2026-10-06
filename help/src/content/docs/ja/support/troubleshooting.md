---
title: "Moonpool のトラブルシューティング: トレイ、起動しないアプリ、更新"
description: "トレイアイコンが見えない、アプリが起動・停止しない、状態のドットがおかしい、更新の失敗、MCP のエラーなど、よくある問題を症状から探して対処します。"
---

症状を探し、その対処に従ってください。引用符で囲んだ文言は Moonpool が表示するものです。メッセージの正確な文言を調べるには、[エラーメッセージ一覧](/ja/support/error-messages/)を参照してください。

## トレイアイコンが見えない

- **Windows。** アイコンが、隠れたアイコンの領域にある場合があります。タスクバーの右側の **^** 矢印をクリックしてください。アイコンをタスクバーにドラッグすると、常に表示されます。
- **標準の GNOME 上の Linux。** GNOME は、AppIndicator 拡張機能がないとトレイアイコンを表示しません。[Linux](/ja/platforms/linux/#gnome-のトレイ)を参照してください。
- **設定。** **トレイに表示**がオフになっている場合があります。タスクバーまたはスタートメニューからハブを開き、[設定](/ja/using/settings/)でオンに戻してください。

## インストーラーにエラーが表示される

| メッセージ | 対処 |
| --- | --- |
| `インストールに失敗しました: <error>` | コロンの後ろのテキストが、失敗した手順を示します。例: `copy exe: ...`。ファイルが使用中の場合は、`%USERPROFILE%\.moonpool` から実行中の Moonpool を終了してから、もう一度試してください。 |
| `target folder does not exist` | ポータブル版用に選んだフォルダーがなくなっています。存在するフォルダーを選んでください。 |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | 空のフォルダーを選ぶか、先にその `.moonpool` フォルダーを削除してください。 |

## インストーラーを実行すると Windows protected your PC が表示される

これは Windows SmartScreen によるもので、`moonpool.exe` がコード署名されていないために表示されます。**More info**(詳細情報)をクリックし、続けて **Run anyway**(実行)をクリックしてください。[Windows protected your PC](/ja/support/windows-protected-your-pc/)を参照してください。

## Windows で Moonpool のウィンドウが空白、または開かない

Microsoft Edge WebView2 ランタイムがない可能性があります。[WebView2 ランタイムがない](/ja/support/webview2-runtime-missing/)を参照してください。

## アプリが起動しない

1. アプリの名前をクリックしてターミナルタブを開き、出力を読みます。エージェントは、`moonpool_app_output` で同じテキストを読めます。
2. `cwd` を確認します。フォルダーがない、または `./` なしの相対パスであることが、よくある原因です。[パスと環境変数](/ja/apps/paths-and-environment/)を参照してください。
3. `command` を確認します。`cwd` のターミナルで手作業で実行してみてください。Windows では、二重引用符の入れ子は避けてください。`cmd /c` が壊してしまいます。
4. 設定で**デバッグ情報をファイルに記録**をオンにして、もう一度起動します。`moonpool.log` に、実際のコマンドとフォルダーが記録されます。[ログ](/ja/data/logs/)を参照してください。

| メッセージ | 意味 |
| --- | --- |
| `already running` | Moonpool がすでにこのアプリのターミナルを保持しています。先に停止するか、再起動を使ってください。 |
| `stopped during launch` | 起動がまだ始まっている途中で、停止が押されました。 |
| `did not reach running in time` | スクリプトまたはエージェントから: アプリが 25 秒以内に実行中と判定されませんでした。その `port` または `processName` と、出力を確認してください。 |

## 状態のドットがおかしい

Moonpool は、`port`、次に `processName`、さらに自身のターミナルがまだ生きているかどうかの順に、実行中かを判定します。[実行中かどうかの判定](/ja/apps/types/#実行中の判定方法)を参照してください。

- **塗りつぶされない。** `web` アプリの `port` が応答していないか、`desktop` アプリの `processName` が一致していません。Linux では、`processName` は 15 文字以下にする必要があります。
- **起動した直後に灰色になる。** `cli` アプリは、そのコマンドが終了すると実行中ではなくなります。開いたままにしたい場合は、`-NoExit` 付きのシェルを使ってください。
- **`static` アプリが実行中と表示されない。** `url` だけを持つエントリでは、これは想定どおりです。
- **起動していないのに実行中と表示される。** 別のものがそのポートまたはプロセス名を使っています。Moonpool はそれを、実行中ではあるものの "managed by Moonpool"(Moonpool による管理下)ではないものとして表示します。

## Error: listen EADDRINUSE or "Port 5173 is in use"

ほかのものが、サーバーが使いたいポートですでに待ち受けています。見つけて終了させるか、アプリに `port` を設定して、停止で解放されるようにしてください。[Fix EADDRINUSE and "Port 5173 is in use"](/ja/support/port-already-in-use/)と、[ポートを使っているプロセスを見つけて終了する](/ja/guides/find-and-kill-process-using-port-windows/)を参照してください。

## 2 つのアプリが同じポートを使っている

**...** メニューの下部に、警告の行が表示されます。例: `ポート 3000: App A / App B`。一方のアプリの `port`(と、`PORT` を読む場合はその `env`)を変更してください。[ポート競合の警告](/ja/using/hub-window/#ポート競合の警告)を参照してください。

## 停止したのにアプリが動き続ける

スクリプトまたはエージェントからは、エラーは `still running after stop`(15 秒後)になります。

- アプリが、そのターミナルより長く生き残っています。`killMode` を `port` または `processName` に設定してください。[停止と再起動](/ja/apps/stop-and-restart/)を参照してください。
- Windows の Docker アプリの場合: `killMode` を `command` にして、`docker compose stop app` のような `stopCommand` を使ってください。`port` は絶対に使わないでください。

## apps.json にエラーがある

サイドバーにバナーが表示されます。「apps.json にエラーがあります。最後に読み込めた一覧を表示しています。」、または起動時なら「apps.json にエラーがあるため、アプリは読み込まれていません。」です。ファイルが再び読み込めるようになるまで、Moonpool からの保存は停止します。

よくあるエラー:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. バナーの **apps.json を編集**を選び、エントリを修正して保存してから、**再読み込み**(F5)を選びます。
2. または、最近の正常なコピーにロールバックします。[バックアップと復旧](/ja/data/backup-and-recovery/#appsjson-をロールバックする)を参照してください。

規則の全一覧は、[検証](/ja/apps/apps-json/#検証)にあります。

設定を変更できず、メッセージが `Repair settings.json and restart Moonpool before changing settings` で終わっている場合は、設定フォルダーの `settings.json` を修正または削除して、Moonpool をもう一度起動してください。削除すると、すべての設定が既定値に戻ります。

## 編集が反映されない

- 手作業での編集には、**再読み込み**(または F5)が必要です。Moonpool はファイルを監視していません。
- 再読み込みは、実行中のアプリを再起動しません。変更した `command`、`cwd`、`env` を使うには、アプリを再起動してください。
- エージェントが、別の `apps.json` を編集しているのかもしれません。`moonpool_launcher_paths` を呼び出させ、ハブのフォルダーと自分のフォルダーを比較してください。Moonpool のコピーが複数ある場合は、どのコピーを編集しているかを確認してください。

## サンプルアプリがない

サンプルが書き込まれるのは、`apps.json` が存在しないときだけです。取り戻すには、[サンプルへのリセット](/ja/data/backup-and-recovery/#サンプルへのリセット)を参照するか、[サンプルダッシュボード](/ja/getting-started/example-dashboards/#サンプルアプリは初回起動時にのみ追加されます)からエントリをコピーしてください。

## 更新に失敗した

バナーに `更新に失敗しました: <error>` と表示されます。[更新に失敗したとき](/ja/data/updating/#更新に失敗したとき)を参照してください。

## Web リンクが開かない

`refusing to open non-web url: <url>` は、`url` が `http://`、`https://`、`mailto:`、`file://` のいずれでもないことを意味します。`url` を修正してください。

## MCP とスクリプトのエラー

| メッセージ | 対処 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Moonpool を起動するか、エージェントに `moonpool_bootup_launcher` を呼び出させてください。 |
| `frontend not loaded` | ハブウィンドウがまだ読み込み終わっていません。少し待ってから再試行してください。 |
| `stale token: ...` | エージェントが読んだ後に `apps.json` が変更されました。もう一度読んでから、書き込んでください。 |
| `rejected invalid manifest: ...` | 新しい `apps.json` が検証に失敗しました。ファイルは変更されていません。 |
| `... A Moonpool process may be hung ...` | 何かが応答せずに制御チャネルを保持しています。トレイから Moonpool を終了するか、プロセスを終了させてから、もう一度起動してください。 |

詳しくは、[MCP のセットアップ](/ja/automation/mcp-setup/#注意事項)と [MCP ツール](/ja/automation/mcp-tools/)を参照してください。

## ウィンドウの問題

- **画面の外にある。** Moonpool は、接続されたどのディスプレイにもない保存済みの位置を無視します。それでもウィンドウが見つからない場合は、Moonpool を終了し、設定フォルダーの `window-state.json` を削除してください。
- **拡大率が大きすぎる、または小さすぎる。** ハブの上で Ctrl + ホイールを回すと変更されます。[ショートカットとズーム](/ja/using/keyboard-shortcuts/#ズーム)を参照してください。
- **設定がハブの後ろに開く。** 設定で**常に手前に表示**をオフにするか、オンにしてください。この設定はすべての Moonpool ウィンドウに適用されるため、同じ階層に保たれます。

## ログはどこにある?

[ログ](/ja/data/logs/)を参照してください。

## バックアップ、リセット、アンインストール

[バックアップと復旧](/ja/data/backup-and-recovery/)と[アンインストール](/ja/getting-started/install/#アンインストール)を参照してください。

## FAQ

**ウィンドウを閉じると、アプリも停止しますか?**
既定では、閉じると Moonpool が終了し、Windows では、終了すると Moonpool が起動したアプリも停止します。ウィンドウを閉じても Moonpool を実行し続けたい場合は、**閉じたらトレイに格納**をオンにしてください。[トレイ、閉じる、最小化](/ja/using/tray-and-closing/)を参照してください。

**Moonpool を 2 つ実行できますか?**
フォルダーごとに 1 つです。同じコピーをもう一度起動すると、そのウィンドウが前面に戻ります。インストール版とポータブル版は、並べて実行できます。[ポータブルモード](/ja/data/portable-mode/#複数のコピーを同時に)を参照してください。

**Moonpool は外部に通信しますか?**
更新の確認のときだけです。起動時(**起動時に更新を確認**がオンの場合)と、**更新を確認**を押したときに、GitHub からリリースファイル(`update.json`)を取得します。ダウンロードしたものはすべて、使用される前に Moonpool の署名鍵で検証されます。

**コマンドはどのシェルで実行されますか?**
Windows では `cmd /c`、Linux と macOS では `$SHELL -c` です。

**シークレットはどこに置けばよいですか?**
`env` の値は、`apps.json` にプレーンテキストで保存されます。アプリ自身が読み込むファイル、またはユーザー環境にすでに設定されている変数(起動したアプリが継承します)を使うことをお勧めします。

**制御チャネルは保護されていますか?**
ログインやトークンはありません。自分として実行されているプロセスなら、どれでもコマンドを送れます。Linux と macOS では、ソケットは自分のユーザーだけが読み取れます。[安全性](/ja/automation/overview/#安全性)を参照してください。
