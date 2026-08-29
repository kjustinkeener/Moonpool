# Contributing to Moonpool

Thanks for your interest in improving Moonpool. This is a Tauri v2 + Svelte 5 desktop app.

## Build and run

Prerequisites: [Rust](https://rustup.rs) (stable), [Node.js](https://nodejs.org) 20+, and the
[Tauri v2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS. Linux
contributors: the exact `apt` packages are in [docs/linux-setup.md](docs/linux-setup.md).

```bash
npm install
npm run tauri dev      # run the app in dev mode
npm run tauri build    # produce release bundles under src-tauri/target/release/bundle/
```

On Windows you can also use `tools/dev-run.ps1`, which kills a stale instance, rebuilds the full
PATH, frees the Vite port, then runs `tauri dev`:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File tools\dev-run.ps1
```

## What CI checks

Every push and pull request runs the [CI workflow](.github/workflows/ci.yml) on Windows, Ubuntu,
and macOS. Please make sure these pass locally before opening a PR (they are the same commands CI
runs):

```bash
# Frontend typecheck (Svelte + TypeScript)
npm run check

# Rust formatting (must already be formatted)
cargo fmt --manifest-path src-tauri/Cargo.toml --check

# Rust lints (warnings are treated as errors)
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings

# Builds
npm run build
cargo build --manifest-path src-tauri/Cargo.toml
```

To auto-apply Rust formatting rather than just checking it, run
`cargo fmt --manifest-path src-tauri/Cargo.toml` (without `--check`).

## Pull requests

- Keep changes focused; describe what and why in the PR body.
- If you change launch/status behavior, update the relevant docs (`README.md`, `AI-README.md`, or
  `docs/`) so they stay accurate.
- Avoid the em-dash character (U+2014) in committed content; a repo git hook blocks it. Use a
  comma, colon, parentheses, or a spaced hyphen instead.

## Reporting bugs and requesting features

Use the issue templates under
[`.github/ISSUE_TEMPLATE`](.github/ISSUE_TEMPLATE). For bugs, include your OS and version so we
can reproduce.
