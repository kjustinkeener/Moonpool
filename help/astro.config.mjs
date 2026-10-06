// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// Moonpool help site.
//
// One static build serves two targets:
//   - in-app (offline): the built `dist/` is bundled with Moonpool and served to the
//     help window through a Tauri custom protocol at the root path, so absolute asset
//     URLs (`/_astro/...`) resolve. Do NOT open it via file://; absolute paths break.
//   - web: the same `dist/` is deployed to the product page.
//
// `base` stays '/' so both targets agree; host the web copy at a root-mapped path
// (subdomain or rewrite) rather than a subdirectory.
//
// Sidebar convention (shared across all apps): each top-level entry is a category
// GROUP whose `label` is a non-navigable heading; only the `items` (pages) are links.
export default defineConfig({
	// No `site` until the help has a real public address: it would only feed canonical
	// URLs and the sitemap with a domain we do not own. Add it when the web help ships.
	// Keep the help root useful without exposing a separate Home page.
	redirects: {
		'/': '/getting-started/overview/',
	},
	integrations: [
		starlight({
			title: 'Moonpool Help',
			customCss: ['./src/styles/screenshots.css'],
			// No external social links in bundled app help.
			social: [],
			// Two-level navigation: category label -> pages.
			// Pages are grouped by task, not by folder: slugs stay where they were so links
			// from the app and between pages keep working.
			sidebar: [
				{
					label: 'Getting started',
					items: [
						{ label: 'What is Moonpool', slug: 'getting-started/overview' },
						{ label: 'Installing', slug: 'getting-started/installing' },
						{ label: 'Your first app', slug: 'getting-started/first-launch' },
						{ label: 'Example dashboards', slug: 'using/example-dashboards' },
						{ label: "What's new", slug: 'getting-started/whats-new' },
					],
				},
				{
					label: 'Using Moonpool',
					items: [
						{ label: 'The hub window', slug: 'using/hub-window' },
						{ label: 'Sidebar and menus', slug: 'using/sidebar-and-menus' },
						{ label: 'Terminal tabs', slug: 'using/terminal' },
						{ label: 'Tray, close and minimize', slug: 'using/tray-and-closing' },
						{ label: 'Shortcuts and zoom', slug: 'using/shortcuts-and-zoom' },
						{ label: 'Settings window', slug: 'using/settings-window' },
						{ label: 'Themes, language and transparency', slug: 'using/appearance' },
					],
				},
				{
					label: 'Configuring apps',
					items: [
						{ label: 'Overview', slug: 'configuration/overview' },
						{ label: 'Adding apps', slug: 'guides/adding-apps' },
						{ label: 'Examples', slug: 'configuration/examples' },
						{ label: 'App types', slug: 'configuration/app-types' },
						{ label: 'App fields', slug: 'configuration/fields' },
						{ label: 'Paths and environment', slug: 'configuration/paths-and-environment' },
						{ label: 'Stop and restart', slug: 'configuration/stop-and-restart' },
					],
				},
				{
					label: 'Data, logs and recovery',
					items: [
						{ label: 'Logs', slug: 'configuration/logs' },
						{ label: 'Backup and recovery', slug: 'configuration/backup-and-recovery' },
						{ label: 'settings.json', slug: 'configuration/settings-and-logs' },
						{ label: 'Portable mode', slug: 'guides/portable-mode' },
						{ label: 'Updating', slug: 'guides/updating' },
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
					label: 'Help',
					items: [
						{ label: 'Troubleshooting and FAQ', slug: 'reference/troubleshooting' },
						{ label: 'Glossary', slug: 'reference/glossary' },
					],
				},
			],
		}),
	],
});
