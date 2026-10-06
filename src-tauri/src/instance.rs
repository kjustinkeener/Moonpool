//! One Moonpool per folder.
//!
//! An installed Moonpool and any number of portable copies (each in its own folder) can run at
//! the same time, fully independent. Three pieces make that work, all keyed by the copy's
//! identity so the hub, the `moonpool.exe mcp` client and a forwarding CLI launch agree on it:
//!
//! - Identity (`copy_id`): the installed copy has none and keeps the bare names it always had
//!   (`\\.\pipe\moonpool`), so existing MCP registrations and scripts keep working. A portable
//!   copy's id is the first 8 hex digits of a stable hash of its canonical config folder
//!   (lowercased on Windows, where paths are case-insensitive).
//! - The per-copy lock (`acquire`): a named mutex on Windows, an `flock`ed lock file in the
//!   copy's data dir elsewhere. Both are atomic, so two simultaneous launches of the same copy
//!   end with exactly one hub, and both are released by the OS when the process dies.
//! - Forwarding (`forward`): a second launch of the SAME copy hands its argv to the running hub
//!   over that copy's control channel (`argv` verb, see `control.rs`), which runs it through
//!   `dispatch_control` exactly as the old single-instance plugin callback did, then exits.
//!
//! This replaced `tauri-plugin-single-instance`, which locked on the app identifier and so
//! allowed only one Moonpool per user no matter which folder it ran from.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::portable;

/// The bare base name every per-copy name is derived from.
const BASE: &str = "moonpool";

/// The identity string for a config folder: canonicalized when possible (so `C:\X\..\Y` and
/// `C:\Y` agree), lowercased on Windows. Pure so tests can feed it any path.
fn identity_key(dir: &Path) -> String {
    let canon = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let s = canon.to_string_lossy().to_string();
    if cfg!(windows) {
        s.to_lowercase()
    } else {
        s
    }
}

/// FNV-1a 32-bit. Hand-rolled because the id must never change between builds or Rust
/// versions (std's `DefaultHasher` makes no such promise), and it only needs to tell a handful
/// of folders apart on one machine, not resist an attacker.
fn fnv1a32(bytes: &[u8]) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    for b in bytes {
        h ^= u32::from(*b);
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

/// The 8-hex-digit id for a config folder.
pub(crate) fn id_for_dir(dir: &Path) -> String {
    format!("{:08x}", fnv1a32(identity_key(dir).as_bytes()))
}

/// The config folder a portable copy is identified by: `<canonical exe dir>\moonpool-config`.
/// Built from the exe dir (which always exists) rather than canonicalizing the config folder
/// itself, which may not exist yet on a copy's very first launch.
fn portable_identity_dir() -> Option<PathBuf> {
    let exe_dir = portable::exe_dir()?;
    let exe_dir = exe_dir.canonicalize().unwrap_or(exe_dir);
    Some(exe_dir.join(portable::DATA_SUBDIR))
}

/// This copy's id, or `None` for the installed copy (which keeps the bare names).
pub fn copy_id() -> Option<&'static str> {
    static ID: OnceLock<Option<String>> = OnceLock::new();
    ID.get_or_init(|| {
        if portable::is_portable() {
            portable_identity_dir().map(|d| id_for_dir(&d))
        } else {
            None
        }
    })
    .as_deref()
}

/// `""` for the installed copy, `"-<id>"` for a portable one. Appended to every per-copy name.
pub(crate) fn suffix_for(id: Option<&str>) -> String {
    id.map(|i| format!("-{i}")).unwrap_or_default()
}

pub fn suffix() -> String {
    suffix_for(copy_id())
}

/// The control pipe name for a copy id (Windows).
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) fn pipe_name_for(id: Option<&str>) -> String {
    format!(r"\\.\pipe\{BASE}{}", suffix_for(id))
}

/// The per-copy lock's mutex name (Windows). `Local\` = this login session, same scope the old
/// plugin's mutex had.
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) fn mutex_name_for(id: Option<&str>) -> String {
    format!(r"Local\{BASE}-instance{}", suffix_for(id))
}

