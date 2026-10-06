---
title: "settings.json を理解し、壊れた場合に修復する"
description: "Moonpool の settings.json の構造、Moonpool が自動で書き込むキー、ファイルが壊れたときの読み方と修復方法を説明します。"
---

アプリ全体の設定は、設定フォルダーの `settings.json` にあります([設定の保存場所](/ja/apps/apps-json/#設定の保存場所)を参照)。設定は[設定ウィンドウ](/ja/using/settings/)で変更します。そこには、すべての設定が JSON のキーと既定値とともに一覧になっています。ログとその保持については、[ログ](/ja/data/logs/)のページにあります。

## 構造

1 つの JSON オブジェクトです。省略したキーは既定値になります。

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| キー | 既定値 |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0`(0 から 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10`(最小 1) |

## 自動で書き込まれるキー

Moonpool は、UI の拡大率(`uiScale`、0.5 から 3.0)と、解決された言語(`localeResolved`)もこのファイルに保存します。どちらも自分で設定する必要はありません。テーマはここにはありません。WebView のストレージに保存されます([テーマ、言語、透明度](/ja/using/themes-and-language/)を参照)。

## 読み込みと修復

Moonpool は起動時にファイルを読み込みます。実行中に加えた編集は反映されないため、先に終了してください。

ファイルの形式が不正な場合、Moonpool は既定値で起動し、設定の変更を拒否します。エラーは `Repair settings.json and restart Moonpool before changing settings` で終わります。ファイルを修正するか、すべての設定をリセットするには削除して、Moonpool を再び起動してください。
