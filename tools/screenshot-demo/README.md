# Screenshot demo kit

A clean, fictional set of apps + a matching icon set for taking presentable
Moonpool screenshots (README hero, store listing, etc.) without exposing your
real machine's paths.

## Contents

- `apps.sample.json` - 12 demo apps across all four groups (Desktop / Web / Docs / CLI).
- `icons/*.svg` - one flat badge icon per app id. Rendered at 16px in the sidebar.
- `make-icons.py` - regenerates the icon set (`python make-icons.py [OUT_DIR]`).
- `metrics-site/` - a real static dashboard the "Metrics Dashboard" app serves on
  `:5173`, so the embedded terminal shows genuine output and the status dot goes green.
- `icons-preview.html` - contact sheet of every icon at 16/24/40px.
- `apply.ps1` / `restore.ps1` - swap the demo in and your real config back out.

## Use it

```powershell
pwsh -NoProfile -File tools\screenshot-demo\apply.ps1   # backs up real apps.json, installs demo + icons
```

Then in Moonpool: **Reload** (⋯ menu / top bar). Launch **Metrics Dashboard** to get a
live terminal + green dot on the right for the hero shot. The other apps point at
`C:\projects\...` placeholders, so they sit "stopped" - which realistically shows the
status feature.

When done:

```powershell
pwsh -NoProfile -File tools\screenshot-demo\restore.ps1  # restores your real apps.json
```

Icons are resolved via `%APPDATA%\Moonpool\icons\<id>.svg` (Moonpool's resolution
step 2), so no `"icon"` field is needed in the manifest.
