//! Custom self-updater. Replaces tauri-plugin-updater: we own the whole path so
//! the update experience is our skinned UI, not a silent NSIS run.
//!
//! Flow: fetch `update.json` from GitHub Releases -> semver-compare -> download the
//! raw `moonpool.exe` -> verify its minisign signature against the committed pubkey
//! -> self-replace and relaunch. The self-replace uses the Windows rename trick:
//! a running .exe can't be overwritten, but it CAN be renamed, so we write the new exe
//! to `moonpool.new` and fsync it, then move ourselves aside to `moonpool.old` and the
//! new exe into our original path back to back, relaunch, and exit. `cleanup_old()`
//! deletes the leftover `.old` on the next startup.

use std::io::Read;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

/// Where the Windows update manifest lives. It carries the raw self-replacing exe.
#[cfg(not(target_os = "linux"))]
const MANIFEST_URL: &str =
    "https://github.com/kjustinkeener/Moonpool/releases/latest/download/update.json";

/// Linux ships both package-manager packages and a portable AppImage. Only the
/// AppImage can safely update itself, so it gets its own signed release manifest.
#[cfg(target_os = "linux")]
const MANIFEST_URL: &str =
    "https://github.com/kjustinkeener/Moonpool/releases/latest/download/linux-update.json";

/// The minisign public key, committed base64 (same value tauri.conf.json used for
/// the old updater plugin). It's the base64 of the whole minisign pubkey *file*
/// (comment line + key line); we decode it and keep the key line.
#[cfg(any(windows, target_os = "linux"))]
const PUBKEY_B64: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEJBRkE1M0EzNkUyQzI3MkMKUldRc0p5eHVvMVA2dW1WbUlZSzFPOGY1UjNWQUwvNnFCTmtncmNYeWcvUjZrQllPTER4QUliY00K";

/// 200 MB hard cap on a downloaded binary, so a bad/hostile manifest can't make us
/// read forever.
const MAX_DOWNLOAD: u64 = 200 * 1024 * 1024;

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    #[serde(default)]
    pub notes: String,
    pub url: String,
    /// The full contents of the `.minisig` file for `url`.
    pub signature: String,
}

/// Result of a check: `available` plus the current running version so the UI can
/// show "you're on X, Y is out". Help pages ship inside the exe, so they update only
/// with the app. Older manifests also carried a `help` object; serde ignores it.
#[derive(Clone, Serialize)]
pub struct CheckResult {
    pub current: String,
    pub available: Option<UpdateInfo>,
    /// Whether `update_apply` would act on this copy (`can_self_update`). The hub hides its
    /// update banner when false, so a dev build or a not-yet-installed download never offers
    /// an update it would refuse.
    pub can_apply: bool,
}

fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Refusal `update_apply` gives on a Windows copy that may not replace itself.
#[cfg(windows)]
const NOT_INSTALLED: &str = "Updates apply only to the installed copy";

/// Whether this copy may replace itself. On Windows that is the installed copy and an
/// established portable copy (its own movable install, which updates in place); never a
/// download that has not been installed yet, or a dev build running out of `target\`. On
/// Linux it is an AppImage (`update_target`); package installs belong to the package manager.
fn can_self_update() -> bool {
    #[cfg(windows)]
    {
        crate::install::is_installed() || crate::portable::is_portable()
    }
    #[cfg(target_os = "linux")]
    {
        update_target().is_some()
    }
    #[cfg(all(not(windows), not(target_os = "linux")))]
    {
        false
    }
}

/// Delete leftover `moonpool.old`, `moonpool.old-<n>` and `moonpool.new` next to the
/// running exe (from a prior update). Best-effort: a file still locked by an MCP server
/// started before the update is retried next launch.
pub fn cleanup_old() {
    if let Some(cur) = update_target() {
        for ext in ["old", "new"] {
            let stray = cur.with_extension(ext);
            if stray.exists() {
                let _ = std::fs::remove_file(&stray);
            }
        }
        let (Some(dir), Some(stem)) = (cur.parent(), cur.file_stem().and_then(|s| s.to_str()))
        else {
            return;
        };
        let prefix = format!("{stem}.old-");
        if let Ok(entries) = std::fs::read_dir(dir) {
            for e in entries.flatten() {
                if e.file_name()
                    .to_str()
                    .is_some_and(|n| n.starts_with(&prefix))
                {
                    let _ = std::fs::remove_file(e.path());
                }
            }
        }
    }
}

