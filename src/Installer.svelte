<script lang="ts">
  import {
    performInstall,
    launchInstalledAndExit,
    establishPortableAt,
    quitApp,
  } from "./lib/api";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import wordmark from "./assets/moonpool-wordmark.png";
  import { t } from "./lib/i18n.svelte";

  // On first run the installer IS the main window, so closing means quit the app.
  // Opened on demand from the hub it's a detached window - just close that window
  // and leave the running hub alone.
  const detached = window.location.hash === "#installer";
  function close() {
    if (detached) getCurrentWindow().close().catch(() => {});
    else quitApp();
  }

  let {
    installDir,
    version,
    buildDate,
    installed = false,
  }: {
    installDir: string;
    version: string;
    buildDate: string;
    // True when this copy is already installed (opened from the hub menu). The
    // "Install moonpool" action is then disabled; "Install portable" stays live.
    installed?: boolean;
  } = $props();

  import { onMount } from "svelte";

  let desktop = $state(true);
  let phase = $state<"idle" | "working" | "done" | "portable" | "error">("idle");
  let error = $state("");

  // The window is transparent, but app.css paints body with a solid theme color -
  // that's the dark "outer border" box behind the card. Clear it so only the card
  // (with its own background + shadow) shows, floating on the desktop.
  onMount(() => {
    const app = document.getElementById("app");
    const els = [document.documentElement, document.body, app].filter(
      Boolean,
    ) as HTMLElement[];
    const prev = els.map((e) => e.style.background);
    for (const e of els) e.style.background = "transparent";
    return () => {
      els.forEach((e, i) => (e.style.background = prev[i]));
    };
  });

  // --- Interactive orb: click to recolor the orb + button, fling sparks, and
  // re-randomize the peripheral gradients behind the card. ------------------
  const rand = (a: number, b: number) => a + Math.random() * (b - a);

  type Grad = { x: number; y: number; a: number; s: number };
  type Spark = { id: number; dx: number; dy: number };

  const DEFAULT_HUE = 198; // cyan-ish, ~matches --accent
  const EDGE_DIST = 22; // % beyond the window edge where gradients are centered

  let poked = $state(false); // false = untouched default look
  let orbHue = $state(DEFAULT_HUE);
  let grads = $state<Grad[]>([]);
  let sparks = $state<Spark[]>([]);
  let sparkSeq = 0;

  // Everything derives from this single color once poked.
  let orbColor = $derived(`hsl(${orbHue} 88% 56%)`);

  // Style attrs are empty until poked, so the CSS defaults show through; a
  // right-click reset clears `poked` and restores that default look.
  let cardStyle = $derived(
    poked
      ? "background: " +
          grads
            .map(
              (g) =>
                `radial-gradient(${g.s}% ${Math.round(g.s * 0.8)}% at ${g.x}% ${g.y}%, color-mix(in srgb, ${orbColor} ${g.a}%, transparent), transparent 60%)`,
            )
            .join(", ") +
          ", var(--bg-panel);"
      : "",
  );
  let orbStyle = $derived(
    poked
      ? `background: radial-gradient(circle at 38% 34%, hsl(${orbHue} 100% 92%), hsl(${orbHue} 85% 60%) 42%, hsl(${orbHue} 78% 48%) 78%); box-shadow: 0 0 34px 6px hsl(${orbHue} 85% 55%), inset -4px -6px 12px rgba(6, 40, 70, 0.5);`
      : "",
  );
  let ctaStyle = $derived(
    poked
      ? `background: linear-gradient(180deg, hsl(${orbHue} 92% 64%), hsl(${orbHue} 82% 50%)); box-shadow: 0 8px 22px hsl(${orbHue} 82% 50% / 0.42);`
      : "",
  );
  let ringStyle = $derived(poked ? `border-color: ${orbColor};` : "");
  let boxStyle = $derived(poked ? `accent-color: ${orbColor};` : "");

  // A gradient centered just outside one edge of the window, EDGE_DIST beyond it.
  function edgeGrad(): Grad {
    const side = Math.floor(rand(0, 4));
    const along = Math.round(rand(0, 100));
    const pos =
      side === 0
        ? { x: along, y: -EDGE_DIST }
        : side === 1
          ? { x: 100 + EDGE_DIST, y: along }
          : side === 2
            ? { x: along, y: 100 + EDGE_DIST }
            : { x: -EDGE_DIST, y: along };
    return { ...pos, a: Math.round(rand(22, 46)), s: Math.round(rand(50, 95)) };
  }

  function pokeOrb() {
    poked = true;
    orbHue = Math.round(rand(0, 360));
    grads = Array.from({ length: Math.round(rand(2, 5)) }, edgeGrad);
    const m = Math.round(rand(12, 20));
    const batch: Spark[] = Array.from({ length: m }, () => {
      const ang = rand(0, Math.PI * 2);
      const dist = rand(42, 98);
      return {
        id: ++sparkSeq,
        dx: Math.round(Math.cos(ang) * dist),
        dy: Math.round(Math.sin(ang) * dist),
      };
    });
    sparks = [...sparks, ...batch];
    const ids = new Set(batch.map((s) => s.id));
    setTimeout(() => (sparks = sparks.filter((s) => !ids.has(s.id))), 750);
  }

  // Right-click the orb -> back to the untouched default look.
  function resetOrb(e: MouseEvent) {
    e.preventDefault();
    poked = false;
    orbHue = DEFAULT_HUE;
    grads = [];
    sparks = [];
  }

  async function install() {
    if (phase === "working" || installed) return;
    phase = "working";
    error = "";
    try {
      const exe = await performInstall(desktop);
      phase = "done";
      // Brief beat so the "Installed" state is visible, then hand off.
      setTimeout(() => launchInstalledAndExit(exe).catch(() => {}), 700);
    } catch (e) {
      error = String(e);
      phase = "error";
    }
  }

  // Portable: let the user pick where the portable copy lives, then stamp it there
  // (or in place if they pick this exe's own folder) and relaunch in portable mode.
  // The backend relaunches and exits this process, so we just show a brief beat.
  async function runPortable() {
    if (phase === "working" || phase === "portable") return;
    error = "";
    let folder: string;
    try {
      const picked = await open({
        directory: true,
        title: t("installer.pickFolder"),
      });
      if (typeof picked !== "string") return; // cancelled
      folder = picked;
    } catch (e) {
      error = String(e);
      phase = "error";
      return;
    }
    phase = "portable";
    try {
      await establishPortableAt(folder);
    } catch (e) {
      error = String(e);
      phase = "error";
    }
  }
