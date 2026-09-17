<script lang="ts">
  import { onMount } from "svelte";
  import { onSysStats } from "./api";
  import type { SysStats } from "./types";
  import { t } from "./i18n.svelte";

  let stats = $state<SysStats | null>(null);

  onMount(() => {
    let unlisten: (() => void) | undefined;
    (async () => {
      unlisten = await onSysStats((s) => (stats = s));
    })();
    return () => unlisten?.();
  });

  // Per-core CPU height (%), clamped so an idle core still shows a sliver.
  const coreH = (u: number) => Math.max(2, Math.min(u, 100));

  // Memory bar reveals more of the fixed themed gauge gradient as usage climbs
  // (the gradient always spans the whole track; the fill width clips it).
  const memPct = $derived(stats ? Math.min(1, Math.max(0, stats.memPct)) * 100 : 0);
  const memBgSize = $derived(10000 / Math.max(0.5, memPct));

  function gib(bytes: number): string {
    return String(Math.round(bytes / 1024 ** 3));
  }
</script>

<div class="statusbar" data-tauri-drag-region>
  {#if stats}
    <span class="tag">{t("status.cpu")}</span>
    <div class="cpustrip" title={t("status.cpuTip")}>
      {#each stats.cpus as u, i (i)}
        <span class="core">
          <span
            class="corefill"
            style="height:{coreH(u)}%; background-size:100% {10000 / coreH(u)}%"
          ></span>
        </span>
      {/each}
    </div>
    <span class="tag">{t("status.mem")}</span>
    <div
      class="membar"
      title={t("status.memTip", { used: gib(stats.memUsed), total: gib(stats.memTotal) })}
      data-tauri-drag-region
    >
      <div class="memfill" style:width="{memPct}%" style:background-size="{memBgSize}% 100%"></div>
    </div>
    <span class="memlabel">{gib(stats.memUsed)}/{gib(stats.memTotal)} GB</span>
  {/if}
</div>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 3px 10px;
    background: color-mix(in srgb, var(--bg) calc(var(--app-alpha) * 100%), transparent);
    border-top: 1px solid var(--border-muted);
    flex: 0 0 auto;
  }
  .cpustrip {
    display: flex;
    align-items: flex-end;
    gap: 1px;
    height: 15px;
    flex: 0 0 auto;
  }
  .core {
    width: 4px;
    height: 100%;
    background: var(--bg-inset);
    border-radius: 1px;
    display: flex;
    align-items: flex-end;
    overflow: hidden;
  }
  .corefill {
    width: 100%;
    display: block;
    border-radius: 1px;
    background-color: var(--dot-run);
    background-image: linear-gradient(0deg, var(--focus) 0%, var(--dot-warn) 52%, var(--dot-run) 100%);
    background-repeat: no-repeat;
    background-position: left bottom;
    transition:
      height 0.4s ease,
      background-size 0.4s ease;
  }
  .membar {
    position: relative;
    flex: 1 1 auto;
    height: 12px;
    background: var(--bg-inset);
    border-radius: 2px;
    overflow: hidden;
  }
  .memfill {
    height: 100%;
    border-radius: 2px;
    opacity: 0.85;
    background-color: var(--dot-run);
    background-image: linear-gradient(90deg, var(--focus) 0%, var(--dot-warn) 52%, var(--dot-run) 100%);
    background-repeat: no-repeat;
    background-position: left center;
    transition: width 0.4s ease;
  }
  .memlabel {
    flex: 0 0 auto;
    color: var(--text-dim);
    font-size: 11px;
    font-variant-numeric: tabular-nums;
    text-align: right;
  }
  .tag {
    flex: 0 0 auto;
    color: var(--text-dim);
    opacity: 0.7;
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.06em;
  }
</style>
