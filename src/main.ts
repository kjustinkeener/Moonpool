import { mount } from "svelte";
import "./app.css";
import { initTheme } from "./lib/theme";
import App from "./App.svelte";
import SettingsWindow from "./SettingsWindow.svelte";
import AboutWindow from "./AboutWindow.svelte";
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

  // The detached About window loads the same bundle at #about.
  if (window.location.hash === "#about") {
    return mount(AboutWindow, { target });
  }

  // The installer opened on demand from the hub menu (both modes). Same card as
  // first run, but `installed` disables the "Install moonpool" action when this
  // copy is already installed (the portable option stays enabled).
  if (window.location.hash === "#installer") {
    const s = await setupState();
    return mount(Installer, {
      target,
      props: {
        installDir: s.installDir,
        version: s.version,
        buildDate: s.buildDate,
        installed: s.installed,
      },
    });
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
