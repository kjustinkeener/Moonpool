---
title: "在 Windows 登入時自動啟動指令碼或開發伺服器"
description: "用「啟動」資料夾中的捷徑在 Windows 登入時啟動 Moonpool，再用一個小型 PowerShell 指令碼在其中啟動開發伺服器或指令碼。沒有現成的設定可以做到。"
---

Windows 通常有兩種在登入時啟動東西的方式：在「啟動」資料夾中放一個捷徑（按 Win+R，輸入 `shell:startup`，按 Enter），或是建立觸發條件為「登入時」的排定工作。兩種方式都會執行一個程式或指令碼，它可以直接就是你的開發伺服器命令，但那樣就沒有東西替你追蹤它、顯示它的輸出或停止它。

## Moonpool 提供什麼

Moonpool 沒有「登入時啟動」的設定，`apps.json` 中的項目也沒有能在 Moonpool 啟動時就啟動它的欄位（完整清單請見[應用程式欄位](/zh-hant/apps/fields/)與 [settings.json](/zh-hant/data/settings-json/)）。你能做的是自己在登入時啟動 Moonpool，然後讓指令碼去啟動你想要的應用程式，用的是[命令列](/zh-hant/automation/command-line/)提供的同一個動詞。

先照常登錄應用程式：

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

然後把下面的內容儲存為 `start-moonpool-apps.ps1`。安裝版的程式是 `%USERPROFILE%\.moonpool\moonpool.exe`；可攜副本請使用該副本執行檔的路徑。

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

要讓 `launch` 交給 Moonpool 處理，Moonpool 必須已經在執行；如果沒有常駐的執行個體，同樣的命令會啟動一個新的 Moonpool，而該動詞不會被執行。這個延遲是給它啟動留出時間，在較慢的電腦上請調大。每個應用程式加一行 `& $mp launch <id>`。

最後在「啟動」資料夾中放一個指向該指令碼的捷徑，目標如下：

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

要檢查發生了什麼，可以在某個動詞後面加上 `--ticket t1`，然後從 `state.json` 中讀取結果（[讀取結果](/zh-hant/automation/command-line/#讀取結果)）。

## 注意事項

- 以這種方式啟動的開發伺服器，和其他應用程式一樣由 Moonpool「管理」，所以停止與結束對它都有效。如果同一個應用程式已經在執行（例如是手動啟動的），Moonpool 會把它顯示為執行中但不受管理。
- Moonpool 不會重新啟動已結束的應用程式，也不會記住你上次結束時哪些應用程式在執行。

## 另請參閱

- [命令列](/zh-hant/automation/command-line/)
- [系統匣、關閉與最小化](/zh-hant/using/tray-and-closing/)
- [在 Windows 上讓 npm 開發伺服器在背景執行](/zh-hant/guides/run-npm-dev-server-in-background-windows/)
