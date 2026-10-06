---
title: "Error: listen EADDRINUSE: address already in use :::3000 と Vite の Port 5173 is in use の対処"
description: "Node の EADDRINUSE と Vite の Port 5173 is in use について、ポートを使っているものの見つけ方と解放方法、Moonpool の port と killMode での再発防止を説明します。"
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

この Node.js のエラーは、別のプロセスがすでにポート 3000 で待ち受けていることを意味します(`:::` は「すべてのアドレス」を表す IPv6 の書き方で、`127.0.0.1:3000` と表示されることもあります)。多くの場合、以前に起動して止め忘れた、同じサーバーのコピーです。

Vite は、同じ状況を別の方法で処理します。既定では次のように出力します。

```text
Port 5173 is in use, trying another one...
```

そして次に空いているポートで起動するため、サーバーは立ち上がっていても、期待した場所ではありません。`--strictPort`(または `server.strictPort: true`)を指定すると、Vite は代わりに終了し、`Error: Port 5173 is already in use` を出力します。

## 自分で解決する

1. ポートを使っているプロセスを見つけて終了させます。Windows では次のようにします。

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   PowerShell 版を含む詳しい手順は、[ポートを使っているプロセスを見つけて終了する](/ja/guides/find-and-kill-process-using-port-windows/)にあります。
2. または、サーバーを別のポートで起動します。たとえば、多くの Node サーバーでは `PORT=3001`、Vite では `--port 5174` を使います。

## Moonpool での対処

Moonpool 経由でサーバーを実行している場合は、そのエントリに `port` を設定してください。すると Moonpool は次のように動作します。

- そのポートで何かが応答している間、アプリを実行中として表示します。そのため、ポートを握ったままの古いサーバーは、実行中ではあるものの "managed by Moonpool"(Moonpool による管理下)ではないものとして表示されます。
- **停止**と**再起動**のとき、`killMode` が `port`(`web` アプリの既定値)であれば、`port` で待ち受けているものをすべて終了させます。そのため、次の起動ではポートが空いています。
- 同じ `port` が設定されている 2 つのアプリに警告を出します。

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool は、起動する前にポートを確認しません。ポートがまだ使われていると、コマンドは上のエラーをアプリのターミナルタブに出力します。**停止**(ポートが解放されます)を押して、もう一度**起動**してください。

Vite では、`--strictPort` を渡し、`port` を要求するポートと同じにしておきます。

```text title="command"
npm run dev -- --port 5173 --strictPort
```

これがないと、Vite が 5174 に移ってしまう一方で、Moonpool は 5173 を監視し続けるため、状態のドットが塗りつぶされることはありません。

`killMode` の `port` は、そのポート上のあらゆるプロセスを終了させます。ほかに何も必要としていないポートにだけ使ってください。Windows の Docker アプリには、絶対に使わないでください。[停止と再起動](/ja/apps/stop-and-restart/#windows-上の-docker-アプリ)を参照してください。

## 関連項目

- [アプリのフィールド](/ja/apps/fields/): `port`、`killMode`。
- [停止と再起動](/ja/apps/stop-and-restart/)
- [トラブルシューティング](/ja/support/troubleshooting/#2-つのアプリが同じポートを使っている)
- [Windows で npm の開発サーバーをバックグラウンドで実行する](/ja/guides/run-npm-dev-server-in-background-windows/)
