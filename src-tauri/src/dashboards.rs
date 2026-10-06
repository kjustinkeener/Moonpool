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
    let log = |m: &str| crate::log_line(app, m);
    seed_into(
        &home.join("dashboards").join("examples"),
        env!("CARGO_PKG_VERSION"),
        &log,
    );
}

/// The seed itself, against any `dest` (tests use a temp folder). Returns how many files were
/// written, or `None` when `dest` already holds this `version`'s examples.
fn seed_into(dest: &Path, version: &str, log: &dyn Fn(&str)) -> Option<usize> {
    let current = std::fs::read_to_string(dest.join(STAMP)).unwrap_or_default();
    if current.trim() == version && dest.join("README.md").exists() {
        return None; // this build's examples are already in place
    }
    // Clear the old tree first so files removed from the examples don't linger.
    let _ = std::fs::remove_dir_all(dest);
    let mut written = 0usize;
    write_dir(log, &DASHBOARDS, dest, &mut written);
    if let Err(e) = std::fs::write(dest.join(STAMP), version) {
        log(&format!("dashboards: write stamp failed: {e}"));
    }
    log(&format!(
        "dashboards: wrote {written} file(s) to {}",
        dest.display()
    ));
    Some(written)
}

/// Recursively write one embedded directory to `dest`.
fn write_dir(log: &dyn Fn(&str), dir: &Dir<'_>, dest: &Path, written: &mut usize) {
    if let Err(e) = std::fs::create_dir_all(dest) {
        log(&format!("dashboards: mkdir {} failed: {e}", dest.display()));
        return;
    }
    for file in dir.files() {
        let Some(name) = file.path().file_name() else {
            continue;
        };
        let out = dest.join(name);
        match std::fs::write(&out, file.contents()) {
            Ok(_) => *written += 1,
            Err(e) => log(&format!("dashboards: write {} failed: {e}", out.display())),
        }
    }
    for sub in dir.dirs() {
        let Some(name) = sub.path().file_name() else {
            continue;
        };
        write_dir(log, sub, &dest.join(name), written);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn scratch(tag: &str) -> PathBuf {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let p =
            std::env::temp_dir().join(format!("moonpool-dash-{tag}-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn quiet(_: &str) {}

    #[test]
    fn seeds_the_embedded_tree_and_stamps_it() {
        let home = scratch("fresh");
        let dest = home.join("dashboards").join("examples");
        let n = seed_into(&dest, "1.2.3", &quiet).expect("first seed writes");
        assert!(n > 0);
        assert!(dest.join("README.md").is_file());
        assert_eq!(std::fs::read_to_string(dest.join(STAMP)).unwrap(), "1.2.3");
        // Every embedded top-level folder lands on disk.
        for sub in DASHBOARDS.dirs() {
            assert!(dest.join(sub.path().file_name().unwrap()).is_dir());
        }
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn same_stamp_is_left_alone() {
        let home = scratch("same");
        let dest = home.join("examples");
        seed_into(&dest, "1.2.3", &quiet).unwrap();
        std::fs::write(dest.join("marker.txt"), "x").unwrap();
        assert_eq!(seed_into(&dest, "1.2.3", &quiet), None);
        assert!(
            dest.join("marker.txt").exists(),
            "a matching stamp must not rewrite"
        );
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn stamp_without_readme_reseeds() {
        let home = scratch("noreadme");
        let dest = home.join("examples");
        std::fs::create_dir_all(&dest).unwrap();
        std::fs::write(dest.join(STAMP), "1.2.3").unwrap();
        assert!(seed_into(&dest, "1.2.3", &quiet).is_some());
        assert!(dest.join("README.md").is_file());
        let _ = std::fs::remove_dir_all(&home);
    }

    /// A different stamp replaces the examples folder wholesale (stale files go), and never
    /// touches the user's own files beside it in `dashboards/`.
    #[test]
    fn new_stamp_replaces_examples_but_not_user_dashboards() {
        let home = scratch("replace");
        let dash = home.join("dashboards");
        let dest = dash.join("examples");
        seed_into(&dest, "1.0.0", &quiet).unwrap();
        std::fs::write(dest.join("stale.txt"), "old").unwrap();
        std::fs::write(dash.join("mine.html"), "user").unwrap();
        std::fs::create_dir_all(dash.join("board")).unwrap();
        std::fs::write(dash.join("board").join("index.html"), "user board").unwrap();

        assert!(seed_into(&dest, "2.0.0", &quiet).is_some());
        assert!(!dest.join("stale.txt").exists());
        assert_eq!(std::fs::read_to_string(dest.join(STAMP)).unwrap(), "2.0.0");
        assert_eq!(
            std::fs::read_to_string(dash.join("mine.html")).unwrap(),
            "user"
        );
        assert!(dash.join("board").join("index.html").is_file());
        let _ = std::fs::remove_dir_all(&home);
    }
}
