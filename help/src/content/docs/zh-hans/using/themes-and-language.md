---
title: "更改 Moonpool 的主题、语言和透明度"
description: "选择颜色主题和界面语言，设置背景透明度和界面缩放，并看到它们立即应用到每个打开的 Moonpool 窗口。"
---

主题、语言和透明度都在[设置窗口](/zh-hans/using/settings/)中设置。三者都会立即应用到每个打开的 Moonpool 窗口。

![设置顶部的语言 (1) 和主题 (2) 选择器](../../../../assets/screenshots/settings-language-theme.png)

1. 语言选择器。
2. 主题按钮。它显示当前主题的名称，并打开主题浏览器。

## 主题

主题浏览器是一个独立的窗口。每个主题有一张预览卡片，用该主题自己的颜色绘制（文字、面板、输入框、按钮、状态圆点、仪表渐变和 16 色终端配色），并按基础、霓虹、暖色、冷色、绿色系、中性、浅色、腮红、明亮、浅粉彩和粉彩分组。点击一张卡片即可应用：每个打开的 Moonpool 窗口会同时改变，并且选择会被保存。这个窗口会保持打开，方便你比较；按 Esc 关闭。

共有 68 个主题外加“自动”，其中约一半是浅色的。主题名称是专有名词，不翻译；只有“自动（跟随系统）”、“深色”和“浅色”会翻译。

**自动（跟随系统）** 会跟随操作系统的浅色或深色偏好，并在系统切换时实时切换。其他任何选择都是固定的。终端的 16 种 ANSI 颜色也会跟随主题。

如果你保存过旧版本的主题，它会被保留。保存的名称如果 Moonpool 已不认识，就会退回到“自动”。有些标签与以前不同（例如 Matrix 现在标为 Terminal，Nord 是 Arctic，Dracula 是 Nocturne，Gruvbox 是 Retro，Solarized 是 Solar），但保存的选择本身不变。

主题保存在 webview 的 `localStorage` 中，而不是 `settings.json`。如果存储不可用，就退回到“自动”。

```text
localStorage key: moonpool.theme
```

## 语言

“自动”跟随操作系统的语言。否则请从 14 种语言中选一种，每种都用它自己的语言显示：

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

选择器会立即应用到主窗口和其他窗口。所选语言以 `locale` 保存在 `settings.json` 中。

## 透明度

**背景透明度** 让窗口背景变得半透明，范围从 0%（不透明）到 90%。

- 把指针悬停在窗口上会立刻让它变为完全不透明。指针离开后，它会在大约 2 秒内渐变回你设置的值。
- 终端跟随相同的色调，而不会自己再加一层。
- 每个窗口（主窗口、设置、关于、应用编辑器和主题浏览器）都会自行应用该设置，而且你拖动滑块时，设置窗口会实时更新其他窗口。

## 界面缩放

用 Ctrl + 鼠标滚轮缩放整个界面。没有键盘缩放。参见[快捷键与缩放](/zh-hans/using/keyboard-shortcuts/)。

## 另请参阅

- [设置窗口](/zh-hans/using/settings/)
- [快捷键与缩放](/zh-hans/using/keyboard-shortcuts/)
