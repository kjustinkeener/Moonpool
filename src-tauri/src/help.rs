//! Offline help site: the built Starlight docs, embedded in the binary and served
//! from `{MP_HOME}/help` over a custom `help` URI scheme so the exe stays
//! self-contained (one file, nothing beside it) and the docs work with no network.
//!
//! Unlike the example dashboards (`dashboards.rs`, user-owned, never clobbered), the
//! help site is APP-owned content the updater replaces (`update::help_apply`). So the
//! seed is version-gated: on first run we write the embedded baseline and stamp a
//! `version.txt`; if that stamp is already present we leave the directory alone (a
//! newer downloaded bundle, or the current baseline, is already in place).

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use base64::Engine as _;
use include_dir::{include_dir, Dir};
use tauri::http::{Request, Response};
use tauri::AppHandle;

/// The built Starlight site, baked into the binary at build time.
static HELP: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../help/dist");

/// Brand assets for the injected header bar, the same files `Titlebar.svelte` and
/// `SettingsControls.svelte` use for the main window's icon+wordmark treatment.
static BRAND_ICON: &[u8] = include_bytes!("../../src/assets/app-icon.png");
static WORDMARK: &[u8] = include_bytes!("../../src/assets/moonpool-wordmark-text.png");

/// Version of the help content bundled in this build. Written to `help/version.txt`
/// on first seed and compared against the updater manifest's `help.version`.
/// Bump when the bundled baseline help content changes.
pub const HELP_BASELINE_VERSION: &str = "2026.09.22";

/// Write the embedded baseline help to `{MP_HOME}/help` on first run, then stamp
/// `version.txt`. If `version.txt` already exists we do nothing: either the current
/// baseline or a newer downloaded bundle is already present, and either is correct.
/// Best-effort: individual write failures are logged, not fatal.
pub fn seed(app: &AppHandle) {
    let Some(home) = crate::portable::mp_home() else {
        crate::log_line(app, "help: no MP_HOME; skipping seed");
        return;
    };
    let dest = home.join("help");
    let version_file = dest.join("version.txt");
    // Version-gated, but also guard against a stamped-but-empty tree: an earlier
    // build whose `help/dist` had not been built yet could write `version.txt` over
    // an empty directory, and the gate would then skip forever, leaving the help
    // window blank. Preserve a matching or newer downloaded bundle, but replace an
    // older embedded baseline when the app ships revised bundled help.
    let index_file = dest.join("index.html");
    if index_file.exists()
        && std::fs::read_to_string(&version_file)
            .map(|version| version.trim() >= HELP_BASELINE_VERSION)
            .unwrap_or(false)
    {
        return; // matching baseline or a newer downloaded bundle is already staged
    }
    if version_file.exists() {
        crate::log_line(
            app,
            "help: version.txt present but index.html missing; re-seeding baseline",
        );
    }
    let mut written = 0usize;
    write_dir(app, &HELP, &dest, &mut written);
    if let Err(e) = std::fs::write(&version_file, HELP_BASELINE_VERSION) {
        crate::log_line(app, &format!("help: write version.txt failed: {e}"));
    }
    crate::log_line(
        app,
        &format!("help: seeded {written} file(s) to {}", dest.display()),
    );
}

/// Recursively write one embedded directory to `dest`. Unlike `dashboards::write_dir`
/// this overwrites: the help tree is app-owned, so a stale/partial copy left without a
/// `version.txt` should be replaced by the current baseline, not preserved.
fn write_dir(app: &AppHandle, dir: &Dir<'_>, dest: &Path, written: &mut usize) {
    if let Err(e) = std::fs::create_dir_all(dest) {
        crate::log_line(app, &format!("help: mkdir {} failed: {e}", dest.display()));
        return;
    }
    for file in dir.files() {
        let Some(name) = file.path().file_name() else {
            continue;
        };
        let out = dest.join(name);
        match std::fs::write(&out, file.contents()) {
            Ok(_) => *written += 1,
            Err(e) => crate::log_line(app, &format!("help: write {} failed: {e}", out.display())),
        }
    }
    for sub in dir.dirs() {
        let Some(name) = sub.path().file_name() else {
            continue;
        };
        write_dir(app, sub, &dest.join(name), written);
    }
}

/// The on-disk help root, `{MP_HOME}/help`.
fn help_root() -> Option<PathBuf> {
    crate::portable::mp_home().map(|h| h.join("help"))
}

/// Map a request path to a file under the help root, appending `index.html` for a
/// directory route (path ending in `/` or with no file extension - Starlight emits
/// `<route>/index.html`). Returns `None` for a path that escapes the help dir.
fn resolve(path: &str) -> Option<PathBuf> {
    let root = help_root()?;
    let decoded = percent_decode(path);
    let mut rel = PathBuf::new();
    for seg in decoded.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            return None; // reject any traversal outright
        }
        rel.push(seg);
    }
    // Root `/`, a trailing-slash route, or an extensionless route -> its index.html.
    let needs_index = rel.as_os_str().is_empty() || rel.extension().is_none();
    let target = if needs_index {
        root.join(&rel).join("index.html")
    } else {
        root.join(&rel)
    };
    // Belt-and-braces: refuse anything that resolved outside the help root.
    if !target.starts_with(&root) {
        return None;
    }
    Some(target)
}

