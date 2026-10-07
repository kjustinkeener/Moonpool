# Moonpool

[![CI](https://github.com/kjustinkeener/Moonpool/actions/workflows/ci.yml/badge.svg)](https://github.com/kjustinkeener/Moonpool/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-24C8DB.svg)](https://tauri.app)

A resident **system-tray launcher hub** for the apps and dev servers on your machine.
Instead of hunting down Desktop shortcuts and remembering `localhost` ports, Moonpool lists
every app you register, shows live status, and launches each one inside an **embedded
terminal** so you see its console output as if you'd opened the CLI yourself, bidirectional
(you can type into it too).

![Moonpool main window](assets/screenshot.png)

<!--
  DEMO GIF: drop a short screen recording at assets/demo.gif (it does not exist yet):
  pick an app, watch it launch in the embedded terminal, then stop it. See assets/README.md.
-->
<!-- ![Moonpool demo](assets/demo.gif) -->

Built with Tauri v2 + Svelte 5. **Cross-platform:** the OS-specific bits (shell wrapper,
process/tree kill, port freeing, exe-icon extraction) live behind a small `platform` module with
Windows and Linux implementations. Windows is the primary/tested target; Linux is supported.
macOS is not supported. Notes: desktop-app **icon extraction from the
binary is Windows-only** (elsewhere it falls back to project-folder icons + favicons), and the
Linux system tray needs `libayatana-appindicator` installed (stock GNOME also needs the
AppIndicator shell extension; see [docs/linux-setup.md](docs/linux-setup.md)).

## Contents

- [What it does](#what-it-does)
- [Install](#install)
- [Configuring your apps](#configuring-your-apps)
- [Build from source](#build-from-source)
- [Security / trust model](#security--trust-model)
- [License](#license)

## What it does

- **One window, every app.** A grouped, searchable sidebar driven by a single JSON manifest.
- **Launch = run the real command in a PTY.** Clicking an app runs its launch command inside a
  real pseudo-terminal (ConPTY on Windows) and streams the bytes to an xterm.js terminal tab.
  Colors, build progress, and errors all show up live; you can type into it.
- **Live status.** A background poller marks each app running/stopped via a TCP port
  health-check (web apps) or a process-name check (desktop apps), even if you launched it
  outside Moonpool.
- **Start / stop / restart** with a busy indicator; stopping tree-kills the process and frees
  the port. Recently-used apps sort to the top of their group.
- **Stays in the tray.** Minimizing hides to the tray (default); turn on **Close to tray** to
  make closing hide it too (by default closing quits). Left-click the tray icon to reopen,
  tray menu -> Quit to exit.
- **Linux-style terminal clipboard**: select to copy (then deselect), middle-click to paste.
- **Command-line remote control.** With Moonpool running, `Moonpool.exe launch|stop|restart|reload|refresh-icons|show|dump <app-id>` drives the resident window (a script or AI agent can start/stop your apps), and a live `state.json` in the config folder reports what's running. Tag a command with `--ticket <key>` to read its success/failure back from `state.json`, and `dump <app-id>` writes that app's console output to a file. Moonpool is also its own MCP server (`moonpool.exe mcp`), so an agent can list, launch, restart and read the console output of your apps as tool calls. See [`AI-README.md`](AI-README.md).

## Install

Prebuilt installers are published on the
[**Releases page**](https://github.com/kjustinkeener/Moonpool/releases). Download the artifact for your OS:

| OS | Download | Notes |
| --- | --- | --- |
| **Windows** | `moonpool.exe` | Recommended. A single self-installing exe: run it and click Install (or Install portable). The `.msi` and NSIS `.exe` bundles are legacy and are not the update path. |
| **Linux** | `.AppImage`, `.deb` or `.rpm` | See [docs/linux-setup.md](docs/linux-setup.md) for GNOME tray setup and runtime deps. |

Moonpool ships an auto-updater: once installed, it checks the Releases feed and can update
itself in place. On Linux only the AppImage updates itself; `.deb` and `.rpm` installs update
through your package manager.

## Configuring your apps

Moonpool reads a user-editable manifest from your config directory:

```text
%USERPROFILE%\.moonpool\moonpool-config\apps.json      (Windows, installed)
<folder>\.moonpool\moonpool-config\apps.json             (Windows, portable)
~/.config/Moonpool/apps.json                         (Linux; $XDG_CONFIG_HOME/Moonpool if set)
```

On first run it's seeded from [`apps.example.json`](src-tauri/resources/apps.example.json).
Use the **... menu** (top-left of the sidebar): **Add app** (a form), **Edit apps.json** (opens the
file), then **Reload** - no rebuild needed. Or hand the copyable prompt on the empty pane to an AI
agent and let it configure your apps (see [`AI-README.md`](AI-README.md)).

Each entry:

```jsonc
{
  "id": "myapp",                 // unique slug
  "name": "My App",
  "group": "Web apps",           // any label; groups render in first-seen order
  "type": "web",                 // desktop | web | static | cli
  "cwd": "C:\\path\\to\\app",
  "command": "npm run dev",      // run through cmd /c ($SHELL -c on Linux)
  "port": 3000,                  // web: status + browser-open
  "url": "http://localhost:3000",
  "openBrowser": true,
  "env": { "PORT": "3000" },     // optional vars injected into the command
  "processName": "myapp",        // desktop: status by process name
  "killMode": "port",            // optional extra cleanup on Stop: processName | port | command | none
                                 //   (default: desktop=processName, web=port, others=none)
  "stopCommand": "docker compose stop app", // only for killMode "command"; run in cwd
  "note": "shown as a tooltip"
}
```

`type` semantics:
- **desktop** - status detected by `processName` (its `.exe`, without extension; on Linux
  15 characters or fewer, since Linux truncates longer process names).
- **web** - status by `port` (TCP health-check); the browser opens when it goes live.
- **static** - opens `url` (or runs `command` in a terminal).
- **cli** - runs `command` in a terminal in `cwd`; it shows as running until the command exits.

`killMode`: Stop always ends the terminal Moonpool started. `killMode` adds one cleanup step for apps
that outlive it: `processName` kills by exe name, `port` kills whatever listens on `port`, `command`
runs `stopCommand`, `none` does nothing. Docker apps on Windows must use `none` or `command`, never
`port` (it would kill Docker Desktop). Full reference: the in-app Help, "Configuration".

**Command tips:** every `command` runs via `cmd /c` (Windows) or `sh -c` (Linux) in `cwd`,
inheriting the environment plus `env`. Prefer a foreground command that streams logs
(`python x.py`, `node server.js`) over a detached/windowless launcher. Avoid nested double-quotes
(they get mangled through the wrapper); to run a one-shot command and keep a shell open, use the
unquoted form `pwsh -NoExit -Command <tokens...>`.

**Icons:** resolved as: manifest `"icon"` (file path / URL / data URI) ->
`<config folder>\icons\<id>.png` -> **auto-discovered from the app's own project folder**
(Tauri `src-tauri/icons/`, Electron `build/icon`, web `public/favicon`, or `icon.png`/`logo.png`)
-> desktop exe icon -> web favicon -> a type glyph. Most apps get their real icon with no config.

## Build from source

**Prerequisites:** [Rust](https://rustup.rs) (stable), [Node.js](https://nodejs.org) 20+, and the
[Tauri v2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS (on Windows:
the Microsoft C++ Build Tools and WebView2, which ships with Windows 11). Linux users: see
[docs/linux-setup.md](docs/linux-setup.md) for the exact `apt` packages.

```bash
npm install
npm run tauri dev
```

`npm run tauri build` produces the bundled release artifacts under
`src-tauri/target/release/bundle/` (`.msi`/`.exe` on Windows, `.deb`/`.rpm`/`.AppImage` on Linux).

**macOS (unsupported):** there are no macOS releases and macOS issues won't be fixed, but the
Unix code paths are shared with Linux, so it should still build. Install the Xcode Command Line
Tools per the Tauri prerequisites, then `npm run tauri build` makes a `.app` and `.dmg` for your
Mac's architecture. For one build that runs on both Apple Silicon and Intel:

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri build -- --target universal-apple-darwin
```

## Security / trust model

Moonpool runs the `command` of each app in `apps.json` **with your full user privileges**, in a
real shell (`cmd /c` on Windows, `sh -c` on Linux). There is no sandbox: an entry can run
any program, script, or shell pipeline you could run yourself.

Treat `apps.json` like a shell script you are about to execute:

- Only add or edit entries whose `command` and `cwd` you understand and trust.
- If you let an **AI agent** populate `apps.json` (see [`AI-README.md`](AI-README.md)), review the
  commands it wrote before launching them, the same way you would review a script an agent
  generated. The agent guide invites automated edits, so this review step is on you.
- Be cautious with an `apps.json` you did not write yourself (for example one copied from
  elsewhere); read every `command` first.

## License

MIT - see [LICENSE](LICENSE).