/// Rename `path` aside so a new file can take its place, and return where it went. A
/// running exe can be renamed but not overwritten, so this is how both the updater and a
/// reinstall over an in-use install get the old bytes out of the way. The aside name is
/// `<stem>.old`, or `<stem>.old-<pid>` when `.old` can't be removed: a `moonpool.exe mcp`
/// started before the last update or reinstall still runs from the previous `.old`, which
/// locks it, and renaming onto it would fail with Access denied. `cleanup_old` sweeps both
/// once the process holding them ends.
pub(crate) fn step_aside(path: &Path) -> std::io::Result<PathBuf> {
    let mut old = path.with_extension("old");
    let _ = std::fs::remove_file(&old);
    if old.exists() {
        old = path.with_extension(format!("old-{}", std::process::id()));
        let _ = std::fs::remove_file(&old);
    }
    std::fs::rename(path, &old)?;
    Ok(old)
}

/// The file we are allowed to replace. In an AppImage process `current_exe()`
/// points into the temporary FUSE mount, not at the AppImage the user launched;
/// APPIMAGE is the stable source file. Packaged Linux installs deliberately return
/// `None` so apt/dnf/rpm remain the sole authority for their upgrades.
fn update_target() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        let path = std::env::var_os("APPIMAGE").map(PathBuf::from)?;
        path.is_file().then_some(path)
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::env::current_exe().ok()
    }
}

/// Whether `candidate` is a strictly newer semver than `current` (both tolerate a
/// leading `v`). Errors if either is unparseable. Single source of truth for the
/// "is this an upgrade?" question so `update_check` and `update_apply` can't diverge.
fn is_newer(current: &str, candidate: &str) -> Result<bool, String> {
    let cur = semver::Version::parse(current.trim_start_matches('v'))
        .map_err(|e| format!("bad current version {current}: {e}"))?;
    let new = semver::Version::parse(candidate.trim_start_matches('v'))
        .map_err(|e| format!("bad manifest version {candidate}: {e}"))?;
    Ok(new > cur)
}

/// Fetch and parse the manifest. Returns whether a newer app version is offered.
#[tauri::command]
pub fn update_check() -> Result<CheckResult, String> {
    let body = http_get_string(MANIFEST_URL)?;
    let mut r = check_manifest(current_version(), &body)?;
    r.can_apply = can_self_update();
    Ok(r)
}

/// Parse a fetched manifest and decide whether it offers an upgrade over `current`. Split out
/// of `update_check` so the parse rules are testable without the network.
fn check_manifest(current: &str, body: &str) -> Result<CheckResult, String> {
    let manifest: UpdateInfo =
        serde_json::from_str(body).map_err(|e| format!("bad update manifest: {e}"))?;
    let available = is_newer(current, &manifest.version)?.then_some(manifest);
    Ok(CheckResult {
        current: current.to_string(),
        available,
        can_apply: false,
    })
}

/// Download, verify, and apply an update, then relaunch. On success this never
/// returns normally: the process exits and the new exe takes over.
#[tauri::command]
pub fn update_apply(app: AppHandle, info: UpdateInfo) -> Result<(), String> {
    // Only a real install replaces itself: a dev build or a download that has not been
    // installed yet would otherwise overwrite whatever exe it happens to be running as.
    // Linux has its own refusal below, which names the package manager.
    #[cfg(windows)]
    if !can_self_update() {
        return Err(NOT_INSTALLED.to_string());
    }

    // Never install a build that isn't strictly newer, even when `update_apply` is
    // called directly: the newer-than check in `update_check` is otherwise the only
    // guard and a caller (or a stale/rolled-back manifest) could bypass it and
    // downgrade the app.
    if !is_newer(current_version(), &info.version)? {
        return Err(format!(
            "refusing to install {}: not newer than current {}",
            info.version,
            current_version()
        ));
    }

    // Linux package installs are upgraded by their package manager. An AppImage is
    // self-contained, however, and can be atomically replaced just like the Windows
    // portable executable.
    #[cfg(target_os = "linux")]
    {
        let target = update_target().ok_or(
            "automatic updates are available for the AppImage only; update the .deb or RPM with your package manager",
        )?;
        let bytes = http_get_bytes(&info.url)?;
        verify_signature(&bytes, &info.signature)?;
        self_replace_appimage_and_relaunch(&app, &target, &bytes)?;
        Ok(())
    }
    #[cfg(all(not(windows), not(target_os = "linux")))]
    {
        let _ = &app;
        Err("automatic updates are not supported on this platform; download the latest release manually".to_string())
    }
    #[cfg(windows)]
    {
        let bytes = http_get_bytes(&info.url)?;
        verify_signature(&bytes, &info.signature)?;
        self_replace_and_relaunch(&app, &bytes, info.version.trim_start_matches('v'))?;
        Ok(())
    }
}

