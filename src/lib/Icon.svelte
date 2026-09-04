<script lang="ts" module>
  /**
   * The app's line-icon set. One 24x24 grid, stroked in `currentColor`, never
   * filled, so an icon inherits the color of whatever it sits in (a muted menu
   * row, an accent hover, a disabled control) with no per-site overrides.
   *
   * This replaced a row of Unicode glyphs. Glyphs are free but they are not a
   * set: each one comes from whichever font on the machine happens to carry it,
   * so weights and optical sizes never match, some render in color on Windows
   * while their neighbours are monochrome, and a few are simply missing and
   * arrive as a box. Drawing them costs a path string and removes the font off
   * the critical path entirely.
   *
   * Adding one: keep the 24x24 viewBox and the 2px stroke of the existing paths
   * so a new icon sits at the same visual weight, and prefer copying the shape
   * from a coherent set (these follow Lucide's geometry) over drawing freehand.
   */
  export type IconName =
    | "plus"
    | "pencil"
    | "refresh"
    | "sliders"
    | "info"
    | "monitor";

  const PATHS: Record<IconName, string[]> = {
    plus: ["M5 12h14", "M12 5v14"],
    pencil: ["M12 20h9", "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"],
    refresh: ["M21 12a9 9 0 1 1-2.64-6.36L21 8", "M21 3v5h-5"],
    // Faders, not a gear: settings here are things you slide and toggle, and
    // the gear reads as "machinery" in a menu that also offers a JSON editor.
    sliders: [
      "M21 4h-7",
      "M10 4H3",
      "M21 12h-9",
      "M8 12H3",
      "M21 20h-5",
      "M12 20H3",
      "M14 2v4",
      "M8 10v4",
      "M16 18v4",
    ],
    info: ["M22 12a10 10 0 1 1-20 0 10 10 0 1 1 20 0", "M12 16v-4", "M12 8h.01"],
    monitor: [
      "M4 3h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z",
      "M8 21h8",
      "M12 17v4",
    ],
  };
</script>

<script lang="ts">
  interface Props {
    name: IconName;
    /** Rendered size in px. The grid is always 24; this only scales it. */
    size?: number;
    /** Stroke weight on the 24-grid, so it scales with `size` like the shape. */
    width?: number;
  }

  let { name, size = 15, width = 2 }: Props = $props();
</script>

<svg
  viewBox="0 0 24 24"
  width={size}
  height={size}
  fill="none"
  stroke="currentColor"
  stroke-width={width}
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {#each PATHS[name] as d}
    <path {d} />
  {/each}
</svg>
