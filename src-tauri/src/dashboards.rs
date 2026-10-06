//! Example dashboards: a set of self-contained, offline static HTML dashboards
//! (drop-your-data explorers + a Markdown docs browser) shipped inside the binary.
//!
//! The whole tree under `resources/examples/dashboards/**` is embedded with
//! `include_dir`, so the exe is self-contained (one file, nothing beside it). It is
//! written to `{MP_HOME}/dashboards/examples/**`, which is APP-owned like the help
//! site: the folder is replaced whenever its stamp differs from the running build, so
//! updates reach existing users. `{MP_HOME}/dashboards` itself is the user's space and
//! is never touched. Builds before this wrote the examples straight into
//! `{MP_HOME}/dashboards`; those copies are left alone.

use std::path::Path;

use include_dir::{include_dir, Dir};
use tauri::AppHandle;

/// The embedded dashboards tree (baked into the binary at build time).
static DASHBOARDS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/examples/dashboards");

/// Stamp file inside the examples folder recording which build wrote it.
const STAMP: &str = ".moonpool-version";

/// Write the embedded dashboards to `{MP_HOME}/dashboards/examples`, replacing the
/// folder when its stamp differs from this build. Best-effort: individual write
/// failures are logged, not fatal.
pub fn seed(app: &AppHandle) {
    let Some(home) = crate::portable::mp_home() else {
        crate::log_line(app, "dashboards: no MP_HOME; skipping seed");
        return;
    };
    let dest = home.join("dashboards").join("examples");
    let version = env!("CARGO_PKG_VERSION");
    let current = std::fs::read_to_string(dest.join(STAMP)).unwrap_or_default();
    if current.trim() == version && dest.join("README.md").exists() {
        return; // this build's examples are already in place
    }
    // Clear the old tree first so files removed from the examples don't linger.
    let _ = std::fs::remove_dir_all(&dest);
    let mut written = 0usize;
    write_dir(app, &DASHBOARDS, &dest, &mut written);
    if let Err(e) = std::fs::write(dest.join(STAMP), version) {
        crate::log_line(app, &format!("dashboards: write stamp failed: {e}"));
    }
    crate::log_line(
        app,
        &format!("dashboards: wrote {written} file(s) to {}", dest.display()),
    );
}

/// Recursively write one embedded directory to `dest`.
fn write_dir(app: &AppHandle, dir: &Dir<'_>, dest: &Path, written: &mut usize) {
    if let Err(e) = std::fs::create_dir_all(dest) {
        crate::log_line(
            app,
            &format!("dashboards: mkdir {} failed: {e}", dest.display()),
        );
        return;
    }
    for file in dir.files() {
        let Some(name) = file.path().file_name() else {
            continue;
        };
        let out = dest.join(name);
        match std::fs::write(&out, file.contents()) {
            Ok(_) => *written += 1,
            Err(e) => crate::log_line(
                app,
                &format!("dashboards: write {} failed: {e}", out.display()),
            ),
        }
    }
    for sub in dir.dirs() {
        let Some(name) = sub.path().file_name() else {
            continue;
        };
        write_dir(app, sub, &dest.join(name), written);
    }
}
