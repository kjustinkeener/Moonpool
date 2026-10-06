---
title: "Windows 已保护你的电脑：仍要运行 Moonpool 安装程序 (SmartScreen)"
description: "运行 moonpool.exe 时 Windows SmartScreen 会显示“Windows 已保护你的电脑”。说明出现的原因、如何依次点“更多信息”和“仍要运行”，以及先要检查什么。"
---

运行下载的 `moonpool.exe` 时，Windows 可能会弹出一个蓝色对话框，标题为 **Windows 已保护你的电脑**，并写着“Microsoft Defender SmartScreen 阻止了一个无法识别的应用启动。运行此应用可能会使你的电脑面临风险。”（英文原文："Microsoft Defender SmartScreen prevented an unrecognized app from starting. Running this app might put your PC at risk."）

## 出现的原因

SmartScreen 会对新程序，或没有在很多电脑上运行过的程序发出警告。`moonpool.exe` 没有代码签名，Windows 找不到可以信任的发布者，所以第一次运行时可能会显示这个警告。这只是信誉检查，并不表示该文件是恶意的。

## 该怎么做

1. 在对话框中点击 **更多信息**。发布者会显示为“未知发布者”（英文界面为 "Unknown publisher"）。
2. 点击 **仍要运行**。安装卡片随即打开。参见[安装](/zh-hans/getting-started/install/)。

如果想先谨慎一些，只从 Moonpool 的官方网站或它的 GitHub releases 下载，并确认文件名是 `moonpool.exe`。

## 如果没有“仍要运行”按钮

在某些受管理的电脑上，管理员关闭了这个选项，你就看不到 **仍要运行**。请联系你的管理员，或改用你自己管理的电脑。从下载的 zip 压缩包中取出的文件也可能带有拦截标记：右键单击该文件，选择 **属性**，如果出现 **解除锁定** 就勾选它，再点 **确定**，然后重新运行。

## 杀毒软件警告

一个会把自己复制到你的用户配置文件、更新时又替换自身的全新未签名 exe，同样可能触发杀毒软件。如果你的杀毒软件拦截或隔离了 `moonpool.exe`，请为 `.moonpool` 文件夹放行。参见 [Windows](/zh-hans/platforms/windows/#运行之前)。

## 另请参阅

- [安装](/zh-hans/getting-started/install/)
- [Windows](/zh-hans/platforms/windows/)
- [安装程序显示错误](/zh-hans/support/troubleshooting/#安装程序显示错误)
