---
title: "在应用中使用路径、MP_HOME 令牌和环境变量"
description: "在应用条目中使用 {MP_HOME} 和 {MP_DATA} 令牌以及 ./ 相对路径，了解哪些字段会展开它们，并设置 env 和工作文件夹。"
---

## 令牌

| 令牌 | 展开为 |
| --- | --- |
| `{MP_HOME}` | 便携模式：存放 `moonpool.exe` 的文件夹（即 `.moonpool\` 文件夹）。Windows 安装版：`%USERPROFILE%\.moonpool`。Linux：`$XDG_CONFIG_HOME/Moonpool`，否则为 `~/.config/Moonpool`，与 `{MP_DATA}` 是同一个文件夹。 |
| `{MP_DATA}` | 配置文件夹，也就是存放 `apps.json` 的那个文件夹。 |

无法解析的令牌会原样保留。

## 哪些字段会展开

| 字段 | 令牌 | 开头的 `./` 或 `.\` |
| --- | --- | --- |
| `cwd` | 是 | 是，以 `{MP_HOME}` 为基准 |
| `command` | 是 | 否 |
| `stopCommand` | 是 | 否（它在 `cwd` 中运行，而 `cwd` 已有基准） |
| `url` | 是 | 否 |
| `icon` | 是 | 是，以 `{MP_HOME}` 为基准 |
| `env` 的值、`processName`、`note` | 否 | 否 |

不带 `./` 的相对路径（例如 `apps\tool`）不会被处理，而是相对于 Moonpool 自己的工作文件夹解析，这很少是你想要的。请优先使用 `./` 或令牌。

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

搬动便携文件夹后，这两种写法都仍然有效。像 `C:\tools\notes` 这样的固定路径则无法随之迁移。在便携模式下，“编辑应用”对话框会给绝对路径的 `cwd` 和 `url` 值打上“不便携”标记。参见[便携模式](/zh-hans/data/portable-mode/)。

## 环境

`env` 是一个由字符串组成的对象。对话框以每行一个 `KEY=VALUE` 的形式编辑它；它在每行的第一个 `=` 处拆分，去掉两侧空白，并忽略不含 `=` 的行。

在对话框中：

```text
PORT=8091
NODE_ENV=development
```

在 `apps.json` 中，作为条目的 `env` 键：

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- 被启动的命令会继承 Moonpool 的环境，再加上 `env`。`env` 中的条目优先。
- `env` 同样会应用到 `stopCommand`。
- 值按原样使用：Moonpool 不会展开 `{MP_HOME}`，也不会展开 `%VAR%`。
- Moonpool 通过 `WEBVIEW2_USER_DATA_FOLDER` 把它自己的 WebView2 指向一个私有的配置文件夹。被启动的应用不会继承它。如果你在启动 Moonpool 之前自己设置过这个变量，它们拿到的是你的值；否则它是未设置的。`env` 条目仍然可以覆盖它。

## 工作文件夹

命令和 `stopCommand` 都在 `cwd` 中运行。省略 `cwd` 时，命令在 Moonpool 自己的工作文件夹中运行，所以只要涉及相对路径，就请设置 `cwd`。
