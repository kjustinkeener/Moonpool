import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// Tauri drives `npm run dev` and expects the dev server on a fixed port.
// 1460 is MoonPool's fixed dev port (see the local dev port registry).
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    port: 1460,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1461 } : undefined,
    watch: {
      ignored: ["**/src-tauri/**", "**/target/**"],
    },
  },
  build: {
    target: "esnext",
    minify: !process.env.TAURI_DEBUG,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
