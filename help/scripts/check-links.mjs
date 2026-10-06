// Link check over a built help site: every internal href/src must resolve to a file in the dist,
// and every #fragment must match an id in the target page.
// Usage: node scripts/check-links.mjs [dist] [base]   (defaults: dist /  ;  web: dist-web /software/moonpool/help/)
import fs from 'node:fs';
import path from 'node:path';

const dist = path.resolve(process.argv[2] || 'dist');
const base = (process.argv[3] || '/').replace(/\/?$/, '/');
const htmlFiles = [];
(function walk(d) {
	for (const e of fs.readdirSync(d, { withFileTypes: true })) {
		const p = path.join(d, e.name);
		if (e.isDirectory()) walk(p);
		else if (e.name.endsWith('.html')) htmlFiles.push(p);
	}
})(dist);

const idCache = new Map();
const idsOf = (file) => {
	if (!idCache.has(file)) {
		const html = fs.readFileSync(file, 'utf8');
		idCache.set(file, new Set([...html.matchAll(/\sid="([^"]*)"/g)].map((m) => m[1])));
	}
	return idCache.get(file);
};

const resolveFile = (urlPath) => {
	const rel = decodeURIComponent(urlPath.slice(base.length));
	const p = path.join(dist, rel);
	if (fs.existsSync(p) && fs.statSync(p).isFile()) return p;
	const idx = path.join(p, 'index.html');
	return fs.existsSync(idx) ? idx : null;
};

let checked = 0;
const errors = [];
for (const file of htmlFiles) {
	const html = fs.readFileSync(file, 'utf8');
	const rel = '/' + path.relative(dist, file).split(path.sep).join('/');
	const pageUrl = base + rel.slice(1).replace(/index\.html$/, '');
	const refs = [];
	for (const m of html.matchAll(/\s(?:href|src)="([^"]*)"/g)) refs.push(m[1]);
	for (const m of html.matchAll(/\ssrcset="([^"]*)"/g)) for (const part of m[1].split(',')) refs.push(part.trim().split(/\s+/)[0]);
	for (const ref of refs) {
		if (!ref || /^(https?:|mailto:|data:|javascript:|\/\/)/.test(ref)) continue;
		const u = new URL(ref.replace(/&amp;/g, '&'), 'http://x' + pageUrl);
		checked++;
		if (!u.pathname.startsWith(base)) { errors.push(`${rel}: ${ref} -> outside base ${base}`); continue; }
		const target = resolveFile(u.pathname);
		if (!target) { errors.push(`${rel}: ${ref} -> missing`); continue; }
		if (u.hash.length > 1 && target.endsWith('.html')) {
			const id = decodeURIComponent(u.hash.slice(1));
			if (!idsOf(target).has(id)) errors.push(`${rel}: ${ref} -> no #${id}`);
		}
	}
}
console.log(`${htmlFiles.length} pages, ${checked} internal references checked, ${errors.length} problems`);
for (const e of errors) console.log('  ' + e);
process.exit(errors.length ? 1 : 0);
