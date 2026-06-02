import { draggedPaneId } from "./tab-drag.ts";
import {
  applyDropPreview,
  clearDropPreview,
  dropZoneToMove,
  measurePaneRects,
  resolvePaneDrop,
} from "../domain/drop-resolver.ts";

export function installDropLayer(pane, targetPaneId, actions, head, body) {
  if (!targetPaneId?.startsWith("nod_")) {
    return;
  }
  let sourcePaneId = "";
  pane.addEventListener("dragover", (event) => {
    sourcePaneId = draggedPaneId(event);
    if (!sourcePaneId) {
      return;
    }
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
    const rects = measurePaneRects(pane, head, body);
    const result = resolvePaneDrop({
      clientX: event.clientX,
      clientY: event.clientY,
      inSourceStrip: stripPriority(sourcePaneId, event.clientY, rects.stripBottom),
      rects,
      targetPane: pane,
      targetPaneId,
    });
    applyDropPreview(pane, body, rects, result.kind === "pane" ? result.zone : "center");
  });
  pane.addEventListener("dragleave", (event) => {
    if (!pane.contains(event.relatedTarget)) {
      clearDropPreview(pane, body);
    }
  });
  pane.addEventListener("drop", (event) => {
    sourcePaneId = draggedPaneId(event);
    const rects = measurePaneRects(pane, head, body);
    const result = resolvePaneDrop({
      clientX: event.clientX,
      clientY: event.clientY,
      inSourceStrip: stripPriority(sourcePaneId, event.clientY, rects.stripBottom),
      rects,
      targetPane: pane,
      targetPaneId,
    });
    clearDropPreview(pane, body);
    if (!sourcePaneId) {
      return;
    }
    event.preventDefault();
    if (result.kind === "rail") {
      actions.movePane?.(sourcePaneId, result.targetPaneId, `tab-${result.insertSide}`);
      return;
    }
    actions.movePane?.(sourcePaneId, targetPaneId, dropZoneToMove(result.zone));
  });
}

function stripPriority(sourcePaneId, clientY, stripBottom) {
  if (!sourcePaneId) {
    return false;
  }
  const sourceStrip = document.querySelector(`[data-pane-id="${sourcePaneId}"] .tab-strip`);
  if (!sourceStrip) {
    return false;
  }
  const stripRect = sourceStrip.getBoundingClientRect();
  return clientY <= Math.max(stripRect.bottom, stripBottom);
}
