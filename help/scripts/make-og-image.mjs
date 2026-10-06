// Regenerates public/og-image.png (1200x630 social card) from the app icon.
// Run manually when the icon changes: node scripts/make-og-image.mjs
import sharp from 'sharp';
const icon = await sharp('../src-tauri/icons/icon.png').resize(340, 340).toBuffer();
const text = Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="660" height="340">
<text x="0" y="150" font-family="Segoe UI, Arial, sans-serif" font-size="96" font-weight="700" fill="#ffffff">Moonpool</text>
<text x="0" y="214" font-family="Segoe UI, Arial, sans-serif" font-size="38" fill="#a9b8d4">Launch and manage your local</text>
<text x="0" y="262" font-family="Segoe UI, Arial, sans-serif" font-size="38" fill="#a9b8d4">apps and dev servers</text>
</svg>`);
await sharp({ create: { width: 1200, height: 630, channels: 4, background: '#0f1729' } })
	.composite([{ input: icon, left: 120, top: 145 }, { input: text, left: 500, top: 145 }])
	.png({ compressionLevel: 9 })
	.toFile('public/og-image.png');
