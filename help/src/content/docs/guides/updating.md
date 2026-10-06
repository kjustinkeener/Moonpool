---
title: Updating
description: How Moonpool keeps itself and its help content up to date.
---

Moonpool updates itself. There is no separate installer to download and no wizard to click
through.

## How updates arrive

Moonpool fetches `update.json` from the project's GitHub Releases, compares versions, and
only offers a strictly newer one. It checks at startup (turn this off with **Check for
updates on startup** in Settings) and whenever you press **Check for updates** in the About
window. The About button installs a newer version straight away and restarts Moonpool.

At startup, a found update shows as a banner on the hub's empty screen (the one shown when
no app tab is open):

```text
Moonpool X is available
```

Click **Download & install** and Moonpool
replaces itself and relaunches, or dismiss it with the x. See
[Settings window](/using/settings-window/).

Every download is verified against Moonpool's minisign signing key before it is applied, so
a tampered or corrupted download is rejected. Moonpool never installs an older version.

On Linux only the AppImage updates itself; see [Linux](/platforms/linux/).

## Help updates too

This help ships inside Moonpool, so each program update brings the matching help with it.
The offline copy always matches the version you run.
