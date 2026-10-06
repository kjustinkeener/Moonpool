---
title: "解决 Error: listen EADDRINUSE: address already in use :::3000 与 Vite 的 Port 5173 is in use"
description: "解决 Node 的 EADDRINUSE 和 Vite 的 Port 5173 is in use：找出占用端口的进程，释放它，并用 Moonpool 的 port 与 killMode 字段避免再次发生。"
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

这个 Node.js 错误表示已经有另一个进程在监听 3000 端口（`:::` 是“所有地址”的 IPv6 写法；你也可能看到 `127.0.0.1:3000`）。常见的原因是你早先启动过同一个服务器，却一直没有停止它。

Vite 对同样的情况处理方式不同。默认情况下它会打印：

```text
Port 5173 is in use, trying another one...
```

（端口 5173 已被占用，正在尝试另一个端口……）然后在下一个空闲端口上启动，所以服务器是起来了，但不在你预期的位置。如果使用 `--strictPort`（或 `server.strictPort: true`），Vite 会改为退出，并显示 `Error: Port 5173 is already in use`。

## 自己解决

1. 找到占用该端口的进程并结束它。在 Windows 上：

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   带 PowerShell 版本的分步说明见[查找并结束占用端口的进程](/zh-hans/guides/find-and-kill-process-using-port-windows/)。
2. 或者让你的服务器改用另一个端口，例如对许多 Node 服务器用 `PORT=3001`，对 Vite 用 `--port 5174`。

## Moonpool 如何帮忙

如果你通过 Moonpool 运行服务器，请在它的条目上设置 `port`。这样 Moonpool 会：

- 只要有东西在该端口上应答，就把应用显示为运行中，所以占着端口的遗留服务器会显示为运行中，但不是“由 Moonpool 管理”；
- 在 **停止** 和 **重启** 时，当 `killMode` 为 `port`（这是 `web` 应用的默认值）时，结束仍在监听 `port` 的任何进程，这样下一次启动时端口是空闲的；
- 标出两个配置了相同 `port` 的应用。

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool 在启动之前不会检查端口。如果端口仍被占用，命令会在应用的终端标签页中打印上面的错误。请按 **停止**（它会释放端口），然后再 **启动**。

对于 Vite，请传入 `--strictPort`，并让 `port` 等于你所要求的端口：

```text title="command"
npm run dev -- --port 5173 --strictPort
```

如果不这样做，Vite 可能会换到 5174，而 Moonpool 仍在盯着 5173，状态圆点就永远不会变成实心。

`killMode` 为 `port` 会结束该端口上的任何进程，所以只对没有其他东西需要的端口使用它。对 Windows 上的 Docker 应用，绝不要使用它。参见 [停止与重启](/zh-hans/apps/stop-and-restart/#windows-上的-docker-应用)。

## 另请参阅

- [应用字段](/zh-hans/apps/fields/)：`port`、`killMode`。
- [停止与重启](/zh-hans/apps/stop-and-restart/)
- [故障排除](/zh-hans/support/troubleshooting/#两个应用使用同一个端口)
- [在 Windows 上让 npm 开发服务器在后台运行](/zh-hans/guides/run-npm-dev-server-in-background-windows/)
