---
title: Shortcuts and Zoom
description: Keyboard shortcuts, mouse shortcuts and UI zoom in the Moonpool hub.
---

## Keyboard

| Keys | Where | Does |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Hub | Reloads `apps.json` from disk, the same as **Reload** in the menu. The page itself is not refreshed. |
| Esc | Context menu | Closes it. |
| Esc | Renaming an app | Cancels the rename. |
| Esc | Settings, About and app editor windows | Closes the window (the editor asks before discarding changes). |
| Enter | Renaming an app | Saves the new name. |

## Mouse

| Action | Where | Does |
| --- | --- | --- |
| Ctrl + wheel | Hub | Zooms the UI. |
| Select text | Terminal | Copies and clears the selection. |
| Middle-click | Terminal | Pastes. |
| Right-click | Sidebar row | Opens the row menu. See [Sidebar and menus](/using/sidebar-and-menus/). |

## Zoom

Hold Ctrl and scroll the wheel over the hub to zoom. Scrolling up zooms in and down zooms out, in steps of about 10 percent per wheel event.

- The range is 0.5x to 3x.
- The window resizes by the same factor, so the layout stays as tight at 2x as at 1x. Once the limit is reached the window stops growing.
- The factor is saved as `uiScale` in `settings.json` and applied at the next start. The saved window size is already the zoomed size, so it is not scaled again. See [Settings and logs](/configuration/settings-and-logs/).

There is no Settings control for `uiScale`; use the wheel.
