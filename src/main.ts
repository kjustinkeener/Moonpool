import { mount } from "svelte";
import "./app.css";
import { initTheme } from "./lib/theme";
import App from "./App.svelte";
import SettingsWindow from "./SettingsWindow.svelte";

// Apply the saved theme before mounting so there's no flash of the wrong theme.
initTheme();

// The detached Settings window loads the same bundle at #settings and mounts a
// settings-only root; every other window is the main hub.
const root =
  window.location.hash === "#settings" ? SettingsWindow : App;

const app = mount(root, { target: document.getElementById("app")! });

export default app;
