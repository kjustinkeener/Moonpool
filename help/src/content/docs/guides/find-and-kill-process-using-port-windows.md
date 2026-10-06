---
title: "Find and kill the process using a port on Windows (3000, 5173, 8080)"
description: "Find which process holds port 3000 or 5173 on Windows with netstat or PowerShell, kill it with taskkill, and let Moonpool free the port when you stop an app."
---

When a dev server fails with a port already in use, something else is listening on that
port. In Command Prompt, list listeners with the owning process ID, then kill it:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

The last column of the `LISTENING` row is the PID (`findstr :3000` also matches `:30001`, so
read the local address). `tasklist /FI "PID eq 12345"` shows which program it is. In
PowerShell the same lookup is:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Add `/T` to `taskkill` to end the process's children as well. Processes owned by another
user or by the system may need an elevated (administrator) window.

## The Moonpool way

For an app you run through Moonpool, you do not look the PID up. Give the app a `port`, and
Stop frees it. For a `web` app that is the default `killMode`, written out here:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Stop first ends the terminal Moonpool started, then force-kills whatever is still listening
on `port`. On Windows that is the same lookup as above (`Get-NetTCPConnection -LocalPort
<port> -State Listen`), followed by `taskkill /PID <pid> /T /F` for each owner.

- If something you did not start is holding the port, Moonpool shows the app as running but
  not "managed by Moonpool". Press **Stop** on it: the `port` step still runs.
- Moonpool refuses to kill a fixed list of shared Windows processes by port, such as Docker
  Desktop's backend, `svchost` and the WSL host. For a Docker app use `killMode` `command`
  or `none`, never `port`. See
  [Docker apps on Windows](/apps/stop-and-restart/#docker-apps-on-windows).
- This only works for ports of apps listed in `apps.json`. For any other port, use the
  commands at the top.
- `port` mode kills whatever listens, including a copy you started by hand, so use it only
  for ports nothing else on the machine needs.

## See also

- [Fix EADDRINUSE and "Port 5173 is in use"](/support/port-already-in-use/)
- [Stop and restart](/apps/stop-and-restart/)
- [App fields](/apps/fields/): `port` and `killMode`.
- [Two apps use the same port](/support/troubleshooting/#two-apps-use-the-same-port)
