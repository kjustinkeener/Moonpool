---
title: "Windows でターミナルウィンドウなしに npm の開発サーバーをバックグラウンドで実行する"
description: "見張っておくコンソールウィンドウなしで、npm run dev や Vite などの開発サーバーを Windows で動かし続け、トレイから起動、停止、出力の確認を行います。"
---

`npm run dev` で起動した開発サーバーは、起動したターミナルの中で動くため、そのウィンドウを閉じると終了します。Windows で動かし続ける素直な方法は、PowerShell の `Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden` のような非表示プロセスですが、その場合は読める出力がなく、停止するには正しい `node.exe` を探し回ることになります ([ポートを使っているプロセスを見つけて終了する](/ja/guides/find-and-kill-process-using-port-windows/)を参照)。

## Moonpool での方法

Moonpool はコマンドを、ハブウィンドウ内の専用の組み込みターミナルタブで実行するので、開いたままにしておく別のコンソールウィンドウはありません。ハブをトレイに隠しても、サーバーは動き続けます。アプリを一度だけ追加します。

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

アプリの **起動** をクリックします。`port` が応答するとステータスの点が点灯し、`openBrowser` によってブラウザーが `url` を開きます。アプリの名前をクリックすると、専用のタブで出力を読めます。**停止** を押すと、ターミナルとそこから起動されたものすべてが終了し、ポートが解放されます (`web` では `killMode` の既定が `port` です)。

## ウィンドウを閉じても動かし続ける

既定では、閉じるボタンで Moonpool が終了し、Windows では終了すると起動したすべてのアプリが停止します。[設定](/ja/using/settings/)で **閉じたらトレイに格納** をオンにすると、ウィンドウを閉じても隠れるだけになります。トレイアイコン (または **Moonpool を表示**) で元に戻せます。詳細は[トレイ、閉じる、最小化](/ja/using/tray-and-closing/)にあります。

## ポートを固定しておく

Moonpool は `port` から「実行中」かどうかを判断します。Vite は指定のポートが使用中だと次の空きポートに移るため、Moonpool が別のポートを見続けることになります。`--strictPort` を渡して Vite が代わりに終了するようにし、`port` もそれに合わせます。

```text title="command"
npm run dev -- --port 5173 --strictPort
```

すでにポートが使われている場合は、[EADDRINUSE と「Port 5173 is in use」を解決する](/ja/support/port-already-in-use/)を参照してください。

## 制限事項

- Moonpool は、クラッシュしたサーバーを再起動しません。アプリを停止中と表示し、タブには `[プロセスが終了しました]` が出力されます。
- Moonpool は、Windows ログイン時に自動では起動しません。[Windows ログイン時にスクリプトや開発サーバーを自動で起動する](/ja/guides/start-app-at-windows-login/)を参照してください。

## 関連ページ

- [アプリのフィールド](/ja/apps/fields/): `port`、`openBrowser`、`killMode`。
- [アプリの種類](/ja/apps/types/): `web` で「実行中」を判定する仕組み。
- [停止と再起動](/ja/apps/stop-and-restart/)
- [例](/ja/apps/examples/#web-開発サーバー)
