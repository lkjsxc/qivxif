const EDGE_FRACTION = 0.22;

export function tileDropZone(tile, event) {
  const box = tile.getBoundingClientRect();
  const edgeX = Math.min(80, Math.max(32, box.width * EDGE_FRACTION));
  const edgeY = Math.min(80, Math.max(32, box.height * EDGE_FRACTION));
  if (event.clientY - box.top < edgeY) {
    return "top";
  }
  if (box.bottom - event.clientY < edgeY) {
    return "bottom";
  }
  if (event.clientX - box.left < edgeX) {
    return "left";
  }
  if (box.right - event.clientX < edgeX) {
    return "right";
  }
  return "center";
}

export function tabInsertSide(tab, event) {
  const box = tab.getBoundingClientRect();
  return event.clientX < box.left + box.width / 2 ? "before" : "after";
}
