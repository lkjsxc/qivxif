import { draggedPaneId } from "./tab-drag.ts";

const EDGE_FRACTION = 0.22;

export function installDropLayer(tile, targetPaneId, actions) {
  if (!targetPaneId?.startsWith("nod_")) {
    return;
  }
  tile.addEventListener("dragover", (event) => {
    const sourcePaneId = draggedPaneId(event);
    if (!sourcePaneId) {
      return;
    }
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
    tile.dataset.dropZone = dropZone(tile, event);
  });
  tile.addEventListener("dragleave", (event) => {
    if (!tile.contains(event.relatedTarget)) {
      delete tile.dataset.dropZone;
    }
  });
  tile.addEventListener("drop", (event) => {
    const sourcePaneId = draggedPaneId(event);
    const zone = dropZone(tile, event);
    delete tile.dataset.dropZone;
    if (!sourcePaneId) {
      return;
    }
    event.preventDefault();
    actions.movePane?.(sourcePaneId, targetPaneId, zone);
  });
}

function dropZone(tile, event) {
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
