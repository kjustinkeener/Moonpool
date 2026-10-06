---
title: "AI エージェント (Claude Code、Codex、Cursor) にローカルアプリを起動・停止する MCP サーバーを渡す"
description: "Moonpool を MCP サーバーとして登録し、Claude Code、Codex、Cursor が開発サーバーを起動、停止、再起動して出力を読めるようにします。余計なコピーは増えません。"
---

AI コーディングエージェントは、たいてい自分のシェルに `npm run dev` と入力して開発サーバーを実行します。そのせいで、エージェントが処理を待たされたり、ポートを握ったままの孤立したプロセスが残ったり、すでに動いているものの 2 つ目のコピーが起動したりすることがあります。MCP サーバーを使うと、エージェントはコマンドラインを組み立て直す代わりに、すでに設定したアプリをツール経由で起動、停止できます。

## Moonpool での方法

Moonpool の実行ファイルは、それ自体が MCP サーバーです。`moonpool.exe` を、引数 `mcp` 1 つだけの stdio サーバーとして登録します。アプリが `apps.json` にあれば、エージェントは ID を指定して起動できます。

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

サーバーを登録します。Claude Code では、次の 1 つのコマンドで登録できます (インストール版の Moonpool の場合。お使いの exe のフルパスを使ってください)。

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Cursor の `mcp.json` のように、MCP サーバーの JSON ファイルを読み込むホストでも、同じ形式を使います (バックスラッシュは二重にします)。

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Codex では、設定 (`~/.codex/config.toml`) に、同じコマンドと `mcp` 引数でサーバーを追加します。

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

正確なファイル名やキー名はホストごとに異なるため、お使いのバージョンで違う場合は、そのホストの MCP ドキュメントを確認してください。Moonpool に必要なのは、`moonpool.exe` のフルパスと、引数の `mcp` だけです。登録後はホストを再起動してください。

## エージェントにできること

ツールは `moonpool_*` として表示されます。日常の作業で使うものは次のとおりです。

| ツール | 用途 |
| --- | --- |
| `moonpool_list_apps` | アプリの ID を調べ、実行中かどうかを確認します。 |
| `moonpool_start_app` | ID を指定してアプリを起動し、そのターミナルタブを開きます。 |
| `moonpool_stop_app` | 子プロセスも含めて停止します。 |
| `moonpool_restart_app` | 停止し、ポートが解放されるのを待って、起動します。コードを変更したあとに使います。 |
| `moonpool_app_output` | アプリが出力した内容を読みます。`tail_lines` で行数を絞れます。 |
| `moonpool_bootup_launcher` | Moonpool が動いていないときに、Moonpool 自体を起動します。 |

典型的な流れは、`moonpool_restart_app` のあとに `moonpool_app_output` を呼ぶことです。残りのツール (`apps.json` の読み書き、スクリーンショット) は [MCP ツール](/ja/automation/mcp-tools/)にあります。

## うまく動かないとき

すべてのツールが `Moonpool is not running - call moonpool_bootup_launcher first` と返す場合は、Moonpool がまだ起動していません。編集が反映されない場合は、たいていエージェントが別の `apps.json` を見ています。`moonpool_launcher_paths` を呼んでください。[ツールが動かないとき](/ja/automation/mcp-setup/#ツールが動作しない場合)を参照してください。

## 関連ページ

- [MCP のセットアップ](/ja/automation/mcp-setup/)
- [MCP ツール](/ja/automation/mcp-tools/)
- [AI エージェント: クイックスタート](/ja/automation/quick-start/)
- [Windows で npm の開発サーバーをバックグラウンドで実行する](/ja/guides/run-npm-dev-server-in-background-windows/)
