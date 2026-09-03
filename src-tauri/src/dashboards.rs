//! Example dashboards: a set of self-contained, offline static HTML dashboards
//! (drop-your-data explorers + a Markdown docs browser) shipped inside the binary.
//!
//! The whole tree under `resources/examples/dashboards/**` is embedded with
//! `include_dir`, so the exe is self-contained (one file, nothing beside it). On
//! first run we write the tree out to `{MP_HOME}/dashboards/**`, skipping any file
//! that already exists so a user's own edits are never clobbered. `{MP_HOME}` is the
//! bundle folder in portable mode and the install dir otherwise, which is exactly
//! what the `file:///{MP_HOME}/dashboards/...` manifest entries resolve to.

use std::path::Path;

use include_dir::{include_dir, Dir};
use tauri::AppHandle;

/// The embedded dashboards tree (baked into the binary at build time).
static DASHBOARDS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/examples/dashboards");

/// Write the embedded dashboards to `{MP_HOME}/dashboards`, creating folders as
/// needed and skipping any file that already exists (so user edits survive an
/// update). Best-effort: individual write failures are logged, not fatal.
pub fn seed(app: &AppHandle) {
    let Some(home) = crate::portable::mp_home() else {
        crate::log_line(app, "dashboards: no MP_HOME; skipping seed");
        return;
    };
    let dest = home.join("dashboards");
    let mut written = 0usize;
    write_dir(app, &DASHBOARDS, &dest, &mut written);
    if written > 0 {
        crate::log_line(
            app,
            &format!("dashboards: wrote {written} file(s) to {}", dest.display()),
        );
    }
}

/// Recursively write one embedded directory to `dest`, skipping existing files.
fn write_dir(app: &AppHandle, dir: &Dir<'_>, dest: &Path, written: &mut usize) {
    if let Err(e) = std::fs::create_dir_all(dest) {
        crate::log_line(
            app,
            &format!("dashboards: mkdir {} failed: {e}", dest.display()),
        );
        return;
    }
    for file in dir.files() {
        let name = match file.path().file_name() {
            Some(n) => n,
            None => continue,
        };
        let out = dest.join(name);
        if out.exists() {
            continue; // never clobber a file that's already there
        }
        match std::fs::write(&out, file.contents()) {
            Ok(_) => *written += 1,
            Err(e) => crate::log_line(
                app,
                &format!("dashboards: write {} failed: {e}", out.display()),
            ),
        }
    }
    for sub in dir.dirs() {
        let name = match sub.path().file_name() {
            Some(n) => n,
            None => continue,
        };
        write_dir(app, sub, &dest.join(name), written);
    }
}
