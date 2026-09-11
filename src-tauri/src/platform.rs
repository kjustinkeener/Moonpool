//! OS-specific process/shell helpers. Everything else in Moonpool is portable
//! (Tauri, portable-pty, sysinfo); this module isolates the platform divergence.

use portable_pty::CommandBuilder;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// A PTY command that runs `command` through the platform's shell.
pub fn shell_command(command: &str) -> CommandBuilder {
    #[cfg(windows)]
    {
        // cmd /c handles .bat/.cmd/npm/python/pwsh -File/.vbs uniformly.
        let mut c = CommandBuilder::new("cmd.exe");
        c.arg("/c");
        c.arg(command);
        c
    }
    #[cfg(not(windows))]
    {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
        let mut c = CommandBuilder::new(shell);
        c.arg("-c");
        c.arg(command);
        c
    }
}

/// Suppress a console window for auxiliary `Command` spawns (no-op off Windows).
/// Only the Windows `cfg` branches call this, so off Windows it is dead code;
/// silence that rather than fail the `-D warnings` clippy CI on Linux/macOS.
#[allow(unused_variables, unused_mut)]
#[cfg_attr(not(windows), allow(dead_code))]
pub fn hidden(cmd: &mut Command) -> &mut Command {
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

/// The executable suffix on this platform (".exe" on Windows, "" elsewhere).
pub const fn exe_suffix() -> &'static str {
    if cfg!(windows) {
        ".exe"
    } else {
        ""
    }
}

/// Open `path` in the default *text* editor, reporting whether that succeeded.
/// Linux only: a config file's own MIME (e.g. `application/json`) often has no
/// registered handler, so `xdg-open` falls back to a web browser. Resolve the
/// `text/plain` default (a real editor) and launch it directly. Other platforms
/// return false so the caller uses the opener default, already an editor there.
#[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
pub fn open_text_file(path: &std::path::Path) -> bool {
    #[cfg(target_os = "linux")]
    {
        let Ok(out) = Command::new("xdg-mime")
            .args(["query", "default", "text/plain"])
            .output()
        else {
            return false;
        };
        let desktop = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if desktop.is_empty() {
            return false;
        }
        Command::new("gtk-launch")
            .arg(&desktop)
            .arg(path)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// Force-kill a process tree rooted at `pid`.
pub fn kill_tree(pid: u32) {
    #[cfg(windows)]
    {
        let mut c = Command::new("taskkill");
        c.args(["/PID", &pid.to_string(), "/T", "/F"]);
        hidden(&mut c);
        let _ = c.spawn();
    }
    #[cfg(unix)]
    {
        // The PTY child is a session/group leader, so its pgid == pid: signal the
        // whole group, then the leader as a fallback. The `--` is required: without
        // it, util-linux `/usr/bin/kill` misparses the leading `-<pgid>` and the
        // group kill silently no-ops, orphaning the child tree (verified on Ubuntu
        // 24.04). `--` ends option scanning so `-<pgid>` is read as a process group.
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg("--")
            .arg(format!("-{pid}"))
            .spawn();
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg("--")
            .arg(pid.to_string())
            .spawn();
    }
}

/// Owns a spawned child and its whole descendant tree so it can be force-killed
/// reliably - even after the top process has exited (its PID could be recycled) and
/// even for grandchildren that re-parented away from the top (a snapshot `taskkill /T`
/// misses those). This is what makes Stop dependable for the deep trees genai launch
/// commands spawn (cmd -> npm/uv -> node/python -> model server/docker/workers).
///
/// Windows: a Job Object the child is assigned to at spawn; `kill` calls
/// `TerminateJobObject`, killing by job membership regardless of PID reuse or
/// re-parenting. `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` also reaps the tree if Moonpool
/// itself dies (closing the last job handle kills the members). If the job can't be
/// created or assigned (older OS, or the child sits in a non-nestable job), it degrades
/// to a by-PID `kill_tree`.
///
/// Unix: the PTY child is a process-group leader, so the group is just its PID and
/// `kill` signals the pgid - exactly what `kill_tree` already does.
#[cfg(windows)]
pub struct ProcessGroup {
    job: JobHandle,
    pid: Option<u32>,
}

/// A Job Object handle. Raw HANDLE isn't `Send`, but the job is only ever created,
/// assigned once, and terminated - all thread-safe kernel operations - so it is sound
/// to move a `RunningApp` (which holds one) across threads.
#[cfg(windows)]
struct JobHandle(windows_sys::Win32::Foundation::HANDLE);
#[cfg(windows)]
unsafe impl Send for JobHandle {}
#[cfg(windows)]
unsafe impl Sync for JobHandle {}
#[cfg(windows)]
impl Drop for JobHandle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // With KILL_ON_JOB_CLOSE set, closing the last handle also kills any
            // members still alive - a backstop for a run removed without an explicit
            // kill (e.g. a clean natural exit) and for Moonpool exiting.
            unsafe { windows_sys::Win32::Foundation::CloseHandle(self.0) };
        }
    }
}

