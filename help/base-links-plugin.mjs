// Prefix root-absolute links ("/guides/updating/") with the site base.
//
// Astro rewrites its own asset URLs and Starlight's chrome for `base`, but a
// root-absolute href written in Markdown is emitted verbatim. This is a Satteri hast
// plugin used only when a base is set (the web build); with base '/' it is not
// registered, so the in-app build is byte-for-byte unchanged.
const ATTRS = { a: 'href', img: 'src', source: 'src' };

/** @param {string} base */
export default function baseLinksPlugin(base) {
	const prefix = base.replace(/\/+$/, '');
	return {
		name: 'base-links',
		element: {
			filter: ['a', 'img', 'source'],
			visit(node, ctx) {
				const attr = ATTRS[node.tagName];
				const value = node.properties?.[attr];
				if (typeof value !== 'string') return;
				// Root-absolute only: not protocol-relative ('//host'), not already prefixed.
				if (!value.startsWith('/') || value.startsWith('//')) return;
				if (value === prefix || value.startsWith(prefix + '/')) return;
				ctx.setProperty(node, attr, prefix + value);
			},
		},
	};
}
