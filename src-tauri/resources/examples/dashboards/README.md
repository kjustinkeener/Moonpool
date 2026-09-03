# Moonpool example dashboards

A small suite of self-contained dashboards that ship with Moonpool. Every one runs
**offline**, with no server, no build step, and no network access at runtime. They
open as plain static HTML in your browser, so the same files work identically on
Windows, macOS, and Linux.

They serve two purposes: a polished example of what a local dashboard can look like,
and working templates you can copy and point at your own data or docs.

## What is here

| Folder  | What it is | Source format |
| ------- | ---------- | ------------- |
| `csv/`  | Drop-your-data explorer | CSV / TSV |
| `json/` | Drop-your-data explorer | JSON (arrays, nested objects, maps) |
| `jsonl/`| Drop-your-data explorer | JSONL / NDJSON |
| `yaml/` | Drop-your-data explorer | YAML |
| `toml/` | Drop-your-data explorer | TOML |
| `xlsx/` | Drop-your-data explorer | Excel `.xlsx` / `.xls` |
| `docs/` | Offline Markdown docs browser (hosts Moonpool's own help) | Markdown |
| `_lib/` | Shared design system, analyzer, and vendored libraries | (support code) |

## The explorers

Each explorer opens on differently-shaped baked sample tabs so it looks good
immediately, and also accepts your own file (the **Load** button, or drag-and-drop in
a browser that allows it). The file is parsed entirely in the browser; nothing is
uploaded.

A dropped file can be any shape, so each explorer auto-profiles the data (inferring
each field's type and cardinality), picks a sensible default chart, and exposes a
field-mapping strip so you can re-shape it: pick the X axis, toggle measures, group
by a field, and switch chart type. Your mapping per tab is remembered in the
browser's local storage.

## The docs browser

`docs/` renders a folder of Markdown with a searchable sidebar, an on-this-page
outline, and copy buttons on code blocks. It hosts Moonpool's own help offline, and
the **Open a .md file** button will render any Markdown file you point it at. Copy
the folder, swap in your own `.md` files, and you have an offline docs site.

## How it is built

- **No build step.** Plain HTML, CSS, and a little ES5-style JavaScript.
- **One shared design system.** `_lib/theme.css` holds the tokens and components and
  `_lib/echarts-theme.js` the matching chart theme, so the whole suite reads as one
  piece. `_lib/profile.js`, `_lib/records.js`, and `_lib/explorer.js` are the shared
  auto-profiler, shape-normalizer, and explorer widget.
- **Everything vendored.** All third-party libraries are copied into `_lib/vendor/`
  and referenced by relative path, so nothing is fetched at runtime.

## Editing and updating

These files are written into your Moonpool folder on first run. You can edit them
freely: on later updates Moonpool writes any missing files but never overwrites one
that already exists, so your changes are safe. To restore an original, delete your
copy and relaunch Moonpool.

Paths in the app entries use `{MP_HOME}` (Moonpool's folder) or a `./` relative path,
so the tiles keep working when a portable bundle is moved.

## Third-party libraries

All are vendored under `_lib/vendor/`, unmodified, and used under their own licenses.

| Library | Version | License | Used by |
| ------- | ------- | ------- | ------- |
| Apache ECharts | 5.6.0 | Apache-2.0 | all explorers (charts) |
| PapaParse | 5.4.1 | MIT | `csv/` |
| js-yaml | 4.1.0 | MIT | `yaml/` |
| j-toml | 1.38.0 | MIT | `toml/` |
| SheetJS (xlsx) | 0.18.5 | Apache-2.0 | `xlsx/` |
| marked | 12.0.2 | MIT | `docs/` |

Each library's copyright and license notice is retained in its vendored file.
