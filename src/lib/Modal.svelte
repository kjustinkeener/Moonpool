<script lang="ts">
  import { onMount, onDestroy, type Snippet } from "svelte";

  let {
    onClose,
    width = "560px",
    children,
  }: {
    onClose: () => void;
    width?: string;
    children: Snippet;
  } = $props();

  let dialog = $state<HTMLDivElement | null>(null);
  let previouslyFocused: HTMLElement | null = null;

  // Overlay click-to-close, guarded so a drag that begins inside the modal and
  // ends on the overlay (e.g. selecting text into the backdrop) does not close it.
  let downOnOverlay = false;
  function onOverlayDown(e: MouseEvent) {
    downOnOverlay = e.target === e.currentTarget;
  }
  function onOverlayClick(e: MouseEvent) {
    if (downOnOverlay && e.target === e.currentTarget) onClose();
  }

  // All tabbable elements currently inside the dialog, in DOM order.
  function focusable(): HTMLElement[] {
    if (!dialog) return [];
    return Array.from(
      dialog.querySelectorAll<HTMLElement>(
        'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
      ),
    );
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      onClose();
      return;
    }
    if (e.key !== "Tab") return;
    // Trap Tab within the dialog.
    const items = focusable();
    if (items.length === 0) {
      e.preventDefault();
      dialog?.focus();
      return;
    }
    const first = items[0];
    const last = items[items.length - 1];
    const active = document.activeElement as HTMLElement | null;
    const inside = !!active && !!dialog && dialog.contains(active);
    if (e.shiftKey) {
      if (!inside || active === first) {
        e.preventDefault();
        last.focus();
      }
    } else {
      if (!inside || active === last) {
        e.preventDefault();
        first.focus();
      }
    }
  }

  onMount(() => {
    previouslyFocused = document.activeElement as HTMLElement | null;
    // Move focus into the dialog: first focusable element, else the dialog itself.
    (focusable()[0] ?? dialog)?.focus();
  });

  onDestroy(() => {
    // Restore focus to whatever was focused before the modal opened.
    previouslyFocused?.focus?.();
  });
</script>

<svelte:window onkeydown={onKey} />

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
<div class="overlay" onmousedown={onOverlayDown} onclick={onOverlayClick} role="presentation">
  <div
    class="modal"
    bind:this={dialog}
    style:width
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    {@render children()}
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: var(--overlay);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }
  .modal {
    max-width: 92vw;
    max-height: 90vh;
    overflow-y: auto;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px 22px;
    color: var(--text);
    box-shadow: 0 12px 40px var(--shadow);
  }
</style>
