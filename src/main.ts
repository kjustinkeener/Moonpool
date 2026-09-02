import { mount } from "svelte";
import "./app.css";
import { initTheme } from "./lib/theme";
import App from "./App.svelte";
import SettingsWindow from "./SettingsWindow.svelte";
import Installer from "./Installer.svelte";
import { setupState } from "./lib/api";

// Apply the saved theme before mounting so there's no flash of the wrong theme.
initTheme();

const target = document.getElementById("app")!;

async function boot() {
  // The detached Settings window loads the same bundle at #settings.
  if (window.location.hash === "#settings") {
    return mount(SettingsWindow, { target });
  }

  // First-run install mode: the portable exe is running from outside its install
  // dir (release builds only). Show the skinned install card instead of the hub.
  try {
    const s = await setupState();
    if (s.needsSetup) {
      return mount(Installer, {
        target,
        props: {
          installDir: s.installDir,
          version: s.version,
          buildDate: s.buildDate,
        },
      });
    }
  } catch {
    // If the check fails, fall through to the hub rather than block startup.
  }

  return mount(App, { target });
}

const app = boot();

export default app;
