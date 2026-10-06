---
title: "Moonpool のバックアップ、apps.json のロールバック、環境の復旧"
description: "バックアップすべきもの、壊れた apps.json のロールバック、サンプルアプリへのリセット、インストール版からポータブル版への移行、アンインストールで消えるものを説明します。"
---

Moonpool が保持するものはすべて、設定フォルダーとダッシュボードフォルダーの 2 か所にあります。各モードでのパスは[設定の保存場所](/ja/apps/apps-json/#設定の保存場所)にあります。

## 設定フォルダー

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

ダッシュボードフォルダーは `{MP_HOME}\dashboards` です。インストール版では `%USERPROFILE%\.moonpool\dashboards`、ポータブル版では `<your .moonpool folder>\dashboards`、Linux では設定フォルダー内の `dashboards/` です。ここにある自分のファイルはバックアップしてください。`examples` フォルダーは Moonpool のもので、更新時に書き直されます。

テーマはウィンドウのブラウザーストレージに保存されており、コピーできるファイルにはありません。そのためバックアップには含まれません。復元後に選び直してください。

## バックアップ

1. Moonpool を終了します。ファイルが書き込みの途中になるのを避けるためです。
2. 設定フォルダーから `apps.json`、`settings.json`、`icons\` をコピーし、`dashboards\` から自分のファイルをコピーします。

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

復元するには、Moonpool を終了し、ファイルを元の場所にコピーして、起動します。

## apps.json をロールバックする

保存、エージェントによる書き込み、復元のたびに、また内容が変わっていた**再読み込み**のたびに、検証済みの `apps.json` が `apps.json.history\` にコピーされ、新しい 10 件が保持されます。各ファイルには、作成された時刻にちなんだ名前が付きます。例: `1767225600000.json`。`apps.json.bak` はありません。

- **手作業で。** スナップショットを `apps.json` に上書きコピーし、**再読み込み**を選びます。

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **スクリプトから。** `moonpool.exe restore-config` でスナップショットを一覧表示し、`moonpool.exe restore-config 1` で最新のものを復元します。[コマンドライン](/ja/automation/command-line/)を参照してください。
- **エージェントから。** `moonpool_restore_config`。[MCP ツール](/ja/automation/mcp-tools/#設定)を参照してください。

自動的に復元されるものはありません。

## 壊れたファイル

- **apps.json。** Moonpool は壊れたファイルを上書きしません。[ファイルに問題がある場合](/ja/apps/apps-json/#ファイルが不正なとき)を参照してください。
- **settings.json。** 修正するか、すべての設定をリセットするには削除して、Moonpool を再起動します。[settings.json](/ja/data/settings-json/#読み込みと修復)を参照してください。

## サンプルへのリセット

Moonpool がサンプルアプリを書き込むのは、`apps.json` がないときだけです。最初からやり直すには、Moonpool を終了し(または実行したまま)、`apps.json` の名前を変えるか削除してから、Moonpool を起動するか**再読み込み**を選びます。サンプルを含む新しい `apps.json` が書き込まれます。

## インストール版からポータブル版へ

新しいポータブル版は、サンプルアプリから始まります。自分のアプリを移すには、[ポータブルモード](/ja/data/portable-mode/#インストーラーからポータブル版を選ぶ)を参照してください。必要であれば、`icons\` と `settings.json` も同じ方法でコピーします。

## アンインストール

インストール版の Moonpool をアンインストールすると、設定フォルダーとダッシュボードを含む `%USERPROFILE%\.moonpool` フォルダー全体が削除されます。先にバックアップしてください。[アンインストール](/ja/getting-started/install/#アンインストール)を参照してください。ポータブル版は、その `.moonpool\` フォルダーを削除すれば取り除けます。