/// The folder name a portable copy is known by: the folder the user picked, i.e. the parent of
/// the `.moonpool` app folder (or the exe's own folder when it is not inside one).
fn portable_folder_name(exe_dir: &Path) -> Option<String> {
    let dir = if exe_dir.file_name().and_then(|n| n.to_str()) == Some(portable::APP_DIR) {
        exe_dir.parent()?
    } else {
        exe_dir
    };
    dir.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .filter(|n| !n.is_empty())
}

/// The portable folder name, or `None` for the installed copy.
pub fn copy_folder_name() -> Option<&'static str> {
    static NAME: OnceLock<Option<String>> = OnceLock::new();
    NAME.get_or_init(|| {
        if !portable::is_portable() {
            return None;
        }
        Some(
            portable::exe_dir()
                .and_then(|d| portable_folder_name(&d))
                .unwrap_or_else(|| "portable".to_string()),
        )
    })
    .as_deref()
}

fn label_for(folder: Option<&str>) -> String {
    match folder {
        Some(f) => format!("Moonpool ({f})"),
        None => "Moonpool".to_string(),
    }
}

/// What the user sees for this copy: "Moonpool" installed, "Moonpool (<folder>)" portable.
/// Used for the tray tooltip, the window title and the MCP server identity.
pub fn label() -> &'static str {
    static LABEL: OnceLock<String> = OnceLock::new();
    LABEL.get_or_init(|| label_for(copy_folder_name()))
}

// ---------------------------------------------------------------------------
// The per-copy lock
// ---------------------------------------------------------------------------

/// Held for the life of the hub process; dropping it (or the process dying) frees the copy.
pub struct InstanceLock {
    #[cfg(windows)]
    handle: windows_sys::Win32::Foundation::HANDLE,
    #[cfg(unix)]
    _file: std::fs::File,
}

// The mutex handle is only ever closed, never shared for waiting.
unsafe impl Send for InstanceLock {}
unsafe impl Sync for InstanceLock {}

#[cfg(windows)]
impl Drop for InstanceLock {
    fn drop(&mut self) {
        unsafe { windows_sys::Win32::Foundation::CloseHandle(self.handle) };
    }
}

#[cfg(windows)]
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Try to take the named mutex `name`. `Ok(None)` means another process already holds it.
/// `CreateMutexW` either creates the object or opens the existing one and says so, atomically,
/// so two racing launches can never both see "created".
#[cfg(windows)]
pub(crate) fn try_lock_named(name: &str) -> std::io::Result<Option<InstanceLock>> {
    use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS};
    use windows_sys::Win32::System::Threading::CreateMutexW;
    let w = wide(name);
    let handle = unsafe { CreateMutexW(std::ptr::null(), 0, w.as_ptr()) };
    if handle.is_null() {
        return Err(std::io::Error::last_os_error());
    }
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        unsafe { CloseHandle(handle) };
        return Ok(None);
    }
    Ok(Some(InstanceLock { handle }))
}

/// Whether a pre-per-folder Moonpool build is running in this session. Those builds used
/// `tauri-plugin-single-instance`, whose mutex is named after the app identifier, and they all
/// claim the bare pipe name, so this only matters to the installed copy: treating that mutex as
/// "held" keeps a freshly downloaded or hand-copied new build from booting a second installed
/// hub beside an old one (it forwards to it instead, as before).
#[cfg(windows)]
fn legacy_hub_present() -> bool {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::OpenMutexW;
    const SYNCHRONIZE: u32 = 0x0010_0000;
    let w = wide("com.moonpool.app-sim");
    let h = unsafe { OpenMutexW(SYNCHRONIZE, 0, w.as_ptr()) };
    if h.is_null() {
        false
    } else {
        unsafe { CloseHandle(h) };
        true
    }
}

/// Try to take an exclusive `flock` on `path`. `Ok(None)` means another process holds it.
#[cfg(unix)]
pub(crate) fn try_lock_file(path: &Path) -> std::io::Result<Option<InstanceLock>> {
    use std::os::unix::io::AsRawFd;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)?;
    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if rc == 0 {
        return Ok(Some(InstanceLock { _file: file }));
    }
    let e = std::io::Error::last_os_error();
    if e.kind() == std::io::ErrorKind::WouldBlock {
        Ok(None)
    } else {
        Err(e)
    }
}

