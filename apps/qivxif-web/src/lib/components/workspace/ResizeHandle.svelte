<script lang="ts">
  let { axis, sizes, index, onResize } = $props();

  function start(event: PointerEvent) {
    event.preventDefault();
    const startPos = axis === "column" ? event.clientY : event.clientX;
    const startSizes = [...sizes];

    function move(ev: PointerEvent) {
      const delta = (axis === "column" ? ev.clientY : ev.clientX) - startPos;
      const next = [...startSizes];
      next[index] = Math.round(Math.max(120, startSizes[index] + delta));
      next[index + 1] = Math.round(Math.max(120, startSizes[index + 1] - delta));
      onResize(next);
    }

    function end() {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", end);
    }

    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", end);
  }
</script>

<div
  class="resize-handle {axis === 'column' ? 'row' : 'column'}"
  role="separator"
  onpointerdown={start}
></div>
