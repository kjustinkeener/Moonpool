// Platform-specific bits of the e2e harness, kept in one file so a Linux/macOS port only has
// to fill in this module. Everything here is read-only except `killTree`, which only ever
// takes a PID the harness itself started (or found running from inside its own temp folder).

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

export const isWindows = process.platform === "win32";

/** File name of the app binary inside a copy's `.moonpool` folder. */
export const exeName = isWindows ? "moonpool.exe" : "moonpool";

/**
 * The seeded example `cli` app the harness launches, and how to recognize its process (a
 * descendant of the hub). Matches `resources/apps.example*.json`.
 */
export const cliApp = isWindows
  ? { id: "example-powershell", process: /^powershell/i }
  : { id: "example-shell", process: /^(sh|bash|zsh|dash|fish)$/ };

/** Default build output, relative to the repo root. */
export function defaultExe(repoRoot) {
  return path.join(repoRoot, "src-tauri", "target", "release", exeName);
}

/** FNV-1a 32-bit, the same hash `instance::fnv1a32` uses. */
export function fnv1a32(str) {
  let h = 0x811c9dc5;
  for (const b of Buffer.from(str, "utf8")) {
    h ^= b;
    h = Math.imul(h, 0x01000193) >>> 0;
  }
  return h >>> 0;
}

/**
 * The copy id the app derives for a portable copy whose exe lives in `exeDir`. Mirrors
 * `instance::portable_identity_dir` + `identity_key`: Rust's `canonicalize` on Windows yields a
 * verbatim `\\?\C:\...` path, lowercased; elsewhere the plain realpath.
 */
export function copyIdFor(exeDir) {
  const real = fs.realpathSync.native(exeDir);
  let key = path.join(real, "moonpool-config");
  if (isWindows) key = ("\\\\?\\" + key).toLowerCase();
  return fnv1a32(key).toString(16).padStart(8, "0");
}

/** The control channel endpoint of a portable copy. */
export function endpointFor(exeDir) {
  const id = copyIdFor(exeDir);
  if (isWindows) return `\\\\.\\pipe\\moonpool-${id}`;
  // Linux/macOS: a portable copy keeps its socket in its own data dir (see
  // `control::socket_path_for`); the /tmp fallback for long paths is not modeled here.
  return path.join(exeDir, "moonpool-config", "moonpool.sock");
}

/** The endpoints the harness must never talk to: the owner's installed hub. */
export function forbiddenEndpoints() {
  if (isWindows) return ["\\\\.\\pipe\\moonpool"];
  const xdg = process.env.XDG_RUNTIME_DIR;
  return xdg ? [path.join(xdg, "moonpool.sock")] : [];
}

/**
 * Every process as `{ pid, ppid, name, exe }`. `exe` may be empty when the OS will not say
 * (another user's process, a protected one).
 */
export function listProcesses() {
  if (isWindows) {
    const out = execFileSync(
      "powershell",
      [
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        "Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,Name,ExecutablePath | ConvertTo-Json -Compress",
      ],
      { encoding: "utf8", maxBuffer: 64 * 1024 * 1024, windowsHide: true },
    );
    const rows = JSON.parse(out);
    return (Array.isArray(rows) ? rows : [rows]).map((r) => ({
      pid: r.ProcessId,
      ppid: r.ParentProcessId,
      name: r.Name || "",
      exe: r.ExecutablePath || "",
    }));
  }
  const rows = [];
  for (const d of fs.readdirSync("/proc")) {
    if (!/^\d+$/.test(d)) continue;
    try {
      const stat = fs.readFileSync(`/proc/${d}/stat`, "utf8");
      // comm is parenthesized and may hold spaces; ppid is the 2nd field after it.
      const rest = stat.slice(stat.lastIndexOf(")") + 2).split(" ");
      const name = stat.slice(stat.indexOf("(") + 1, stat.lastIndexOf(")"));
      let exe = "";
      try {
        exe = fs.readlinkSync(`/proc/${d}/exe`);
      } catch {}
      rows.push({ pid: Number(d), ppid: Number(rest[1]), name, exe });
    } catch {}
  }
  return rows;
}

export function isAlive(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (e) {
    return e.code === "EPERM";
  }
}

/** PIDs of every descendant of `pid` (not including it), from one process snapshot. */
export function descendants(pid, procs = listProcesses()) {
  const out = [];
  const queue = [pid];
  while (queue.length) {
    const p = queue.shift();
    for (const c of procs) {
      if (c.ppid === p && c.pid !== p && !out.includes(c.pid)) {
        out.push(c.pid);
        queue.push(c.pid);
      }
    }
  }
  return out;
}

/** Kill `pid` and its tree. Only call this with a PID the harness owns. */
export function killTree(pid) {
  if (!isAlive(pid)) return;
  if (isWindows) {
    try {
      execFileSync("taskkill", ["/PID", String(pid), "/T", "/F"], {
        stdio: "ignore",
        windowsHide: true,
      });
    } catch {}
    return;
  }
  for (const c of descendants(pid).reverse()) {
    try {
      process.kill(c, "SIGKILL");
    } catch {}
  }
  try {
    process.kill(pid, "SIGKILL");
  } catch {}
}

/** Case-folded comparison key for paths on this OS. */
export function pathKey(p) {
  const n = path.resolve(p);
  return isWindows ? n.toLowerCase() : n;
}
