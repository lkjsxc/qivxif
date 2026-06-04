<script lang="ts">
  let { paneId, context, actions, maximized = false } = $props();
  let open = $state(false);
  let rootEl: HTMLElement | undefined;
  let triggerEl: HTMLElement | undefined;
  let menuStyle = $state("");

  function run(action: () => void) {
    open = false;
    action();
  }

  function place() {
    const rect = triggerEl?.getBoundingClientRect();
    if (!rect) return;
    const top = Math.min(window.innerHeight - 8, rect.bottom + 4);
    const right = Math.max(8, window.innerWidth - rect.right);
    menuStyle = `top:${top}px;right:${right}px`;
  }

  $effect(() => {
    if (!open) return;
    requestAnimationFrame(place);
    const onPointer = (event: PointerEvent) => {
      if (rootEl && !rootEl.contains(event.target as Node)) open = false;
    };
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") open = false;
    };
    window.addEventListener("pointerdown", onPointer);
    window.addEventListener("keydown", onKey);
    window.addEventListener("resize", place);
    return () => {
      window.removeEventListener("pointerdown", onPointer);
      window.removeEventListener("keydown", onKey);
      window.removeEventListener("resize", place);
    };
  });
</script>

<div class="tile-menu-wrap" bind:this={rootEl}>
  <button
    bind:this={triggerEl}
    type="button"
    class="icon-button tile-menu-trigger"
    aria-label="Tile menu"
    aria-haspopup="menu"
    aria-expanded={open}
    onclick={() => (open = !open)}
  >
    ⋯
  </button>
  {#if open}
    <div class="tile-menu-popover" role="menu" style={menuStyle}>
      <button type="button" role="menuitem" onclick={() => run(() => actions.splitPane?.(paneId, { ...context, direction: "right" }))}>
        Split right
      </button>
      <button type="button" role="menuitem" onclick={() => run(() => actions.splitPane?.(paneId, { ...context, direction: "bottom" }))}>
        Split down
      </button>
      <button type="button" role="menuitem" onclick={() => run(() => actions.stackTab?.(paneId, context))}>
        Stack tab
      </button>
      <button type="button" role="menuitem" onclick={() => run(() => actions.maximizePane?.(paneId))}>
        {maximized ? "Restore pane" : "Maximize pane"}
      </button>
      <button type="button" role="menuitem" onclick={() => run(() => actions.closePane?.(paneId))}>
        Close tile
      </button>
    </div>
  {/if}
</div>