</script>

<!-- Whole window drags; Tauri auto-excludes interactive controls (button, input). -->
<div class="wrap" data-tauri-drag-region>
  <div
    class="card"
    class:busy={phase === "working"}
    style={cardStyle}
    data-tauri-drag-region
  >
    <button class="close" onclick={close} title={t("common.close")} aria-label={t("common.close")}>✕</button>
    <div class="top">
      <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
      <div
        class="art"
        onclick={pokeOrb}
        oncontextmenu={resetOrb}
        title={t("installer.poke")}
      >
        <div class="orb" style={orbStyle}></div>
        <div class="ripple r1" style={ringStyle}></div>
        <div class="ripple r2" style={ringStyle}></div>
        <div class="ripple r3" style={ringStyle}></div>
        {#each sparks as s (s.id)}
          <span class="spark" style="--dx: {s.dx}px; --dy: {s.dy}px; background: {orbColor}; color: {orbColor};"></span>
        {/each}
      </div>

      <img class="word" src={wordmark} alt="moonpool" />
      <p class="tag">{t("installer.tagline")}</p>
    </div>

    {#if phase === "done"}
      <div class="state ok">
        <span class="check">✓</span>
        {t("installer.installed")}
      </div>
    {:else if phase === "portable"}
      <div class="state ok">
        <span class="check">✓</span>
        {t("installer.portableDone")}
      </div>
    {:else if phase === "error"}
      <div class="state err">{t("installer.failed", { error })}</div>
      <button class="cta" onclick={install}>{t("common.tryAgain")}</button>
    {:else}
      <button
        class="cta"
        style={ctaStyle}
        onclick={install}
        disabled={phase === "working" || installed}
        title={installed ? t("installer.alreadyInstalledTitle") : ""}
      >
        {installed
          ? t("installer.alreadyInstalled")
          : phase === "working"
            ? t("installer.installing")
            : t("installer.install")}
      </button>
      <label class="opt">
        <input
          type="checkbox"
          style={boxStyle}
          bind:checked={desktop}
          disabled={phase === "working" || installed}
        />
        <span>{t("installer.desktopShortcut")}</span>
      </label>
      <button
        class="portable"
        onclick={runPortable}
        disabled={phase === "working"}
        title={t("installer.portableHint")}
      >
        {t("installer.installPortable")}
      </button>
    {/if}

    <div class="foot">
      <div class="fver">v{version} · {buildDate}</div>
      <div class="path" title={installDir}>{t("installer.installPath", { dir: installDir })}</div>
    </div>
  </div>
</div>

<style>
  /* The card IS the window - no surrounding box, so there's no "outer border".
     The window is sized to the card in the backend; the card fills it edge to
     edge and the OS rounds the window corners to match. */
  .wrap {
    height: 100vh;
    width: 100vw;
    background: transparent;
    /* The card is chrome, not a document: a drag-to-move that starts on text would
       otherwise highlight the heading/copy and read as a broken UI. Inputs and
       buttons stay interactive; nothing here needs copying. */
    user-select: none;
    cursor: default;
  }
  .card {
    position: absolute;
    inset: 0;
    padding: 34px 40px 24px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    /* No DOM rounding: the window isn't transparent at the corners on Windows
       (WebView2 quirk), so a rounded card reveals a dark sliver. Fill the window
       square and let the OS round the window frame instead. */
    border-radius: 0;
    text-align: center;
    color: var(--text);
    background:
      radial-gradient(120% 70% at 50% -6%, color-mix(in srgb, var(--accent) 26%, transparent), transparent 60%),
      var(--bg-panel);
    border: 1px solid var(--border);
    overflow: hidden;
  }
  /* The top block (orb + wordmark + tagline) is non-interactive: let clicks fall
     through to the card so this whole area drags the window. */
  .top {
    pointer-events: none;
    width: 100%;
  }
  /* moon-pool motif: a glowing orb with concentric ripples. Clickable (poke),
     so re-enable pointer events that .top turned off. */
  .art {
    position: relative;
    height: 96px;
    margin: 4px auto 14px;
    width: 96px;
    pointer-events: auto;
    cursor: pointer;
  }
  .orb {
    position: absolute;
    inset: 26px;
    border-radius: 50%;
    background: radial-gradient(circle at 38% 34%, #d6f6ff, #35c8ff 42%, #1f8fe0 78%);
    box-shadow:
      0 0 34px 6px color-mix(in srgb, var(--accent) 60%, #35c8ff),
      inset -4px -6px 12px rgba(6, 40, 70, 0.5);
    animation: bob 3.6s ease-in-out infinite;
  }
  .ripple {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    border: 1.5px solid color-mix(in srgb, var(--accent) 55%, #35c8ff);
    opacity: 0;
    animation: ring 3.4s ease-out infinite;
  }
  .r2 { animation-delay: 1.1s; }
  .r3 { animation-delay: 2.2s; }
  @keyframes ring {
    0% { transform: scale(0.42); opacity: 0.55; }
    80% { opacity: 0; }
    100% { transform: scale(1); opacity: 0; }
  }
  @keyframes bob {
    50% { transform: translateY(-4px); }
  }
  .art:active .orb {
    transform: scale(0.9);
    transition: transform 0.1s ease;
  }
  /* Sparks fly from the orb center outward and fade. */
  .spark {
    position: absolute;
    left: 50%;
    top: 50%;
    width: 5px;
    height: 5px;
    border-radius: 50%;
    pointer-events: none;
    box-shadow: 0 0 6px 1px currentColor;
    animation: spark 0.75s ease-out forwards;
  }
  @keyframes spark {
    0% {
      transform: translate(-50%, -50%) scale(1);
      opacity: 1;
    }
    100% {
      transform: translate(
        calc(-50% + var(--dx)),
        calc(-50% + var(--dy))
      ) scale(0.3);
      opacity: 0;
    }
  }

  .word {
    display: block;
    height: 30px;
    width: auto;
    margin: 0 auto 8px;
    object-fit: contain;
    filter: drop-shadow(0 1px 8px color-mix(in srgb, var(--accent) 40%, transparent));
  }
  .tag {
    margin: 0 0 22px;
    color: var(--text-secondary);
    font-size: 13px;
  }

  .cta {
    appearance: none;
    border: none;
    cursor: pointer;
    width: 100%;
    padding: 12px 18px;
    border-radius: 12px;
    font-size: 15px;
    font-weight: 600;
    color: var(--on-accent);
    background: linear-gradient(180deg, color-mix(in srgb, var(--accent) 92%, white), var(--accent));
    box-shadow: 0 8px 22px color-mix(in srgb, var(--accent) 40%, transparent);
    transition: transform 0.08s ease, box-shadow 0.15s ease, filter 0.15s ease;
  }
  .cta:hover:not(:disabled) { filter: brightness(1.06); box-shadow: 0 10px 26px color-mix(in srgb, var(--accent) 52%, transparent); }
  .cta:active:not(:disabled) { transform: translateY(1px); }
  .cta:disabled { cursor: default; opacity: 0.8; }

  .opt {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    margin-top: 14px;
    font-size: 13px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
  }
  .opt input { accent-color: var(--accent); cursor: pointer; }

  /* Secondary, low-emphasis action: a plain text link under the primary CTA. */
  .portable {
    display: block;
    margin: 12px auto 0;
    padding: 2px 4px;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 12px;
    color: var(--text-dim);
    text-decoration: underline;
    text-underline-offset: 2px;
  }
  .portable:hover:not(:disabled) { color: var(--text-secondary); }
  .portable:disabled { cursor: default; opacity: 0.6; }

  .state { margin-top: 6px; font-size: 14px; }
  .state.ok { color: var(--text-strong); }
  .state.ok .check {
    color: #35c8ff;
    font-weight: 700;
    margin-right: 4px;
  }
  .state.err { color: var(--danger, #f06a6a); margin-bottom: 12px; }

  .foot {
    position: absolute;
    left: 0;
    right: 0;
    bottom: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    font-size: 11px;
    color: var(--text-dim);
    padding: 0 16px;
  }
  .fver,
  .path {
    font-size: 11px;
    line-height: 1.45;
  }
  .path {
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .busy .art .orb { animation-duration: 1.1s; }

  /* Close button: the installer window is frameless, so this is the only in-window
     way out. Sits above the drag region (top-right) so the click isn't eaten by it. */
  .close {
    position: absolute;
    top: 8px;
    right: 8px;
    z-index: 2;
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: none;
    border-radius: 7px;
    background: transparent;
    color: var(--text-dim);
    font-size: 13px;
    line-height: 1;
    cursor: pointer;
    transition: background 0.12s ease, color 0.12s ease;
  }
  .close:hover {
    background: color-mix(in srgb, var(--text) 12%, transparent);
    color: var(--text);
  }
</style>
