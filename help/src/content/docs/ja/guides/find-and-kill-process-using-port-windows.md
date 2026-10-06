---
title: "Windows でポート (3000、5173、8080) を使っているプロセスを見つけて終了する"
description: "netstat や PowerShell で Windows のポート 3000 や 5173 を握っているプロセスを探し、taskkill で終了します。Moonpool でアプリを停止してポートを解放する方法も紹介します。"
---

開発サーバーが「ポートはすでに使用中」というエラーで起動できないときは、別のものがそのポートで待ち受けています。コマンドプロンプトで、待ち受けているものを所有プロセス ID 付きで一覧表示し、終了します。

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

`LISTENING` の行の最後の列が PID です (`findstr :3000` は `:30001` にも一致するので、ローカルアドレスを確認してください)。`tasklist /FI "PID eq 12345"` で、どのプログラムかを確認できます。PowerShell では、同じ調査を次のように行います。

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

`taskkill` に `/T` を付けると、そのプロセスの子プロセスも終了します。別のユーザーやシステムが所有するプロセスには、管理者として実行したウィンドウが必要な場合があります。

## Moonpool での方法

Moonpool 経由で実行するアプリでは、PID を調べる必要はありません。アプリに `port` を設定すれば、停止時にそのポートが解放されます。`web` アプリでは、これが既定の `killMode` です。ここでは明示的に書いています。

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

停止すると、まず Moonpool が起動したターミナルが終了し、そのあと `port` で待ち受けているものがまだあれば強制終了します。Windows では上記と同じ調査 (`Get-NetTCPConnection -LocalPort
<port> -State Listen`) を行い、所有者ごとに `taskkill /PID <pid> /T /F` を実行します。

- 自分が起動していないものがそのポートを握っている場合、Moonpool はそのアプリを実行中と表示しますが、「managed by Moonpool」とは表示しません。そのアプリの **停止** を押してください。`port` の手順は引き続き実行されます。
- Moonpool は、Docker Desktop のバックエンド、`svchost`、WSL ホストなど、共有される Windows のプロセスを固定リストとして、ポート経由では終了しません。Docker アプリでは、`port` ではなく `killMode` に `command` か `none` を使ってください。[Windows 上の Docker アプリ](/ja/apps/stop-and-restart/#windows-上の-docker-アプリ)を参照してください。
- これが使えるのは、`apps.json` に登録されたアプリのポートだけです。それ以外のポートには、冒頭のコマンドを使ってください。
- `port` モードは、手動で起動したコピーも含め、待ち受けているものをすべて終了します。マシン上の他のものが必要としないポートにだけ使ってください。

## 関連ページ

- [EADDRINUSE と「Port 5173 is in use」を解決する](/ja/support/port-already-in-use/)
- [停止と再起動](/ja/apps/stop-and-restart/)
- [アプリのフィールド](/ja/apps/fields/): `port` と `killMode`。
- [2 つのアプリが同じポートを使っている](/ja/support/troubleshooting/#2-つのアプリが同じポートを使っている)
