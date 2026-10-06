---
title: "停止开发服务器及其启动的所有进程"
description: "用 killMode 和 stopCommand 让“停止”和“重启”干净地结束应用及其子进程，包括各类型的默认行为和 Windows 上的 Docker。"
---

“停止”总是先做这件事：Moonpool 结束它为该应用启动的终端，包括这个终端启动的所有东西。对许多应用来说，这就够了。

有些应用的存活时间比这个终端更长（桌面窗口会与启动它的开发服务器脱离，或者某个服务器子进程一直占着它的端口）。**`killMode`** 用来选择随后再执行的一个额外步骤。

| `killMode` | 停止时的额外步骤 | 读取 | 默认用于 |
| --- | --- | --- | --- |
| `processName` | 强制结束所有同名进程。在 Windows 上连同它们的子进程一起结束（`taskkill /IM <name>.exe /T /F`）。在其他系统上是 `pkill -KILL -x <name>`：按名称精确匹配且区分大小写，不含子进程。 | `processName` | `desktop` |
| `port` | 强制结束正在监听 `port` 的任何进程。 | `port` | `web` |
| `command` | 在 `cwd` 中运行 `stopCommand`，并等待它结束。 | `stopCommand`、`cwd`、`env` | 无 |
| `none` | 什么都不做。 | 无 | `static`、`cli` |

省略 `killMode` 就会使用该应用类型的默认值，只有在停止后仍有东西在运行时才需要设置它。

![“编辑应用”对话框中的 killMode 下拉框，设为“默认（按类型）”，其提示行列出了各类型默认的做法](../../../../assets/screenshots/edit-app-killmode.png)

1. `killMode` 下拉框。“默认（按类型）”等同于省略这个键。

- 如果该模式需要的字段为空（例如 `port` 模式却没有 `port`），额外步骤会被跳过，这不是错误。
- `killMode` 与 `type` 相互独立：`port` 可用于 `cli` 应用，`processName` 可用于 `web` 应用。
- 空字符串或无法识别的值不会执行任何额外操作，也不会退回到类型默认值。

对于桌面应用，`processName` 模式执行的相当于：

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## 多个 Moonpool，或你自己的进程

`processName` 和 `port` 并不知道进程是谁启动的。`processName` 会结束所有同名进程，`port` 会结束监听该端口的任何进程，包括另一个 Moonpool 副本启动的进程（已安装的副本和便携副本各自独立运行，参见[便携模式](/zh-hans/data/portable-mode/#同时运行多个副本)）以及你自己启动的进程。只对不会这样冲突的应用使用这些模式：即机器上没有其他东西使用的名称或端口。如果两个副本登记了同一个应用，或者你还会手动运行它，请给它设置 `killMode` 为 `none`，或者设置一个只停止它自己实例的 `command`。

## stopCommand

仅当 `killMode` 为 `command` 时使用。它在 Windows 上通过 `cmd /c`、在其他系统上通过 `$SHELL -c` 运行，工作目录为 `cwd`，并带上你的 `env`。其中可以使用 `{MP_HOME}` 和 `{MP_DATA}`。Moonpool 会等它结束后再做其他事，所以重启绝不会在它还在运行时就重新启动。它的退出码会被忽略。如果 60 秒后它仍在运行，Moonpool 会结束它及其子进程，然后继续。

## 重启

重启就是先停止，再用同一个 `command` 启动。Moonpool 最多等待 4 秒，让旧实例显示为已停止（这样它的端口就空出来了），然后再重新启动。只有 `url` 的 `static` 条目没有可停止的东西：重启只是再次打开页面。

## Windows 上的 Docker 应用

请使用 `none`，或者使用 `command` 加一条真正的停止命令，例如 `docker compose stop app`。不要使用 `port`。

Docker Desktop 通过一个共享的后台进程发布每个容器的端口。在 Windows 上，“正在监听该端口的进程”就是那个共享进程，所以 `port` 模式会强制结束 Docker Desktop，让所有容器都停掉，而不只是这个应用。作为兜底，Moonpool 拒绝按端口结束一份固定清单里的 Windows 共享进程：Docker Desktop 的后端、代理和服务进程、`dockerd`、`vpnkit`、WSL 宿主进程，以及 `svchost` 之类的核心系统进程。这不能替代选择正确的模式。

如果你的 `command` 本身就会重新创建容器（`docker compose up -d --build`），那么 `none` 是正确的选择：重启只是再运行一次它。

另请参阅[查找并结束占用端口的进程](/zh-hans/guides/find-and-kill-process-using-port-windows/)和[解决 EADDRINUSE 与 "Port 5173 is in use"](/zh-hans/support/port-already-in-use/)。

## 示例

一个有时会留下占着端口的 node 进程的开发服务器（这是 `web` 的默认行为，这里显式写出）：

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

一个 Docker Compose 应用：

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
