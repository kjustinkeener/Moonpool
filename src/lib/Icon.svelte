<script lang="ts" module>
  import { SHARED_PATHS, type SharedIconName } from "./icons";

  /**
   * Drawings this app needs that the shared set does not carry, because
   * only this app uses them. They go here, not in `icons.ts`: that file
   * is generated and a rebuild would drop them, and a set every app
   * copies should not carry one app's private shapes. Same 24 grid and
   * same nominal stroke, or they will not sit right beside the rest.
   * `satisfies` rather than an annotation, so the names stay in the
   * union instead of widening to string.
   */
  const LOCAL = {
    // MogStudio, src/lib/components/Icon.svelte
    "arrow-up": `<path d="M12 19V5" /><path d="m5 12 7-7 7 7" />`,
    // MogStudio, src/lib/components/Icon.svelte
    check: `<path d="M20 6 9 17l-5-5" />`,
    // MoonPool's own, from the set this file replaced.
    monitor: `<path d="M4 3h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z" /><path d="M8 21h8" /><path d="M12 17v4" />`,
    // MoonPool's own, from the set this file replaced.
    pencil: `<path d="M12 20h9" /><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />`,
    // Lucide "circle-help", for the offline help window entry.
    help: `<circle cx="12" cy="12" r="10" /><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" /><path d="M12 17h.01" />`,
    // Lucide "text-cursor-input", for the row menu's Rename.
    rename: `<path d="M5 4h1a3 3 0 0 1 3 3 3 3 0 0 1 3-3h1" /><path d="M13 20h-1a3 3 0 0 1-3-3 3 3 0 0 1-3 3H5" /><path d="M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1" /><path d="M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7" /><path d="M9 7v10" />`,
    // Lucide "image", for the row menu's Set icon.
    image: `<rect width="18" height="18" x="3" y="3" rx="2" ry="2" /><circle cx="9" cy="9" r="2" /><path d="m21 15-3.09-3.09a2 2 0 0 0-2.82 0L6 21" />`,
  } satisfies Record<string, string>;

  export type IconName = SharedIconName | keyof typeof LOCAL;

  const PATHS: Record<IconName, string> = { ...SHARED_PATHS, ...LOCAL };
</script>

<script lang="ts">
  interface Props {
    name: IconName;
    /** Rendered size in px. The grid does not change with it. */
    size?: number;
    /** Grid units, so the weight holds as the size changes. */
    width?: number;
  }
  let { name, size = 16, width = 2 }: Props = $props();
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
  {@html PATHS[name]}
</svg>

<style>
  /* Out of the line box on purpose. A glyph inherits font-size,
     line-height and letter-spacing, and the whitespace between it and its
     label collapses to a space no CSS can remove. A block-level svg in a
     flex row has none of those problems. */
  svg {
    display: block;
    flex: none;
  }
</style>
