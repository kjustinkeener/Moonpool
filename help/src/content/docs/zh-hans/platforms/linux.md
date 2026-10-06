---
title: "在 Linux 上安装和使用 Moonpool"
description: "在 Linux 上安装 Moonpool，解决 GNOME 托盘的限制，了解更新方式，并查看它与 Windows 版本在功能上的差异。"
---

Moonpool 通过 WebKitGTK 在 Linux 上运行。它主要在 Windows 上开发，因此 Linux 受支持，但经受的实战检验较少。Linux 上没有安装卡片，也没有便携模式选择器，“...”菜单中也没有 **安装 Moonpool...** 项。

## 安装

从项目的 Releases 页面下载一个安装包。

| 安装包 | 更新方式 |
| --- | --- |
| AppImage | Moonpool 自行更新 |
| `.deb` | 通过你的包管理器 |
| RPM（用你所用发行版的 RPM 工具安装） | 通过你的包管理器 |

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

`.deb` 会自动装上它的运行时依赖。AppImage 需要系统中已有 WebKitGTK 和 AppIndicator 库，例如在 Debian 或 Ubuntu 上：

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

在 Fedora 或 Arch 上请使用对应的包：

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## GNOME 上的托盘

原生 GNOME 不显示托盘图标，所以在安装并启用 AppIndicator 扩展之前，Moonpool 的托盘图标不会出现：

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

然后注销并重新登录。没有它时，主窗口和内嵌终端依然可以正常使用。KDE、Cinnamon、XFCE 和 MATE 开箱即可显示托盘。

## 更新

只有 AppImage 会自行更新。它从 GitHub Releases 读取 `linux-update.json`，校验 minisign 签名，并就地替换 AppImage 文件，所以请把它放在你有写入权限的文件夹中。Moonpool 绝不会覆盖 `.deb` 和 RPM 安装：更新检查仍可能提示有新版本，但在 Moonpool 中安装会失败，并提示你改用包管理器。参见[更新](/zh-hans/data/updating/)。

## 配置位置

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

首次运行时，`apps.json` 由示例文件生成。参见[配置概览](/zh-hans/apps/apps-json/)。

## 与 Windows 的差异

- 启动命令通过 `$SHELL -c <command>` 运行（如果未设置 `SHELL`，则用 `/bin/sh`），所以请使用你的 shell 能识别的语法。
- 停止会结束整个进程组，然后执行由 `killMode` 选定的额外清理。在 `killMode: "port"` 下释放端口会使用 `lsof`，不可用时退回到 `fuser`；如果你的发行版没有自带 `lsof`，请自行安装。参见[停止与重启](/zh-hans/apps/stop-and-restart/)。
- `desktop` 应用的 `processName` 不能超过 15 个字符。Linux 会把进程名截断到 15 个字符，所以更长的名称永远不会被检测为运行中，也无法按名称停止。`web` 应用按端口匹配，不受影响。
- 图标会从应用的 `src-tauri/icons/`、`public/favicon.*`、`icon.png` 或它的在线 favicon 中查找。从二进制文件中提取图标仅限 Windows。
- 各个“打开所在位置”按钮会打开所在文件夹，而不是选中该文件。
- 配置文件会用你的默认文本编辑器打开（根据 `text/plain` 关联解析）。
- Windows 安装程序、快捷方式和“添加/删除程序”条目均不适用。

## 另请参阅

- [Windows](/zh-hans/platforms/windows/#各平台的差异)：按平台列出差异的表格。
- [更新](/zh-hans/data/updating/#linux)
