---
title: "スクリプトと AI エージェントで Moonpool を自動化する"
description: "スクリプトや AI エージェントから実行中の Moonpool を操作する 3 つの方法(MCP、コマンドライン、制御動詞)の関係と、それぞれでできることを説明します。"
---

Moonpool は、ウィンドウに触れなくても操作できます。操作の入り口は 3 つあり、いずれも同じ常駐 Moonpool(トレイのインスタンスで、ここではハブと呼びます)が処理します。

Moonpool のコピーはそれぞれが独自のハブです。インストール版とすべてのポータブル版は独立して動作し、それぞれ独自の制御チャネルを持ちます。どの入り口も、使っている `moonpool.exe` のコピーに必ず届きます。[ポータブルモード](/ja/data/portable-mode/#複数のコピーを同時に)を参照してください。

| 入り口 | 内容 | リファレンス |
| --- | --- | --- |
| MCP サーバー | `moonpool.exe mcp`。AI ホストが起動する stdio の [MCP](https://modelcontextprotocol.io) サーバーです。 | [MCP のセットアップ](/ja/automation/mcp-setup/)、[MCP ツール](/ja/automation/mcp-tools/) |
| コマンドライン | `moonpool.exe <verb> [args]`。同じコピーを 2 回目に実行すると、動詞が制御チャネル経由でそのハブに渡され、実行は終了します。 | [コマンドライン](/ja/automation/command-line/) |
| 制御チャネル | Windows では名前付きパイプ `\\.\pipe\moonpool`(ポータブル版は `\\.\pipe\moonpool-<id>`)、Linux と macOS では Unix ソケットで、1 行に 1 つの JSON リクエストを受け付けます。 | [制御動詞](/ja/automation/control-verbs/) |

## 相互の関係

- ハブがすべてを管理します。アプリの起動、セッションログ、`apps.json` がそうです。
- MCP サーバーはハブのクライアントであり、ハブの 2 つ目のコピーではありません。ほとんどのツール呼び出しは制御チャネル経由でハブに転送され、応答がツールの結果として返ります。例外は次のとおりです。`moonpool_bootup_launcher` は `moonpool.exe` 自体を起動します。`moonpool_app_output` と設定系のツールは、ハブにファイルを書き出させてから読み取ります。`moonpool_launcher_paths` は、ハブのパスに MCP プロセス自身のパスを付け加えます。
- ハブが実行中かどうかは、プロセスを探すのではなく、そのチャネルに ping を送って判断します。応答するハブは実行中で、パイプまたはソケットがなければ実行中ではありません。
- どの入り口も、ウィンドウと同じハンドラーを実行します。そのため、動詞はそれに対応するクリックと同じことを行います。
- ハブが実行されていない場合、`moonpool_list_apps` を含め、ハブを操作するツールは "Moonpool is not running" と答えて拒否します。古い一覧は返りません。`moonpool_bootup_launcher` で起動できます。何かがチャネルを保持しているのに数秒以内に応答しない場合、エラーには Moonpool プロセスがハングしている可能性があると示されます。
- MCP サーバーは、制御チャネルより前のビルドのハブを操作するフォールバックを行わなくなりました。そのコピーを更新するか、終了してから再び起動してください。

## 変更できるもの

| 変更できること | 入り口 |
| --- | --- |
| アプリの起動、停止、再起動 | MCP、コマンドライン、パイプ |
| `apps.json` の書き換え | MCP(`moonpool_write_config`、`moonpool_restore_config`)、コマンドライン、パイプ |
| Moonpool の終了 | MCP(`moonpool_shutdown_launcher`)、コマンドライン(`quit`)、パイプ |
| アプリの MCP ヘルパープロセスの終了 | MCP(`moonpool_stop_mcp_server`)、パイプ(`stop-mcp`) |
| `apps.json` の再読み込み、アイコンの再取得、ウィンドウの表示 | MCP(`moonpool_reload_config`、`moonpool_refresh_app_icons`、`moonpool_raise_launcher`)、コマンドライン(`reload`、`refresh-icons`、`show`)、パイプ |
| ウィンドウまたはターミナルタブを開く | パイプ(`open-window`) |
| 記憶された MCP ヘルパーの検出記録の消去 | MCP(`moonpool_reset_mcp_seen`)、パイプ(`reset-mcp-seen`) |

読み取り専用のツール: `moonpool_list_apps`、`moonpool_app_output`、`moonpool_read_config`、`moonpool_launcher_paths`、`moonpool_window_state`、`moonpool_screenshot`。

## 安全性

- **設定の書き込みは保護されています。** 書き込みには直前の読み取りで得たバージョントークンが必要で、古いトークンは拒否され、何かを書き込む前に新しい `apps.json` が検証されます。拒否された書き込みは `apps.json` を変更しません。[MCP ツール](/ja/automation/mcp-tools/#設定)を参照してください。
- **アプリ ID は制限されています。** MCP サーバーが受け付けるのは英字、数字、`.`、`_`、`-` だけで、先頭の `-` は認められません。そのため、ID がコマンドラインのフラグとして解釈されることはありません。
- **スクリーンショットは Moonpool のみです。** `moonpool_screenshot` が取得するのは、Moonpool 自身の 6 つのウィンドウ(`main`、`settings`、`about`、`installer`、`editor`、`help`、`themes`)のいずれかで、画面全体やほかのアプリは取得しません。PNG はメモリ上で作られてインラインで返され、Moonpool がファイルに保存することはありません。
- **チャネルに認証はありません。** Moonpool は、制御パイプやソケットにログインやトークンを追加していません。開けるプロセスなら、どれでも動詞を送れます。Linux と macOS では、ソケットファイルがモード `0600` で作られるため、自分のユーザーだけが開けます。
- **サンドボックス化されたホストを検出します。** MCP サーバーが、Moonpool のファイルの非公開コピーが見えてしまうパッケージ化された(Store/MSIX)サンドボックス内で動作していることを検出すると、ファイルを読み書きするツール(`moonpool_app_output`、`moonpool_read_config`、`moonpool_write_config`、`moonpool_restore_config`)は、古いデータの代わりにその理由を説明するエラーを返します。制御チャネルだけを使うツールはブロックされません。[MCP のセットアップ](/ja/automation/mcp-setup/#サンドボックス化されたホスト)を参照してください。

## プラットフォーム

制御チャネルはすべてのプラットフォームにあります。Windows では名前付きパイプ、Linux と macOS では Unix ソケットです(場所は[制御動詞](/ja/automation/control-verbs/#待ち受け先)にあります)。Windows 専用なのは `screenshot`(したがって `moonpool_screenshot`)だけで、Linux と macOS では "not supported on this platform" を返します。コマンドラインの動詞は、すべてのプラットフォームで動作します。

## 関連項目

- [AI エージェント: クイックスタート](/ja/automation/quick-start/)
- [MCP のセットアップ](/ja/automation/mcp-setup/)
