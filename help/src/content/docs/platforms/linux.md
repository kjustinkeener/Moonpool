---
title: Linux
description: Installing Moonpool on Linux, the GNOME tray caveat, updates and platform differences.
---

Moonpool runs on Linux through WebKitGTK. It is developed mainly on Windows, so Linux is
supported but less battle-tested. There is no install card or portable-mode chooser on
Linux, and the "..." menu has no **Install Moonpool...** item.

## Install

Download a package from the project's Releases page.

| Package | Updates |
| --- | --- |
| AppImage | Moonpool updates itself |
| `.deb` | Your package manager |
| RPM (install with your distribution's RPM tool) | Your package manager |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

The `.deb` pulls in its runtime dependencies. The AppImage needs the WebKitGTK and
AppIndicator libraries present, for example on Debian or Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

On Fedora or Arch use the equivalents:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Tray on GNOME

Stock GNOME does not show tray icons, so Moonpool's tray icon will not appear until the
AppIndicator extension is installed and enabled:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Then log out and in. The hub window and embedded terminals work without it. KDE, Cinnamon,
XFCE and MATE show the tray out of the box.

## Updates

Only the AppImage self-updates. It reads `linux-update.json` from GitHub Releases, verifies
the minisign signature, and replaces the AppImage file in place, so keep it in a folder you
can write to. `.deb` and RPM installs are never overwritten by Moonpool: the update check
can still report a newer version, but installing it from Moonpool fails with a message to
use your package manager. See [Updating](/data/updating/).

## Config location

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` is seeded from the example on first run. See
[Configuration overview](/apps/apps-json/).

## Differences from Windows

- Launch commands run through `$SHELL -c <command>` (`/bin/sh` if `SHELL` is unset), so use syntax your shell understands.
- Stop kills the process group, then does the extra cleanup chosen by `killMode`. Freeing a port under `killMode: "port"` uses `lsof`, falling back to `fuser`; install `lsof` if your distribution does not ship it. See [Stop and restart](/apps/stop-and-restart/).
- A `desktop` app's `processName` must be 15 characters or fewer. Linux truncates a process name to 15 characters, so a longer name is never detected as running and cannot be stopped by name. `web` apps match on their port and are unaffected.
- Icons are found from an app's `src-tauri/icons/`, `public/favicon.*`, `icon.png` or its live favicon. Extracting an icon from a binary is Windows-only.
- Reveal buttons open the containing folder rather than selecting the file.
- Config files open in your default text editor (resolved from the `text/plain` association).
- The Windows installer, shortcuts and Add/Remove entry do not apply.

## See also

- [Windows](/platforms/windows/#what-differs-by-platform): a table of what differs by platform.
- [Updating](/data/updating/#linux)