/// Atomically replace the AppImage that launched us, then start the replacement
/// behind the normal per-copy lock handoff. The running image remains mapped by
/// its inode after the rename, so the swap is safe on Linux filesystems.
#[cfg(target_os = "linux")]
fn self_replace_appimage_and_relaunch(
    app: &AppHandle,
    target: &Path,
    new_bytes: &[u8],
) -> Result<(), String> {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    let old = target.with_extension("old");
    let staged = target.with_extension("new");
    let _ = std::fs::remove_file(&old);
    let _ = std::fs::remove_file(&staged);

    let result = (|| -> Result<(), String> {
        let mut f = std::fs::File::create(&staged).map_err(|e| format!("create update: {e}"))?;
        f.write_all(new_bytes)
            .map_err(|e| format!("write update: {e}"))?;
        f.flush().map_err(|e| format!("flush update: {e}"))?;
        f.sync_all().map_err(|e| format!("sync update: {e}"))?;
        std::fs::set_permissions(&staged, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("make update executable: {e}"))?;
        std::fs::rename(target, &old).map_err(|e| format!("move current AppImage aside: {e}"))?;
        if let Err(e) = std::fs::rename(&staged, target) {
            let _ = std::fs::rename(&old, target);
            return Err(format!("activate update: {e}"));
        }
        Ok(())
    })();
    if let Err(e) = result {
        let _ = std::fs::remove_file(&staged);
        return Err(e);
    }

    let mut cmd = std::process::Command::new(target);
    cmd.arg("--wait-pid").arg(std::process::id().to_string());
    if let Err(e) = cmd.spawn() {
        let _ = std::fs::remove_file(target);
        let _ = std::fs::rename(&old, target);
        return Err(format!("relaunch updated AppImage: {e}"));
    }
    app.cleanup_before_exit();
    app.exit(0);
    Ok(())
}

/// Verify `data` against `sig_text` (a full .minisig file) using the committed key.
#[cfg(any(windows, target_os = "linux"))]
fn verify_signature(data: &[u8], sig_text: &str) -> Result<(), String> {
    use base64::Engine as _;
    let pubkey_file = base64::engine::general_purpose::STANDARD
        .decode(PUBKEY_B64)
        .map_err(|e| format!("pubkey decode: {e}"))?;
    let pubkey_file = String::from_utf8(pubkey_file).map_err(|e| format!("pubkey utf8: {e}"))?;
    // The pubkey file is "untrusted comment: ...\n<base64 key>\n"; take the key line.
    let key_line = pubkey_file
        .lines()
        .find(|l| !l.trim().is_empty() && !l.starts_with("untrusted comment:"))
        .ok_or("pubkey file has no key line")?;

    let pk = minisign_verify::PublicKey::from_base64(key_line.trim())
        .map_err(|e| format!("parse pubkey: {e}"))?;
    let sig = minisign_verify::Signature::decode(sig_text)
        .map_err(|e| format!("parse signature: {e}"))?;
    // Tauri's signer may emit prehashed or legacy minisign signatures; accept
    // either (allow_legacy = true) since both are produced by the same trusted key.
    pk.verify(data, &sig, true)
        .map_err(|_| "signature verification FAILED - refusing to install".to_string())
}

