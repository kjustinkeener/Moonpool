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
7. **Register.** Natural, idiomatic technical documentation. Use the same form of address as the
   app's own catalog for that language (German uses "Sie"; Dutch uses "je"), so the help and the
   UI never disagree. Keep terminology consistent across
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

## Terms (Japanese)

Register and address follow the app catalog (`src/lib/locales/ja.ts`): polite desu/masu, no
second-person address. Use the catalog first. Words it does not settle: app = アプリ, dev server =
開発サーバー, tray = トレイ, hub = ハブ, hub window = ハブウィンドウ, CLI pane = CLI パネル,
process = プロセス, port = ポート, log = ログ, session = セッション, session log = セッションログ,
dump = ダンプ, snapshot = スナップショット, scrollback = スクロールバック, control channel =
制御チャネル, verb = 動詞, ticket = チケット, token = トークン, config folder = 設定フォルダー,
manifest = マニフェスト, backup = バックアップ, portable mode = ポータブルモード (catalog), portable
copy = ポータブル版, installed = インストール版, launch = 起動, stop = 停止, restart = 再起動,
background = バックグラウンド. Windows SmartScreen strings stay in English with a Japanese gloss in
parentheses (Windows によって PC が保護されました, 詳細情報, 実行, 不明な発行元). Error strings printed by
Moonpool's backend, Node or Vite stay in English; strings the app catalog translates (update banner,
"name は必須です。", the apps.json banners) use the catalog text.

## Terms (Korean)

Register and address follow the app catalog (`src/lib/locales/ko.ts`): polite 합니다체 and 해요체 as
the catalog uses them in prose, no second-person address; particles as in the catalog (Moonpool을,
apps.json에). Use the catalog first. Words it does not settle: app = 앱, dev server = 개발 서버, tray
= 트레이, hub = 허브, hub window = 허브 창, CLI pane = CLI 창, process = 프로세스, port = 포트, log =
로그, session = 세션, session log = 세션 로그, dump = 덤프, snapshot = 스냅샷, scrollback = 스크롤백,
control channel = 제어 채널, verb = 동사, ticket = 티켓, token = 토큰, config folder = 설정 폴더,
manifest = 매니페스트, backup = 백업, portable mode = 포터블 모드 (catalog), portable copy = 포터블
복사본, installed = 설치형, launch = 실행, stop = 중지, restart = 다시 시작, background = 백그라운드,
taskbar = 작업 표시줄. Windows SmartScreen strings stay in English with a Korean gloss in parentheses
(추가 정보, 실행, 알 수 없는 게시자). Error strings printed by Moonpool's backend, Node or Vite stay in
English; strings the app catalog translates (update banner, "name은 필수입니다.", the apps.json
banners) use the catalog text.

## Terms (French)

Address form follows the app catalog (`src/lib/locales/fr.ts`): vous. Use the catalog first. Words it does not settle: app = app, dev server = serveur de développement, tray = zone de notification, hub window = fenêtre du hub, sidebar = barre latérale, CLI pane = panneau CLI, log = journal (log file: fichier journal), backup = sauvegarde, portable mode = mode portable, process = processus, background = arrière-plan, tab = onglet, launch = lancer, stop = arrêter, restart = redémarrer, kill (a process) = arrêter or terminer. Windows SmartScreen: « Windows a protégé votre ordinateur », « Informations complémentaires », « Exécuter quand même » (English in parentheses on first mention).

## Terms (Italian)

Address form follows the app catalog (`src/lib/locales/it.ts`): tu. Use the catalog first. Words it does not settle: app = app, dev server = server di sviluppo, tray = area di notifica (the catalog also says barra delle applicazioni in the close/minimize labels, copied verbatim), hub window = finestra hub, sidebar = barra laterale, log = registro (log file: file di registro), backup = backup, portable mode = modalità portatile, process = processo, background = in background, tab = scheda, launch = avviare, stop = arrestare, restart = riavviare, kill (a process) = terminare. Windows SmartScreen strings are kept in English with an Italian translation in parentheses.

## Terms (Dutch)

Use the app's catalog first (`src/lib/locales/nl.ts`). The address form follows the app catalog: "je"/"jouw" throughout, never "u". Words the catalog does not settle: tray = systeemvak,
sidebar = zijbalk, dev server = dev-server, hub window = hubvenster, process = proces, port = poort,
tab = tabblad, log = log (a log file: logbestand), backup = back-up, portable mode = draagbare modus
(the app badge stays "niet portable"), kill (a process) = beëindigen, launch = starten, running =
actief, control channel = besturingskanaal, command line = opdrachtregel. Windows SmartScreen strings
stay English with a Dutch gloss in parentheses, except the page name "Windows heeft uw pc beschermd".

## Terms (Polish)

Use the app's catalog first (`src/lib/locales/pl.ts`). The address form follows the app catalog:
imperatives and impersonal constructions, informal "ty" where a pronoun is needed, never "Państwo".
Words the catalog does not settle: tray = zasobnik systemowy (zasobnik), sidebar = pasek boczny, dev
server = serwer deweloperski, hub window = okno huba, process = proces, port = port, tab = karta,
log = dziennik, backup = kopia zapasowa, portable mode = tryb przenośny (the app badge stays
"nieprzenośna"), kill (a process) = zakończyć, launch = uruchomić, running = działa, control channel =
kanał sterowania, command line = wiersz poleceń, installer window = karta instalacji. Windows
SmartScreen strings stay English with a Polish gloss in parentheses.

## Terms (Russian)

Use the app's catalog first (`src/lib/locales/ru.ts`). The catalog addresses the user with the polite
plural lowercase "вы"/"ваш"; follow it. Quotes are «». Words the catalog does not settle: app =
приложение, dev server = сервер разработки, tray = трей, hub (the resident Moonpool instance) = хаб,
hub window = главное окно, port = порт, process = процесс, log = журнал, backup = резервная копия,
portable mode = портативный режим, sidebar = боковая панель, verb (control command) = команда, AI
agent = ИИ-агент. Windows SmartScreen strings are kept in English with the Russian in parentheses.

## Terms (Turkish)

Use the app's catalog first (`src/lib/locales/tr.ts`). Address the user with the formal "siz", as the
catalog does. Words the catalog does not settle: app = uygulama, dev server = geliştirme sunucusu, tray =
sistem tepsisi (tepsi where the catalog says so), hub = merkez, hub window = merkez penceresi, port =
bağlantı noktası, process = süreç (not "işlem", the catalog uses "süreç"), log = günlük, backup = yedek,
portable mode = taşınabilir mod, sidebar = kenar çubuğu, verb = fiil, session = oturum. Windows
SmartScreen strings are kept in English with the Turkish in parentheses.

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