/// Content-Type for a static asset, inferred from its extension.
fn content_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("woff2") => "font/woff2",
        Some("xml") => "application/xml",
        Some("ico") => "image/x-icon",
        Some("map") => "application/json",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// Serve one `help://` request from the on-disk help tree. 404 for a missing or
/// out-of-bounds path. Logs the request path, the resolved file, and hit/miss via
/// `crate::log_line` (a no-op unless Debug logging is on) so a blank help window is
/// diagnosable from the log.
pub fn handle_request(app: &AppHandle, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let path = request.uri().path().to_string();
    let not_found = || {
        Response::builder()
            .status(404)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(Cow::from(b"not found".as_slice()))
            .expect("static 404 response is valid")
    };
    // Virtual asset for the injected custom titlebar (not a file on disk - see
    // `inject_titlebar`/`TITLEBAR_SCRIPT`).
    if path == "/_mp-titlebar.js" {
        return Response::builder()
            .status(200)
            .header("Content-Type", "text/javascript; charset=utf-8")
            .body(Cow::from(TITLEBAR_SCRIPT.as_bytes()))
            .expect("titlebar script response is valid");
    }
    let Some(target) = resolve(&path) else {
        crate::log_line(app, &format!("help: {path} -> rejected (out of bounds)"));
        return not_found();
    };
    match std::fs::read(&target) {
        Ok(bytes) => {
            crate::log_line(
                app,
                &format!(
                    "help: {path} -> {} ({} bytes)",
                    target.display(),
                    bytes.len()
                ),
            );
            let is_html = content_type(&target) == "text/html; charset=utf-8";
            let body = if is_html {
                inject_titlebar(bytes)
            } else {
                bytes
            };
            Response::builder()
                .status(200)
                .header("Content-Type", content_type(&target))
                .body(Cow::from(body))
                .expect("help response is valid")
        }
        Err(e) => {
            crate::log_line(
                app,
                &format!("help: {path} -> MISS {}: {e}", target.display()),
            );
            not_found()
        }
    }
}

/// Data-URI'd brand icon + wordmark, computed once and reused across requests.
fn brand_data_uris() -> &'static (String, String) {
    static URIS: OnceLock<(String, String)> = OnceLock::new();
    URIS.get_or_init(|| {
        let b64 = base64::engine::general_purpose::STANDARD;
        (
            format!("data:image/png;base64,{}", b64.encode(BRAND_ICON)),
            format!("data:image/png;base64,{}", b64.encode(WORDMARK)),
        )
    })
}

/// Same-origin script for the injected custom titlebar (served from `/_mp-titlebar.js`,
/// not from disk - see the intercept in `handle_request`). Runs with `withGlobalTauri`
/// enabled in `tauri.conf.json`, so `window.__TAURI__` is available regardless of the
/// page's own origin. Wired as an external file rather than an inline `<script>` tag
/// because CSP's `script-src 'self'` blocks inline script; a same-origin file passes.
const TITLEBAR_SCRIPT: &str = r#"(function () {
  function wire() {
    if (!window.__TAURI__) return;
    var win = window.__TAURI__.window.getCurrentWindow();
    var closeBtn = document.getElementById("mp-tb-close");
    var minBtn = document.getElementById("mp-tb-min");
    var maxBtn = document.getElementById("mp-tb-max");
    if (closeBtn) closeBtn.addEventListener("click", function () { win.close(); });
    if (minBtn) minBtn.addEventListener("click", function () { win.minimize(); });
    if (maxBtn) {
      maxBtn.addEventListener("click", function () { win.toggleMaximize(); });
      var sync = function () {
        win.isMaximized().then(function (m) {
          maxBtn.textContent = m ? "❒" : "□";
        });
      };
      sync();
      win.onResized(sync);
    }
  }
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", wire);
  } else {
    wire();
  }
})();
"#;

