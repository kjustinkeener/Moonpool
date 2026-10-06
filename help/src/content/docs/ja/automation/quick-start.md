---
title: "AI エージェントに Moonpool を設定・操作させる: クイックスタート"
description: "AI エージェントやスクリプトに Moonpool を設定・操作させる 3 つの方法、エージェントに合った選び方、同じ操作をそれぞれで行う例を紹介します。"
---

入り口は 3 つあります。エージェントができることに合わせて選んでください。

| やりたいこと | 使うもの | 最初に読むところ |
| --- | --- | --- |
| エージェントにアプリを見つけて追加してもらう(1 回だけ) | ハブの空の画面にある**プロンプトをコピー** | 下記 |
| エージェントにツール呼び出しでアプリを起動・停止・確認してもらう | MCP サーバー `moonpool.exe mcp` | [MCP のセットアップ](/ja/automation/mcp-setup/) |
| スクリプト、または MCP を使えないエージェント | コマンドラインの動詞 | [コマンドライン](/ja/automation/command-line/) |

## プロンプトをコピー

タブが 1 つも開いていないとき、CLI ペインには用意されたプロンプトが表示されます(「はじめてですか? これを AI エージェントに渡してアプリを設定してもらいましょう:」)。**プロンプトをコピー**を押すと、それがクリップボードにコピーされます。エージェントに貼り付けてください。プロンプトは、設定フォルダーにある `AI-README.md` と `apps.json` をエージェントに示し、アプリを見つけて登録するよう依頼します。終わったら、**再読み込み**を選びます。

Moonpool は起動のたびに `apps.json` の隣にある `AI-README.md` を書き直すため、常に実行中のバージョンと一致しています。ここに自分の編集を残さないでください。

## 同じ操作を 3 通りで

| 操作 | コマンドライン | 制御チャネルの動詞 | MCP ツール |
| --- | --- | --- | --- |
| アプリを起動する | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| アプリを停止する | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| アプリを再起動する | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| アプリの出力を読む | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| アプリと状態を一覧表示する | `state.json` を読む | `list` | `moonpool_list_apps` |
| `apps.json` を再読み込みする | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| `apps.json` を読む | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| `apps.json` を置き換える | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| `apps.json` を元に戻す | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| ウィンドウを表示する | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Moonpool を起動する | `moonpool.exe` | なし | `moonpool_bootup_launcher` |
| Moonpool を終了する | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| 使用中のフォルダーを表示する | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

コマンドラインは何も出力しないため、結果は `--ticket` で読み取ります([結果を読み取る](/ja/automation/command-line/#結果を読み取る)を参照)。チャネルと MCP は、直接答えを返します。

## エージェントのツールが失敗するとき

- `Moonpool is not running - call moonpool_bootup_launcher first`: Moonpool を起動するか、エージェントにそのツールを呼び出させてください。
- 編集が「反映されない」: エージェントに `moonpool_launcher_paths` を実行させてください。ハブと MCP のフォルダーが違っていれば、エージェントは別の `apps.json` を読んでいます。[サンドボックス化されたホスト](/ja/automation/mcp-setup/#サンドボックス化されたホスト)を参照してください。
- 複数の Moonpool のコピー: それぞれを別の名前で登録してください。[複数の Moonpool](/ja/automation/mcp-setup/#複数の-moonpool)を参照してください。

Claude Code、Codex、Cursor での実際の手順は、[AI エージェントに、ローカルアプリを起動・停止する MCP サーバーを用意する](/ja/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)にあります。

そのほかの症状は[トラブルシューティング](/ja/support/troubleshooting/#mcp-とスクリプトのエラー)を参照してください。
