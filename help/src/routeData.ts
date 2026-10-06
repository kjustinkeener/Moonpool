// Per-page <head> additions for the web build only (when `site` is set): BreadcrumbList
// JSON-LD on every page, FAQPage JSON-LD on the troubleshooting page, and an absolute
// og:image / twitter:image. The in-app build has no `site`, so it is left untouched.
import { defineRouteMiddleware } from '@astrojs/starlight/route-data';

type SidebarNode = { type: 'link' | 'group'; label: string; href?: string; isCurrent?: boolean; entries?: SidebarNode[] };

const FAQ_ID = 'support/troubleshooting';

function findTrail(nodes: SidebarNode[], trail: SidebarNode[] = []): SidebarNode[] | undefined {
	for (const n of nodes) {
		if (n.type === 'link' && n.isCurrent) return [...trail, n];
		if (n.type === 'group') {
			const hit = findTrail(n.entries ?? [], [...trail, n]);
			if (hit) return hit;
		}
	}
}

function firstHref(n: SidebarNode): string | undefined {
	if (n.type === 'link') return n.href;
	for (const e of n.entries ?? []) {
		const h = firstHref(e);
		if (h) return h;
	}
}

function plain(md: string): string {
	return md
		.replace(/```[\s\S]*?```/g, ' ')
		.replace(/!\[[^\]]*\]\([^)]*\)/g, '')
		.replace(/\[([^\]]+)\]\([^)]*\)/g, '$1')
		.replace(/^\s*\|?[\s:|-]+\|[\s:|-]*$/gm, '')
		.replace(/\|/g, ' ')
		.replace(/^\s{0,3}(#{1,6}|[-*]|\d+\.|>)\s+/gm, '')
		.replace(/[*_`]/g, '')
		.replace(/\s+/g, ' ')
		.trim();
}

function faq(body: string) {
	const out: { '@type': string; name: string; acceptedAnswer: { '@type': string; text: string } }[] = [];
	for (const part of body.split(/^## /m).slice(1)) {
		const nl = part.indexOf('\n');
		const name = plain(part.slice(0, nl));
		const text = plain(part.slice(nl + 1));
		if (name && text) {
			out.push({ '@type': 'Question', name, acceptedAnswer: { '@type': 'Answer', text } });
		}
	}
	return out;
}

export const onRequest = defineRouteMiddleware((context) => {
	const site = context.site;
	if (!site) return;
	const route = context.locals.starlightRoute;
	const abs = (href: string) => new URL(href, site).href;
	const base = import.meta.env.BASE_URL;
	const ld = (data: unknown) =>
		route.head.push({ tag: 'script', attrs: { type: 'application/ld+json' }, content: JSON.stringify(data) });

	// Breadcrumbs: Home > sidebar group (linked to its first page) > this page.
	const items: { name: string; href?: string }[] = [{ name: route.siteTitle, href: base }];
	const trail = findTrail(route.sidebar as SidebarNode[]);
	if (trail && route.id !== '') {
		for (const n of trail) {
			items.push({ name: n.type === 'group' ? n.label : route.entry.data.title, href: firstHref(n) });
		}
	}
	// The home page is its own sidebar entry; do not list it twice.
	const crumbs = route.id === '' || route.id === 'index' ? items.slice(0, 1) : items;
	ld({
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement: crumbs.map((c, i) => ({
			'@type': 'ListItem',
			position: i + 1,
			name: c.name,
			...(c.href && i < crumbs.length - 1 ? { item: abs(c.href) } : {}),
		})),
	});

	if (route.id === FAQ_ID && route.entry.body) {
		ld({ '@context': 'https://schema.org', '@type': 'FAQPage', mainEntity: faq(route.entry.body) });
	}

	const image = abs(`${base}og-image.png`);
	const alt = 'Moonpool: launch and manage your local apps and dev servers';
	route.head.push(
		{ tag: 'meta', attrs: { property: 'og:image', content: image } },
		{ tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
		{ tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
		{ tag: 'meta', attrs: { property: 'og:image:alt', content: alt } },
		{ tag: 'meta', attrs: { name: 'twitter:image', content: image } },
	);
});
