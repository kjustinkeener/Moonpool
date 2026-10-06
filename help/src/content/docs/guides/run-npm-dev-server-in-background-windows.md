---
title: "Run an npm dev server in the background on Windows without a terminal window"
description: "Keep npm run dev, Vite or another dev server running on Windows without a console window to babysit, then start, stop and read its output from the tray."
---

A dev server started with `npm run dev` runs in the terminal that launched it, so closing
that window ends it. The plain Windows way to keep it going is a hidden process, for example
`Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden` in PowerShell, but then
you have no output to read and stopping it means hunting for the right `node.exe` (see
[Find and kill the process using a port](/guides/find-and-kill-process-using-port-windows/)).

## The Moonpool way

Moonpool runs the command in its own embedded terminal tab inside the hub window, so there
is no separate console window to keep open. Hide the hub to the tray and the server keeps
running. Add the app once:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

Click the app's **Launch** control. The status dot is solid once `port` answers, and the
browser opens to `url` because of `openBrowser`. Click the app's name to read its output in
its own tab. **Stop** ends the terminal and everything it started, and frees the port
(`killMode` `port` is the default for `web`).

## Keep it running when you close the window

By default the close button quits Moonpool, and on Windows quitting stops every app it
launched. Turn on **Close to tray** in [Settings](/using/settings/), and closing the window
only hides it. The tray icon (or **Show Moonpool**) brings it back. Details are in
[Tray, close and minimize](/using/tray-and-closing/).

## Keep the port predictable

Moonpool decides Running from `port`. Vite moves to the next free port when its port is
taken, which would leave Moonpool watching the wrong one. Pass `--strictPort` so Vite
exits instead, and set `port` to match:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

If the port is already taken, see
[Fix EADDRINUSE and "Port 5173 is in use"](/support/port-already-in-use/).

## Limits

- Moonpool does not restart a server that crashes. It shows the app as stopped and the tab
  prints `[process exited]`.
- Moonpool does not start by itself at Windows login. See
  [Start a script or dev server automatically at Windows login](/guides/start-app-at-windows-login/).

## See also

- [App fields](/apps/fields/): `port`, `openBrowser`, `killMode`.
- [App types](/apps/types/): how Running is decided for `web`.
- [Stop and restart](/apps/stop-and-restart/)
- [Examples](/apps/examples/#web-dev-server)
