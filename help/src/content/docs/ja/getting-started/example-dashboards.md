---
title: "Moonpool に同梱されているサンプルダッシュボードを試す"
description: "同梱のオフラインサンプルダッシュボードを開き、保存場所とサンプルアプリからの参照方法、既存の設定への追加方法を確認します。"
---

Moonpool には、プログラム内に自己完結型のダッシュボードが同梱されています。サーバーも CDN も使わず、完全にオフラインで動作します。

| ダッシュボード | 内容 |
| --- | --- |
| CSV explorer | CSV または TSV ファイルをドロップすると、列を分析してデータを表示します。 |
| JSON explorer | JSON (配列、ネストしたオブジェクト、マップ) をドロップします。 |
| Excel explorer | `.xlsx` または `.xls` ファイルをドロップします。解析はオフラインで行われます。 |
| Moonpool Docs | オフラインの Markdown ドキュメントブラウザーです。 |

## 保存場所

Moonpool は起動時に、ダッシュボードを `{MP_HOME}\dashboards\examples` に書き出します。

| モード | フォルダー |
| --- | --- |
| インストール版 (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| ポータブル | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (または `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

`examples` フォルダーは Moonpool の管理下にあり、Moonpool が更新されるたびに置き換えられるため、そこでの編集は失われます。ダッシュボードをカスタマイズするには、そのフォルダーと共有の `_lib` フォルダーを `dashboards` 直下にコピーし、アプリがそのコピーを指すようにしてください。Moonpool は `dashboards` 内のそれ以外には一切手を加えません。

バージョン 0.3.16 より前は、サンプルが `dashboards` に直接書き出されていました。それらのコピーはそのまま残り、更新は受け取りません。そこを指しているアプリは引き続き動作します。更新版を使うには、`url` を下記の `dashboards/examples/...` パスに変更してください。

## アプリからの参照方法

各ダッシュボードは `static` アプリで、`url` は `{MP_HOME}` を基準にした `file:///` URL です。

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` はインストール先のフォルダー、ポータブルモードではバンドルのフォルダーに解決されるため、バンドルを移動してもエントリーは動作します。`file://` URL は使用できます。[パスと環境変数](/ja/apps/paths-and-environment/)を参照してください。

## サンプルアプリは初回起動時にのみ追加されます

サンプルのエントリーが `apps.json` に書き込まれるのは、設定ファイルがまだ存在しないときだけです。すでに `apps.json` がある場合は、ダッシュボードのエントリーを自分で追加します (「...」メニューの **apps.json を編集**、続けて **再読み込み**)。次の 4 つを、最上位の配列の中に、他のエントリーとカンマで区切って追加してください。

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

各フィールドの意味は[アプリのフィールド](/ja/apps/fields/)にあります。

## 関連ページ

- [例](/ja/apps/examples/): コピーして使える、より完全なエントリー。
- [アプリの種類](/ja/apps/types/#static)