/// Outcome of trying to become this copy's hub.
pub enum Acquire {
    /// We own the copy: boot the hub and keep the lock alive.
    Owned(InstanceLock),
    /// Another process is this copy's hub: forward and exit.
    HeldElsewhere,
}

/// Take this copy's lock once.
fn acquire_once() -> std::io::Result<Acquire> {
    #[cfg(windows)]
    {
        let id = copy_id();
        match try_lock_named(&mutex_name_for(id))? {
            Some(lock) => {
                if id.is_none() && legacy_hub_present() {
                    // Dropping `lock` releases it again.
                    return Ok(Acquire::HeldElsewhere);
                }
                Ok(Acquire::Owned(lock))
            }
            None => Ok(Acquire::HeldElsewhere),
        }
    }
    #[cfg(unix)]
    {
        let dir = portable::data_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no data dir for the lock file",
            )
        })?;
        match try_lock_file(&dir.join("moonpool.lock"))? {
            Some(lock) => Ok(Acquire::Owned(lock)),
            None => Ok(Acquire::HeldElsewhere),
        }
    }
}

/// Take this copy's lock, retrying for `patience` while another process holds it (used after a
/// `--wait-pid` relaunch, where the previous hub may still be releasing it). An error creating
/// the lock at all is returned so the caller can decide to boot without it.
pub fn acquire(patience: Duration) -> std::io::Result<Acquire> {
    let deadline = Instant::now() + patience;
    loop {
        match acquire_once()? {
            Acquire::HeldElsewhere if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(200));
            }
            other => return Ok(other),
        }
    }
}

// ---------------------------------------------------------------------------
// Forwarding a second launch
// ---------------------------------------------------------------------------

/// How long a second launch keeps trying to reach the running hub. The hub takes the lock
/// before it builds any window and binds the channel only once setup runs, so a launch landing
/// in that gap has to wait for it.
const FORWARD_FOR: Duration = Duration::from_secs(20);

/// Hand `args` (argv without the exe path) to this copy's running hub, the way the old
/// single-instance plugin did. Returns whether the hub took it.
///
/// A hub from before the `argv` verb answers "unknown cmd"; then the action is sent as a plain
/// control verb instead (a bare launch becomes `show`), which covers everything a user or script
/// passes on the command line except `--ticket` bookkeeping.
pub fn forward(args: &[String]) -> bool {
    allow_hub_to_take_focus();
    let endpoint = crate::control::default_endpoint();
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let deadline = Instant::now() + FORWARD_FOR;
    loop {
        match crate::mcp::call_at(&endpoint, "argv", &argv, Duration::from_secs(5)) {
            Ok(reply) if reply.get("ok").and_then(|v| v.as_bool()) == Some(true) => return true,
            Ok(reply) => {
                let unknown = reply
                    .get("error")
                    .and_then(|v| v.as_str())
                    .is_some_and(|e| e.starts_with("unknown cmd"));
                if !unknown {
                    return false;
                }
                let positional: Vec<&str> = argv
                    .iter()
                    .copied()
                    .take_while(|a| *a != "--ticket")
                    .collect();
                let (cmd, rest) = match positional.split_first() {
                    Some((c, r)) => (*c, r),
                    None => ("show", &[][..]),
                };
                return crate::mcp::call_at(&endpoint, cmd, rest, Duration::from_secs(55)).is_ok();
            }
            // Sent, then the stream broke: the hub got it (a forwarded `quit` exits before it
            // can reply). Retrying would only wait out the deadline against a gone hub.
            Err(crate::mcp::CallError::Io(_)) => return true,
            // Not answering yet (the hub binds its channel only once setup runs): keep trying.
            Err(_) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(250)),
            Err(_) => return false,
        }
    }
}

