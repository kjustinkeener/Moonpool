---
title: "缺少 WebView2 运行时：修复 Windows 上 Moonpool 窗口空白或不出现"
description: "如果 Moonpool 的窗口在 Windows 上始终打不开或一直空白，可能是缺少 Microsoft Edge WebView2 运行时。本文说明如何检查并安装。"
---

如果 Moonpool 的窗口在 Windows 上始终打不开，或者打开后一直空白，很可能是缺少 Microsoft Edge WebView2 运行时。Moonpool 是一个 Tauri 应用，它的窗口都是由 WebView2 渲染的网页。

WebView2 随 Windows 11 和较新的 Windows 10 一同提供，所以大多数电脑已经具备。在较旧或被精简过的 Windows 10，或者被卸载过它的电脑上，它可能缺失。Moonpool 自己的源代码里没有针对这种情况的专门提示，因此这里不引用任何错误文字：症状就是窗口没有出现或内容为空。

## 检查是否已安装

在 PowerShell 中，从注册表里查找该运行时的版本（第一个路径是系统级安装，第二个是当前用户级安装）：

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

出现 `120.0.2210.91` 这样的版本号，说明已经安装。两条命令都报错，说明没有安装。

## 安装

从 Microsoft 的 WebView2 页面下载 **Evergreen** WebView2 运行时（搜索“WebView2 Runtime download”），运行安装程序，然后重新启动 Moonpool。Evergreen 运行时会自动更新。

## 已安装但窗口仍然空白

- 从托盘退出所有 Moonpool（或在任务管理器中结束 `moonpool.exe`），然后重新启动。
- 如果你能打开[设置](/zh-hans/using/settings/)，就打开 **把调试信息写入文件**，然后查看 `moonpool.log`。参见[日志](/zh-hans/data/logs/)。
- 如果窗口能打开但跑到了屏幕之外，参见[窗口问题](/zh-hans/support/troubleshooting/#窗口问题)。

## 另请参阅

- [Windows](/zh-hans/platforms/windows/#运行之前)
- [安装](/zh-hans/getting-started/install/)
- [故障排除](/zh-hans/support/troubleshooting/)
