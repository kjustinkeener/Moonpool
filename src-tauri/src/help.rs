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

use include_dir::{include_dir, Dir};
use tauri::http::{Request, Response};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// The built Starlight site, baked into the binary at build time.
static HELP: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../help/dist");

/// Version of the help content bundled in this build. Written to `help/version.txt`
/// on first seed and compared against the updater manifest's `help.version`.
/// Bump when the bundled baseline help content changes.
pub const HELP_BASELINE_VERSION: &str = "2026.09.22";

/// The first real help document. Avoid the site's root redirect here: WebView2 can
/// navigate a custom protocol's initial document but fail its immediate meta-refresh,
/// which leaves a blank window. Wry translates this `help://` URL to
/// `http://help.localhost/` on Windows and routes it back to our handler.
const HELP_ROOT_URL: &str = "help://localhost/getting-started/overview/";

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
            Response::builder()
                .status(200)
                .header("Content-Type", content_type(&target))
                .body(Cow::from(bytes))
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

/// Open (or focus) the help window. It loads the embedded docs over the `help` URI
/// scheme, so it works fully offline.
#[tauri::command]
pub fn open_help(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("help") {
        let _ = win.unminimize();
        let _ = win.show();
        let _ = win.set_focus();
        return Ok(());
    }
    let url = HELP_ROOT_URL
        .parse()
        .map_err(|e| format!("bad help url: {e}"))?;
    // Keep help in the same z-band as the hub when always-on-top is set, so it does
    // not sink behind the hub (see the always-on-top z-band trap in settings).
    let on_top = app
        .try_state::<crate::HubState>()
        .map(|s| crate::lock(&s.settings).always_on_top)
        .unwrap_or(false);
    // Native OS decorations (a real titlebar with a working X). The hub's own windows
    // are borderless and draw a Svelte titlebar, but the help window shows external
    // Starlight HTML that has no such control, so without a native frame there is no
    // way to close it. The close-to-tray / minimize-to-tray handlers in `lib.rs` only
    // act on the "main" window, so this X closes and destroys the help window normally.
    let mut builder = WebviewWindowBuilder::new(&app, "help", WebviewUrl::CustomProtocol(url))
        .title("Moonpool Help")
        .inner_size(1000.0, 720.0)
        .resizable(true);

    // The hub is deliberately borderless and transparent. Help is a normal document
    // window, so make it opaque on platforms that expose this builder option. macOS
    // does not expose `transparent` here and already creates an opaque native window.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        builder = builder.transparent(false);
    }

    builder
        // The hub is deliberately borderless and transparent. Help is a normal
        // document window, so make its native frame explicit.
        .decorations(true)
        .always_on_top(on_top)
        .build()
        .map_err(|e| format!("build help window: {e}"))?;
    Ok(())
}
