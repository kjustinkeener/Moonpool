---
title: Paths and environment
description: The {MP_HOME} and {MP_DATA} tokens, ./ paths, which fields expand them, and how env and the working folder behave.
---

## Tokens

| Token | Expands to |
| --- | --- |
| `{MP_HOME}` | Portable: the folder holding `moonpool.exe`. Installed on Windows: `%USERPROFILE%\.moonpool`. |
| `{MP_DATA}` | The config folder, the one that holds `apps.json`. |

A token that cannot be resolved is left as written.

## Which fields expand

| Field | Tokens | Leading `./` or `.\` |
| --- | --- | --- |
| `cwd` | yes | yes, anchored to `{MP_HOME}` |
| `command` | yes | no |
| `stopCommand` | yes | no (it runs in `cwd`, which is anchored) |
| `url` | yes | no |
| `icon` | yes | yes, anchored to `{MP_HOME}` |
| `env` values, `processName`, `note` | no | no |

A relative path without `./` (such as `apps\tool`) is left alone and resolves against
Moonpool's own working folder, which is rarely what you want. Prefer `./` or a token.

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

Both forms keep working when you move the portable folder. A fixed path such as
`C:\tools\notes` does not travel. In portable mode the Edit app dialog marks absolute `cwd`
and `url` values with a "not portable" badge. See [Portable mode](/guides/portable-mode/).

## Environment

`env` is an object of strings. The dialog edits it as one `KEY=VALUE` per line; it splits
each line at the first `=`, trims both sides, and ignores lines without one.

In the dialog:

```text
PORT=8091
NODE_ENV=development
```

In `apps.json`, as the `env` key of the entry:

```json
"env": { "PORT": "8091", "NODE_ENV": "development" }
```

- The launched command inherits Moonpool's environment plus `env`. Entries in `env` win.
- `env` is also applied to `stopCommand`.
- Values are used as written: no `{MP_HOME}` expansion and no `%VAR%` expansion by Moonpool.
- Moonpool points its own WebView2 at a private profile folder through
  `WEBVIEW2_USER_DATA_FOLDER`. Launched apps do not inherit that. If you had set the
  variable yourself before starting Moonpool, they get your value; otherwise it is unset.
  An `env` entry can still override it.

## Working folder

The command and `stopCommand` run in `cwd`. When `cwd` is omitted, the command runs in
Moonpool's own working folder, so set `cwd` for anything that uses relative paths.
