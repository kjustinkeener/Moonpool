export type AppType = "desktop" | "web" | "static" | "cli";

export interface AppEntry {
  id: string;
  name: string;
  group: string;
  type: AppType;
  cwd?: string;
  command?: string;
  port?: number;
  processName?: string;
  url?: string;
  openBrowser?: boolean;
  // Manifest-schema field (an explicit icon path/URL in apps.json). Not read by
  // the frontend today - icons are resolved at runtime via appIcon() into iconSrc.
  icon?: string;
  note?: string;
  env?: Record<string, string>;
}

export interface AppStatus {
  id: string;
  running: boolean;
  managed: boolean;
  /** Whether a `<processName> mcp` shim is currently attached for this app. */
  mcpRunning: boolean;
  /** Whether an MCP shim has EVER been observed for this app - stays true after it
   *  stops, so the sidebar's MCP sub-row doesn't vanish the moment it exits. */
  mcpSeen: boolean;
}

export interface SysStats {
  /** Per-core CPU usage, 0..100, in core order. */
  cpus: number[];
  memUsed: number;
  memTotal: number;
  /** Used / total, 0..1. */
  memPct: number;
}
