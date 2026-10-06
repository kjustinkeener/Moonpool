---
title: "將應用程式或開發伺服器新增到 Moonpool"
description: "為本機應用程式或開發伺服器登錄啟動命令、工作資料夾與環境，讓 Moonpool 替你啟動、停止並監看它。"
---

Moonpool 中的每個應用程式都是一個項目，包含啟動命令、工作資料夾與選用的環境。Moonpool 會在它自己管理的終端機中執行這條命令。

## 新增應用程式

1. 開啟側邊欄頂端的 **...** 選單，選擇 **新增應用程式**。
2. 輸入 **name**，並選擇 **group**。
3. 選擇 **type**：`web`（監聽連接埠的伺服器）、`desktop`（原生應用程式）、`static`（一個頁面）或 `cli`（一條命令）。
4. 設定 **command**，以及它執行所在的 **cwd**。
5. 填寫該類型需要的內容：web 需要 **port** 與 **url**，desktop 需要 **processName**，static 需要 **url**。只有 `url` 的 `static` 應用程式不需要 **command** 或 **cwd**。
6. 儲存。應用程式會出現在側邊欄中。使用它的 **啟動** 控制項來啟動。

![應用程式編輯器中的 type 下拉選單 (1) 與 port 欄位 (2)，cwd 與 command 位於兩者之間](../../../../assets/screenshots/edit-app-type-and-port.png)

1. **type** 下拉選單；它的提示說明了該類型的執行方式。
2. **port** 欄位，供 `web` 應用程式使用。

最終結果是 `apps.json` 中的一個項目，例如：

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

按一下應用程式的名稱只會開啟它的終端機分頁，請參閱[應用程式狀態](/zh-hant/support/glossary/#應用程式狀態)。

## 應用程式編輯器

- **Group。** 從清單中選一個群組，或選擇 **+ 新增群組...** 並輸入名稱。**返回清單** 會回到清單。留空的群組會儲存為 `Apps`。
- **變暗的欄位** 代表所選類型用不到它們，但它們仍會被儲存。
- **沒填名稱就儲存** 會顯示「name 為必填欄位。」
- 有未儲存的變更時按 **Esc** 或關閉編輯器，會詢問「要放棄你的變更嗎？」。
- 之後要修改某個應用程式，請使用它所在列的鉛筆圖示，或在它上面按右鍵並選擇 **編輯**。

## 手動編輯

在同一個選單中選擇 **編輯 apps.json**，儲存檔案，然後選擇 **重新載入**。格式、驗證規則與復原方式請見[設定概觀](/zh-hant/apps/apps-json/)。

## 接下來看什麼

- [應用程式欄位](/zh-hant/apps/fields/)：每個索引鍵及其作用。
- [應用程式類型](/zh-hant/apps/types/)：各類型如何啟動並顯示「執行中」。
- [停止與重新啟動](/zh-hant/apps/stop-and-restart/)：停止後仍有東西在執行時該設定什麼，以及為什麼 Docker 應用程式需要特別留意。
- [路徑與環境](/zh-hant/apps/paths-and-environment/)：`{MP_HOME}`、`./` 路徑與 `env`。
- [範例](/zh-hant/apps/examples/)：可直接複製的完整項目。
- [操作指南](/zh-hant/guides/run-npm-dev-server-in-background-windows/)：在背景執行開發伺服器、Python 指令碼、連接埠。
- [可攜模式](/zh-hant/data/portable-mode/)
- [更新](/zh-hant/data/updating/)