/// Insert a real custom titlebar (drag region + minimize/maximize/close, wired via
/// `TITLEBAR_SCRIPT`) matching the main/Settings windows' icon+wordmark treatment,
/// right after the opening `<body>` tag. The help window loads with
/// `decorations:false` (see `openHelpWindow` in `api.ts`) so this bar is the ONLY
/// window chrome - there's no native titlebar underneath it.
///
/// Starlight's own layout parameterizes header/content height entirely on the
/// `--sl-nav-height` CSS custom property (used for the sticky nav bar's height,
/// the sidebar's top padding, the main content's top padding, and scroll-padding),
/// so bumping that variable - rather than hacking `body` padding directly, which an
/// earlier attempt tried and broke the sidebar header's responsive layout - pushes
/// every dependent measurement down together correctly.
fn inject_titlebar(html: Vec<u8>) -> Vec<u8> {
    const BAR_HEIGHT_PX: u32 = 32;
    let (icon, wordmark) = brand_data_uris();
    let text = String::from_utf8_lossy(&html);
    let Some(body_tag_start) = text.to_ascii_lowercase().find("<body") else {
        return html;
    };
    let Some(tag_end_offset) = text[body_tag_start..].find('>') else {
        return html;
    };
    let insert_at = body_tag_start + tag_end_offset + 1;
    let bar = format!(
        r#"<div id="mp-titlebar" data-tauri-drag-region>
  <span class="mp-tb-brand" data-tauri-drag-region>
    <img src="{icon}" alt="" />
    <img src="{wordmark}" alt="moonpool" />
    <span class="mp-tb-label">Help</span>
  </span>
  <div class="mp-tb-controls">
    <button id="mp-tb-min" title="Minimize" aria-label="Minimize">&#x2212;</button>
    <button id="mp-tb-max" title="Maximize" aria-label="Maximize">&#x25a1;</button>
    <button id="mp-tb-close" title="Close" aria-label="Close">&#x2715;</button>
  </div>
</div>
<script src="/_mp-titlebar.js"></script>
<style>
:root {{
  --sl-nav-height: calc(3.5rem + {BAR_HEIGHT_PX}px) !important;
  --mp-orig-nav-height: calc(var(--sl-nav-height) - {BAR_HEIGHT_PX}px);
}}
@media (width >= 50em) {{
  :root {{
    --sl-nav-height: calc(4rem + {BAR_HEIGHT_PX}px) !important;
    --mp-orig-nav-height: calc(var(--sl-nav-height) - {BAR_HEIGHT_PX}px);
  }}
}}
html, body {{ height: 100%; overflow: hidden !important; }}
/* Starlight's own header sizes its title image off --sl-nav-height directly; since we
   inflate that variable so the sidebar/toc/content all get pushed down below our bar,
   the header's own box (still anchored at top:0) grew to include the space our bar now
   covers, dragging its centered content up under the bar instead of centering it in the
   visible strip. Give the header back its original (uninflated) height and start it right
   below our bar instead, so its own internal centering (which we don't touch) is correct
   again. */
header.header {{ top: {BAR_HEIGHT_PX}px !important; height: var(--mp-orig-nav-height) !important; }}
.site-title img {{ height: calc(var(--mp-orig-nav-height) - 2 * var(--sl-nav-pad-y)) !important; }}
#mp-titlebar {{
  position: fixed; top: 0; left: 0; right: 0; z-index: 999999;
  display: flex; align-items: center; height: {BAR_HEIGHT_PX}px; padding-left: 10px;
  background: #14181c; border-bottom: 1px solid rgba(255,255,255,0.08);
  user-select: none; -webkit-user-select: none;
}}
.mp-tb-brand {{ display: inline-flex; align-items: center; gap: 6px; }}
.mp-tb-brand img:first-child {{
  height: 16px; width: 16px; pointer-events: none;
  filter: drop-shadow(0 0 4px rgba(97,252,237,0.455)) drop-shadow(0 0 8px rgba(97,252,237,0.28));
}}
.mp-tb-brand img:last-of-type {{
  height: 15px; width: auto; pointer-events: none;
  filter: drop-shadow(1px 1px 1px rgba(0,0,0,0.55));
}}
.mp-tb-label {{
  display: inline-flex; align-items: center; align-self: stretch;
  color: #9aa4ac; font-size: 13px; line-height: 1; font-family: inherit; pointer-events: none;
  border-left: 1px solid rgba(255,255,255,0.14); padding-left: 8px; margin-left: 2px;
}}
.mp-tb-controls {{ margin-left: auto; display: flex; height: 100%; padding-right: 4px; }}
.mp-tb-controls button {{
  width: 30px; height: 100%; display: grid; place-items: center; border: none;
  background: transparent; color: #d7dee3; cursor: pointer; font-size: 13px;
  font-family: inherit; padding: 0;
}}
.mp-tb-controls button:hover {{ background: rgba(255,255,255,0.12); }}
#mp-tb-close:hover {{ background: #e81123; color: #fff; }}
#mp-help-scroll {{
  position: absolute; top: {BAR_HEIGHT_PX}px; left: 0; right: 0; bottom: 0;
  overflow-y: auto; overflow-x: hidden;
}}
</style>
<div id="mp-help-scroll">"#
    );
    let body_close_at = text
        .to_ascii_lowercase()
        .rfind("</body>")
        .unwrap_or(text.len());
    let mut out = String::with_capacity(text.len() + bar.len() + 32);
    out.push_str(&text[..insert_at]);
    out.push_str(&bar);
    out.push_str(&text[insert_at..body_close_at]);
    out.push_str("</div>");
    out.push_str(&text[body_close_at..]);
    out.into_bytes()
}

/// Decode `%XX` escapes to bytes (so multi-byte UTF-8 escapes round-trip); a
/// stray/invalid `%` is left as-is. Mirrors `lib.rs::percent_decode`.
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
