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
}
