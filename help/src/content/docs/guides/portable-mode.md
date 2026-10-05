---
title: Portable Mode
description: Run Moonpool from a movable folder with all of its data beside it.
---

Portable mode keeps Moonpool and everything it writes inside one `.moonpool\` folder, so
you can carry it on a USB stick or drop it in a synced folder and run it on any PC.

## How it works

When you install portably, Moonpool creates a `.moonpool\` folder inside the location you
choose. That folder holds the program, your configuration, and its help content. Nothing
is written to Windows AppData, so moving or copying the folder moves your whole setup with
it.

```text
<chosen location>\.moonpool\
```

## Making your apps travel too

Use the `{MP_HOME}` token in an app's path so it points inside the portable folder rather
than at a fixed location on one machine:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

See [Paths and environment](/configuration/paths-and-environment/) for how paths resolve.

## Choosing portable from the installer

Portable mode is set up from the installer card, which offers **Install portable**
alongside **Install Moonpool**.

![The install card: the Install portable link sits under the main Install Moonpool button](../../../assets/screenshots/installer-window.png)

Pick a folder and Moonpool creates the `.moonpool\` folder
there and starts from it with a fresh configuration. The card is also available from the
"..." menu as **Install Moonpool...** in both installed and portable mode, so you can create
a portable copy from a running Moonpool. A portable copy starts fresh and does not copy your
existing apps; copy `apps.json` across by hand if you want them.
