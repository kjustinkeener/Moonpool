---
title: "Fix Error: listen EADDRINUSE: address already in use :::3000 and Vite Port 5173 is in use"
description: "Fix Node's EADDRINUSE and Vite's Port 5173 is in use: find what holds the port, free it, and use Moonpool's port and killMode fields to stop it recurring."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

This Node.js error means another process is already listening on port 3000 (the `:::` is the
IPv6 form of "all addresses"; you may also see `127.0.0.1:3000`). Often it is a copy of the
same server you started earlier and never stopped.

Vite handles the same situation differently. By default it prints:

```text
Port 5173 is in use, trying another one...
```

and starts on the next free port, so the server is up but not where you expect. With
`--strictPort` (or `server.strictPort: true`) Vite exits instead, with
`Error: Port 5173 is already in use`.

## Fix it yourself

1. Find the process that owns the port and end it. On Windows:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Step by step, with the PowerShell version, in
   [Find and kill the process using a port](/guides/find-and-kill-process-using-port-windows/).
2. Or start your server on another port, for example `PORT=3001` for many Node servers or
   `--port 5174` for Vite.

## How Moonpool helps

If you run the server through Moonpool, set `port` on its entry. Moonpool then:

- shows the app as Running while something answers on that port, so a leftover server
  holding it appears as running but not "managed by Moonpool";
- on **Stop** and **Restart**, kills whatever still listens on `port` when `killMode` is
  `port`, which is the default for `web` apps, so the next Launch finds the port free;
- flags two apps that are configured with the same `port`.

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

Moonpool does not check the port before it launches. If the port is still taken, the
command prints the error above in the app's terminal tab. Press **Stop** (which frees the
port) and **Launch** again.

For Vite, pass `--strictPort` and keep `port` equal to the port you ask for:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Without it, Vite may move to 5174 while Moonpool keeps watching 5173, and the status dot
never turns solid.

`killMode` `port` kills any process on the port, so use it only for ports nothing else needs.
For Docker apps on Windows, never use it. See
[Stop and restart](/apps/stop-and-restart/#docker-apps-on-windows).

## See also

- [App fields](/apps/fields/): `port`, `killMode`.
- [Stop and restart](/apps/stop-and-restart/)
- [Troubleshooting](/support/troubleshooting/#two-apps-use-the-same-port)
- [Run an npm dev server in the background on Windows](/guides/run-npm-dev-server-in-background-windows/)
