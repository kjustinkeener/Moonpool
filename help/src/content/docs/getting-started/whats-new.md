---
title: "Moonpool release notes and recent changes"
description: "See what changed in recent Moonpool releases, the requirements for running it, and where to find the full release notes on GitHub."
---

The full notes for every release are on the project's
[Releases page](https://github.com/kjustinkeener/Moonpool/releases). This help ships inside
Moonpool, so it always describes the version you run. Moonpool updates itself; see
[Updating](/data/updating/).

## 0.3.28

- apps.json now lists each app inside its group. Older files are converted automatically and the original is kept as apps.json.pre-groups.
- Drag a group header in the sidebar to reorder groups. Alt+Up and Alt+Down do the same from the keyboard.
- Groups can be empty. Use New group in the sidebar menu, and Delete group on an empty one.

## 0.3.27

- The star thank-you card's close button now shows as a plain x.

## 0.3.26

- The update notice now sits at the top of the sidebar, and the star card waits until it is gone. The star card appears after 10 minutes of use since launch, counting only time the window is shown.
- The star thank-you card has a close button.

## 0.3.23

- The star card's message is now brighter and easier to read.

## 0.3.22

- **Star Supporter themes.** A small card now asks once for a GitHub star. After you say you gave one, nine new Star Supporter themes unlock in the theme browser.
- The row pencil button is gone. Rename, Set icon and the rest are in the right-click menu, which now has icons.

## 0.3.21

- **Faster status.** App icons update as soon as an app starts or stops, Stop on a web app no longer waits about a second, and status checks no longer slow down for each stopped web app.
- The launch, stop or restart button you clicked pulses until the action finishes.

## 0.3.20

- **Safer updates and installs.** Update and uninstall only act on the installed copy, a failed update rolls back, and a newer installed Moonpool is offered to open instead of being overwritten.
- About now says why an update check or install failed.

## 0.3.19

- **Reinstalling works while a Moonpool MCP server is running.**
- The Desktop shortcut goes on your real Desktop, including one OneDrive has moved, and a
  shortcut that can't be made no longer stops the install.

## 0.3.18

- **Updating no longer fails with "Access is denied"** while a Moonpool MCP server started
  before the previous update is still running.

## 0.3.17

- **Help in 14 languages.** The help opens in Moonpool's language: English, German, Spanish,
  French, Italian, Dutch, Polish, Brazilian Portuguese, Russian, Turkish, Japanese, Korean, and
  Simplified and Traditional Chinese.
- **New guides and support pages** for dev servers, ports, starting at login, MCP agents,
  Python scripts and common error messages.
- **`mcpProcessName`.** A wildcard for the process name of an app's MCP server, for servers that
  run under a different name. See [mcpProcessName](/apps/fields/#mcpprocessname).
- **macOS is no longer supported.** There are no macOS builds. Windows and Linux are unchanged.

## 0.3.16

- **Several Moonpools at once.** The installed Moonpool and any number of portable copies can
  run side by side, one per folder, each with its own apps, tray icon and control channel.
  See [Portable mode](/data/portable-mode/#several-copies-at-once).
- **Theme browser.** 68 themes, each previewed in its own colors. See
  [Themes, language and transparency](/using/themes-and-language/).
- **Runnable examples.** A fresh `apps.json` holds example apps that all run as they are.
  The example dashboards now live in an app-owned `dashboards/examples` folder that updates
  with Moonpool. See [Example dashboards](/getting-started/example-dashboards/).
- **apps.json errors are shown.** A banner over the sidebar shows the error, and a failed
  Reload keeps the last list that loaded. See
  [When apps.json has an error](/using/hub-window/#when-appsjson-has-an-error).
- **Control channel on Linux**, through a Unix socket, plus the `list` verb. See
  [Control verbs](/automation/control-verbs/).
- About and the app editor follow theme and language changes live. The **Install
  Moonpool...** menu item is hidden off Windows.

## 0.3.15

- A restarted app keeps its earlier output, with a dated "restarted" divider. See
  [Terminal tabs](/using/terminal-tabs/#restart).
- Each app has its own `cli-output` folder, so log pruning never touches another app's logs.
- `killMode` and `stopCommand` are in the app editor. See
  [Stop and restart](/apps/stop-and-restart/).
- Launched apps no longer inherit Moonpool's own WebView2 profile.

## 0.3.14

- Session logs can be kept between sessions, with a size cap per app. See
  [Logs](/data/logs/).
- Reveal and Copy buttons for the log folders in Settings.
- Fixes to the Help window's title bar.

## Requirements

- Windows 10 or 11 with WebView2 (see [Windows](/platforms/windows/)).
- Linux with WebKitGTK 4.1 and an AppIndicator library (see [Linux](/platforms/linux/)).
