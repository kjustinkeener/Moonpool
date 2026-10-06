---
title: "Windows protected your PC: Moonpool のインストーラーを実行する(SmartScreen)"
description: "moonpool.exe を実行すると Windows SmartScreen が Windows protected your PC を表示することがあります。理由、More info から Run anyway を選ぶ手順、事前確認です。"
---

ダウンロードした `moonpool.exe` を実行すると、Windows が **Windows protected your PC**(Windows によって PC が保護されました)というタイトルの青いボックスを表示することがあります。ボックスには "Microsoft Defender SmartScreen prevented an unrecognized app from starting. Running this app might put your PC at risk."(Microsoft Defender SmartScreen は、認識されないアプリの起動を停止しました。このアプリを実行すると、PC が危険にさらされる可能性があります。)という文が表示されます。

## 表示される理由

SmartScreen は、新しいプログラム、または多くの PC で実行された実績がないプログラムについて警告します。`moonpool.exe` はコード署名されていないため、Windows には信頼できる発行元がなく、初回に警告が出ることがあります。これは評判に基づくチェックであり、ファイルが悪意のあるものだと判定されたわけではありません。

## 対処

1. ボックスの中で **More info**(詳細情報)をクリックします。発行元は "Unknown publisher"(不明な発行元)と表示されます。
2. **Run anyway**(実行)をクリックします。インストールカードが開きます。[インストール](/ja/getting-started/install/)を参照してください。

事前に慎重を期したい場合は、Moonpool の公式サイトまたは GitHub のリリースからだけダウンロードし、ファイル名が `moonpool.exe` であることを確認してください。

## Run anyway ボタンがない場合

管理されている一部の PC では、管理者がこのオプションをオフにしているため、**Run anyway** が表示されません。管理者に相談するか、自分で管理している PC を使ってください。ダウンロードした zip に入っていたファイルは、ブロックが付いていることもあります。ファイルを右クリックして **Properties**(プロパティ)を選び、**Unblock**(許可する)が表示されていればチェックを入れ、**OK** を押してから、もう一度実行してください。

## ウイルス対策ソフトの警告

自分のプロファイルにコピーされ、更新時に自分自身を置き換える、署名のない新しい exe は、ウイルス対策ソフトに引っかかることもあります。お使いのソフトが `moonpool.exe` をブロックしたり隔離したりする場合は、`.moonpool` フォルダーについて許可してください。[Windows](/ja/platforms/windows/#実行する前に)を参照してください。

## 関連項目

- [インストール](/ja/getting-started/install/)
- [Windows](/ja/platforms/windows/)
- [インストーラーにエラーが表示される](/ja/support/troubleshooting/#インストーラーにエラーが表示される)
