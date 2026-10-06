---
title: "在 Windows 或 Linux 上安装 Moonpool"
description: "几次点击即可安装 Moonpool，选择安装模式或便携模式，之后还能通过“安装 Moonpool”菜单项再次安装，用完后也可以干净地卸载。"
---

本页针对 Windows。在 Windows 上，Moonpool 自带安装程序：下载的只有一个 `moonpool.exe`。Linux 没有安装卡片，也没有便携模式选择器，参见 [Linux](/zh-hans/platforms/linux/)。

## 安装模式

运行下载的 `moonpool.exe`。首次启动时会显示安装卡片，上面有三个控件：**安装 Moonpool** 按钮、**创建桌面快捷方式** 复选框（默认勾选）和 **安装便携版** 链接。

安装会把 Moonpool 复制到你的用户配置文件下的 `.moonpool\` 中，添加一个开始菜单快捷方式（如果勾选了该选项，还会添加桌面快捷方式），并在“添加/删除程序”中登记一项。随后它会启动已安装的副本并关闭自身。你下载的那个文件还留在原处，可以删除。之后就像启动其他应用一样，通过快捷方式启动 Moonpool。

![安装卡片：“安装 Moonpool”按钮、桌面快捷方式复选框、“安装便携版”链接和安装路径](../../../../assets/screenshots/installer-window.png)

Moonpool 需要的一切都放在这一个文件夹里：程序本体、你的配置，以及随附的帮助。

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## 从菜单中选择“安装 Moonpool...”

在 Windows 上，“...”菜单在两种模式下都有 **安装 Moonpool...**，它会打开同一张安装卡片。从便携副本可以把它正式安装下来。从已安装的副本中，**安装 Moonpool** 会被禁用（显示“已安装”），而 **安装便携版** 仍可使用。

## 卸载

使用 Windows 的“添加/删除程序”（已安装的应用），或者用 `--uninstall` 运行已安装的副本。它不在你的 PATH 中，所以要写出完整路径：

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

这会删除开始菜单和桌面快捷方式、注册表项，以及整个 `%USERPROFILE%\.moonpool` 文件夹，**包括你的配置**（`apps.json`、设置和日志）。如果想保留配置，请先备份这个文件夹：

```text
%USERPROFILE%\.moonpool\moonpool-config
```

卸载时，所有正在运行的 Moonpool 都会被停止。

## 便携模式

更想用 U 盘或可以随意搬动的文件夹？在安装卡片上点击 **安装便携版**，然后选一个文件夹。参见[便携模式](/zh-hans/data/portable-mode/)。

## 下一步

- [Windows 已保护你的电脑](/zh-hans/support/windows-protected-your-pc/)：SmartScreen 拦截了安装程序时看这里。
- [缺少 WebView2 运行时](/zh-hans/support/webview2-runtime-missing/)：窗口一直空白时看这里。
- [你的第一个应用](/zh-hans/getting-started/first-app/)
