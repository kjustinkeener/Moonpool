---
title: Control verbs
description: The named-pipe protocol and every verb the running Moonpool answers, with arguments, replies and which are for testing.
---

Moonpool listens on the named pipe `\\.\pipe\moonpool` (Windows only). It is the channel the
[MCP server](/automation/mcp-setup/) uses. The same verbs are also reachable from the
[command line](/automation/command-line/), except the diagnostic verbs below.

## Protocol

One JSON object per line in, one JSON line out, in order. A connection can carry many
requests.

```json
{"cmd": "restart", "args": ["my-app"]}
```

```json
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

- `args` is a list of strings and may be omitted. Other fields are ignored.
- `result` is a string or null. Verbs that return structured data return it as a JSON
  string.
- A line that is not valid JSON gets `{"ok": false, "error": "bad request: ..."}`.
- An unknown `cmd` gets `unknown cmd: <name>`.
- A verb that goes through the window (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`) is answered when the action finishes, or with a timeout error
  after 45 s. If the hub window's UI has not loaded, it fails at once with `frontend not
  loaded`.
- If Moonpool cannot bind the pipe it logs that and keeps running without it. The command
  line still works.

## Verbs

| Verb | Args | Result |
| --- | --- | --- |
| `ping` | none | `pong`. Pipe only. |
| `show` | none | null. Brings the window to the front. |
| `quit` | none | null. Exits Moonpool. |
| `launch` | `<id>` | null on success. Errors: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null on success. Error: `still running after stop`. |
| `restart` | `<id>` | null on success. Same errors as `launch`. |
| `reload` | none | null on success. |
| `refresh-icons` | none | null on success. |
| `help` | none | null. Opens the Help window. |
| `dump` | `<id>` [`out-path`] | Path of the app's session log, or of the plain-text copy at `out-path`. |
| `paths` | none | Multi-line report of the folders and exe the hub uses. |
| `read-config` | none | Path of `dumps\read-config.json`, which holds `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | The new version token. Errors: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` or `filename`] | No argument: path of `dumps\restore-config.json` (`count`, `snapshots`). With one: `restored <file> (<n> apps); new version token <token>`. |

`write-config` and `restore-config` load the new manifest at once, record a snapshot in
`apps.json.history\`, and refresh the window.

## Diagnostic verbs (testing)

Pipe only, Windows only. The command line does not accept these.

| Verb | Args | Result |
| --- | --- | --- |
| `screenshot` | [`window`] | Base64 of a PNG of that Moonpool window (default `main`). Windows allowed: `main`, `settings`, `about`, `installer`, `editor`, `help`. Errors: `unknown window '<name>'`, `window '<name>' is not open`. Not written to disk. |
| `window-state` | [`window`] | JSON string: `{"open":false}`, or `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Kills the app's `<processName> mcp` helper, not the app. Errors: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` or `<id>: was not marked seen`; with no id, `cleared <n> entries`. Clears the remembered MCP-helper sightings. |

The command line's `--ticket` and `state.json` outcome records belong to the other channel;
see [Command line](/automation/command-line/#reading-the-outcome). Pipe requests get their
answer in the reply.