#[cfg(windows)]
impl ProcessGroup {
    /// Create an owning job for a freshly spawned child `pid` and assign it. Any step
    /// failing leaves a null job so `kill` falls back to `kill_tree(pid)`.
    pub fn own(pid: Option<u32>) -> Self {
        use windows_sys::Win32::Foundation::{CloseHandle, FALSE};
        use windows_sys::Win32::System::JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
            SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        };
        use windows_sys::Win32::System::Threading::{
            OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE,
        };
        let job = unsafe {
            let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
            if job.is_null() {
                return Self {
                    job: JobHandle(std::ptr::null_mut()),
                    pid,
                };
            }
            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let set_ok = SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const _,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            ) != 0;
            let assigned = set_ok
                && pid
                    .map(|pid| {
                        let h = OpenProcess(PROCESS_TERMINATE | PROCESS_SET_QUOTA, FALSE, pid);
                        if h.is_null() {
                            return false;
                        }
                        let ok = AssignProcessToJobObject(job, h) != 0;
                        CloseHandle(h);
                        ok
                    })
                    .unwrap_or(false);
            if assigned {
                job
            } else {
                CloseHandle(job);
                std::ptr::null_mut()
            }
        };
        Self {
            job: JobHandle(job),
            pid,
        }
    }

    /// Force-kill the whole group. Uses the job when we have one; otherwise a by-PID
    /// tree kill (still correct while the top PID is live, i.e. an explicit Stop).
    pub fn kill(&self) {
        if !self.job.0.is_null() {
            unsafe { windows_sys::Win32::System::JobObjects::TerminateJobObject(self.job.0, 1) };
        } else if let Some(pid) = self.pid {
            kill_tree(pid);
        }
    }
}

#[cfg(not(windows))]
pub struct ProcessGroup {
    pid: Option<u32>,
}
#[cfg(not(windows))]
impl ProcessGroup {
    pub fn own(pid: Option<u32>) -> Self {
        Self { pid }
    }
    pub fn kill(&self) {
        if let Some(pid) = self.pid {
            kill_tree(pid);
        }
    }
}

/// Force-kill every process matching `name` (its base exe name).
pub fn kill_by_name(name: &str) {
    #[cfg(windows)]
    {
        let image = if name.to_lowercase().ends_with(".exe") {
            name.to_string()
        } else {
            format!("{name}.exe")
        };
        let mut c = Command::new("taskkill");
        c.args(["/IM", &image, "/T", "/F"]);
        hidden(&mut c);
        let _ = c.spawn();
    }
    #[cfg(unix)]
    {
        let _ = Command::new("pkill").args(["-KILL", "-x", name]).spawn();
    }
}

/// Free a TCP port by killing whatever is listening on it.
pub fn free_port(port: u16) {
    #[cfg(windows)]
    {
        let script = format!(
            "$l = Get-NetTCPConnection -LocalPort {port} -State Listen -ErrorAction SilentlyContinue; \
             foreach ($p in ($l.OwningProcess | Sort-Object -Unique)) {{ taskkill /PID $p /T /F 2>$null | Out-Null }}"
        );
        // Windows PowerShell 5.1 (always present), not the Store `pwsh`.
        let mut c = Command::new("powershell");
        c.args(["-NoProfile", "-Command", &script]);
        hidden(&mut c);
        let _ = c.spawn();
    }
    #[cfg(unix)]
    {
        let script = format!(
            "lsof -ti tcp:{port} | xargs -r kill -9 2>/dev/null || fuser -k {port}/tcp 2>/dev/null"
        );
        let _ = Command::new("sh").args(["-c", &script]).spawn();
    }
}

/// Extract `exe`'s icon to a PNG at `out`. Windows-only; returns false elsewhere,
/// where Moonpool falls back to project-folder discovery / favicons.
#[cfg(windows)]
pub fn extract_exe_icon(exe: &std::path::Path, out: &std::path::Path) -> bool {
    if let Some(parent) = out.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // Escape single quotes for the single-quoted PowerShell string literals.
    let exe_ps = exe.to_string_lossy().replace('\'', "''");
    let out_ps = out.to_string_lossy().replace('\'', "''");
    let script = format!(
        "Add-Type -AssemblyName System.Drawing; try {{ \
         $i=[System.Drawing.Icon]::ExtractAssociatedIcon('{exe_ps}'); \
         $b=$i.ToBitmap(); \
         $b.Save('{out_ps}',[System.Drawing.Imaging.ImageFormat]::Png) }} catch {{ exit 1 }}"
    );
    let mut cmd = Command::new("powershell");
    cmd.args(["-NoProfile", "-Command", &script]);
    hidden(&mut cmd);
    cmd.status().map(|s| s.success()).unwrap_or(false) && out.is_file()
}

#[cfg(not(windows))]
pub fn extract_exe_icon(_exe: &std::path::Path, _out: &std::path::Path) -> bool {
    false
}
