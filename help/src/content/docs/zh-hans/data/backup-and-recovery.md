---
title: "备份 Moonpool、回滚 apps.json 并恢复配置"
description: "了解该备份什么，如何回滚出错的 apps.json、重置为示例应用、把已安装的配置迁移到便携副本，以及卸载会删除什么。"
---

Moonpool 保存的所有内容都在两个地方：配置文件夹和仪表盘文件夹。各模式对应的路径见[配置位于何处](/zh-hans/apps/apps-json/#配置位于何处)。

## 配置文件夹

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

仪表盘文件夹是 `{MP_HOME}\dashboards`：安装版为 `%USERPROFILE%\.moonpool\dashboards`，便携版为 `<your .moonpool folder>\dashboards`，Linux 上则是配置文件夹内的 `dashboards/`。请备份其中你自己的任何内容。它的 `examples` 文件夹属于 Moonpool，更新时会被重写。

主题保存在窗口的浏览器存储中，而不是你可以复制的文件里。它不会随备份一起迁移；恢复后请重新选择一次。

## 备份

1. 退出 Moonpool，避免有文件只写了一半。
2. 从配置文件夹复制 `apps.json`、`settings.json` 和 `icons\`，并从 `dashboards\` 复制你自己的文件。

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

要恢复，先退出 Moonpool，把文件复制回去，然后启动它。

## 回滚 apps.json

每次成功的保存、代理写入和恢复，以及每次发现内容有变化的重新加载，都会把校验通过的 `apps.json` 复制到 `apps.json.history\` 中，保留最新的 10 份。每个文件以生成时间命名，例如 `1767225600000.json`。没有 `apps.json.bak`。

- **手动。** 把某个快照复制覆盖 `apps.json`，然后选择 **重新加载**。

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **通过脚本。** `moonpool.exe restore-config` 会列出快照；`moonpool.exe restore-config 1` 会恢复最新的那个。参见[命令行](/zh-hans/automation/command-line/)。
- **通过代理。** `moonpool_restore_config`。参见 [MCP 工具](/zh-hans/automation/mcp-tools/#配置)。

不会自动恢复任何东西。

## 文件损坏

- **apps.json。** Moonpool 绝不会覆盖损坏的文件。参见[如果文件有错误](/zh-hans/apps/apps-json/#如果文件有错误)。
- **settings.json。** 修复它，或者删除它以重置所有设置，然后重新启动 Moonpool。参见 [settings.json](/zh-hans/data/settings-json/#读取与修复)。

## 重置为示例

Moonpool 只在没有 `apps.json` 时才写入示例应用。要重新开始，请退出 Moonpool（或让它继续运行），把 `apps.json` 重命名或删除，然后启动 Moonpool 或选择 **重新加载**。会写入一份带有示例的全新 `apps.json`。

## 从安装版迁移到便携版

新的便携副本一开始带的是示例应用。要把你自己的应用带过去，参见[便携模式](/zh-hans/data/portable-mode/#从安装程序选择便携模式)。如果需要，用同样的方式复制 `icons\` 和 `settings.json`。

## 卸载

卸载已安装的 Moonpool 会删除整个 `%USERPROFILE%\.moonpool` 文件夹，包括配置文件夹和仪表盘。请先备份。参见[卸载](/zh-hans/getting-started/install/#卸载)。便携副本只需删除它的 `.moonpool\` 文件夹即可移除。
