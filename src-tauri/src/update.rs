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

/// Delete a leftover `moonpool.old` next to the running exe (from a prior update).
/// Best-effort: if the file is somehow still locked, we just try again next launch.
pub fn cleanup_old() {
    if let Ok(cur) = std::env::current_exe() {
        let old = cur.with_extension("old");
        if old.exists() {
            let _ = std::fs::remove_file(&old);
        }
    }
}

/// Fetch and parse the manifest. Returns whether a newer version is offered.
#[tauri::command]
pub fn update_check() -> Result<CheckResult, String> {
    let current = current_version().to_string();
    let body = http_get_string(MANIFEST_URL)?;
    let info: UpdateInfo =
        serde_json::from_str(&body).map_err(|e| format!("bad update manifest: {e}"))?;

    let cur = semver::Version::parse(&current)
        .map_err(|e| format!("bad current version {current}: {e}"))?;
    let new = semver::Version::parse(info.version.trim_start_matches('v'))
        .map_err(|e| format!("bad manifest version {}: {e}", info.version))?;

    Ok(CheckResult {
        current,
        available: (new > cur).then_some(info),
    })
}

/// Download, verify, and apply an update, then relaunch. On success this never
/// returns normally: the process exits and the new exe takes over.
#[tauri::command]
pub fn update_apply(app: AppHandle, info: UpdateInfo) -> Result<(), String> {
    let bytes = http_get_bytes(&info.url)?;
    verify_signature(&bytes, &info.signature)?;
    self_replace_and_relaunch(&app, &bytes, info.version.trim_start_matches('v'))?;
    Ok(())
}

/// Verify `data` against `sig_text` (a full .minisig file) using the committed key.
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

    // Launch the freshly written exe, then bow out.
    if let Err(e) = std::process::Command::new(&cur).spawn() {
        let _ = std::fs::remove_file(&cur);
        let _ = std::fs::rename(&old, &cur);
        return Err(format!("relaunch: {e}"));
    }

    // Give the child a beat to come up before we drop the tray icon.
    app.cleanup_before_exit();
    app.exit(0);
    Ok(())
}

fn write_new_exe(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    std::fs::write(path, bytes)?;
    Ok(())
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
