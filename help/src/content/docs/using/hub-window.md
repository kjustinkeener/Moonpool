---
title: The Hub Window
description: A tour of the Moonpool hub window, its menu, tray icon, update banner, and how it remembers its size and position.
---

![The hub with three apps running: the outlined areas are numbered 1 to 4](../../../assets/screenshots/hub-window.png)

1. Running apps: a lit status dot and a stop button instead of play.
2. The tab strip, one tab per opened app, with the active tab highlighted.
3. Live output from the active app.
4. The CPU and memory status bar.

## Layout

| Area | What it holds |
| --- | --- |
| Title bar | Minimize, maximize and close. |
| Sidebar | The filter box, the **...** menu, and your apps grouped by `group`. See [Sidebar and menus](/using/sidebar-and-menus/). |
| CLI pane | One terminal tab per opened app. See [Terminal](/using/terminal/). |
| Status bar | Live CPU and memory, along the bottom. |

Drag the divider between the sidebar and the CLI pane to resize the sidebar.

## Status bar

The status bar shows one thin bar per CPU core (hover for "Per-core CPU usage"), then a memory bar with a `used/total GB` label. Turn it off with **Show CPU/memory status bar** in Settings (`showStatusbar` in [Settings and logs](/configuration/settings-and-logs/)). The change applies immediately.

## The ... menu

The **...** button left of the filter box opens the menu.

![The ... menu button (1) and the Filter apps box (2) at the top of the sidebar](../../../assets/screenshots/sidebar-filter-and-menu.png)

1. The **...** menu button.
2. The **Filter apps...** box.

| Item | Does |
| --- | --- |
| Add app | Opens the app editor. See [Adding apps](/guides/adding-apps/). |
| Edit apps.json | Opens the manifest file for hand editing. |
| Reload | Re-reads `apps.json` from disk (also F5, see [Shortcuts and zoom](/using/shortcuts-and-zoom/)). |
| Settings | Opens the Settings window. |
| Help | Opens this help. |
| About | Opens the About window, with the version and update check. |
| Install Moonpool... | Opens the installer window, to install the app or make a portable copy. See [Installing](/getting-started/installing/) and [Portable mode](/guides/portable-mode/). |

### Port conflict warning

If two apps in `apps.json` use the same `port`, a warning row appears at the bottom of the menu, for example:

```text
port 3000: App A / App B
```

Hover it for the full sentence. Fix the clash in the manifest or the editor; the row disappears once no port is shared.

## Empty screen

With no tab open, the CLI pane shows "Pick an app on the left to launch it."

### Update banner

If **Check for updates on startup** is on and a newer version exists, a banner appears here:

```text
Moonpool {version} is available (you have {current}).
```

It has a **Download & install** button and a dismiss button. A newer help bundle uses the same banner. See [Updating](/guides/updating/).

If the update fails, the banner shows:

```text
Update failed: <error>
```

The button becomes available again so you can retry.

If you collapsed the CLI pane, the banner is hidden with it. The chevron that re-expands the pane pulses while an update waits.

### Copy prompt for AI agents

Below the banner is a ready-made prompt ("New here? Hand this to an AI agent to set up your apps"). **Copy prompt** puts it on the clipboard. The prompt points the agent at `AI-README.md` and `apps.json` in your config folder, and asks it to find your apps and register them. When it is done, choose **Reload**.

Moonpool writes `AI-README.md` next to `apps.json` and refreshes it at each launch, so it matches the installed version. Do not keep your own edits in it. The empty screen also reminds you that you can use **Add app** or **Edit file** instead.

## Tray icon

| Action | Result |
| --- | --- |
| Left-click | Shows the hub window (restores it if minimized or hidden). |
| Right-click | Menu with **Show** and **Quit** only. |

**Quit** exits Moonpool. Whether the tray icon and taskbar button are visible is controlled by Settings (`showInTray`, `showInTaskbar`).

## Closing and minimizing

The close button quits Moonpool by default (`closeToTray` is `false`). Turn on **Close to tray** in Settings and closing hides the window to the tray instead; Moonpool keeps running and the tray icon or **Show** brings it back. **Minimize to tray** (`minimizeToTray`, default on) hides the window to the tray when it is minimized.

## Size, position and maximized state

Moonpool remembers the hub window's size, position and maximized state between runs. The first run opens at 1200x780, centered. If the saved position is no longer on any connected display (for example an unplugged monitor), the position is ignored and the saved size is used at the default location. The file is `window-state.json` in the config folder (see [Overview](/configuration/overview/#where-the-config-lives)).

The sidebar width and whether the CLI pane is collapsed are remembered too.

## Always on top

**Always on top** in Settings keeps the hub, and the Settings and About windows, above other windows. It is off by default.
