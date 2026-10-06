---
title: What's new
description: Highlights of recent Moonpool releases, and where to find the full release notes.
---

The full notes for every release are on the project's
[Releases page](https://github.com/kjustinkeener/Moonpool/releases). This help ships inside
Moonpool, so it always describes the version you run. Moonpool updates itself; see
[Updating](/guides/updating/).

## 0.3.16

- **Several Moonpools at once.** The installed Moonpool and any number of portable copies can
  run side by side, one per folder, each with its own apps, tray icon and control channel.
  See [Portable mode](/guides/portable-mode/#several-copies-at-once).
- **Theme browser.** 68 themes, each previewed in its own colors. See
  [Appearance](/using/appearance/).
- **Runnable examples.** A fresh `apps.json` holds example apps that all run as they are.
  The example dashboards now live in an app-owned `dashboards/examples` folder that updates
  with Moonpool. See [Example dashboards](/using/example-dashboards/).
- **apps.json errors are shown.** A banner over the sidebar shows the error, and a failed
  Reload keeps the last list that loaded. See
  [When apps.json has an error](/using/hub-window/#when-appsjson-has-an-error).
- **Control channel on Linux and macOS**, through a Unix socket, plus the `list` verb. See
  [Control verbs](/automation/control-verbs/).
- About and the app editor follow theme and language changes live. The **Install
  Moonpool...** menu item is hidden off Windows.

## 0.3.15

- A restarted app keeps its earlier output, with a dated "restarted" divider. See
  [Terminal tabs](/using/terminal/#restart).
- Each app has its own `cli-output` folder, so log pruning never touches another app's logs.
- `killMode` and `stopCommand` are in the app editor. See
  [Stop and restart](/configuration/stop-and-restart/).
- Launched apps no longer inherit Moonpool's own WebView2 profile.

## 0.3.14

- Session logs can be kept between sessions, with a size cap per app. See
  [Logs](/configuration/logs/).
- Reveal and Copy buttons for the log folders in Settings.
- Fixes to the Help window's title bar.

## Requirements

- Windows 10 or 11 with WebView2 (see [Windows](/platforms/windows/)).
- Linux with WebKitGTK 4.1 and an AppIndicator library (see [Linux](/platforms/linux/)).
- macOS: build from source; not yet distributed or tested.
