---
title: "试用 Moonpool 自带的示例仪表盘"
description: "打开随程序附带的离线示例仪表盘，了解它们存放在哪里、示例应用如何引用它们，以及如何把它们添加到已有的配置中。"
---

Moonpool 在程序内自带一组独立的仪表盘。它们完全离线运行，不需要服务器，也不依赖 CDN。

| 仪表盘 | 说明 |
| --- | --- |
| CSV explorer | 拖入 CSV 或 TSV 文件，它会分析各列并显示数据。 |
| JSON explorer | 拖入 JSON（数组、嵌套对象或映射）。 |
| Excel explorer | 拖入 `.xlsx` 或 `.xls` 文件，离线解析。 |
| Moonpool Docs | 离线的 Markdown 文档浏览器。 |

## 它们存放在哪里

Moonpool 在启动时会把这些仪表盘写入 `{MP_HOME}\dashboards\examples`：

| 模式 | 文件夹 |
| --- | --- |
| 已安装（Windows） | `%USERPROFILE%\.moonpool\dashboards\examples` |
| 便携 | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples`（或 `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`） |

`examples` 文件夹归 Moonpool 所有：每次 Moonpool 更新时它都会被替换，因此你在里面做的修改
会丢失。若要自定义某个仪表盘，请把它的文件夹和共用的 `_lib` 文件夹复制到上一级的
`dashboards` 中，再让你的应用指向这份副本。Moonpool 绝不会改动 `dashboards` 中的其他任何内容。

0.3.16 之前的版本会把示例直接写入 `dashboards`。这些副本会保留在原处，不再获得更新；
指向它们的应用仍可继续使用。若想获得更新后的版本，请把它们的 `url` 改为下面的
`dashboards/examples/...` 路径。

## 应用如何引用它们

每个都是一个 `static` 应用，其 `url` 是以 `{MP_HOME}` 为基准的 `file:///` 网址：

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` 会解析为安装文件夹；在便携模式下则是整个便携包所在的文件夹，所以这条配置在
便携包被移动之后依然有效。`file://` 网址是允许使用的。参见
[路径与环境](/zh-hans/apps/paths-and-environment/)。

## 示例应用只在首次运行时出现

只有在尚不存在配置文件时，示例条目才会被写入 `apps.json`。如果你已经有 `apps.json`，
请自行添加这些仪表盘条目（在“...”菜单中选择 **编辑 apps.json**，然后选择 **重新加载**）。
把下面这四项加到最外层的数组中，并用逗号与其他条目隔开：

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

各字段的含义见[应用字段](/zh-hans/apps/fields/)。

## 另请参阅

- [示例](/zh-hans/apps/examples/)：更多可直接复制的完整条目。
- [应用类型](/zh-hans/apps/types/#static)
