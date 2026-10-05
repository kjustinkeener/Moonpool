---
title: Sidebar and Menus
description: Sidebar rows, status dots, groups, the filter box, the row context menu, and resizing the sidebar.
---

The sidebar lists every app in `apps.json`, grouped by each app's `group` field. See [Fields](/configuration/fields/).

## Rows

Each row shows a status dot, the app icon (or a type glyph if there is no icon), the name, the port if set (`:3000`), and controls.

| Dot | Meaning |
| --- | --- |
| Solid | running |
| Pulsing | starting: Moonpool started the app but it is not detected up yet |
| Grey | stopped |

Hover the dot for the word.

| Control | Does |
| --- | --- |
| Pencil | Edit the app. |
| Restart | Stop and relaunch. On a stopped app it simply launches it. |
| Launch (play) | Starts the app and opens its terminal tab. Shown when the app is stopped. |
| Stop (square) | Stops the app. Shown while it is running or starting. |

While a launch or stop is in progress the controls are replaced by a spinner (`Working...`).

Clicking an app's **name** opens or focuses its terminal tab and never launches anything. A tab for a stopped app shows the log from this session. Use Launch or Restart to start it. A `static` app with only a `url` and no `command` has no terminal: Launch opens the URL in your browser.

### Tooltip

Hovering the name shows the app's `note` if it has one, otherwise its name. Set `note` in the editor or in `apps.json`.

### MCP sub-row

When an AI client has used Moonpool's MCP tools for an app, a dimmed sub-row `MCP server` appears under it. Its dot is lit and the tooltip reads "MCP client attached" while the client is connected, and a stop button ends that process. Hide these rows with **Show MCP processes** in Settings. See [MCP setup](/automation/mcp-setup/).

## Groups

- Click a group heading to collapse or expand it. The count beside it is the number of apps shown. Collapsed groups are remembered.
- Inside a group, the most recently started app is on top. Apps never started keep their `apps.json` order. A freshly launched app glows and rises to the top.

## Filter box

Type in **Filter apps...** to narrow the list. It matches app name and group name, ignoring case. If nothing matches, the list shows `No apps match "<text>".`

## Right-click menu

Right-click a row for:

| Item | Does |
| --- | --- |
| Edit | Opens the app editor. |
| Rename | Turns the name into an edit field. **Enter** or clicking away saves, **Esc** cancels. An empty or unchanged name is ignored. |
| Set icon... | Pick an image file (png, jpg, jpeg, gif, svg, webp, ico) to use as the icon. |
| Delete | Asks `Delete "<name>"?` and removes the entry from `apps.json`. If Moonpool is running the app, it is stopped first. |

**Esc** closes the menu without acting.

## Resizing

Drag the divider between the sidebar and the CLI pane. The width is limited to 180 to 620 px (default 280) and is remembered. The divider is locked while the CLI pane is collapsed. See [Terminal](/using/terminal/).

![Hub window with the sidebar expanded](../../../assets/screenshots/hub-expanded.png)
