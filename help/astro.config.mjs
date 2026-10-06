// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { satteri } from '@astrojs/markdown-satteri';
import baseLinksPlugin from './base-links-plugin.mjs';
import { starlightLocales, group, page } from './locales.mjs';

// Moonpool help site.
//
// One static build serves two targets:
//   - in-app (offline): the built `dist/` is bundled with Moonpool and served to the
//     help window through a Tauri custom protocol at the root path, so absolute asset
//     URLs (`/_astro/...`) resolve. Do NOT open it via file://; absolute paths break.
//   - web: a second build (see below) deployed under the product site.
//
// The in-app build MUST keep `base: '/'`: the custom URI scheme serves it at its root.
// The web copy lives in a subdirectory of the product site, so `npm run build:web`
// sets HELP_BASE (for example '/software/moonpool/help/'), which switches on:
//   - `base` and `site` (canonical URLs, sitemap),
//   - a Markdown (hast) plugin that prefixes root-absolute Markdown links, which Astro does
//     not rewrite on its own,
//   - a separate `dist-web/` output so it never overwrites the in-app `dist/`.
// With HELP_BASE unset nothing below changes the default build.
//
// Sidebar convention (shared across all apps): each top-level entry is a category
// GROUP whose `label` is a non-navigable heading; only the `items` (pages) are links.
const webBase = process.env.HELP_BASE;

export default defineConfig({
	...(webBase && {
		base: webBase,
		site: 'https://fasterdb.com',
		outDir: 'dist-web',
		markdown: { processor: satteri({ hastPlugins: [baseLinksPlugin(webBase)] }) },
	}),
	// Older Moonpool builds (v0.3.15 and earlier) opened the help at this path. The root is now
	// the overview page itself. Astro does not prefix redirect targets with `base`, so do it here.
	redirects: {
		'/getting-started/overview': (webBase ? webBase.replace(/\/+$/, '') : '') + '/',
	},
	integrations: [
		starlight({
			title: 'Moonpool Help',
			// English is the root locale (no URL prefix); the others live under /<dir>/. Locale map and
			// the app-locale -> directory rule: ./locales.mjs. Pages missing from a locale fall back to
			// the English page (with a notice), so a partial translation still builds.
			defaultLocale: 'root',
			locales: starlightLocales,
			customCss: ['./src/styles/screenshots.css'],
			// "Last updated" from git history (the web build is made from a full checkout).
			lastUpdated: true,
			// Breadcrumb/FAQ JSON-LD and og:image (web build only; see src/routeData.ts).
			routeMiddleware: './src/routeData.ts',
			// Web-only links out to the product site; the in-app build keeps Starlight's defaults.
			...(webBase && {
				components: {
					SocialIcons: './src/components/SocialIcons.astro',
					Footer: './src/components/Footer.astro',
				},
			}),
			// No external social links in bundled app help.
			social: [],
			// Two-level navigation: category label -> pages.
			// Folders match these groups. The overview is the site root (index).
			sidebar: [
				group('Getting started', [
					page('index', 'What is Moonpool', 'Was ist Moonpool'),
					page('getting-started/install', 'Installing', 'Installation'),
					page('getting-started/first-app', 'Your first app', 'Ihre erste App'),
					page('getting-started/example-dashboards', 'Example dashboards', 'Beispiel-Dashboards'),
					page('getting-started/whats-new', "What's new", 'Neuigkeiten'),
				]),
				group('How-to guides', [
					page('guides/run-npm-dev-server-in-background-windows', 'Run an npm dev server in the background', 'npm-Entwicklungsserver im Hintergrund ausführen'),
					page('guides/start-app-at-windows-login', 'Start an app at Windows login', 'App bei der Windows-Anmeldung starten'),
					page('guides/find-and-kill-process-using-port-windows', 'Find and kill the process using a port', 'Prozess finden und beenden, der einen Port belegt'),
					page('guides/mcp-server-for-ai-agent-to-start-stop-local-apps', 'Give an AI agent an MCP server for your apps', 'KI-Agenten einen MCP-Server für Ihre Apps geben'),
					page('guides/keep-python-script-running-background-windows', 'Keep a Python script running', 'Python-Skript dauerhaft laufen lassen'),
				]),
				group('Using Moonpool', [
					page('using/hub-window', 'The hub window', 'Das Hub-Fenster'),
					page('using/sidebar-and-menus', 'Sidebar and menus', 'Seitenleiste und Menüs'),
					page('using/terminal-tabs', 'Terminal tabs', 'Terminal-Tabs'),
					page('using/tray-and-closing', 'Tray, close and minimize', 'Tray, Schließen und Minimieren'),
					page('using/keyboard-shortcuts', 'Shortcuts and zoom', 'Tastenkürzel und Zoom'),
					page('using/settings', 'Settings window', 'Einstellungsfenster'),
					page('using/themes-and-language', 'Themes, language and transparency', 'Designs, Sprache und Transparenz'),
				]),
				group('Configuring apps', [
					page('apps/apps-json', 'Overview', 'Überblick'),
					page('apps/add-an-app', 'Adding apps', 'Apps hinzufügen'),
					page('apps/examples', 'Examples', 'Beispiele'),
					page('apps/types', 'App types', 'App-Typen'),
					page('apps/fields', 'App fields', 'App-Felder'),
					page('apps/paths-and-environment', 'Paths and environment', 'Pfade und Umgebung'),
					page('apps/stop-and-restart', 'Stop and restart', 'Stoppen und Neustarten'),
				]),
				group('Data, updates and recovery', [
					page('data/logs', 'Logs', 'Protokolle'),
					page('data/backup-and-recovery', 'Backup and recovery', 'Sicherung und Wiederherstellung'),
					page('data/settings-json', 'settings.json', 'settings.json'),
					page('data/portable-mode', 'Portable mode', 'Portabler Modus'),
					page('data/updating', 'Updating', 'Aktualisieren'),
				]),
				group('Automation and AI agents', [
					page('automation/quick-start', 'AI agents: quick start', 'KI-Agenten: Schnellstart'),
					page('automation/overview', 'Overview', 'Überblick'),
					page('automation/mcp-setup', 'MCP setup', 'MCP-Einrichtung'),
					page('automation/mcp-tools', 'MCP tools', 'MCP-Tools'),
					page('automation/command-line', 'Command line', 'Befehlszeile'),
					page('automation/control-verbs', 'Control verbs (advanced)', 'Steuerbefehle (erweitert)'),
				]),
				group('Platform notes', [
					page('platforms/windows', 'Windows', 'Windows'),
					page('platforms/linux', 'Linux', 'Linux'),
				]),
				group('Support', [
					page('support/troubleshooting', 'Troubleshooting and FAQ', 'Fehlerbehebung und FAQ'),
					page('support/error-messages', 'Error messages explained', 'Fehlermeldungen erklärt'),
					page('support/port-already-in-use', 'Port already in use (EADDRINUSE)', 'Port bereits belegt (EADDRINUSE)'),
					page('support/windows-protected-your-pc', 'Windows protected your PC', 'Der Computer wurde durch Windows geschützt'),
					page('support/webview2-runtime-missing', 'WebView2 runtime missing', 'WebView2-Runtime fehlt'),
					page('support/glossary', 'Glossary', 'Glossar'),
				]),
			],
		}),
	],
});
