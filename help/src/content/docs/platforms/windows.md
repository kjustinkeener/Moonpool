---
title: "Use Moonpool on Windows"
description: "Windows is the main Moonpool platform: how to install it, and a table of what differs between Windows and Linux so you know what to expect."
---

Windows is Moonpool's main platform. Install it as described in
[Installing](/getting-started/install/).

## Before you run it

- **SmartScreen.** `moonpool.exe` is not code-signed, so Windows may show "Windows protected
  your PC" the first time. Choose **More info**, then **Run anyway**.
- **Antivirus.** A new, unsigned exe that copies itself and replaces itself on update can
  trip an antivirus. If yours blocks or quarantines `moonpool.exe`, allow it for the
  `.moonpool` folder.
- **WebView2.** Moonpool's windows use Microsoft Edge WebView2, which ships with Windows 11
  and current Windows 10. If the window stays blank or never opens, install the Evergreen
  WebView2 Runtime from Microsoft.

## Tray

On Windows 11 a new tray icon often goes into the hidden-icons area. Click the **^** arrow at
the right of the taskbar to find it, and drag it onto the taskbar to keep it visible.

## Commands

- Commands run through `cmd /c`. Avoid nested double quotes in `command`; `cmd /c` mangles
  them. For a script that should leave a shell open, use
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` without quotes around the
  script part.
- `processName` matches with or without `.exe`, ignoring case.
- Stop ends the whole process tree Moonpool started, including processes that detached from
  it.
- Docker Desktop apps need `killMode` `none` or `command`, never `port`. See
  [Docker apps on Windows](/apps/stop-and-restart/#docker-apps-on-windows).

## What differs by platform

| | Windows | Linux |
| --- | --- | --- |
| Install | Self-installing `moonpool.exe`, or portable | AppImage, `.deb` or RPM; no install card |
| Self-update | Yes, installed and portable | AppImage only |
| Config folder | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Shell for commands | `cmd /c` | `$SHELL -c` |
| `processName` | Any length, `.exe` optional, case-insensitive | 15 characters or fewer, exact case |
| Stop by `processName` | Kills the process and its children | Kills processes with that exact name only |
| Control channel | Named pipe | Unix socket |
| Window screenshots (testing) | Yes | No |
| Icons from a program file | Yes | No |
| Tray | Works out of the box | Needs AppIndicator; stock GNOME needs an extension |

Linux details are on the [Linux](/platforms/linux/) page.
