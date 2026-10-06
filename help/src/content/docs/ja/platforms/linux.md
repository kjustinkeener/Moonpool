---
title: "Linux に Moonpool をインストールして使う"
description: "Linux に Moonpool をインストールし、GNOME のトレイの注意点に対処し、更新の仕組みと、Windows 版との機能の違いを確認します。"
---

Moonpool は、Linux では WebKitGTK を通して動作します。主に Windows で開発されているため、Linux はサポートされていますが、実績は少なめです。Linux にはインストールカードもポータブルモードの選択画面もなく、「...」メニューに **Moonpool をインストール…** の項目もありません。

## インストール

プロジェクトのリリースページからパッケージをダウンロードします。

| パッケージ | 更新 |
| --- | --- |
| AppImage | Moonpool が自分で更新 |
| `.deb` | パッケージマネージャー |
| RPM (ディストリビューションの RPM ツールでインストール) | パッケージマネージャー |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

`.deb` は、実行時に必要な依存関係も一緒にインストールします。AppImage には、WebKitGTK と AppIndicator のライブラリーがあらかじめ必要です。たとえば Debian や Ubuntu では次のようにします。

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

Fedora や Arch では、同等のものを使います。

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## GNOME のトレイ

標準の GNOME はトレイアイコンを表示しないため、AppIndicator 拡張機能をインストールして有効にするまで、Moonpool のトレイアイコンは表示されません。

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

そのあと、ログアウトしてログインし直してください。ハブウィンドウと組み込みターミナルは、これがなくても動作します。KDE、Cinnamon、XFCE、MATE では、最初からトレイが表示されます。

## 更新

自己更新するのは AppImage だけです。GitHub のリリースから `linux-update.json` を読み、minisign の署名を検証し、AppImage ファイルをその場で置き換えます。そのため、書き込みできるフォルダーに置いてください。`.deb` と RPM のインストールは、Moonpool が上書きすることはありません。更新の確認で新しいバージョンが報告されることはありますが、Moonpool からインストールしようとすると、パッケージマネージャーを使うよう促すメッセージとともに失敗します。[更新](/ja/data/updating/)を参照してください。

## 設定の場所

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` は、初回起動時にサンプルから作られます。[設定の概要](/ja/apps/apps-json/)を参照してください。

## Windows との違い

- 起動コマンドは `$SHELL -c <command>` を通して実行されます (`SHELL` が未設定なら `/bin/sh`)。そのため、お使いのシェルが理解できる構文を使ってください。
- 停止はプロセスグループを終了し、そのあと `killMode` で選んだ追加の後始末を行います。`killMode: "port"` でポートを解放するには `lsof` を使い、なければ `fuser` にフォールバックします。ディストリビューションに `lsof` が含まれていなければ、インストールしてください。[停止と再起動](/ja/apps/stop-and-restart/)を参照してください。
- `desktop` アプリの `processName` は 15 文字以内にする必要があります。Linux はプロセス名を 15 文字に切り詰めるため、それより長い名前は実行中として検出されず、名前で停止することもできません。`web` アプリはポートで判定するので、影響を受けません。
- アイコンは、アプリの `src-tauri/icons/`、`public/favicon.*`、`icon.png`、またはライブのファビコンから探します。バイナリからのアイコンの抽出は Windows のみです。
- 「開く」ボタンは、ファイルを選択状態にする代わりに、それを含むフォルダーを開きます。
- 設定ファイルは、既定のテキストエディター (`text/plain` の関連付けから解決) で開きます。
- Windows のインストーラー、ショートカット、「アプリと機能」のエントリーは該当しません。

## 関連ページ

- [Windows](/ja/platforms/windows/#プラットフォームごとの違い): プラットフォームごとの違いの表。
- [更新](/ja/data/updating/#linux)
