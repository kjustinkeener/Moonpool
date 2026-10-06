// Rewrite #fragment links in a translated locale so they match the translated headings.
//
// Starlight derives a heading's id from its text, so translating a heading changes its id and
// breaks `[x](/de/apps/fields/#mcpprocessname)`-style links. Translators keep the ENGLISH
// fragment in the link; this script maps each English id to the id of the heading at the same
// position in the translated page (headings must correspond one to one) and rewrites the link.
// Run it after translating, then run `npm run check:links` on both builds.
//
// Usage: node scripts/fix-fragments.mjs <dir>        (for example: de, pt-br, zh-hans)
import fs from 'node:fs';
import path from 'node:path';
import GithubSlugger from 'github-slugger';

const dir = process.argv[2];
if (!dir) {
	console.error('usage: node scripts/fix-fragments.mjs <locale-dir>');
	process.exit(2);
}
const docs = path.resolve('src/content/docs');
const locRoot = path.join(docs, dir);

const walk = (d) =>
	fs.readdirSync(d, { withFileTypes: true }).flatMap((e) => {
		const p = path.join(d, e.name);
		return e.isDirectory() ? walk(p) : e.name.endsWith('.md') ? [p] : [];
	});

// Plain text of a heading as it renders (what Starlight slugs).
const plain = (s) =>
	s
		.replace(/!\[[^\]]*\]\([^)]*\)/g, '')
		.replace(/\[([^\]]*)\]\([^)]*\)/g, '$1')
		.replace(/[`*_]/g, '')
		.trim();

function headings(file) {
	const out = [];
	let fence = false;
	for (const line of fs.readFileSync(file, 'utf8').split(/\r?\n/)) {
		if (/^\s*(```|~~~)/.test(line)) fence = !fence;
		if (fence) continue;
		const m = /^(#{2,6})\s+(.*?)\s*#*\s*$/.exec(line);
		if (m) out.push(plain(m[2]));
	}
	const slugger = new GithubSlugger();
	return out.map((t) => slugger.slug(t));
}

// page key ('apps/fields') -> { en: id[], loc: id[] }
const maps = new Map();
let problems = 0;
for (const file of walk(locRoot)) {
	const key = path.relative(locRoot, file).replace(/\\/g, '/').replace(/\.md$/, '');
	const enFile = path.join(docs, key + '.md');
	if (!fs.existsSync(enFile)) {
		console.log(`no English page for ${dir}/${key}.md`);
		problems++;
		continue;
	}
	const en = headings(enFile);
	const loc = headings(file);
	if (en.length !== loc.length) {
		console.log(`${dir}/${key}.md: ${loc.length} headings, English has ${en.length}`);
		problems++;
		continue;
	}
	maps.set(key, new Map(en.map((id, i) => [id, loc[i]])));
}

let changed = 0;
for (const file of walk(locRoot)) {
	const own = path.relative(locRoot, file).replace(/\\/g, '/').replace(/\.md$/, '');
	const text = fs.readFileSync(file, 'utf8');
	const out = text.replace(/\]\((\/[^)#\s]*)?#([^)\s]+)\)/g, (m, p, frag) => {
		let key;
		if (p === undefined) key = own;
		else {
			const rel = p.replace(/^\/+|\/+$/g, '');
			if (!rel.startsWith(dir + '/') && rel !== dir) return m; // not a link into this locale
			key = rel.slice(dir.length + 1).replace(/\/+$/, '') || 'index';
		}
		const map = maps.get(key);
		const next = map?.get(frag);
		if (!next || next === frag) return m;
		changed++;
		return `](${p ?? ''}#${next})`;
	});
	if (out !== text) fs.writeFileSync(file, out);
}
console.log(`${changed} fragment link(s) rewritten, ${problems} structural problem(s)`);
process.exit(problems ? 1 : 0);
