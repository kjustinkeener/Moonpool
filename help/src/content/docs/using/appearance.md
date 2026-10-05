---
title: Appearance
description: Themes, languages, background transparency and UI scale.
---

Theme, language and transparency are set in the [Settings window](/using/settings-window/).
All three apply instantly to every open Moonpool window.

## Themes

| Theme | Theme | Theme |
| --- | --- | --- |
| Auto (system) | Dark | Light |
| Midnight Purple | Ocean | Matrix |
| Amber | Rose | Nord |
| Dracula | Gruvbox | Solarized |
| Crimson | Mint | Paper |
| Sky | Lavender | Sage |

**Auto (system)** follows the operating system's light or dark preference and switches live
when the OS does. Any other choice is fixed.

The theme is stored in the webview's `localStorage` (key `moonpool.theme`), not in
`settings.json`. If storage is unavailable it falls back to Auto.

## Languages

Auto follows the OS language. Otherwise pick one of 14, each shown in its own language:

English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文.

The picker applies instantly to the hub and the other windows. The choice is saved as
`locale` in `settings.json`.

## Transparency

**Background transparency** makes the window background see-through, from 0% (solid) to
90%.

- Hovering the pointer over a window wakes it to fully opaque at once. When the pointer leaves, it fades back to your setting over about 2 seconds.
- Terminals follow the same tint rather than adding their own.
- Each window (hub, Settings, About) applies the setting itself, and Settings updates the others live as you drag the slider.

## UI scale

Zoom the whole interface with the keyboard or mouse wheel. See
[Shortcuts and zoom](/using/shortcuts-and-zoom/).
