# Moonpool on Linux

Moonpool runs on Linux via WebKitGTK. You can install a prebuilt package or build from source.
(Note: it's primarily developed and tested on Windows; Linux is supported but less battle-tested.)

## Install a prebuilt package

Grab the `.AppImage` or `.deb` from the project's Releases page.

**AppImage** (portable, no install):

```bash
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

**.deb** (Debian/Ubuntu):

```bash
sudo apt install ./Moonpool_*_amd64.deb
```

The `.deb` pulls in its runtime dependencies automatically. For the AppImage you need the
WebKitGTK and AppIndicator runtime libraries present:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

(Package names above are for Debian/Ubuntu; on Fedora/Arch use the equivalents, e.g.
`webkit2gtk4.1` / `libayatana-appindicator`.)

## System tray on GNOME (important)

Stock **GNOME does not show legacy tray icons**, so Moonpool's tray icon won't appear until you
install and enable the AppIndicator extension:

```bash
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com   # then log out/in
```

The main window and embedded terminals work without it - only the tray icon needs it. KDE,
Cinnamon, XFCE, and MATE show the tray out of the box.

## Build from source

Requires the WebKitGTK **dev** packages plus Rust and Node:

```bash
# System dependencies (Debian/Ubuntu)
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libgtk-3-dev \
  build-essential curl file git

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

# Node.js 20+ (any method; use your distro's or nvm)
```

Then:

```bash
git clone https://github.com/kjustinkeener/Moonpool.git moonpool
cd moonpool
npm install
npm run tauri dev      # run in dev
npm run tauri build    # produce .deb / .AppImage under src-tauri/target/release/bundle/
```

## Configuring your apps

Moonpool reads a user-editable manifest at:

```
~/.config/Moonpool/apps.json
```

Seeded from an example on first run. Edit it via the sidebar **⋯ menu** (Add app / Edit
apps.json / Reload), or by hand, then Reload. See the app schema in `AI-README.md`.

## Platform notes

- Launch commands run through `sh -c <command>` (vs `cmd /c` on Windows), so use shell syntax
  your `$SHELL` understands.
- Stop kills the process group; port-freeing uses `lsof` (falls back to `fuser`) - install
  `lsof` if your distro doesn't ship it.
- **A `desktop` app's `processName` must be 15 characters or fewer on Linux.** Running-status
  detection and stop-by-name match the kernel's process name (`comm`), which Linux truncates to
  15 characters, so a longer binary name won't be detected or killed by name. (`web` apps match on
  their port instead, so they're unaffected.)
- **Icon discovery works cross-platform** (an app's `src-tauri/icons/`, `public/favicon.*`,
  `icon.png`, or its live favicon), but extracting an icon from a binary is Windows-only, so
  Linux relies on those sources plus favicons.
