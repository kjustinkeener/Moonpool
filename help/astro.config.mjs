// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { satteri } from '@astrojs/markdown-satteri';
import baseLinksPlugin from './base-links-plugin.mjs';

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
			customCss: ['./src/styles/screenshots.css'],
			// "Last updated" from git history (the web build is made from a full checkout).
			lastUpdated: true,
			// Breadcrumb/FAQ JSON-LD and og:image (web build only; see src/routeData.ts).
			routeMiddleware: './src/routeData.ts',
			// No external social links in bundled app help.
			social: [],
			// Two-level navigation: category label -> pages.
			// Folders match these groups. The overview is the site root (index).
			sidebar: [
				{
					label: 'Getting started',
					items: [
						{ label: 'What is Moonpool', slug: 'index' },
						{ label: 'Installing', slug: 'getting-started/install' },
						{ label: 'Your first app', slug: 'getting-started/first-app' },
						{ label: 'Example dashboards', slug: 'getting-started/example-dashboards' },
						{ label: "What's new", slug: 'getting-started/whats-new' },
					],
				},
				{
					label: 'Using Moonpool',
					items: [
						{ label: 'The hub window', slug: 'using/hub-window' },
						{ label: 'Sidebar and menus', slug: 'using/sidebar-and-menus' },
						{ label: 'Terminal tabs', slug: 'using/terminal-tabs' },
						{ label: 'Tray, close and minimize', slug: 'using/tray-and-closing' },
						{ label: 'Shortcuts and zoom', slug: 'using/keyboard-shortcuts' },
						{ label: 'Settings window', slug: 'using/settings' },
						{ label: 'Themes, language and transparency', slug: 'using/themes-and-language' },
					],
				},
				{
					label: 'Configuring apps',
					items: [
						{ label: 'Overview', slug: 'apps/apps-json' },
						{ label: 'Adding apps', slug: 'apps/add-an-app' },
						{ label: 'Examples', slug: 'apps/examples' },
						{ label: 'App types', slug: 'apps/types' },
						{ label: 'App fields', slug: 'apps/fields' },
						{ label: 'Paths and environment', slug: 'apps/paths-and-environment' },
						{ label: 'Stop and restart', slug: 'apps/stop-and-restart' },
					],
				},
				{
					label: 'Data, updates and recovery',
					items: [
						{ label: 'Logs', slug: 'data/logs' },
						{ label: 'Backup and recovery', slug: 'data/backup-and-recovery' },
						{ label: 'settings.json', slug: 'data/settings-json' },
						{ label: 'Portable mode', slug: 'data/portable-mode' },
						{ label: 'Updating', slug: 'data/updating' },
					],
				},
				{
					label: 'Automation and AI agents',
					items: [
						{ label: 'AI agents: quick start', slug: 'automation/quick-start' },
						{ label: 'Overview', slug: 'automation/overview' },
						{ label: 'MCP setup', slug: 'automation/mcp-setup' },
						{ label: 'MCP tools', slug: 'automation/mcp-tools' },
						{ label: 'Command line', slug: 'automation/command-line' },
						{ label: 'Control verbs (advanced)', slug: 'automation/control-verbs' },
					],
				},
				{
					label: 'Platform notes',
					items: [
						{ label: 'Windows', slug: 'platforms/windows' },
						{ label: 'Linux', slug: 'platforms/linux' },
					],
				},
				{
					label: 'Support',
					items: [
						{ label: 'Troubleshooting and FAQ', slug: 'support/troubleshooting' },
						{ label: 'Glossary', slug: 'support/glossary' },
					],
				},
			],
		}),
	],
});
