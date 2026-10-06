---
title: "在 Windows 登录时自动启动脚本或开发服务器"
description: "用“启动”文件夹中的快捷方式在 Windows 登录时启动 Moonpool，再用一个小型 PowerShell 脚本在其中启动开发服务器或脚本。没有现成的设置可以做到。"
---

Windows 通常有两种在登录时启动东西的方式：在“启动”文件夹中放一个快捷方式（按 Win+R，输入 `shell:startup`，按 Enter），或者创建触发条件为“登录时”的计划任务。两种方式都会运行一个程序或脚本，它可以直接就是你的开发服务器命令，但那样就没有东西替你跟踪它、显示它的输出或停止它。

## Moonpool 提供什么

Moonpool 没有“登录时启动”的设置，`apps.json` 中的条目也没有能在 Moonpool 启动时就启动它的字段（完整列表见[应用字段](/zh-hans/apps/fields/)和 [settings.json](/zh-hans/data/settings-json/)）。你能做的是自己在登录时启动 Moonpool，然后让脚本去启动你想要的应用，用的是[命令行](/zh-hans/automation/command-line/)提供的同一个动词。

先照常登记应用：

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

然后把下面的内容保存为 `start-moonpool-apps.ps1`。安装版的程序是 `%USERPROFILE%\.moonpool\moonpool.exe`；便携副本请使用该副本 exe 的路径。

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

要让 `launch` 交给 Moonpool 处理，Moonpool 必须已经在运行；如果没有常驻的实例，同样的命令会启动一个新的 Moonpool，而该动词不会被执行。这个延迟是给它启动留出时间，在较慢的机器上请调大。每个应用加一行 `& $mp launch <id>`。

最后在“启动”文件夹中放一个指向该脚本的快捷方式，目标如下：

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

要检查发生了什么，可以给某个动词加上 `--ticket t1`，然后从 `state.json` 中读取结果（[读取结果](/zh-hans/automation/command-line/#读取结果)）。

## 注意事项

- 以这种方式启动的开发服务器，和其他应用一样由 Moonpool“管理”，所以停止和退出对它都有效。如果同一个应用已经在运行（比如是手动启动的），Moonpool 会把它显示为运行中但不受管理。
- Moonpool 不会重新启动已退出的应用，也不会记住你上次退出时哪些应用在运行。

## 另请参阅

- [命令行](/zh-hans/automation/command-line/)
- [托盘、关闭与最小化](/zh-hans/using/tray-and-closing/)
- [在 Windows 上让 npm 开发服务器在后台运行](/zh-hans/guides/run-npm-dev-server-in-background-windows/)
