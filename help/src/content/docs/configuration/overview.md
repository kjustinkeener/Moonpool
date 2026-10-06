---
title: Configuration overview
description: Where apps.json lives, how to edit and reload it, how it is validated, and how to recover from a bad edit.
---

Every app Moonpool manages is one entry in `apps.json`. You can edit it from the Add/Edit
dialog or by hand. Both write the same file.

## Where the config lives

| Mode | Config folder |
| --- | --- |
| Installed (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Portable | `moonpool-config\` beside `moonpool.exe` (inside the `.moonpool\` folder) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, else `~/.config/Moonpool/` |

`apps.json` is in that folder, next to these:

| Item | Purpose |
| --- | --- |
| `apps.json.history\` | Rollback ring of the last 10 valid manifests. |
| `settings.json` | App settings. See [Settings and logs](/configuration/settings-and-logs/). |
| `cli-output\<id>\` | Per-app terminal logs. |
| `icons\` | Optional `<id>.png` (also `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`) icon overrides. |
| `state.json` | Live status snapshot, refreshed every couple of seconds. |

A typical config folder:

```text
moonpool-config\
  apps.json
  apps.json.history\
  settings.json
  state.json
  cli-output\<id>\
  icons\<id>.png
```

On first run Moonpool seeds `apps.json` with example entries. A file that already exists is
never overwritten.

## Editing

- **Dialog.** Use **Add app** in the **...** menu at the top of the sidebar. To change an
  app, use the pencil on its row or right-click it and choose **Edit**. The dialog
  validates and saves immediately.
- **By hand.** **Edit apps.json** in the same menu opens the file in your default editor.
  Save it, then choose **Reload** in the menu (or press F5 or Ctrl+R).

Hand edits are not picked up until you reload. Reload only reads the file; it does not
rewrite it.

Saving from the dialog rewrites the whole file in a normalized, indented form. Keys Moonpool
does not know are dropped, and JSON has no comments, so keep notes in the `note` field.

## Shape

The file is a JSON array of objects. Four keys are required on every entry: `id`, `name`,
`group`, `type`. Everything else is optional. See [App fields](/configuration/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Groups appear in the sidebar in the order they first occur in the file.

## What reload does

Reload replaces Moonpool's in-memory list with the file's contents. Launch, Stop and Restart
read the entry when you click them, so an edited `command`, `cwd`, `env` or kill setting
applies the next time you start or restart that app. Reload never restarts anything: an
app that is already running keeps running with the settings it started with.

## Validation

Moonpool validates the whole file when it loads, on every save and on every agent write.
A single bad entry rejects the whole file.

| Rule | Error contains |
| --- | --- |
| Not valid JSON, a required key is missing, or a value has the wrong type | the JSON parser message |
| `id` is empty, starts with `-`, or has characters other than letters, digits, `.`, `_`, `-` | `invalid id` |
| Two entries share an `id` | `duplicate app id` |
| `name` is blank | `has an empty name` |
| `group` is blank | `has an empty group` |
| `type` is not `desktop`, `web`, `static` or `cli` | `unknown type` |
| `port` is `0` (a `port` above 65535 fails to parse) | `invalid port 0` |
| `static` entry with no `url` | `requires a url` |
| Any other type with no `command` | `requires a command` |

Errors name the entry by position, for example:

```text
apps.json entry 2 (site) requires a command
```

### The id

The `id` is the entry's permanent key. It names the log folder and icon file, and it is what
you pass to `moonpool.exe launch <id>` and to agents. The dialog derives it from the name
when you add an app. It lowercases the name, turns every run
of characters other than `a` to `z` and `0` to `9` into one `-`, and trims `-` from both
ends. An empty result becomes `app`. If the id is taken, it adds `-2`, `-3` and so on. It
never changes the id afterward, so renaming an app keeps its id. The name `Habit Tracker` gets this id:

```text
habit-tracker
```

## If the file is bad

- **On Reload**, a file that fails validation is left untouched and Moonpool keeps the last
  list that loaded. A banner over the sidebar shows the error, with a button to open the
  file; the list stays usable but dimmed. See
  [When apps.json has an error](/using/hub-window/#when-appsjson-has-an-error).
- **At startup**, a broken file means there is no list to keep, so Moonpool starts with no
  apps and the banner says so. Fix the file and choose **Reload**, or restore a snapshot
  (below, or the `moonpool_restore_config` tool).
- Either way, saves from the dialog (and rename, delete, set icon) are refused until the
  file loads again, so the broken file is never overwritten. Fix the file and choose
  **Reload**.
- **From the dialog, an agent, or a restore**, an invalid change is rejected and the file
  on disk stays as it was.

Every successful save, agent write and restore, and every reload that finds changed
content, copies the validated manifest into `apps.json.history\`, keeping the newest 10. To
roll back by hand, copy a snapshot over `apps.json` and Reload. Nothing is restored
automatically.

```powershell frame="terminal"
Copy-Item "$env:USERPROFILE\.moonpool\moonpool-config\apps.json.history\<snapshot>" "$env:USERPROFILE\.moonpool\moonpool-config\apps.json"
```

## Agents

An AI agent can use Moonpool's MCP server (`moonpool.exe mcp`) instead of touching the file. Setup and the full tool list are in [MCP setup](/automation/mcp-setup/) and [MCP tools](/automation/mcp-tools/):

| Tool | What it does |
| --- | --- |
| `moonpool_read_config` | Returns the manifest text, a version token, and whether it is currently valid. |
| `moonpool_write_config` | Replaces the manifest. Needs the token from the last read, rejects a stale token, validates first, then applies the change without a separate reload. |
| `moonpool_restore_config` | With no argument, lists the snapshots newest first. With a 1-based index or a filename, restores that snapshot if it is valid. |
| `moonpool_reload_config` | Same as Reload. |
| `moonpool_launcher_paths` | Reports the exact folders the running hub uses. |

Going through these tools matters because an agent running in a sandboxed host can be shown
a private copy of the config folder instead of the real one.
