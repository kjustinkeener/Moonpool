# Translating the Moonpool help

The help site is Starlight/Astro. English is the root locale (no URL prefix, `src/content/docs/`).
Each translation lives in a directory under `src/content/docs/<dir>/` with the same file paths
and slugs as the English pages. German (`de`) is the pilot and the reference for the other
languages. English is the source: when in doubt, re-read the English page.

## Locale directories

One rule maps the app's locale ids (`src/lib/locales/<id>.ts`) to help directories: lowercase the
id. The single source of truth is `help/locales.mjs`; the app side is `helpUrl()` in `src/lib/api.ts`.

| App locale | Help directory | `lang` (html, hreflang) | URL |
| --- | --- | --- | --- |
| en | (root) | en | `/` |
| de | `de` | de | `/de/` |
| es | `es` | es | `/es/` |
| fr | `fr` | fr | `/fr/` |
| it | `it` | it | `/it/` |
| ja | `ja` | ja | `/ja/` |
| ko | `ko` | ko | `/ko/` |
| nl | `nl` | nl | `/nl/` |
| pl | `pl` | pl | `/pl/` |
| pt-BR | `pt-br` | pt-BR | `/pt-br/` |
| ru | `ru` | ru | `/ru/` |
| tr | `tr` | tr | `/tr/` |
| zh-Hans | `zh-hans` | zh-Hans | `/zh-hans/` |
| zh-Hant | `zh-hant` | zh-Hant | `/zh-hant/` |

## Fallback

A page that has no file in a locale directory is still built under that locale's URL, as the
English page with Starlight's "not translated yet" notice. Its canonical URL points at the English
page. So a partial translation always builds; translate pages in any order.

## Rules for every page

1. **Same path, same slug.** `docs/apps/fields.md` becomes `docs/de/apps/fields.md`. Never
   translate file names or URLs.
2. **Frontmatter.** Translate `title` and `description`; keep `description` at about 120-160
   characters (CJK languages: roughly 60-90 characters). Keep any other frontmatter keys as they are.
3. **UI labels must match the app.** Buttons, menu items, settings names, status names and
   dialog titles quoted from Moonpool must use the exact text the app shows in that language. Find
   the English string in `src/lib/locales/en.ts`, take the same key from
   `src/lib/locales/<id>.ts`, and copy that text. Never invent a different translation for a label
   the app already translates. If the app does not translate a label (it shows English or the
   catalog lacks the key), keep the English text.
   The two tray menu items ("Show Moonpool", "Quit") are not in the JS catalog: take them from
   `src-tauri/src/i18n.rs`.
4. **Never translate:**
   - code blocks and inline code;
   - `apps.json` field names and values (`killMode`, `"type": "web"`), CLI verbs, MCP tool names,
     file paths, environment variable names, JSON keys;
   - error strings that the app, Node, Vite, Python, Docker or Windows print in English. Keep
     the English text in quotes or code; you may add a translation in parentheses after it;
   - Windows UI text (for example SmartScreen): use the official localized Windows string only if you
     are certain of it, otherwise keep English and add a translation in parentheses.
5. **Links.** Keep root-absolute links, but point them at the same locale:
   `[Fields](/apps/fields/)` becomes `[...](/de/apps/fields/)`. Starlight does not rewrite
   them, so every internal link in a translated page needs the `/<dir>/` prefix (the web build adds
   the `/software/moonpool/help/` base automatically; do not write it). Fragment links (`#some-heading`): write the ENGLISH fragment, unchanged. Starlight builds a heading id from the heading text, so a translated heading has a different id; after translating, run `npm run fix:fragments -- <dir>` and it rewrites every fragment link to the id of the translated heading at the same position (so each translated page must keep exactly the same headings, in the same order, as the English page).
6. **Images.** Keep the image, translate the alt text. Relative paths gain one `../` because the
   file is one directory deeper (`../../../assets/...` becomes `../../../../assets/...`). The
   screenshots show the English UI; do not edit them.
7. **Register.** Natural, idiomatic technical documentation. Use the formal address where the
   language has one (German "Sie", French "vous", and so on). Keep terminology consistent across
   pages (see "Terms" below) and with the app's catalog.
8. **Structure.** Keep headings, lists, tables, bold and code formatting one to one with the English.
   The troubleshooting page builds FAQ structured data from its `##` headings, so keep each
   question a `##` heading.
9. **No em-dashes** (U+2014) in any committed content or commit message. Use a comma, colon,
   parentheses or a hyphen with spaces where English used one.
10. Product and feature names (Moonpool, FasterDB, Claude Code, Codex, Cursor, Windows, WebView2,
    Tauri, npm) stay as they are. Theme names are proper nouns and are not translated.

## Terms (German)

