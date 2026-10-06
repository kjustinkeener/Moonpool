---
title: "在 Linux 上安裝與使用 Moonpool"
description: "在 Linux 上安裝 Moonpool，處理 GNOME 系統匣的限制，了解更新方式，並查看它與 Windows 版本在功能上的差異。"
---

Moonpool 透過 WebKitGTK 在 Linux 上執行。它主要在 Windows 上開發，因此 Linux 雖有支援，但經過的實戰測試較少。Linux 上沒有安裝卡片，也沒有可攜模式選擇器，「...」選單中也沒有 **安裝 Moonpool...** 項目。

## 安裝

從專案的 Releases 頁面下載一個安裝套件。

| 套件 | 更新方式 |
| --- | --- |
| AppImage | Moonpool 自行更新 |
| `.deb` | 透過你的套件管理員 |
| RPM（用你所用發行版的 RPM 工具安裝） | 透過你的套件管理員 |

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

`.deb` 會一併安裝它的執行階段相依套件。AppImage 則需要系統上已有 WebKitGTK 與 AppIndicator 函式庫，例如在 Debian 或 Ubuntu 上：

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

在 Fedora 或 Arch 上請使用對應的套件：

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## GNOME 上的系統匣

原生的 GNOME 不會顯示系統匣圖示，所以在安裝並啟用 AppIndicator 擴充功能之前，Moonpool 的系統匣圖示不會出現：

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

接著登出再重新登入。沒有它時，主視窗與內嵌終端機仍可正常運作。KDE、Cinnamon、XFCE 與 MATE 則開箱即可顯示系統匣。

## 更新

只有 AppImage 會自行更新。它會從 GitHub Releases 讀取 `linux-update.json`，驗證 minisign 簽章，並就地取代 AppImage 檔案，所以請把它放在你有寫入權限的資料夾中。Moonpool 絕不會覆寫 `.deb` 與 RPM 的安裝：更新檢查仍可能回報有新版本，但在 Moonpool 中安裝會失敗，並提示你改用套件管理員。請參閱[更新](/zh-hant/data/updating/)。

## 設定檔位置

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

第一次執行時，`apps.json` 會由範例檔產生。請參閱[設定概觀](/zh-hant/apps/apps-json/)。

## 與 Windows 的差異

- 啟動命令會透過 `$SHELL -c <command>` 執行（若未設定 `SHELL`，則使用 `/bin/sh`），所以請使用你的 shell 看得懂的語法。
- 停止會結束整個處理程序群組，接著執行由 `killMode` 選定的額外清理。在 `killMode: "port"` 下釋放連接埠會使用 `lsof`，若無法使用則改用 `fuser`；如果你的發行版沒有內建 `lsof`，請自行安裝。請參閱[停止與重新啟動](/zh-hant/apps/stop-and-restart/)。
- `desktop` 應用程式的 `processName` 不得超過 15 個字元。Linux 會把處理程序名稱截斷為 15 個字元，所以較長的名稱永遠不會被偵測為執行中，也無法依名稱停止。`web` 應用程式是依連接埠比對，不受影響。
- 圖示會從應用程式的 `src-tauri/icons/`、`public/favicon.*`、`icon.png` 或它的線上 favicon 中尋找。從執行檔中擷取圖示僅限 Windows。
- 各個「顯示位置」按鈕會開啟所在的資料夾，而不是選取該檔案。
- 設定檔會用你的預設文字編輯器開啟（依 `text/plain` 的關聯解析）。
- Windows 的安裝程式、捷徑與「新增或移除程式」項目均不適用。

## 另請參閱

- [Windows](/zh-hant/platforms/windows/#各平台的差異)：依平台列出差異的表格。
- [更新](/zh-hant/data/updating/#linux)
