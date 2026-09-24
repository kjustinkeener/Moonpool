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
	// `site` is used for canonical URLs / sitemap on the web build; harmless in-app.
	site: 'https://moonpool.app',
	// Keep the help root useful without exposing a separate Home page.
	redirects: {
		'/': '/getting-started/overview/',
	},
	integrations: [
		starlight({
			title: 'Moonpool Help',
			// No external social links in bundled app help.
			social: [],
			// Two-level navigation: category label -> pages.
			sidebar: [
				{
					label: 'Getting Started',
					items: [
						{ label: 'What is Moonpool', slug: 'getting-started/overview' },
						{ label: 'Installing', slug: 'getting-started/installing' },
						{ label: 'First Launch', slug: 'getting-started/first-launch' },
					],
				},
				{
					label: 'Using Moonpool',
					items: [
						{ label: 'Adding Apps', slug: 'guides/adding-apps' },
						{ label: 'Portable Mode', slug: 'guides/portable-mode' },
						{ label: 'Updating', slug: 'guides/updating' },
					],
				},
			],
		}),
	],
});
