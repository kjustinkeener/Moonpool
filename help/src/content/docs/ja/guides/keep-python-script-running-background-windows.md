---
title: "Windows で Python スクリプトをバックグラウンドで動かし続ける"
description: "Windows で長時間動く Python スクリプトや小さな Web アプリをバックグラウンドで実行し、出力を確認して、きれいに停止します。pythonw と Moonpool の両方を紹介します。"
---

コンソールウィンドウから実行した Python スクリプトは、そのウィンドウを閉じると止まります。Windows での一般的な対処法は、`pythonw.exe` (コンソールウィンドウのない同じインタープリター。出力はどこにも表示されません)、切り離して起動する `Start-Process pythonw -ArgumentList worker.py`、ログイン時やタイマーで実行したいものにはタスクスケジューラです。どの方法でも、止めたいときにタスクマネージャーでプロセスを探すのは自分です。

## Moonpool での方法

Moonpool はコマンドを専用のターミナルタブで実行するので、自分でコンソールウィンドウを用意しなくても、出力と停止ボタンが手に入ります。止めるまで動き続けるスクリプトには、`cli` アプリを使います。`-u` を付けると Python が出力をすぐに書き出すので、タブにリアルタイムで表示されます。

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

起動して、アプリの名前をクリックすると出力を確認できます。`cli` アプリは、コマンドが動いている間は「実行中」と数えられ、スクリプトが終了すると灰色になり、タブには `[プロセスが終了しました]` が残ります。**停止** を押すと、スクリプトとそこから起動されたものすべてが終了します。仮想環境の `python.exe` をパスで指定すれば、アクティベートの手順は不要です。

スクリプトが HTTP を提供する場合 (Flask、FastAPI、`python -m http.server`) は、ポートに応じて「実行中」になるよう `web` アプリにします。

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## 制限事項

- Moonpool を動かし続けてください。既定ではウィンドウを閉じると Moonpool が終了し、Windows では終了すると起動したすべてのアプリが停止します。**閉じたらトレイに格納** をオンにすると、代わりにウィンドウが隠れるだけになります。[トレイ、閉じる、最小化](/ja/using/tray-and-closing/)を参照してください。
- Moonpool は、クラッシュしたスクリプトを再起動せず、Windows ログイン時に自動で起動することもありません。[Windows ログイン時にスクリプトや開発サーバーを自動で起動する](/ja/guides/start-app-at-windows-login/)を参照してください。
- `command` の中で二重引用符を入れ子にするのは避けてください。`cmd /c` のラッパーが壊してしまいます。

## 関連ページ

- [アプリの種類](/ja/apps/types/#cli): `cli` アプリと `web` アプリの追跡方法。
- [停止と再起動](/ja/apps/stop-and-restart/)
- [例](/ja/apps/examples/)
- [ログ](/ja/data/logs/): セッションの出力の保存場所。
