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

## Making your apps travel too

Use the `{MP_HOME}` token in an app's path so it points inside the portable folder rather
than at a fixed location on one machine. See [Adding apps](/guides/adding-apps/) for how
paths resolve.

## Turning it into a portable copy later

From an installed Moonpool you can export a portable copy to a folder you pick, optionally
cloning your current apps and settings into it. The original install is left untouched.
