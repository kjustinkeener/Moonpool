---
title: "Moonpool のリリースノートと最近の変更点"
description: "最近の Moonpool リリースでの変更点、動作要件、GitHub にある完全なリリースノートの場所を確認できます。"
---

各リリースの完全なノートは、プロジェクトの[リリースページ](https://github.com/kjustinkeener/Moonpool/releases)にあります。このヘルプは Moonpool に同梱されているため、常に実行しているバージョンの内容を説明しています。Moonpool は自動で更新されます。[更新](/ja/data/updating/)を参照してください。

## 0.3.16

- **複数の Moonpool を同時に実行。** インストール版の Moonpool と任意の数のポータブル版コピーを、フォルダーごとに 1 つずつ並行して実行できます。それぞれが独自のアプリ、トレイアイコン、制御チャネルを持ちます。[ポータブルモード](/ja/data/portable-mode/#複数のコピーを同時に)を参照してください。
- **テーマブラウザー。** 68 種類のテーマを、それぞれの配色でプレビューできます。[テーマ、言語、透明度](/ja/using/themes-and-language/)を参照してください。
- **そのまま動くサンプル。** 新しい `apps.json` には、そのまま動作するサンプルアプリが入っています。サンプルダッシュボードは、アプリが管理する `dashboards/examples` フォルダーに置かれ、Moonpool とともに更新されます。[サンプルダッシュボード](/ja/getting-started/example-dashboards/)を参照してください。
- **apps.json のエラーを表示。** サイドバーの上にバナーでエラーが表示され、再読み込みに失敗しても、最後に読み込めた一覧が保たれます。[apps.json にエラーがあるとき](/ja/using/hub-window/#appsjson-にエラーがあるとき)を参照してください。
- **Linux の制御チャネル。** Unix ソケットを使い、`list` 動詞も追加されました。[制御コマンド](/ja/automation/control-verbs/)を参照してください。
- 情報画面とアプリエディターが、テーマと言語の変更にリアルタイムで追従するようになりました。**Moonpool をインストール…** メニュー項目は、Windows 以外では非表示になります。

## 0.3.15

- 再起動したアプリは以前の出力を保持し、日付入りの「restarted」区切りが表示されます。[ターミナルタブ](/ja/using/terminal-tabs/#再起動)を参照してください。
- アプリごとに専用の `cli-output` フォルダーが用意され、ログの整理が他のアプリのログに影響しなくなりました。
- `killMode` と `stopCommand` がアプリエディターに追加されました。[停止と再起動](/ja/apps/stop-and-restart/)を参照してください。
- 起動したアプリが Moonpool 自身の WebView2 プロファイルを継承しなくなりました。

## 0.3.14

- セッションのログをセッション間で保持でき、アプリごとにサイズの上限を設定できます。[ログ](/ja/data/logs/)を参照してください。
- 設定にログフォルダー用の「開く」ボタンと「パスをコピー」ボタンを追加しました。
- ヘルプウィンドウのタイトルバーを修正しました。

## 動作要件

- WebView2 を備えた Windows 10 または 11 ([Windows](/ja/platforms/windows/)を参照)。
- WebKitGTK 4.1 と AppIndicator ライブラリーを備えた Linux ([Linux](/ja/platforms/linux/)を参照)。
