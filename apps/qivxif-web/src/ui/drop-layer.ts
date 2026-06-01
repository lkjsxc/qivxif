import { draggedPaneId } from "./tab-drag.ts";
import { tileDropZone } from "./drop-resolver.ts";

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
    tile.dataset.dropZone = tileDropZone(tile, event);
  });
  tile.addEventListener("dragleave", (event) => {
    if (!tile.contains(event.relatedTarget)) {
      delete tile.dataset.dropZone;
    }
  });
  tile.addEventListener("drop", (event) => {
    const sourcePaneId = draggedPaneId(event);
    const zone = tileDropZone(tile, event);
    delete tile.dataset.dropZone;
    if (!sourcePaneId) {
      return;
    }
    event.preventDefault();
    actions.movePane?.(sourcePaneId, targetPaneId, zone);
  });
}
