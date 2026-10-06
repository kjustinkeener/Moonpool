---
title: "WebView2 ランタイムがない: Windows で Moonpool のウィンドウが空白または開かない"
description: "Windows で Moonpool のウィンドウが開かない、または空白のままの場合は、Microsoft Edge WebView2 ランタイムがない可能性があります。確認方法と導入方法です。"
---

Windows で Moonpool のウィンドウが開かない、または開いても空白のままの場合、原因は Microsoft Edge WebView2 ランタイムがないことである可能性が高いです。Moonpool は Tauri アプリで、そのウィンドウは WebView2 が描画する Web ページです。

WebView2 は Windows 11 と最新の Windows 10 に付属しているため、ほとんどの PC にはすでにあります。古い Windows 10 や機能を削った Windows 10、または削除された PC では、ない場合があります。Moonpool のソースコードには、この場合専用のメッセージは見当たらないため、ここにはエラーメッセージを載せていません。症状は、ウィンドウが表示されないか、中身が空であることです。

## インストールされているか確認する

PowerShell で、レジストリからランタイムのバージョンを調べます(1 つ目のパスはシステム全体へのインストール、2 つ目はユーザーごとのインストールです)。

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

`120.0.2210.91` のようなバージョン番号が表示されれば、インストールされています。両方でエラーになる場合は、インストールされていません。

## インストールする

Microsoft の WebView2 のページ("WebView2 Runtime download" で検索)から **Evergreen** 版の WebView2 ランタイムをダウンロードし、インストーラーを実行してから、Moonpool をもう一度起動します。Evergreen ランタイムは、自動的に更新されます。

## インストール済みなのにウィンドウが空白のままの場合

- トレイからすべての Moonpool を終了し(またはタスクマネージャーで `moonpool.exe` を終了し)、もう一度起動します。
- 開ける場合は、[設定](/ja/using/settings/)で**デバッグ情報をファイルに記録**をオンにして、`moonpool.log` を確認します。[ログ](/ja/data/logs/)を参照してください。
- ウィンドウは開くものの画面の外にある場合は、[ウィンドウの問題](/ja/support/troubleshooting/#ウィンドウの問題)を参照してください。

## 関連項目

- [Windows](/ja/platforms/windows/#実行する前に)
- [インストール](/ja/getting-started/install/)
- [トラブルシューティング](/ja/support/troubleshooting/)
