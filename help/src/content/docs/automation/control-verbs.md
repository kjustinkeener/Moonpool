---
title: Control verbs
description: The control channel (named pipe or Unix socket), its protocol, and every verb the running Moonpool answers, with arguments, replies and which are for testing.
---

## Where it listens

Every Moonpool copy has its own channel, so the installed Moonpool and any portable copies
can run side by side without answering for each other. On Windows the installed Moonpool
listens on the named pipe `\\.\pipe\moonpool`. A portable copy adds an id made from its
folder: `\\.\pipe\moonpool-<id>`.

`<id>` is 8 hex digits derived from the copy's `moonpool-config` folder path, so it stays the
same for that folder across restarts and updates, and changes if you move the folder. The
`moonpool.exe` of a copy, including `moonpool.exe mcp`, always finds its own copy's channel.

On Linux and macOS it listens on a Unix domain socket instead, with mode `0600`:

| Case | Socket path |
| --- | --- |
| Normal | `$XDG_RUNTIME_DIR/moonpool.sock` when that variable is set, else `moonpool.sock` in Moonpool's config folder |
| Portable mode | `moonpool.sock` in the portable copy's config folder, so a portable copy never collides with an installed one |
| Path too long for a socket (about 100 characters) | `/tmp/moonpool-<uid>/moonpool.sock`, in a directory only you can open (`moonpool-<id>.sock` for a portable copy) |

A socket file left behind by a crash is detected and replaced on the next start. A socket that
something still answers on is never taken over. The file is removed when Moonpool quits
normally.

The channel is also how the [MCP server](/automation/mcp-setup/) knows whether Moonpool is
running: if a `ping` is answered it is, and if the pipe or socket is missing it is not. The
same verbs are also reachable from the [command line](/automation/command-line/), except the
diagnostic verbs below.

## Protocol

One JSON object per line in, one JSON line out, in order. A connection can carry many
requests.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

A request and its reply from PowerShell:

For a portable copy, use its pipe name (`moonpool-<id>`, shown by the `paths` verb) in
place of `moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` is a list of strings and may be omitted. Other fields are ignored.
- `result` is a string or null. Verbs that return structured data return it as a JSON
  string.
- A line that is not valid JSON gets `{"ok": false, "error": "bad request: ..."}`.
- An unknown `cmd` gets `unknown cmd: <name>`.
- A verb that goes through the window (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`) is answered when the action finishes, or with a timeout error
  after 45 s. If the hub window's UI has not loaded, it fails at once with `frontend not
  loaded`.
- A Moonpool that starts while a previous one is still exiting retries binding the channel
  for about 8 seconds. If it still cannot, it logs that and keeps running without it.

## Verbs

| Verb | Args | Result |
| --- | --- | --- |
| `ping` | none | `pong`. Channel only. |
| `list` | none | JSON string `{"apps": [...], "statuses": [...]}` read from the running hub's memory, the same `apps` and `statuses` shape as `state.json`. Adds `"statusNotReady": true` when apps are registered but the first status check has not run yet. While `apps.json` fails to load, adds `"manifestError": "<message>"` (the apps are then the last list that loaded) and, when no list has loaded since startup, `"manifestLoaded": false`. Channel only. |
| `show` | none | null. Brings the window to the front. |
| `quit` | none | null. Exits Moonpool. |
| `launch` | `<id>` | null on success, or `opened` for a `static` entry with only a `url`. Errors: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null on success, or `stopped` for a `static` entry with only a `url`. Error: `still running after stop`. |
| `restart` | `<id>` | Same results and errors as `launch`. |
| `reload` | none | null on success. |
| `refresh-icons` | none | null on success. |
| `help` | none | null. Opens the Help window. |
| `dump` | `<id>` [`out-path`] | Path of the app's session log, or of the plain-text copy at `out-path`. |
| `paths` | none | Multi-line report of the folders and exe the hub uses. |
| `read-config` | none | Path of `dumps\read-config.json`, which holds `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | The new version token. Errors: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` or `filename`] | No argument: path of `dumps\restore-config.json` (`count`, `snapshots`). With one: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | the command-line arguments | null, at once. Runs them exactly as a second `moonpool.exe <args>` of this copy would, including `--ticket`. This is how that second launch hands its arguments over before it exits. |

Example exchanges:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` and `restore-config` load the new manifest at once, record a snapshot in
`apps.json.history\`, and refresh the window.

## Diagnostic verbs (testing)

Channel only: the command line does not accept these. All work on Windows, Linux and macOS
except `screenshot`, which is Windows only and answers `screenshot is not supported on this
platform (Windows only)` elsewhere.

| Verb | Args | Result |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Windows only. Base64 of a PNG of that Moonpool window (default `main`). Optional `max_dim` caps the longer side in pixels (clamped to 320-2400, default 320; the MCP tool always uses the default). A non-integer `max_dim` is an error. Windows allowed: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Errors: `unknown window '<name>'`, `window '<name>' is not open`. Not written to disk. |
| `open-window` | `<kind>` [`<id>`] | null. Opens a window the way its menu item does. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (optional `<id>` opens that app's Edit App dialog, none opens Add App), `terminal` (`<id>` required: selects that app's terminal tab and widens the hub so the CLI pane shows; does not launch it), `cli` (only widens the hub). Errors: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Answered through the hub window like `launch`. |
| `window-state` | [`window`] | JSON string: `{"open":false}`, or `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Kills the app's `<processName> mcp` helper, not the app. Errors: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` or `<id>: was not marked seen`; with no id, `cleared <n> entries`. Clears the remembered MCP-helper sightings. |

The command line's `--ticket` and `state.json` outcome records belong to the other channel;
see [Command line](/automation/command-line/#reading-the-outcome). Channel requests get their
answer in the reply.

## See also

- [Command line](/automation/command-line/)
- [AI agents: quick start](/automation/quick-start/#the-same-action-three-ways)
