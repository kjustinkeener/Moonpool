---
title: "Keep a Python script running in the background on Windows"
description: "Run a long-lived Python script or small web app in the background on Windows, see its output, and stop it cleanly, with pythonw and with Moonpool."
---

A Python script run from a console window stops when you close that window. The usual
Windows fixes are `pythonw.exe` (the same interpreter without a console window, so output
goes nowhere), `Start-Process pythonw -ArgumentList worker.py` to launch it detached, or a
scheduled task for something that should run at login or on a timer. Each leaves you to find
the process in Task Manager when you want it gone.

## The Moonpool way

Moonpool runs the command in its own terminal tab, so you keep the output and a Stop button
without a console window of your own. For a script that runs until you stop it, use a `cli`
app. `-u` makes Python flush output right away so the tab shows it live:

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Launch it and click the app's name to watch its output. A `cli` app counts as Running while
its command runs, and goes grey when the script exits, with `[process exited]` left in the
tab. **Stop** ends the script and anything it started. Using the virtual environment's
`python.exe` by path means no activation step is needed.

If the script serves HTTP (Flask, FastAPI, `python -m http.server`), make it a `web` app so
Running follows its port:

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Limits

- Keep Moonpool running. Closing its window quits it by default, and on Windows quitting
  stops every app it launched. Turn on **Close to tray** to hide the window instead; see
  [Tray, close and minimize](/using/tray-and-closing/).
- Moonpool does not restart a script that crashes, and does not start it at Windows login
  by itself. See [Start a script or dev server automatically at Windows login](/guides/start-app-at-windows-login/).
- Avoid nested double quotes in `command`: the `cmd /c` wrapper mangles them.

## See also

- [App types](/apps/types/#cli): how `cli` and `web` apps are tracked.
- [Stop and restart](/apps/stop-and-restart/)
- [Examples](/apps/examples/)
- [Logs](/data/logs/): where session output is kept.
