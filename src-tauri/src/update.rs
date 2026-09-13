//! Custom self-updater. Replaces tauri-plugin-updater: we own the whole path so
//! the update experience is our skinned UI, not a silent NSIS run.
//!
//! Flow: fetch `update.json` from GitHub Releases -> semver-compare -> download the
//! raw `moonpool.exe` -> verify its minisign signature against the committed pubkey
//! -> self-replace and relaunch. The self-replace uses the Windows rename trick:
//! a running .exe can't be overwritten, but it CAN be renamed, so we move ourselves
//! aside to `moonpool.old`, write the new exe into our original path, relaunch, and
//! exit. `cleanup_old()` deletes the leftover `.old` on the next startup.

use std::io::Read;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

/// Where the update manifest lives. Same GitHub Releases channel as before, but a
/// custom manifest (raw exe + minisign sig) instead of Tauri's latest.json.
const MANIFEST_URL: &str =
    "https://github.com/kjustinkeener/Moonpool/releases/latest/download/update.json";

/// The minisign public key, committed base64 (same value tauri.conf.json used for
/// the old updater plugin). It's the base64 of the whole minisign pubkey *file*
/// (comment line + key line); we decode it and keep the key line.
#[cfg(windows)]
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
/// show "you're on X, Y is out".
#[derive(Clone, Serialize)]
pub struct CheckResult {
    pub current: String,
    pub available: Option<UpdateInfo>,
}

fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Delete leftover `moonpool.old` / `moonpool.new` next to the running exe (from a
/// prior update). Best-effort: if a file is somehow still locked, we retry next launch.
pub fn cleanup_old() {
    if let Ok(cur) = std::env::current_exe() {
        for ext in ["old", "new"] {
            let stray = cur.with_extension(ext);
            if stray.exists() {
                let _ = std::fs::remove_file(&stray);
            }
        }
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

/// Fetch and parse the manifest. Returns whether a newer version is offered.
#[tauri::command]
pub fn update_check() -> Result<CheckResult, String> {
    let current = current_version().to_string();
    let body = http_get_string(MANIFEST_URL)?;
    let info: UpdateInfo =
        serde_json::from_str(&body).map_err(|e| format!("bad update manifest: {e}"))?;

    let available = is_newer(&current, &info.version)?.then_some(info);
    Ok(CheckResult { current, available })
}

/// Download, verify, and apply an update, then relaunch. On success this never
/// returns normally: the process exits and the new exe takes over.
#[tauri::command]
pub fn update_apply(app: AppHandle, info: UpdateInfo) -> Result<(), String> {
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

    // The self-replace is the Windows rename trick plus a registry version sync; there
    // is no in-place updater on other platforms. Point the user at a manual download
    // instead - the About panel links the releases page. Guard before downloading so a
    // non-Windows build never runs Windows-only file/registry surgery.
    #[cfg(not(windows))]
    {
        let _ = &app;
        Err("automatic update is Windows-only; download the latest release manually".to_string())
    }
    #[cfg(windows)]
    {
        let bytes = http_get_bytes(&info.url)?;
        verify_signature(&bytes, &info.signature)?;
        self_replace_and_relaunch(&app, &bytes, info.version.trim_start_matches('v'))?;
        Ok(())
    }
}

/// Verify `data` against `sig_text` (a full .minisig file) using the committed key.
#[cfg(windows)]
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

/// The Windows self-replace: rename running exe aside, write the new one in place,
/// relaunch it, and exit this (old) process.
#[cfg(windows)]
fn self_replace_and_relaunch(
    app: &AppHandle,
    new_bytes: &[u8],
    new_version: &str,
) -> Result<(), String> {
    let cur = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let old = cur.with_extension("old");
    let _ = std::fs::remove_file(&old);

    // Rename works on a running exe; overwrite does not.
    std::fs::rename(&cur, &old).map_err(|e| format!("rename self aside: {e}"))?;

    if let Err(e) = write_new_exe(&cur, new_bytes) {
        // Roll back so the app still launches next time.
        let _ = std::fs::rename(&old, &cur);
        return Err(format!("write new exe: {e}"));
    }

    // The exe is swapped but the uninstall key still shows the old version; sync it
    // so "Installed apps" is right. Best-effort, after the write is committed.
    crate::install::update_display_version(new_version);

    // Launch the freshly written exe, then bow out. Tell the child to wait for THIS
    // process to exit (`--wait-pid`) before it builds anything, so it doesn't race the
    // single-instance lock we still hold and get routed straight back into us (leaving
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

/// Write the new exe durably: stream to a temp file beside the target, flush and
/// fsync it, then atomically rename it into place. A failure or crash mid-write leaves
/// only the temp file (cleaned up on error and on next launch), never a truncated exe
/// at `path` that would brick the app.
#[cfg(windows)]
fn write_new_exe(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let tmp = path.with_extension("new");
    let result = (|| {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.flush()?;
        f.sync_all()?;
        std::fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
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
    use super::is_newer;

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
