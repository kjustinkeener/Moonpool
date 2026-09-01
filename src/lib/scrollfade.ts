// Fade a pane's scrollbar with the transparency setting, jump it opaque on
// hover, then ease it back over 1s on mouse-out.
//
// WebView2 repaints the ::-webkit-scrollbar-thumb when --scroll-alpha changes
// (so the hover jump works), but it ignores CSS *transitions* on the pseudo-
// element, so the fade has to be stepped from JS. Idle, we leave --scroll-alpha
// unset so it follows the stylesheet default (var(--app-alpha)) and tracks the
// slider live; we only pin an inline value while hovered or animating.
export function scrollFade(node: HTMLElement) {
  let raf = 0;

  const baseAlpha = () =>
    Number(
      getComputedStyle(document.documentElement).getPropertyValue("--app-alpha"),
    ) || 1;

  const cancel = () => {
    if (raf) cancelAnimationFrame(raf);
    raf = 0;
  };

  function enter() {
    cancel();
    node.style.setProperty("--scroll-alpha", "1");
  }

  function leave() {
    cancel();
    const to = baseAlpha();
    const dur = 1000;
    let start = 0;
    const step = (t: number) => {
      if (!start) start = t;
      const p = Math.min(1, (t - start) / dur);
      node.style.setProperty("--scroll-alpha", String(1 + (to - 1) * p));
      if (p < 1) {
        raf = requestAnimationFrame(step);
      } else {
        raf = 0;
        // Back to idle: follow var(--app-alpha) live again.
        node.style.removeProperty("--scroll-alpha");
      }
    };
    raf = requestAnimationFrame(step);
  }

  node.addEventListener("mouseenter", enter);
  node.addEventListener("mouseleave", leave);
  return {
    destroy() {
      cancel();
      node.removeEventListener("mouseenter", enter);
      node.removeEventListener("mouseleave", leave);
    },
  };
}
