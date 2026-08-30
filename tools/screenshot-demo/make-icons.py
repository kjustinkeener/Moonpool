#!/usr/bin/env python3
"""Generate the Moonpool screenshot-demo icon set.

Writes one flat SVG per sample-app id. Icons render at 16x16 in Moonpool's
sidebar, so each is a bold rounded-square badge (24 viewBox) with a single
white glyph that stays legible when scaled down, tuned for the dark theme.

Usage:  python make-icons.py [OUT_DIR]
Default OUT_DIR is ./icons next to this script.
"""
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE / "icons"
OUT.mkdir(parents=True, exist_ok=True)

# id -> (hue, inner glyph markup). {H} is substituted with the hue.
GLYPHS = {
    # --- Desktop apps ---
    "notes": ("#f2a541", """
      <path d="M6 5.6a1 1 0 011-1h7l4 4v9.8a1 1 0 01-1 1H7a1 1 0 01-1-1z" fill="#fff"/>
      <path d="M14 4.6V8a.6.6 0 00.6.6H18z" fill="rgba(0,0,0,.20)"/>
      <g stroke="{H}" stroke-width="1.4" stroke-linecap="round">
        <line x1="8.4" y1="11" x2="15" y2="11"/>
        <line x1="8.4" y1="13.6" x2="15" y2="13.6"/>
        <line x1="8.4" y1="16.2" x2="12.4" y2="16.2"/>
      </g>"""),
    "pixel-forge": ("#a371f7", """
      <rect x="4.5" y="5.5" width="15" height="13" rx="2.2" fill="#fff"/>
      <circle cx="8.6" cy="9.6" r="1.7" fill="{H}"/>
      <path d="M5.4 17.6l4.2-5 3 3.6 2.6-3.2 3.6 4.6z" fill="{H}"/>"""),
    "db-inspector": ("#2dd4bf", """
      <path d="M6 6.5c0 1.33 2.69 2.4 6 2.4s6-1.07 6-2.4v11c0 1.33-2.69 2.4-6 2.4s-6-1.07-6-2.4z" fill="#fff"/>
      <ellipse cx="12" cy="6.5" rx="6" ry="2.4" fill="#fff" stroke="{H}" stroke-width="1.1"/>
      <path d="M6 11c0 1.2 2.7 2.2 6 2.2s6-1 6-2.2M6 15c0 1.2 2.7 2.2 6 2.2s6-1 6-2.2" stroke="{H}" stroke-width="1.1" fill="none"/>"""),
    # --- Web apps ---
    "metrics": ("#388bfd", """
      <g fill="#fff">
        <rect x="5" y="13" width="3.3" height="6" rx="1"/>
        <rect x="10.35" y="9" width="3.3" height="10" rx="1"/>
        <rect x="15.7" y="5.5" width="3.3" height="13.5" rx="1"/>
      </g>"""),
    "api-gateway": ("#3fb950", """
      <g fill="none" stroke="#fff" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">
        <path d="M10 5.5C8 5.5 8.2 7 8.2 8.6c0 1.5-.2 2.4-1.9 3.4 1.7 1 1.9 1.9 1.9 3.4 0 1.6-.2 3.1 1.8 3.1"/>
        <path d="M14 5.5c2 0 1.8 1.5 1.8 3.1 0 1.5.2 2.4 1.9 3.4-1.7 1-1.9 1.9-1.9 3.4 0 1.6.2 3.1-1.8 3.1"/>
      </g>"""),
    "blog": ("#f78166", """
      <path d="M15.4 5.2l3.4 3.4-8.7 8.7-4 .6.6-4z" fill="#fff"/>
      <path d="M13.8 6.8l3.4 3.4" stroke="{H}" stroke-width="1.3" stroke-linecap="round"/>"""),
    "habit-tracker": ("#ec6cb9", """
      <rect x="5" y="6" width="14" height="13" rx="2.2" fill="#fff"/>
      <path d="M5 8.2a2.2 2.2 0 012.2-2.2h9.6A2.2 2.2 0 0119 8.2v1.3H5z" fill="{H}"/>
      <path d="M8.6 14.6l2.2 2.2 4.6-4.9" stroke="{H}" stroke-width="1.9" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"""),
    # --- Docs ---
    "handbook": ("#6e8bff", """
      <g fill="#fff">
        <path d="M12 7.2C10.3 6 8.2 5.6 6.1 5.9a.9.9 0 00-.8.9v8.9c0 .55.5.96 1.05.86 1.9-.32 3.85.06 5.6 1.25z"/>
        <path d="M12 7.2C13.7 6 15.8 5.6 17.9 5.9a.9.9 0 01.8.9v8.9c0 .55-.5.96-1.05.86-1.9-.32-3.85.06-5.6 1.25z"/>
      </g>
      <line x1="12" y1="7.2" x2="12" y2="18.2" stroke="{H}" stroke-width="1.2"/>"""),
    "design-system": ("#d371e8", """
      <circle cx="8.7" cy="9" r="3.3" fill="#fff"/>
      <rect x="12.6" y="5.8" width="6.2" height="6.2" rx="1.3" fill="#fff" opacity=".9"/>
      <path d="M12 13.4l3.4 5.6H8.6z" fill="#fff" opacity=".78"/>"""),
    # --- CLI tools ---
    "deploy": ("#8b98a8", """
      <path d="M12 3.6c2.6 1.7 4.1 4.7 4.1 7.8 0 1.7-.5 3.2-1.2 4.4H9.1c-.7-1.2-1.2-2.7-1.2-4.4 0-3.1 1.5-6.1 4.1-7.8z" fill="#fff"/>
      <circle cx="12" cy="10" r="1.6" fill="{H}"/>
      <g stroke="#fff" stroke-width="1.5" stroke-linecap="round">
        <path d="M9.4 16.4l-1.6 3"/><path d="M14.6 16.4l1.6 3"/><path d="M12 16.6v3"/>
      </g>"""),
    "backup": ("#56d4dd", """
      <path d="M8.2 17.3a3.6 3.6 0 01-.4-7.16A5 5 0 0117.2 10.6a3.4 3.4 0 01-.7 6.7z" fill="#fff"/>
      <path d="M12 16.4V9.4m0 0l-2.3 2.3M12 9.4l2.3 2.3" stroke="{H}" stroke-width="1.6" fill="none" stroke-linecap="round" stroke-linejoin="round"/>"""),
    "log-tail": ("#9ec25b", """
      <rect x="4.5" y="5.5" width="15" height="13" rx="2.2" fill="#fff"/>
      <path d="M7.6 9.2l2.2 1.9-2.2 1.9" stroke="{H}" stroke-width="1.6" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
      <line x1="11.2" y1="13" x2="16.4" y2="13" stroke="{H}" stroke-width="1.6" stroke-linecap="round"/>
      <line x1="7.6" y1="15.6" x2="16.4" y2="15.6" stroke="{H}" stroke-width="1.6" stroke-linecap="round" opacity=".55"/>"""),
}

TPL = ('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" '
       'width="24" height="24">\n'
       '  <rect width="24" height="24" rx="5.5" fill="{H}"/>\n'
       '  <rect width="24" height="24" rx="5.5" fill="url(#g)"/>\n'
       '  <defs><linearGradient id="g" x1="0" y1="0" x2="0" y2="1">'
       '<stop offset="0" stop-color="#fff" stop-opacity=".18"/>'
       '<stop offset="1" stop-color="#000" stop-opacity=".14"/></linearGradient></defs>'
       '{BODY}\n</svg>\n')

for name, (hue, body) in GLYPHS.items():
    svg = TPL.replace("{H}", hue).replace("{BODY}", body.replace("{H}", hue))
    (OUT / f"{name}.svg").write_text(svg, encoding="utf-8")

print(f"Wrote {len(GLYPHS)} icons to {OUT}")
