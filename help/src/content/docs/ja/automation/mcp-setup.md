---
title: "MCP で AI エージェントを Moonpool に接続する"
description: "moonpool.exe mcp を stdio の MCP サーバーとしてホストに登録する方法(インストール版・ポータブル版)と、アプリ自身の MCP ヘルパーの扱いを説明します。"
---

Moonpool の実行ファイルは、それ自体が MCP サーバーです。`moonpool.exe` を引数 `mcp` ひとつだけで実行する stdio サーバーとして、ホストに登録してください。

## サーバーを登録する

インストール版では、プログラムは `%USERPROFILE%\.moonpool\moonpool.exe` です。ポータブル版では、`.moonpool\` フォルダー内の `moonpool.exe` です。そのフルパスを `command` に指定します。`.mcp.json` を読み込むホストの場合:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

JSON ファイルでは、上のようにバックスラッシュを二重にする必要があります。Claude Code のようにコマンドラインで登録できるホストでは、1 ステップで追加できます。

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

サーバーは自分を `moonpool` と名乗り、MCP プロトコルのリビジョン `2025-06-18` で通信し、ツールだけを公開します(リソースやプロンプトは一覧に出しません)。ツールはエージェントからは `moonpool_*` として見えます。[MCP ツール](/ja/automation/mcp-tools/)を参照してください。

## 複数の Moonpool

インストール版の Moonpool と各ポータブル版は、それぞれ独自のアプリを持つ別々のランチャーで、すべて同時に実行できます。あるコピーの `moonpool.exe mcp` は、常にそのコピーを操作します。エージェントに複数のコピーを使わせるには、それぞれのコピーの exe を指す、別々の名前で登録します。

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

2 つのコピーを同じ名前で登録すると、多くのホストでは一方がもう一方を置き換えてしまいます。ツール名はどのコピーでも同じなので、ホストは登録した名前で区別します。ポータブル版は自分を `moonpool (<folder>)` とも名乗り、サーバーの説明文にそのフォルダーが含まれるため、エージェントはどのコピーと通信しているかを確認できます。

## 注意事項

- `moonpool.exe mcp` はウィンドウを開かず、インストーラーも起動しません。ホストが入力を閉じると終了します。
- 起動した exe の設定フォルダーと制御チャネルを使います。そのため、ポータブル版の exe はポータブルフォルダーのデータを読み、そのポータブル版を操作します。exe がポータブルとみなされるのは、`moonpool.portable` がその隣にある間だけです。それ以外の `moonpool.exe` は、どこにあってもインストール版の Moonpool のフォルダー(`%USERPROFILE%\.moonpool\moonpool-config\`)を使い、インストール版の Moonpool を操作します。
- ほとんどのツールは Moonpool が実行中である必要があります。実行中でなければ、エージェントは先に `moonpool_bootup_launcher` を呼び出せます。
- `moonpool_launcher_paths` は、ハブが使うフォルダーを、MCP プロセスが解決したフォルダーと並べて表示します。違いがあれば、エージェントがハブとは別の `apps.json` を見ているということです。

## サンドボックス化されたホスト

ホストによっては、ツールをパッケージ化された(Store/MSIX)サンドボックス内で実行し、AppData をパッケージごとの非公開コピーにリダイレクトします。Moonpool は、設定フォルダーや exe が `...\Packages\<package>\LocalCache\...` のようなパスの下に解決される場合に、これを検出します。

制御チャネルは応答するのに `state.json` が読めない場合にも検出します。ファイルを読み書きするツール(`moonpool_app_output`、`moonpool_read_config`、`moonpool_write_config`、`moonpool_restore_config`)は、その場合、空のデータや古いデータを返す代わりに、原因を示すエラーを返します。`moonpool_list_apps` のように制御チャネルだけを使うツールは、チャネルに到達できる間はブロックされません。サンドボックスがチャネルも隠している場合、ツールは "Moonpool is not running" ではなくサンドボックスのことを報告します。代わりに、サンドボックスの外のシェルから[コマンドライン](/ja/automation/command-line/)を使ってください。

## 独自の MCP サーバーを持つアプリ

Moonpool 内の多くのアプリは、MCP ホストが `<exe> mcp` ヘルパープロセスを通じて接続します。Moonpool は、名前がアプリの `processName` に一致し、最初の引数が `mcp` であるプロセス(`notes-app.exe mcp` など)を探します。サーバーが別の名前(名前を変えたコピーなど)で動いている場合は、アプリの `mcpProcessName` ワイルドカードを設定してください([フィールド](/ja/apps/fields/#mcpprocessname)を参照)。これに一致するプロセスは、`mcp` 引数がなくても対象になります。

- ヘルパーが接続されている間、アプリのサイドバーには MCP の子行が実行中として表示され、`moonpool_list_apps` はアプリの行に `[mcp: running]` を付け加えます。ヘルパーは、アプリ自体が実行中であることには数えられません。
- 一度ヘルパーが検出されると、Moonpool はそれを(設定フォルダーの `mcp_seen.json` に)記憶します。そのため、ヘルパーが終了した後も、MCP の子行は停止中として表示され続け、`moonpool_list_apps` は `[mcp: stopped]` を表示します。
- MCP の子行は、`showMcpProcesses` 設定で制御されます([設定ウィンドウ](/ja/using/settings/))。
- `moonpool_stop_mcp_server` はヘルパーを終了させ、アプリには手を触れません。対になる起動ツールはありません。ヘルパーを所有するホストが、次のツール呼び出しで再び起動します。

## ツールが動作しない場合

- **ホストに `moonpool_*` ツールが表示されない。** `command` が `moonpool.exe` へのフルパスで、`args` が `["mcp"]` になっているか確認し、ホストを再起動してください。
- **すべてのツールが Moonpool は実行されていないと答える。** Moonpool を起動するか、`moonpool_bootup_launcher` を呼び出してください。登録した exe が、実行中のコピーのものであることも確認してください。
- **編集が反映されない。** `moonpool_launcher_paths` を呼び出し、ハブのフォルダーと MCP プロセスのフォルダーを比較してください。[サンドボックス化されたホスト](#サンドボックス化されたホスト)を参照してください。

詳しくは[トラブルシューティング](/ja/support/troubleshooting/#mcp-とスクリプトのエラー)を参照してください。

## 関連項目

- [AI エージェント(Claude Code、Codex、Cursor)に、ローカルアプリを起動・停止する MCP サーバーを用意する](/ja/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [MCP ツール](/ja/automation/mcp-tools/)
- [AI エージェント: クイックスタート](/ja/automation/quick-start/)