/// Windows only lets the foreground process hand focus on. The user's double-click made THIS
/// process foreground, so pass that right to whichever process shows the window (the hub), as
/// the old plugin did; otherwise a "come back" launch would only flash the taskbar button.
fn allow_hub_to_take_focus() {
    #[cfg(windows)]
    unsafe {
        let _ = windows::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow(
            windows::Win32::UI::WindowsAndMessaging::ASFW_ANY,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let p = std::env::temp_dir().join(format!(
            "moonpool-instance-{tag}-{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn same_folder_gives_same_id() {
        let d = scratch("same");
        let a = id_for_dir(&d.join("moonpool-config"));
        let b = id_for_dir(&d.join("moonpool-config"));
        assert_eq!(a, b);
        assert_eq!(a.len(), 8);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        // A non-canonical spelling of the same folder agrees too.
        let sub = d.join("x");
        std::fs::create_dir_all(&sub).unwrap();
        assert_eq!(id_for_dir(&sub.join("..")), id_for_dir(&d));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[cfg(windows)]
    #[test]
    fn id_ignores_case_on_windows() {
        let d = scratch("case");
        let upper = PathBuf::from(d.to_string_lossy().to_uppercase());
        assert_eq!(id_for_dir(&d), id_for_dir(&upper));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn different_folders_give_different_ids() {
        let a = scratch("a");
        let b = scratch("b");
        assert_ne!(id_for_dir(&a), id_for_dir(&b));
        assert_ne!(
            pipe_name_for(Some(&id_for_dir(&a))),
            pipe_name_for(Some(&id_for_dir(&b)))
        );
        let _ = std::fs::remove_dir_all(&a);
        let _ = std::fs::remove_dir_all(&b);
    }

    #[test]
    fn id_is_stable_across_builds() {
        // Pinned value: if this changes, every portable copy's pipe name changes with it and a
        // running copy's MCP client would stop reaching its hub after an update.
        assert_eq!(
            format!("{:08x}", fnv1a32(b"c:\\tools\\moonpool-config")),
            "26aa8587"
        );
        // Published FNV-1a 32 test vectors, so the pinned value above is the real algorithm.
        assert_eq!(fnv1a32(b""), 0x811c_9dc5);
        assert_eq!(fnv1a32(b"a"), 0xe40c_292c);
        assert_eq!(fnv1a32(b"foobar"), 0xbf9c_f968);
    }

    #[test]
    fn installed_keeps_the_bare_names() {
        assert_eq!(pipe_name_for(None), r"\\.\pipe\moonpool");
        assert_eq!(mutex_name_for(None), r"Local\moonpool-instance");
        assert_eq!(suffix_for(None), "");
        assert_eq!(
            pipe_name_for(Some("0badf00d")),
            r"\\.\pipe\moonpool-0badf00d"
        );
        assert_eq!(
            mutex_name_for(Some("0badf00d")),
            r"Local\moonpool-instance-0badf00d"
        );
    }

    #[test]
    fn label_names_the_portable_folder() {
        assert_eq!(label_for(None), "Moonpool");
        assert_eq!(label_for(Some("Work")), "Moonpool (Work)");
        let root = PathBuf::from("/media/usb/Work");
        assert_eq!(
            portable_folder_name(&root.join(".moonpool")).as_deref(),
            Some("Work")
        );
        assert_eq!(portable_folder_name(&root).as_deref(), Some("Work"));
    }

    /// Two different copies can each hold their own lock; a second take of the same one fails
    /// until the first is released. Unique test-only names, never the real hub's.
    #[cfg(windows)]
    #[test]
    fn named_lock_is_per_copy_and_exclusive() {
        let tag = format!("{}-{}", std::process::id(), id_for_dir(&scratch("lock")));
        let a = format!(r"Local\moonpool-test-lock-a-{tag}");
        let b = format!(r"Local\moonpool-test-lock-b-{tag}");
        let la = try_lock_named(&a).unwrap().expect("first take of a");
        let lb = try_lock_named(&b).unwrap().expect("b is independent of a");
        assert!(try_lock_named(&a).unwrap().is_none(), "a is already held");
        drop(la);
        assert!(try_lock_named(&a).unwrap().is_some(), "a is free again");
        drop(lb);
    }

    #[cfg(unix)]
    #[test]
    fn file_lock_is_per_copy_and_exclusive() {
        let d = scratch("flock");
        let la = try_lock_file(&d.join("a.lock"))
            .unwrap()
            .expect("first take");
        let _lb = try_lock_file(&d.join("b.lock"))
            .unwrap()
            .expect("independent");
        // flock is per open file description, so a second open in this process conflicts.
        assert!(try_lock_file(&d.join("a.lock")).unwrap().is_none());
        drop(la);
        assert!(try_lock_file(&d.join("a.lock")).unwrap().is_some());
        let _ = std::fs::remove_dir_all(&d);
    }
}
