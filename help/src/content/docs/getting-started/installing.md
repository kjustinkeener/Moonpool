---
title: Installing
description: How to install Moonpool, and how portable mode differs.
---

This page is for Windows. On Windows, Moonpool is its own installer: the download is a
single `moonpool.exe`. Linux has no install card or portable chooser; see
[Linux](/platforms/linux/).

## Installed mode

Run the downloaded `moonpool.exe`. On first launch it shows the install card. It has three
controls: the **Install Moonpool** button, a **desktop shortcut** checkbox (on by default)
and an **Install portable** link.

Installing copies Moonpool into your user profile under `.moonpool\`, adds a Start Menu
shortcut (and a desktop one if the box is ticked), and registers an entry in Add/Remove
Programs. Then it starts the installed copy and closes. The file you downloaded stays where
it was; you can delete it. After that, launch Moonpool from the shortcut like any other app.

![The install card: Install Moonpool button, desktop shortcut checkbox, Install portable link and the install path](../../../assets/screenshots/installer-window.png)

Everything Moonpool needs lives under that one folder: the program, your
configuration, and its bundled help.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Install Moonpool... from the menu

On Windows the "..." menu has **Install Moonpool...** in both modes. It opens the same install card.
From a portable copy you can install it properly. From an installed copy **Install
Moonpool** is disabled ("Already installed") and **Install portable** stays available.

## Uninstalling

Use Windows Add/Remove Programs (Installed apps), or run the installed copy with
`--uninstall`. It is not on your PATH, so give its full path:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

This removes the Start Menu and desktop shortcuts, the registry entry, and the whole
`%USERPROFILE%\.moonpool` folder, **including your configuration** (`apps.json`, settings
and logs). Back up this folder first if you want to keep your configuration:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Any running Moonpool is stopped as part of uninstalling.

## Portable mode

Prefer a USB stick or a movable folder? Click **Install portable** on the install card and
pick a folder. See [Portable mode](/guides/portable-mode/).

## Next

- [Your first app](/getting-started/first-launch/)
