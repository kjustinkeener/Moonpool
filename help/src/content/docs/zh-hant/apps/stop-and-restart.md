---
title: "停止開發伺服器及其啟動的所有處理程序"
description: "用 killMode 與 stopCommand 讓「停止」與「重新啟動」乾淨地結束應用程式及其子處理程序，包括各類型的預設行為與 Windows 上的 Docker。"
---

「停止」一律先做這件事：Moonpool 會結束它為該應用程式啟動的終端機，包括這個終端機啟動的所有東西。對許多應用程式來說，這樣就夠了。

有些應用程式的存活時間比這個終端機更長（桌面視窗會與啟動它的開發伺服器分離，或是某個伺服器子處理程序一直佔著它的連接埠）。**`killMode`** 用來選擇隨後再執行的一個額外步驟。

| `killMode` | 停止時的額外步驟 | 讀取 | 預設用於 |
| --- | --- | --- | --- |
| `processName` | 強制結束所有同名的處理程序。在 Windows 上連同它們的子處理程序一起結束（`taskkill /IM <name>.exe /T /F`）。在其他系統上是 `pkill -KILL -x <name>`：依名稱完全比對且區分大小寫，不含子處理程序。 | `processName` | `desktop` |
| `port` | 強制結束正在監聽 `port` 的任何處理程序。 | `port` | `web` |
| `command` | 在 `cwd` 中執行 `stopCommand`，並等它結束。 | `stopCommand`、`cwd`、`env` | 無 |
| `none` | 什麼都不做。 | 無 | `static`、`cli` |

省略 `killMode` 就會使用該應用程式類型的預設值，只有在停止後仍有東西在執行時才需要設定它。

![「編輯應用程式」對話方塊中的 killMode 下拉選單，設為「預設（依類型）」，其提示行列出了各類型預設的做法](../../../../assets/screenshots/edit-app-killmode.png)

1. `killMode` 下拉選單。「預設（依類型）」等同於省略這個索引鍵。

- 如果該模式需要的欄位為空（例如 `port` 模式卻沒有 `port`），額外步驟會被略過，這不是錯誤。
- `killMode` 與 `type` 彼此獨立：`port` 可用於 `cli` 應用程式，`processName` 可用於 `web` 應用程式。
- 空字串或無法辨識的值不會執行任何額外動作，也不會退回到類型預設值。

對於桌面應用程式，`processName` 模式執行的相當於：

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## 多個 Moonpool，或你自己的處理程序

`processName` 與 `port` 並不知道處理程序是誰啟動的。`processName` 會結束所有同名的處理程序，`port` 會結束監聽該連接埠的任何處理程序，包括另一個 Moonpool 副本啟動的處理程序（已安裝的副本與可攜副本各自獨立執行，請參閱[可攜模式](/zh-hant/data/portable-mode/#同時執行多個副本)）以及你自己啟動的處理程序。請只對不會這樣衝突的應用程式使用這些模式：也就是電腦上沒有其他東西使用的名稱或連接埠。如果兩個副本登錄了同一個應用程式，或你還會手動執行它，請替它設定 `killMode` 為 `none`，或設定一個只停止它自己執行個體的 `command`。

## stopCommand

僅在 `killMode` 為 `command` 時使用。它在 Windows 上透過 `cmd /c`、在其他系統上透過 `$SHELL -c` 執行，工作目錄為 `cwd`，並帶上你的 `env`。其中可以使用 `{MP_HOME}` 與 `{MP_DATA}`。Moonpool 會等它結束後再做其他事，所以重新啟動絕不會在它還在執行時就再次啟動。它的結束代碼會被忽略。如果 60 秒後它仍在執行，Moonpool 會結束它與它的子處理程序，然後繼續。

## 重新啟動

重新啟動就是先停止，再用同一個 `command` 啟動。Moonpool 最多等待 4 秒，讓舊的執行個體顯示為已停止（這樣它的連接埠就空出來了），然後再重新啟動。只有 `url` 的 `static` 項目沒有可停止的東西：重新啟動只是再次開啟頁面。

## Windows 上的 Docker 應用程式

請使用 `none`，或使用 `command` 加上一條真正的停止命令，例如 `docker compose stop app`。不要使用 `port`。

Docker Desktop 透過一個共用的背景處理程序發布每個容器的連接埠。在 Windows 上，「正在監聽該連接埠的處理程序」就是那個共用的處理程序，所以 `port` 模式會強制結束 Docker Desktop，讓所有容器都停掉，而不只是這個應用程式。作為保險，Moonpool 拒絕依連接埠結束一份固定清單中的 Windows 共用處理程序：Docker Desktop 的後端、代理與服務處理程序、`dockerd`、`vpnkit`、WSL 主機處理程序，以及 `svchost` 之類的核心系統處理程序。這不能取代選擇正確的模式。

如果你的 `command` 本身就會重新建立容器（`docker compose up -d --build`），那麼 `none` 是正確的選擇：重新啟動只是再執行一次它。

另請參閱[找出並結束佔用連接埠的處理程序](/zh-hant/guides/find-and-kill-process-using-port-windows/)與[解決 EADDRINUSE 與 "Port 5173 is in use"](/zh-hant/support/port-already-in-use/)。

## 範例

一個有時會留下佔著連接埠的 node 處理程序的開發伺服器（這是 `web` 的預設行為，這裡明確寫出）：

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

一個 Docker Compose 應用程式：

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
