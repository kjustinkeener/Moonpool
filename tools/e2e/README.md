# End-to-end tests

`e2e.mjs` drives a built Moonpool binary against throwaway portable copies and exits
non-zero on any failure. No dependencies beyond Node 18+, no network.

## Run

Build a release binary first (the CLI build is required: a bare `cargo build` produces a
dev-frontend exe whose window has no UI). From the repo root:

```powershell
cd <repo root>
npm --prefix help ci; npm --prefix help run build   # only if help/dist is missing
npx tauri build --no-bundle
npm run e2e
```

`npm run e2e` is `node tools/e2e/e2e.mjs`. Options:

| Option          | Meaning                                                            |
|-----------------|--------------------------------------------------------------------|
| `--exe <path>`  | binary to test (default `src-tauri/target/release/moonpool[.exe]`) |
| `--tmp <dir>`   | where the temp copies go (default: the OS temp dir)                |
| `--only <text>` | run only scenarios whose name contains `<text>`                    |
| `--keep`        | leave the temp folder behind for inspection                        |

A full run takes under a minute. The test hubs open their windows while they run; nothing
is clicked or typed into them.

## What it covers

Two portable copies (`CopyA`, `CopyB`, each `<tmp>/<name>/.moonpool/moonpool.exe` plus the
`moonpool.portable` flag) are created and exercised over the control channel and the stdio
MCP server only:

- each copy derives its own channel (`\\.\pipe\moonpool-<id>`, id recomputed here from the
  folder the same way `instance.rs` does, and checked against the hub's `paths` report);
- both run side by side and each answers `ping`, `paths` and `list` for itself only;
- a second launch of a running copy (bare, `show`, `launch <id>`, `stop <id>`) forwards to
  the hub and exits 0 without a second hub;
- two simultaneous launches of a stopped copy end with exactly one hub;
- `moonpool.exe mcp` from each copy: `initialize` names the copy, `tools/list`,
  `moonpool_launcher_paths` / `moonpool_list_apps` / `moonpool_raise_launcher` reach only
  that copy's hub; with the hub down `moonpool_list_apps` says "not running", and
  `moonpool_bootup_launcher` / `moonpool_shutdown_launcher` start and stop only that copy;
- help, `dashboards/examples` and `apps.json` are seeded; altering the stamps re-seeds
  (stale files removed) while the user's own files in `dashboards/` survive;
- a launched `cli` example app's process dies when the hub quits;
- `quit` over the channel ends the hub and frees the lock (a relaunch becomes the hub);
- a `--wait-pid <old hub>` relaunch waits, then takes over when the old hub exits.

## Safety

The harness never talks to the installed copy's channel (`\\.\pipe\moonpool` or the
`$XDG_RUNTIME_DIR` socket) and refuses to start a copy that lacks its portable flag. It
only kills PIDs whose executable lives inside its own temp folder, and it checks at the end
that none are left before deleting the folder.

## Porting to Linux / macOS

Everything OS-specific is in `platform.mjs`: the binary name, the channel endpoint
(`<copy>/moonpool-config/moonpool.sock`; the `/tmp/moonpool-<uid>` fallback for long paths
is not modeled, so keep `--tmp` short), the example `cli` app id, process listing (`/proc`
on Linux; macOS needs a `ps`-based `listProcesses`), and tree kill. The scenarios in
`e2e.mjs` are platform-neutral. Not yet run on Linux.
