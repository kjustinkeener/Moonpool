---
title: "了解 settings.json 并修复损坏的文件"
description: "了解 Moonpool 的 settings.json 的结构、Moonpool 会替你写入哪些键，以及文件损坏时如何读取并修复它。"
---

应用范围的设置保存在配置文件夹中的 `settings.json` 里（参见[配置位于何处](/zh-hans/apps/apps-json/#配置位于何处)）。请在[设置窗口](/zh-hans/using/settings/)中更改它们，那里列出了每项设置及其 JSON 键和默认值。日志及其保留规则见[日志](/zh-hans/data/logs/)页面。

## 结构

一个 JSON 对象。你省略的键会取默认值：

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| 键 | 默认值 |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0`（0 到 90） |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10`（最小为 1） |

## 替你写入的键

Moonpool 还会把界面缩放（`uiScale`，0.5 到 3.0）和解析出的语言（`localeResolved`）存放在这个文件中。你不需要设置它们。主题不在这里：它保存在 webview 的存储中（参见[主题、语言与透明度](/zh-hans/using/themes-and-language/)）。

## 读取与修复

Moonpool 在启动时读取该文件。运行期间所做的编辑不会被读取；请先退出。

如果文件格式有误，Moonpool 会用默认值启动，并拒绝更改设置。错误信息以 `Repair settings.json and restart Moonpool before changing settings`（请先修复 settings.json 并重启 Moonpool，再更改设置）结尾。请修复文件，或删除它以重置所有设置，然后重新启动 Moonpool。