/// The Windows self-replace: stage the new exe beside the running one, swap it in, relaunch
/// it, and exit this (old) process.
#[cfg(windows)]
fn self_replace_and_relaunch(
    app: &AppHandle,
    new_bytes: &[u8],
    new_version: &str,
) -> Result<(), String> {
    let cur = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;

    let old = swap_in_new_exe(&cur, new_bytes)?;

    // The exe is swapped but the uninstall key still shows the old version; sync it
    // so "Installed apps" is right. Best-effort, after the write is committed.
    crate::install::update_display_version(new_version);

    // Launch the freshly written exe, then bow out. Tell the child to wait for THIS
    // process to exit (`--wait-pid`) before it builds anything, so it doesn't race the
    // per-copy lock (`instance`) we still hold and get routed straight back into us (leaving
    // no resident instance). Mirrors `install::relaunch_and_exit`.
    let mut c = std::process::Command::new(&cur);
    c.arg("--wait-pid").arg(std::process::id().to_string());
    crate::platform::hidden(&mut c);
    if let Err(e) = c.spawn() {
        let _ = std::fs::remove_file(&cur);
        let _ = std::fs::rename(&old, &cur);
        return Err(format!("relaunch: {e}"));
    }

    // Give the child a beat to come up before we drop the tray icon.
    app.cleanup_before_exit();
    app.exit(0);
    Ok(())
}

/// Replace the exe at `path` with `bytes` and return where the old one went. All the slow,
/// failure-prone work happens first, while `path` is untouched: the new exe is written to
/// `<stem>.new` beside it and fsynced. Only then is the old exe renamed aside (`step_aside`;
/// a running exe can be renamed but not overwritten) and the new one renamed into place, back
/// to back, so the window in which `path` has no exe is two renames wide. Any failure puts the
/// old exe back and removes `.new`, leaving the app as it was.
#[cfg(windows)]
fn swap_in_new_exe(path: &Path, bytes: &[u8]) -> Result<PathBuf, String> {
    use std::io::Write;
    let staged = path.with_extension("new");
    let write = (|| {
        let mut f = std::fs::File::create(&staged)?;
        f.write_all(bytes)?;
        f.flush()?;
        f.sync_all()
    })();
    if let Err(e) = write {
        let _ = std::fs::remove_file(&staged);
        return Err(format!("write new exe: {e}"));
    }
    let old = match step_aside(path) {
        Ok(old) => old,
        Err(e) => {
            let _ = std::fs::remove_file(&staged);
            return Err(format!("rename self aside: {e}"));
        }
    };
    if let Err(e) = std::fs::rename(&staged, path) {
        let _ = std::fs::rename(&old, path);
        let _ = std::fs::remove_file(&staged);
        return Err(format!("move new exe into place: {e}"));
    }
    Ok(old)
}

// --- tiny HTTP helpers (ureq, blocking) -----------------------------------

fn http_get_string(url: &str) -> Result<String, String> {
    let bytes = http_get_bytes(url)?;
    String::from_utf8(bytes).map_err(|e| format!("response not utf8: {e}"))
}

fn http_get_bytes(url: &str) -> Result<Vec<u8>, String> {
    let resp = ureq::get(url)
        .set("User-Agent", "Moonpool-Updater")
        .call()
        .map_err(|e| format!("download failed: {e}"))?;
    let mut buf = Vec::new();
    resp.into_reader()
        .take(MAX_DOWNLOAD)
        .read_to_end(&mut buf)
        .map_err(|e| format!("read body: {e}"))?;
    Ok(buf)
}

/// Best-effort helper other modules can use to find the running exe dir.
#[allow(dead_code)]
pub fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
}

#[cfg(test)]
mod tests {
    use super::{check_manifest, is_newer};

    const SIG: &str = "untrusted comment: x\\nRUQ=\\ntrusted comment: y\\nAA==\\n";

    fn manifest(version: &str, extra: &str) -> String {
        format!(
            r#"{{"version":"{version}","notes":"n","url":"https://example.invalid/m.exe","signature":"{SIG}"{extra}}}"#
        )
    }

    #[test]
    fn manifest_without_help_parses() {
        let r = check_manifest("0.3.15", &manifest("0.3.16", "")).unwrap();
        assert_eq!(r.current, "0.3.15");
        let a = r.available.expect("newer is offered");
        assert_eq!(a.version, "0.3.16");
        assert_eq!(a.notes, "n");
        assert!(a.signature.starts_with("untrusted comment"));
    }

