---
title: "向 Moonpool 添加应用或开发服务器"
description: "为本地应用或开发服务器登记启动命令、工作文件夹和环境，让 Moonpool 替你启动、停止并监视它。"
---

Moonpool 中的每个应用都是一个条目，包含启动命令、工作文件夹和可选的环境。Moonpool 会在它自己管理的终端中运行这条命令。

## 添加应用

1. 打开侧边栏顶部的 **...** 菜单，选择 **添加应用**。
2. 输入 **name**，并选择 **group**。
3. 选择 **type**：`web`（监听端口的服务器）、`desktop`（原生应用）、`static`（一个页面）或 `cli`（一条命令）。
4. 设置 **command**，以及它运行所在的 **cwd**。
5. 填写该类型需要的内容：web 需要 **port** 和 **url**，desktop 需要 **processName**，static 需要 **url**。只有 `url` 的 `static` 应用不需要 **command** 或 **cwd**。
6. 保存。应用会出现在侧边栏中。使用它的 **启动** 控件来启动。

![应用编辑器中的 type 下拉框 (1) 和 port 字段 (2)，cwd 与 command 位于二者之间](../../../../assets/screenshots/edit-app-type-and-port.png)

1. **type** 下拉框；它的提示说明了该类型的运行方式。
2. **port** 字段，供 `web` 应用使用。

最终结果是 `apps.json` 中的一个条目，例如：

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

点击应用的名称只会打开它的终端标签页，参见[应用状态](/zh-hans/support/glossary/#应用状态)。

## 应用编辑器

- **Group。** 从列表中选一个分组，或选择 **+ 新建分组...** 并输入名称。**返回列表** 会回到列表。留空的分组会保存为 `Apps`。
- **变暗的字段** 表示所选类型用不到它们，但它们仍会被保存。
- **不填名称就保存** 会显示“name 为必填项。”
- 有未保存的更改时按 **Esc** 或关闭编辑器，会询问“放弃你的修改？”。
- 以后要修改某个应用，使用它所在行的铅笔图标，或右键单击它并选择 **编辑**。

## 手动编辑

在同一个菜单中选择 **编辑 apps.json**，保存文件，然后选择 **重新加载**。格式、校验规则和恢复方式见[配置概览](/zh-hans/apps/apps-json/)。

## 接下来看什么

- [应用字段](/zh-hans/apps/fields/)：每个键及其作用。
- [应用类型](/zh-hans/apps/types/)：各类型如何启动并显示“运行中”。
- [停止与重启](/zh-hans/apps/stop-and-restart/)：停止后仍有东西在运行时该设置什么，以及为什么 Docker 应用需要特别留意。
- [路径与环境](/zh-hans/apps/paths-and-environment/)：`{MP_HOME}`、`./` 路径和 `env`。
- [示例](/zh-hans/apps/examples/)：可直接复制的完整条目。
- [操作指南](/zh-hans/guides/run-npm-dev-server-in-background-windows/)：在后台运行开发服务器、Python 脚本、端口。
- [便携模式](/zh-hans/data/portable-mode/)
- [更新](/zh-hans/data/updating/)
