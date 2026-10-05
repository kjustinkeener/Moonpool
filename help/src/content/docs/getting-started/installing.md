---
title: Installing
description: How to install Moonpool, and how portable mode differs.
---

Moonpool is its own installer. The download is a single `moonpool.exe`.

## Installed mode

Run the downloaded `moonpool.exe`. On first launch it shows a one-button install card.
Installing copies Moonpool into your user profile under `.moonpool\`, adds Start Menu
(and optional desktop) shortcuts, and registers an entry in Add/Remove Programs. After
that, launch it from the shortcut like any other app.

![The install card: Install Moonpool button, desktop shortcut checkbox, Install portable link and the install path](../../../assets/screenshots/installer-window.png)

Everything Moonpool needs lives under that one folder: the program, your
configuration, and its bundled help.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Install Moonpool... from the menu

The "..." menu has **Install Moonpool...** in both modes. It opens the same install card.
From a portable copy you can install it properly. From an installed copy **Install
Moonpool** is disabled ("Already installed") and **Install portable** stays available.

## Uninstalling

Use Windows Add/Remove Programs (Installed apps), or run:

```powershell frame="terminal"
moonpool.exe --uninstall
```

This removes the Start Menu and desktop shortcuts, the registry entry, and the whole
`%USERPROFILE%\.moonpool` folder, **including your configuration** (`apps.json`, settings
and logs). Back up this folder first if you want to keep your configuration:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Any running Moonpool is stopped as part of uninstalling.

## Portable mode

Prefer to keep Moonpool on a USB stick or a movable folder? Choose **Portable** during
setup and pick a folder. Moonpool creates a single `.moonpool\` folder inside it holding
the program and all of its data, so you can move or copy the whole folder to another PC
and run it there. Nothing is written to Windows AppData.

## Next

- [First launch](/getting-started/first-launch/)