    /// Manifests from before help-only updates were dropped carry a `help` object. They must
    /// still parse, or every older release's update.json would read as "bad manifest".
    #[test]
    fn manifest_with_legacy_help_field_still_parses() {
        let help =
            r#","help":{"version":"2026.1","url":"https://example.invalid/h.zip","signature":"s"}"#;
        let r = check_manifest("0.3.15", &manifest("0.4.0", help)).unwrap();
        assert_eq!(r.available.unwrap().version, "0.4.0");
        // Any other unknown field is ignored too.
        let r = check_manifest("0.3.15", &manifest("0.4.0", r#","future":[1,2]"#)).unwrap();
        assert!(r.available.is_some());
    }

    #[test]
    fn notes_are_optional() {
        let body = format!(r#"{{"version":"1.0.0","url":"u","signature":"{SIG}"}}"#);
        let r = check_manifest("0.1.0", &body).unwrap();
        assert_eq!(r.available.unwrap().notes, "");
    }

    #[test]
    fn same_or_older_version_offers_nothing() {
        for v in ["0.3.15", "0.3.14", "v0.1.0"] {
            let r = check_manifest("0.3.15", &manifest(v, "")).unwrap();
            assert!(r.available.is_none(), "{v} must not be offered");
        }
    }

    #[test]
    fn malformed_manifests_are_errors() {
        assert!(check_manifest("0.3.15", "not json").is_err());
        assert!(
            check_manifest("0.3.15", r#"{"version":"1.0.0"}"#).is_err(),
            "no url/signature"
        );
        assert!(check_manifest("0.3.15", &manifest("banana", "")).is_err());
    }

    #[cfg(any(windows, target_os = "linux"))]
    #[test]
    fn garbage_signatures_are_refused() {
        assert!(super::verify_signature(b"payload", "").is_err());
        assert!(super::verify_signature(b"payload", "untrusted comment: x\nnot base64\n").is_err());
    }

    /// A fresh, empty temp folder for one test.
    #[cfg(windows)]
    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "moonpool-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[cfg(windows)]
    #[test]
    fn swap_in_new_exe_replaces_the_target_and_leaves_no_staged_file() {
        let dir = scratch("update-swap");
        let target = dir.join("app.exe");
        std::fs::write(&target, b"old").unwrap();
        let old = super::swap_in_new_exe(&target, b"new bytes").unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"new bytes");
        assert_eq!(std::fs::read(&old).unwrap(), b"old");
        assert!(!target.with_extension("new").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The new exe can't be staged (here `.new` is a folder, so creating the file fails):
    /// the running exe must never have been moved.
    #[cfg(windows)]
    #[test]
    fn swap_in_new_exe_leaves_the_target_alone_when_staging_fails() {
        let dir = scratch("update-stagefail");
        let target = dir.join("app.exe");
        std::fs::write(&target, b"old").unwrap();
        std::fs::create_dir(target.with_extension("new")).unwrap();
        let err = super::swap_in_new_exe(&target, b"new bytes").unwrap_err();
        assert!(err.starts_with("write new exe:"), "{err}");
        assert_eq!(std::fs::read(&target).unwrap(), b"old");
        assert!(!target.with_extension("old").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The old exe can't be renamed aside (held without delete sharing): `.new` is removed
    /// and the target keeps its old bytes.
    #[cfg(windows)]
    #[test]
    fn swap_in_new_exe_rolls_back_when_the_old_exe_cannot_move() {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_SHARE_READ: u32 = 0x1;
        let dir = scratch("update-pinned");
        let target = dir.join("app.exe");
        std::fs::write(&target, b"old").unwrap();
        let pinned = std::fs::OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&target)
            .unwrap();
        let err = super::swap_in_new_exe(&target, b"new bytes").unwrap_err();
        drop(pinned);
        assert!(err.starts_with("rename self aside:"), "{err}");
        assert_eq!(std::fs::read(&target).unwrap(), b"old");
        assert!(!target.with_extension("new").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn only_strictly_newer_versions_apply() {
        assert!(is_newer("0.3.7", "0.3.8").unwrap());
        assert!(is_newer("0.3.7", "v0.4.0").unwrap());
        assert!(is_newer("v0.3.7", "0.4.0").unwrap());
        assert!(!is_newer("0.3.7", "0.3.7").unwrap(), "equal is not newer");
        assert!(!is_newer("0.3.7", "0.3.6").unwrap(), "older is not newer");
        assert!(is_newer("0.3.7", "not-a-version").is_err());
    }
}
