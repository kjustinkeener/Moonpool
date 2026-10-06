---
title: "Moonpool のエラーメッセージ一覧: already running、requires a command など"
description: "already running、requires a command、stale token、更新に失敗しました など、Moonpool のエラーメッセージの正確な文言と、それぞれの意味と対処を調べられます。"
---

表示されたメッセージをページ検索に貼り付けるか、表をざっと見てください。メッセージは Moonpool が表示するとおりに引用しています。`<angle brackets>` で囲んだ部分は、値(アプリの ID、パス、システムからのエラー)に置き換わります。エラーメッセージではない症状は、[トラブルシューティングと FAQ](/ja/support/troubleshooting/)にあります。

## アプリの起動と停止

| メッセージ | 意味と対処 |
| --- | --- |
| `already running` | Moonpool がすでにこのアプリのターミナルを保持しています。先に停止するか、再起動を使ってください。 |
| `stopped during launch` | 起動がまだ始まっている途中で、停止が押されました。もう一度起動してください。 |
| `app has no launch command` | エントリに `command` がありません。アプリエディターまたは `apps.json` で追加してください。`url` だけを持つ `static` エントリだけは、なくても構いません。 |
| `unknown app: <id>` | その `id` のアプリが読み込まれていません。ID を確認し、`apps.json` を手作業で編集した場合は再読み込みしてください。 |
| `unknown app id: <id>` | 同じ問題が、スクリプトまたはエージェントに報告されたものです。`moonpool_list_apps` でアプリを一覧表示してください。 |
| `did not reach running in time` | スクリプトまたはエージェントから: アプリが 25 秒以内に実行中と判定されませんでした。`port` または `processName` を確認し、出力を読んでください。[状態のドットがおかしい](/ja/support/troubleshooting/#状態のドットがおかしい)を参照してください。 |
| `still running after stop` | 15 秒たってもアプリがまだ実行中と判定されます。`killMode` を設定してください。[停止と再起動](/ja/apps/stop-and-restart/)を参照してください。 |
| `refusing to open non-web url: <url>` | `url` が `http://`、`https://`、`mailto:`、`file://` のいずれでもありません。`url` を修正してください。 |
| `[プロセスが終了しました]` | エラーではありません。アプリのコマンドが終了したことを示します。ターミナルタブに表示されます。 |

## apps.json の検証

Moonpool は、規則に違反する `apps.json` を拒否し、最後に読み込めた一覧を保持します。`<n>` は、ファイル内でのエントリの位置で、1 から数えます。

| メッセージ | 対処 |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | `id` の名前を変えてください。 |
| `duplicate app id "<id>"` | 2 つのエントリが同じ `id` を使っています。それぞれ一意にしてください。 |
| `apps.json entry <n> (<id>) has an empty name` | `name` を入力してください。 |
| `apps.json entry <n> (<id>) has an empty group` | `group` を入力してください。 |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` は `web`、`desktop`、`static`、`cli` のいずれかにしてください。 |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` は 1 から 65535 にしてください。 |
| `apps.json entry <n> (<id>) requires a url` | `static` エントリには `url` が必要です。 |
| `apps.json entry <n> (<id>) requires a command` | それ以外のタイプにはすべて `command` が必要です。 |

アプリエディターで名前なしで保存しようとすると、`name は必須です。` と表示されます。バナーの文言「apps.json にエラーがあります。最後に読み込めた一覧を表示しています。」または「apps.json にエラーがあるため、アプリは読み込まれていません。」と、その復旧方法は、[apps.json にエラーがある](/ja/support/troubleshooting/#appsjson-にエラーがある)にあります。バナーに保存が停止していると表示される場合、メッセージは `Repair apps.json and reload it before saving from Moonpool` で終わります。規則の全一覧は、[検証](/ja/apps/apps-json/#検証)にあります。

## 設定、更新、インストーラー

| メッセージ | 意味と対処 |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` の形式が不正です。修正するか削除して、再起動してください。[settings.json](/ja/data/settings-json/#読み込みと修復)を参照してください。 |
| `更新に失敗しました: <error>` | 更新のダウンロードまたはインストールに失敗しました。[更新に失敗したとき](/ja/data/updating/#更新に失敗したとき)を参照してください。 |
| `更新の確認に失敗しました: <error>` | 「このアプリについて」での更新の確認に失敗しました。コロンの後ろのテキストが理由を示します。後でもう一度試してください。 |
| `インストールに失敗しました: <error>` | コロンの後ろに示された手順(例: `copy exe: ...`)でインストーラーが停止しました。`%USERPROFILE%\.moonpool` から実行中の Moonpool があれば終了して、再試行してください。 |
| `target folder does not exist` | ポータブル版用に選んだフォルダーがなくなっています。存在するフォルダーを選んでください。 |

## MCP とスクリプト

| メッセージ | 意味と対処 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Moonpool を起動するか、エージェントにそのツールを呼び出させてください。ポータブル版では、メッセージにコピーの名前が入ります。 |
| `frontend not loaded` | ハブウィンドウがまだ読み込み終わっていません。待ってから再試行してください。 |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | エージェントが、MCP サーバーが受け付けない ID を渡しました。`moonpool_list_apps` に表示される ID を使ってください。 |
| `stale token: apps.json changed since it was read ...` | `apps.json` をもう一度読み、編集をやり直してから、書き込んでください。 |
| `rejected invalid manifest: ...` | 新しい `apps.json` が検証に失敗しました(上記を参照)。ファイルは変更されていません。 |
| `no console output recorded for '<id>' (not launched this session)` | Moonpool の起動後に実行されていないアプリについて、`moonpool_app_output` が呼び出されました。 |

詳しくは、[MCP ツール](/ja/automation/mcp-tools/)と [MCP のセットアップ](/ja/automation/mcp-setup/#ツールが動作しない場合)を参照してください。

## ほかのプログラムのエラー

- [`Error: listen EADDRINUSE: address already in use :::3000` と `Port 5173 is in use`](/ja/support/port-already-in-use/)
- [`Windows protected your PC`](/ja/support/windows-protected-your-pc/)
- [WebView2 ランタイムがない](/ja/support/webview2-runtime-missing/)
