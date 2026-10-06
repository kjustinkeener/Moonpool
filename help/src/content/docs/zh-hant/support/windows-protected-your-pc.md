---
title: "Windows 已保護您的電腦：仍要執行 Moonpool 安裝程式 (SmartScreen)"
description: "執行 moonpool.exe 時 Windows SmartScreen 會顯示「Windows 已保護您的電腦」。說明出現的原因、如何依序按「其他資訊」與「仍要執行」，以及該先檢查什麼。"
---

執行下載的 `moonpool.exe` 時，Windows 可能會跳出一個藍色對話方塊，標題是 **Windows 已保護您的電腦**，並寫著「Microsoft Defender SmartScreen 已防止無法辨識的應用程式啟動。執行此應用程式可能會使您的電腦面臨風險。」（英文原文："Microsoft Defender SmartScreen prevented an unrecognized app from starting. Running this app might put your PC at risk."）

## 出現的原因

SmartScreen 會對新的程式，或是它沒看過在很多電腦上執行過的程式發出警告。`moonpool.exe` 沒有程式碼簽章，Windows 找不到可以信任的發行者，所以第一次執行時可能會顯示這個警告。這只是信譽檢查，並不代表該檔案是惡意的。

## 該怎麼做

1. 在對話方塊中按一下 **其他資訊**。發行者會顯示為「不明的發行者」（英文介面為 "Unknown publisher"）。
2. 按一下 **仍要執行**。安裝卡片隨即開啟。請參閱[安裝](/zh-hant/getting-started/install/)。

如果想先謹慎一點，請只從 Moonpool 的官方網站或它的 GitHub releases 下載，並確認檔名是 `moonpool.exe`。

## 如果沒有「仍要執行」按鈕

在某些受管理的電腦上，系統管理員關閉了這個選項，你就看不到 **仍要執行**。請洽詢你的系統管理員，或改用你自己管理的電腦。從下載的 zip 壓縮檔中取出的檔案也可能帶有封鎖標記：在該檔案上按右鍵，選擇 **內容**，如果出現 **解除封鎖** 就勾選它，再按 **確定**，然後重新執行。

## 防毒軟體警告

一個會把自己複製到你的使用者設定檔、更新時又取代自身的全新未簽署執行檔，同樣可能觸發防毒軟體。如果你的防毒軟體封鎖或隔離了 `moonpool.exe`，請針對 `.moonpool` 資料夾加以放行。請參閱 [Windows](/zh-hant/platforms/windows/#執行之前)。

## 另請參閱

- [安裝](/zh-hant/getting-started/install/)
- [Windows](/zh-hant/platforms/windows/)
- [安裝程式顯示錯誤](/zh-hant/support/troubleshooting/#安裝程式顯示錯誤)
