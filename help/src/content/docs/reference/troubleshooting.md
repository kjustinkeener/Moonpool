---
title: Troubleshooting and FAQ
description: Fixes for common problems, found by what you see, plus short answers to common questions.
---

Find the symptom, then follow the fix. Quoted text is what Moonpool shows.

## I cannot see the tray icon

- **Windows.** The icon may be in the hidden-icons area. Click the **^** arrow at the right
  of the taskbar. Drag the icon onto the taskbar to keep it visible.
- **Linux on stock GNOME.** GNOME shows no tray icons without the AppIndicator extension.
  See [Linux](/platforms/linux/#tray-on-gnome).
- **Settings.** **Show in tray** may be off. Open the hub from the taskbar or Start Menu
  and turn it back on in [Settings](/using/settings-window/).

## The installer shows an error

| Message | What to do |
| --- | --- |
| `Install failed: <error>` | The text after the colon names the step that failed, for example `copy exe: ...`. If a file is in use, quit any Moonpool running from `%USERPROFILE%\.moonpool` and try again. |
| `target folder does not exist` | The folder you picked for a portable copy is gone. Pick an existing folder. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Pick an empty folder, or remove that `.moonpool` folder first. |

## An app will not start

1. Click the app's name to open its terminal tab and read the output. An agent can read the
   same text with `moonpool_app_output`.
2. Check `cwd`. A missing folder, or a relative path without `./`, is the usual cause. See
   [Paths and environment](/configuration/paths-and-environment/).
3. Check `command`. Run it by hand in a terminal in `cwd`. On Windows avoid nested double
   quotes; `cmd /c` mangles them.
4. Turn on **Log debug info to a file** in Settings and launch again. `moonpool.log` records
   the exact command and folder. See [Logs](/configuration/logs/).

| Message | Meaning |
| --- | --- |
| `already running` | Moonpool already has a terminal for this app. Stop it first, or use Restart. |
| `stopped during launch` | Stop was pressed while the launch was still starting. |
| `did not reach running in time` | From a script or agent: the app did not read as Running within 25 seconds. Check its `port` or `processName`, and its output. |

## The status dot is wrong

Moonpool decides Running from `port`, then `processName`, then whether its own terminal is
still alive. See [How Running is decided](/configuration/app-types/#how-running-is-decided).

- **Never turns solid.** A `web` app's `port` does not answer, or a `desktop` app's
  `processName` does not match. On Linux `processName` must be 15 characters or fewer.
- **Goes grey right after launch.** A `cli` app stops being Running when its command
  exits. Use a shell with `-NoExit` if you want it to stay open.
- **A `static` app never shows Running.** That is expected for an entry with only a `url`.
- **Shows Running although you did not start it.** Something else is using that port or
  process name. Moonpool shows it as running but not "managed by Moonpool".

## Two apps use the same port

A warning row appears at the bottom of the **...** menu, for example `port 3000: App A / App B`.
Change one app's `port` (and its `env`, if it reads `PORT`). See
[Port conflict warning](/using/hub-window/#port-conflict-warning).

## The app keeps running after Stop

From a script or agent the error is `still running after stop` (after 15 seconds).

- The app outlives its terminal. Set `killMode` to `port` or `processName`. See
  [Stop and restart](/configuration/stop-and-restart/).
- A Docker app on Windows: use `killMode` `command` with a `stopCommand` such as
  `docker compose stop app`. Never `port`.

## apps.json has an error

The sidebar shows a banner, "apps.json has an error, showing the last list that loaded", or,
at startup, "apps.json has an error, so no apps are loaded." Saving from Moonpool is paused
until the file loads again.

Typical errors:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Choose **Edit apps.json** in the banner, fix the entry, save, then **Reload** (F5).
2. Or roll back to a recent good copy. See
   [Backup and recovery](/configuration/backup-and-recovery/#roll-back-appsjson).

The full rule list is in [Validation](/configuration/overview/#validation).

If a setting cannot be changed and the message ends with `Repair settings.json and restart
Moonpool before changing settings`, fix or delete `settings.json` in the config folder and
start Moonpool again. Deleting it resets every setting to its default.

## My edit did not take effect

- Hand edits need **Reload** (or F5). Moonpool does not watch the file.
- Reload does not restart running apps. Restart the app to use a changed `command`, `cwd`
  or `env`.
- An agent may be editing a different `apps.json`. Ask it to call `moonpool_launcher_paths`
  and compare the hub's folder with its own. With several copies of Moonpool, check which
  copy you are editing.

## The example apps are missing

Examples are written only when no `apps.json` exists. To get them back, see
[Reset to the examples](/configuration/backup-and-recovery/#reset-to-the-examples), or copy
the entries from [Example dashboards](/using/example-dashboards/#example-apps-appear-only-on-first-run).

## An update failed

The banner shows `Update failed: <error>`. See
[When an update fails](/guides/updating/#when-an-update-fails).

## A web link will not open

`refusing to open non-web url: <url>` means the `url` is not `http://`, `https://`,
`mailto:` or `file://`. Fix the `url`.

## MCP and script errors

| Message | What to do |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Start Moonpool, or let the agent call `moonpool_bootup_launcher`. |
| `frontend not loaded` | The hub window has not finished loading. Wait a moment and retry. |
| `stale token: ...` | `apps.json` changed since the agent read it. Read it again, then write. |
| `rejected invalid manifest: ...` | The new `apps.json` failed validation. The file was not changed. |
| `... A Moonpool process may be hung ...` | Something holds the control channel without answering. Quit Moonpool from the tray, or end the process, and start it again. |

More in [MCP setup](/automation/mcp-setup/#notes) and
[MCP tools](/automation/mcp-tools/).

## Window problems

- **Off screen.** Moonpool ignores a saved position that is on no connected display. If the
  window is still lost, quit Moonpool and delete `window-state.json` in the config folder.
- **Zoom stuck too big or too small.** Ctrl + wheel over the hub changes it. See
  [Shortcuts and zoom](/using/shortcuts-and-zoom/#zoom).
- **Settings opens behind the hub.** Turn **Always on top** off, or on, in Settings. It
  applies to every Moonpool window, so they stay in the same layer.

## Where are the logs?

See [Logs](/configuration/logs/).

## Back up, reset or uninstall

See [Backup and recovery](/configuration/backup-and-recovery/) and
[Uninstalling](/getting-started/installing/#uninstalling).

## FAQ

**Does closing the window stop my apps?**
By default closing quits Moonpool, and on Windows quitting stops the apps it launched. Turn
on **Close to tray** to keep Moonpool running when you close the window. See
[Tray, close and minimize](/using/tray-and-closing/).

**Can I run Moonpool twice?**
One per folder. Starting the same copy again brings its window back. The installed copy and
portable copies can run side by side. See
[Portable mode](/guides/portable-mode/#several-copies-at-once).

**Does Moonpool phone home?**
Only to check for updates: it fetches the release manifest from GitHub at startup (if
**Check for updates on startup** is on) and when you press **Check for updates**. Every
download is verified against Moonpool's signing key before it is used.

**Which shell runs my commands?**
`cmd /c` on Windows, `$SHELL -c` on Linux and macOS.

**Where do I put secrets?**
`env` values are stored in plain text in `apps.json`. Prefer a file your app reads itself,
or a variable already set in your user environment, which launched apps inherit.

**Is the control channel protected?**
It has no login or token. Any process running as you can send it commands. On Linux and
macOS the socket is readable only by your user. See
[Safety properties](/automation/overview/#safety-properties).