Use the app's catalog first (`src/lib/locales/de.ts`). Words the catalog does not settle:
app = App, dev server = Entwicklungsserver, tray = Tray (Infobereich on first use), hub window =
Hub-Fenster, port = Port, process = Prozess, background = Hintergrund, tab = Tab, log = Protokoll
(a log file: Protokolldatei), backup = Sicherung, portable mode = portabler Modus, kill (a process) =
beenden, launch = starten. Windows SmartScreen: "Der Computer wurde durch Windows geschützt",
"Weitere Informationen", "Trotzdem ausführen", "Unbekannter Herausgeber".

## Terms (Simplified Chinese, zh-Hans)

Mainland usage. Use the app's catalog first (`src/lib/locales/zh-Hans.ts`). Words the catalog does
not settle: app = 应用, dev server = 开发服务器, tray = 托盘, hub window = 主窗口, hub (the resident
process) = hub, port = 端口, process = 进程, kill a process = 结束进程, terminal = 终端, tab = 标签页,
log = 日志, backup = 备份, portable mode = 便携模式, launch = 启动, folder = 文件夹, working
directory = 工作目录, command line = 命令行, environment variable = 环境变量, session = 会话,
sidebar = 侧边栏, group = 分组, URL = 网址, file = 文件, server = 服务器, window = 窗口. Address
the reader as 你. Put a space between CJK and Latin letters or digits. The "Reveal" and "Copy" log
buttons in Settings have no catalog key and stay English in the text. Windows SmartScreen: "Windows
已保护你的电脑", "更多信息", "仍要运行", "未知发布者".

## Terms (Traditional Chinese, zh-Hant)

Taiwan usage, translated from English on its own, never converted from zh-Hans. Use the app's
catalog first (`src/lib/locales/zh-Hant.ts`). Words the catalog does not settle: app = 應用程式,
dev server = 開發伺服器, tray = 系統匣, hub window = 主視窗, port = 連接埠, process = 處理程序, kill a
process = 結束處理程序, terminal = 終端機, tab = 分頁, log = 記錄 (log file: 記錄檔), backup = 備份,
portable mode = 可攜模式, launch = 啟動, folder = 資料夾, working directory = 工作目錄, command line =
命令列, environment variable = 環境變數, session = 工作階段, sidebar = 側邊欄, group = 群組, URL =
網址, file = 檔案, server = 伺服器, window = 視窗, program = 程式, script = 指令碼, theme = 佈景主題
(the catalog's word), token = 權杖, manifest = 資訊清單, JSON key = 索引鍵, named pipe = 具名管道,
socket = 通訊端, argument = 引數, parse = 剖析, sandbox = 沙箱, runtime = 執行階段, restore a
snapshot = 還原, roll back = 回復. Address the reader as 你 (the catalog does). Put a space between CJK
and Latin letters or digits. The "Reveal" and "Copy" log buttons in Settings have no catalog key and
stay English in the text. Windows SmartScreen: "Windows 已保護您的電腦", "其他資訊", "仍要執行",
"不明的發行者" (Microsoft's own wording uses 您).

## Sidebar and site strings

- Sidebar group labels: `GROUPS` in `help/locales.mjs` (all 13 languages done).
- Sidebar page labels: German is the third argument of `page(...)` in `help/astro.config.mjs`.
  Every other language goes in its own file, `help/sidebar-labels/<tag>.json`, using the tag from
  the table above (`fr.json`, `pt-BR.json`, `zh-Hans.json`). It maps each page slug from
  `astro.config.mjs` to its label: `{ "index": "...", "apps/fields": "..." }`. Do not edit
  `astro.config.mjs` or `locales.mjs` for a new language's labels.
- Web-only Download link and footer line: `WEB_STRINGS` in `help/locales.mjs`.
- Starlight's own UI text (search, "On this page", ...) comes from Starlight's built-in
  translations. `zh-Hant` has no built-in entry under that tag, so `src/content/i18n/zh-hant.json`
  supplies it.

## Adding a new language

1. Add it to `NON_ENGLISH` in `help/locales.mjs` and add its entries to `GROUPS` and `WEB_STRINGS`.
2. Add the locale to the app (`src/lib/locales/`, `LOCALES` in `src/lib/i18n.svelte.ts`); the help
   directory is its lowercased id automatically.
3. Create `src/content/docs/<dir>/` and translate pages.

## Verifying

From `help/`:

```
npm run build
npm run build:web
npm run fix:fragments -- <dir>   # before checking links, after translating
npm run check:links
```

`check:links` must report 0 problems for both builds. It checks every built page, including
locale pages and fallback pages: internal links, images and `#fragments`. Then open one translated
page in `dist-web/<dir>/.../index.html` and check `<html lang>`, the `rel="alternate" hreflang`
links (absolute URLs on the web build), the canonical link and the BreadcrumbList JSON-LD. Run
`npm run dev` and read a few pages in the browser if you can.

For label fidelity, grep your translation for each quoted UI label and confirm it appears in
`src/lib/locales/<id>.ts`.
