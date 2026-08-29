import { mount } from "svelte";
import "./app.css";
import { initTheme } from "./lib/theme";
import App from "./App.svelte";

// Apply the saved theme before mounting so there's no flash of the wrong theme.
initTheme();

const app = mount(App, { target: document.getElementById("app")! });

export default app;
