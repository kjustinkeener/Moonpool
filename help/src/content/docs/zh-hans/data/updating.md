---
title: "更新 Moonpool 并解决更新失败"
description: "了解 Moonpool 如何检查、下载并应用更新，更新横幅的作用，便携副本和 Linux 如何更新，以及失败时该怎么办。"
---

Moonpool 会自行更新。不需要另外下载安装程序，也不需要点一堆向导。

## 更新如何到来

Moonpool 从项目的 GitHub Releases 获取 `update.json`（Linux 上是 `linux-update.json`），比较版本，并且只提供严格更新的版本。它会在以下时机检查：

- 启动时，除非在[设置](/zh-hans/using/settings/)中关闭了 **启动时检查更新**；
- 每当你在“关于”窗口中按下 **检查更新** 时。这个按钮会直接安装较新的版本并重启 Moonpool。否则它会告诉你已是最新版本，或者显示错误。

“关于”窗口会在名称下方显示你正在运行的版本：

![“关于”窗口的顶部：徽标、名称 (1)，以及它下方的版本行](../../../../assets/screenshots/about-header.png)

1. 名称。它下面一行是版本号和构建日期。

每个下载在应用之前，都会用 Moonpool 的 minisign 签名密钥校验，所以被篡改或损坏的下载会被拒绝。Moonpool 绝不会安装旧版本。

## 更新横幅

启动时如果发现更新，会在主窗口的空白界面上显示为一条横幅：

```text
Moonpool {version} 已发布（你当前是 {current}）。
```

横幅只在没有打开任何应用标签页、并且 CLI 面板处于展开状态时显示。面板折叠时，改为筛选框旁边的箭头闪动。打开了标签页时则完全没有提示。要看到横幅，请关闭所有标签页（并展开面板），或使用“关于”窗口中的 **检查更新**。

点击 **下载并安装**，Moonpool 就会替换自己并重新启动；也可以用 x 关闭横幅。

## 便携副本

便携副本会以同样的方式更新它自己 `.moonpool\` 文件夹中的 `moonpool.exe`。每个副本各自检查和更新。该文件夹必须可写，所以位于只读 U 盘或共享上的副本无法自行更新；请手动把较新的 `moonpool.exe` 复制过去覆盖。

## Linux

只有 AppImage 会自行更新。它会就地替换 AppImage 文件，所以请把它放在你有写入权限的文件夹中。`.deb` 或 RPM 安装由你的包管理器更新：在 Moonpool 中安装会失败，并提示

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

参见 [Linux](/zh-hans/platforms/linux/#更新)。

## 更新失败时

横幅会显示原因，按钮也会重新可用，让你重试：

```text
更新失败：<error>
```

| 错误包含 | 可能的原因 | 该怎么做 |
| --- | --- | --- |
| `download failed` | 没有网络连接、使用了代理，或 GitHub 限制了请求 | 等一会儿再重试，或手动更新。 |
| `signature verification FAILED - refusing to install` | 下载的文件已损坏或被改动 | 重试。如果一直失败，请从 Releases 页面手动更新。 |
| `rename self aside` 或 `write new exe` | 文件夹是只读的，或杀毒软件占用了该文件 | 让文件夹可写，或在杀毒软件中放行 `moonpool.exe`，然后重试。 |
| `refusing to install ... not newer than current` | 提供的版本并不比当前更新 | 无需处理。 |

### 手动更新

退出 Moonpool，从项目的 [Releases 页面](https://github.com/kjustinkeener/Moonpool/releases)下载 `moonpool.exe`，并复制覆盖旧的那个：安装版是 `%USERPROFILE%\.moonpool\moonpool.exe`，便携副本则是你 `.moonpool\` 文件夹中的那个。你的配置文件夹不会被动。在 Linux 上，替换 AppImage，或使用你的包管理器。

## 帮助也会一起更新

这份帮助随 Moonpool 一起发布，所以每次程序更新都会带上与之匹配的帮助。离线副本始终与你运行的版本一致。

## 另请参阅

- [新增内容](/zh-hans/getting-started/whats-new/)
- [设置窗口](/zh-hans/using/settings/)
