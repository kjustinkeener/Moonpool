---
title: "Windows で Moonpool を使う"
description: "Windows は Moonpool の主要なプラットフォームです。インストール方法と、Windows と Linux の違いをまとめた表で、何が期待できるかを確認できます。"
---

Windows は Moonpool の主要なプラットフォームです。[インストール](/ja/getting-started/install/)の説明に従ってインストールしてください。

## 実行する前に

- **SmartScreen。** `moonpool.exe` はコード署名されていないため、初回に Windows が「Windows protected your PC」(Windows によって PC が保護されました) と表示することがあります。**More info** (詳細情報) を選び、続けて **Run anyway** (実行) を選びます。
- **ウイルス対策ソフト。** 署名のない新しい exe が自分自身をコピーし、更新時に自分を置き換える動作は、ウイルス対策ソフトに検出されることがあります。お使いのソフトが `moonpool.exe` をブロックまたは隔離する場合は、`.moonpool` フォルダーを許可してください。
- **WebView2。** Moonpool のウィンドウは Microsoft Edge WebView2 を使います。これは Windows 11 と最新の Windows 10 に付属しています。ウィンドウが空白のままか、まったく開かない場合は、Microsoft から Evergreen WebView2 Runtime をインストールしてください。

詳しくは、[Windows によって PC が保護されました](/ja/support/windows-protected-your-pc/)、[WebView2 ランタイムが見つからない](/ja/support/webview2-runtime-missing/)、[Windows ログイン時にスクリプトや開発サーバーを自動で起動する](/ja/guides/start-app-at-windows-login/)を参照してください。

## トレイ

Windows 11 では、新しいトレイアイコンが、非表示アイコンの領域に入ることがよくあります。タスクバー右側の **^** 矢印をクリックして探し、タスクバーにドラッグすると、表示したままにできます。

## コマンド

- コマンドは `cmd /c` を通して実行されます。`command` の中で二重引用符を入れ子にするのは避けてください。`cmd /c` が壊してしまいます。シェルを開いたままにしたいスクリプトには、スクリプト部分を引用符で囲まずに、`pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` を使います。
- `processName` は、`.exe` の有無や大文字小文字を問わず一致します。
- 停止は、Moonpool が起動したプロセスツリー全体を、そこから切り離されたプロセスも含めて終了します。
- Docker Desktop のアプリには、`killMode` に `none` か `command` が必要で、`port` は使えません。[Windows 上の Docker アプリ](/ja/apps/stop-and-restart/#windows-上の-docker-アプリ)を参照してください。

## プラットフォームごとの違い

| | Windows | Linux |
| --- | --- | --- |
| インストール | 自己インストールする `moonpool.exe`、またはポータブル | AppImage、`.deb`、RPM。インストールカードなし |
| 自己更新 | あり (インストール版とポータブル版) | AppImage のみ |
| 設定フォルダー | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| コマンドのシェル | `cmd /c` | `$SHELL -c` |
| `processName` | 長さの制限なし、`.exe` は省略可、大文字小文字を区別しない | 15 文字以内、大文字小文字を区別する |
| `processName` による停止 | プロセスとその子プロセスを終了 | 完全に同じ名前のプロセスだけを終了 |
| 制御チャネル | 名前付きパイプ | Unix ソケット |
| ウィンドウのスクリーンショット (テスト用) | あり | なし |
| プログラムファイルからのアイコン | あり | なし |
| トレイ | 最初から動作 | AppIndicator が必要。標準の GNOME は拡張機能が必要 |

Linux の詳細は [Linux](/ja/platforms/linux/) のページにあります。
