import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { AppEntry, AppStatus } from "./types";

export const getApps = () => invoke<AppEntry[]>("get_apps");

export const reloadManifest = () => invoke<AppEntry[]>("reload_manifest");

export const openManifest = () => invoke<void>("open_manifest");

export const manifestDir = () => invoke<string>("manifest_dir");

export const saveManifest = (entries: AppEntry[]) =>
  invoke<void>("save_manifest", { entries });

export const appIcon = (id: string, refresh = false) =>
  invoke<string | null>("app_icon", { id, refresh });

export interface Settings {
  debugLogging: boolean;
  closeToTray: boolean;
  minimizeToTray: boolean;
  checkOnStartup: boolean;
}
export const getSettings = () => invoke<Settings>("get_settings");
export const setDebugLogging = (enabled: boolean) =>
  invoke<void>("set_debug_logging", { enabled });
export const setCloseToTray = (enabled: boolean) =>
  invoke<void>("set_close_to_tray", { enabled });
export const setMinimizeToTray = (enabled: boolean) =>
  invoke<void>("set_minimize_to_tray", { enabled });
export const setCheckOnStartup = (enabled: boolean) =>
  invoke<void>("set_check_on_startup", { enabled });
export const openLog = () => invoke<void>("open_log");

export const launchApp = (id: string, cols: number, rows: number) =>
  invoke<void>("launch_app", { id, cols, rows });

export const stopApp = (id: string) => invoke<void>("stop_app", { id });

export const termInput = (id: string, data: string) =>
  invoke<void>("term_input", { id, data });

export const termResize = (id: string, cols: number, rows: number) =>
  invoke<void>("term_resize", { id, cols, rows });

export const openUrl = (url: string) => invoke<void>("open_url", { url });

export const onStatus = (cb: (s: AppStatus[]) => void): Promise<UnlistenFn> =>
  listen<AppStatus[]>("status://update", (e) => cb(e.payload));

export interface TermOutput {
  id: string;
  data: string; // base64-encoded PTY bytes
}

export const onTermOutput = (
  cb: (o: TermOutput) => void,
): Promise<UnlistenFn> =>
  listen<TermOutput>("term://output", (e) => cb(e.payload));

export const onTermExit = (cb: (id: string) => void): Promise<UnlistenFn> =>
  listen<string>("term://exit", (e) => cb(e.payload));

// External control channel: commands forwarded from a second `moonpool.exe` run
// (single-instance) that the UI executes as if the user had clicked.
export interface ControlCommand {
  action: "launch" | "stop" | "restart" | "reload" | "refresh-icons";
  arg: string | null;
  /** Caller-supplied correlation key; when set, the outcome is reported back. */
  ticket: string | null;
}

export const onControl = (
  cb: (c: ControlCommand) => void,
): Promise<UnlistenFn> =>
  listen<ControlCommand>("control://command", (e) => cb(e.payload));

// Report a ticketed command's final outcome so it lands in state.json for the
// caller to read back. Fire-and-forget; failures here are non-fatal.
export const reportOutcome = (
  ticket: string,
  action: string,
  arg: string | null,
  status: "ok" | "error",
  detail: string | null,
): Promise<void> =>
  invoke("report_outcome", { ticket, action, arg, status, detail });
