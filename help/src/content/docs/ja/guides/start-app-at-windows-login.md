---
title: "Windows ログイン時にスクリプトや開発サーバーを自動で起動する"
description: "スタートアップフォルダーのショートカットで Windows ログイン時に Moonpool を起動し、小さな PowerShell スクリプトで開発サーバーやスクリプトを起動します。専用の設定はありません。"
---

Windows でログイン時に何かを起動する一般的な方法は 2 つあります。スタートアップフォルダーにショートカットを置く方法 (Win+R を押して `shell:startup` と入力し、Enter を押します) と、「ログオン時」トリガーのタスクスケジューラのタスクです。どちらもプログラムやスクリプトを実行します。開発サーバーのコマンドを直接実行することもできますが、その場合は追跡も、出力の表示も、停止も行われません。

## Moonpool でできること

Moonpool にはログイン時に起動する設定がなく、`apps.json` のエントリーにも、Moonpool の起動時にそのアプリを起動するフィールドはありません (一覧は[アプリのフィールド](/ja/apps/fields/)と [settings.json](/ja/data/settings-json/)にあります)。できるのは、自分で Moonpool をログイン時に起動し、そのあとスクリプトで起動したいアプリを起動することです。[コマンドライン](/ja/automation/command-line/)にあるのと同じ動詞を使います。

まず、いつもどおりアプリを登録します。

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

次に、これを `start-moonpool-apps.ps1` として保存します。インストール版では、プログラムは `%USERPROFILE%\.moonpool\moonpool.exe` です。ポータブル版では、そのコピーの exe のパスを使ってください。

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

`launch` が Moonpool に渡されるには、Moonpool がすでに動いている必要があります。常駐していない状態で実行すると、同じコマンドが新しい Moonpool を起動するだけで、動詞は実行されません。待ち時間は起動に余裕を持たせるためのもので、遅いマシンでは長くしてください。アプリごとに `& $mp launch <id>` の行を 1 行ずつ追加します。

最後に、このスクリプトへのショートカットをスタートアップフォルダーに置きます。ターゲットは次のとおりです。

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

何が起きたかを確認するには、動詞に `--ticket t1` を付けて、結果を `state.json` から読みます ([結果の読み取り](/ja/automation/command-line/#結果を読み取る))。

## 注意点

- この方法で起動した開発サーバーは、ほかのアプリと同様に Moonpool の「管理下」になるので、停止や終了が効きます。同じアプリがすでに動いている場合 (たとえば手動で起動した場合) は、Moonpool はそれを実行中と表示しますが、管理下にはありません。
- Moonpool は、終了したアプリを再起動せず、前回終了したときにどのアプリが動いていたかも記憶しません。

## 関連ページ

- [コマンドライン](/ja/automation/command-line/)
- [トレイ、閉じる、最小化](/ja/using/tray-and-closing/)
- [Windows で npm の開発サーバーをバックグラウンドで実行する](/ja/guides/run-npm-dev-server-in-background-windows/)
