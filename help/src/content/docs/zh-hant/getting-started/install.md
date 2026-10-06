---
title: "在 Windows 或 Linux 上安裝 Moonpool"
description: "按幾下就能安裝 Moonpool，選擇安裝模式或可攜模式，之後還能透過「安裝 Moonpool」選單項目再次安裝，用完後也能乾淨地解除安裝。"
---

本頁針對 Windows。在 Windows 上，Moonpool 自帶安裝程式：下載的只有一個 `moonpool.exe`。Linux 沒有安裝卡片，也沒有可攜模式選擇器，請參閱 [Linux](/zh-hant/platforms/linux/)。

## 安裝模式

執行下載的 `moonpool.exe`。第一次啟動時會顯示安裝卡片，上面有三個控制項：**安裝 Moonpool** 按鈕、**建立桌面捷徑** 核取方塊（預設勾選）與 **安裝可攜版** 連結。

安裝會把 Moonpool 複製到你的使用者設定檔下的 `.moonpool\` 中，加入一個「開始」功能表捷徑（若勾選了該選項，還會加入桌面捷徑），並在「新增或移除程式」中登錄一個項目。接著它會啟動已安裝的副本並關閉自己。你下載的那個檔案仍留在原處，可以刪除。之後就像啟動其他應用程式一樣，透過捷徑啟動 Moonpool。

![安裝卡片：「安裝 Moonpool」按鈕、桌面捷徑核取方塊、「安裝可攜版」連結與安裝路徑](../../../../assets/screenshots/installer-window.png)

Moonpool 需要的一切都放在這一個資料夾裡：程式本體、你的設定，以及隨附的說明。

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## 從選單選擇「安裝 Moonpool...」

在 Windows 上，「...」選單在兩種模式下都有 **安裝 Moonpool...**，它會開啟同一張安裝卡片。從可攜副本可以把它正式安裝起來。從已安裝的副本中，**安裝 Moonpool** 會被停用（顯示「已安裝」），而 **安裝可攜版** 仍可使用。

## 解除安裝

使用 Windows 的「新增或移除程式」（已安裝的應用程式），或是用 `--uninstall` 執行已安裝的副本。它不在你的 PATH 中，所以要寫出完整路徑：

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

這會移除「開始」功能表與桌面捷徑、登錄檔項目，以及整個 `%USERPROFILE%\.moonpool` 資料夾，**包括你的設定**（`apps.json`、設定與記錄）。如果想保留設定，請先備份這個資料夾：

```text
%USERPROFILE%\.moonpool\moonpool-config
```

解除安裝時，所有正在執行的 Moonpool 都會被停止。

## 可攜模式

比較想用 USB 隨身碟或可以隨意搬動的資料夾？請在安裝卡片上按一下 **安裝可攜版**，然後選擇一個資料夾。請參閱[可攜模式](/zh-hant/data/portable-mode/)。

## 下一步

- [Windows 已保護您的電腦](/zh-hant/support/windows-protected-your-pc/)：SmartScreen 封鎖了安裝程式時請看這裡。
- [缺少 WebView2 執行階段](/zh-hant/support/webview2-runtime-missing/)：視窗一直空白時請看這裡。
- [你的第一個應用程式](/zh-hant/getting-started/first-app/)
