export function installResizeHandle(split, index, sizes, onResize) {
  const handle = document.createElement("div");
  handle.className = `resize-handle ${split.axis === "column" ? "row" : "column"}`;
  let pending = null;
  handle.addEventListener("pointerdown", (event) => {
    event.preventDefault();
    const start = split.axis === "column" ? event.clientY : event.clientX;
    const startSizes = [...sizes];
    const move = (moveEvent) => {
      const delta =
        split.axis === "column" ? moveEvent.clientY - start : moveEvent.clientX - start;
      const next = [...startSizes];
      next[index] = Math.max(180, startSizes[index] + delta);
      next[index + 1] = Math.max(180, startSizes[index + 1] - delta);
      pending = next;
    };
    const end = () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", end);
      if (pending) {
        onResize?.(pending);
        pending = null;
      }
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", end, { once: true });
  });
  return handle;
}
