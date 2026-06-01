import { tabInsertSide, tileDropZone } from "./drop-resolver.ts";

const TAB_MIME = "application/x-qivxif-pane";
const LONG_PRESS_MS = 420;
const MOVE_TOLERANCE = 10;

export function markDraggableTab(button, paneId, actions) {
  if (!paneId?.startsWith("nod_")) {
    return;
  }
  button.draggable = true;
  button.addEventListener("dragstart", (event) => {
    event.dataTransfer?.setData(TAB_MIME, paneId);
    event.dataTransfer?.setData("text/plain", paneId);
    event.dataTransfer.effectAllowed = "move";
  });
  button.addEventListener("dragover", (event) => tabDragOver(button, event));
  button.addEventListener("dragleave", (event) => clearTabSide(button, event));
  button.addEventListener("drop", (event) => tabDrop(button, paneId, actions, event));
  button.addEventListener("pointerdown", (event) => armPointerDrag(button, paneId, actions, event));
}

export function draggedPaneId(event) {
  return event.dataTransfer?.getData(TAB_MIME) || event.dataTransfer?.getData("text/plain") || "";
}

function tabDragOver(button, event) {
  if (!draggedPaneId(event)) {
    return;
  }
  event.preventDefault();
  event.stopPropagation();
  event.dataTransfer.dropEffect = "move";
  button.dataset.dropSide = tabInsertSide(button, event);
}

function tabDrop(button, targetPaneId, actions, event) {
  const sourcePaneId = draggedPaneId(event);
  const side = tabInsertSide(button, event);
  delete button.dataset.dropSide;
  if (!sourcePaneId) {
    return;
  }
  event.preventDefault();
  event.stopPropagation();
  actions.movePane?.(sourcePaneId, targetPaneId, `tab-${side}`);
}

function clearTabSide(button, event) {
  if (!button.contains(event.relatedTarget)) {
    delete button.dataset.dropSide;
  }
}

function armPointerDrag(button, paneId, actions, event) {
  if (!isCoarsePointer(event) || event.button !== 0) {
    return;
  }
  const drag = pointerDrag(button, paneId, actions, event);
  button.addEventListener("pointermove", drag.move);
  button.addEventListener("pointerup", drag.end, { once: true });
  button.addEventListener("pointercancel", drag.cancel, { once: true });
}

function pointerDrag(button, paneId, actions, start) {
  let armed = false;
  const origin = { x: start.clientX, y: start.clientY };
  const timer = setTimeout(() => {
    armed = true;
    document.body.classList.add("tab-drag-armed");
    button.dataset.dragArmed = "true";
    try {
      button.setPointerCapture(start.pointerId);
    } catch (error) {
      /* synthetic tests do not create capturable browser pointers */
    }
  }, LONG_PRESS_MS);
  const cleanup = () => {
    clearTimeout(timer);
    document.body.classList.remove("tab-drag-armed");
    delete button.dataset.dragArmed;
    clearDropMarks();
    button.removeEventListener("pointermove", move);
  };
  const move = (event) => {
    if (!armed && movedTooFar(origin, event)) {
      cleanup();
      return;
    }
    if (!armed) {
      return;
    }
    event.preventDefault();
    markPointerTarget(event);
  };
  const end = (event) => {
    const target = armed ? pointerTarget(event) : null;
    cleanup();
    if (target) {
      event.preventDefault();
      actions.movePane?.(paneId, target.paneId, target.zone);
    }
  };
  return { cancel: cleanup, end, move };
}

function pointerTarget(event) {
  const element = document.elementFromPoint(event.clientX, event.clientY);
  const tab = element?.closest?.("[role='tab'][data-pane-id]");
  if (tab) {
    return { paneId: tab.dataset.paneId, zone: `tab-${tabInsertSide(tab, event)}` };
  }
  const tile = element?.closest?.("article.tile[data-pane-id]");
  return tile ? { paneId: tile.dataset.paneId, zone: tileDropZone(tile, event) } : null;
}

function markPointerTarget(event) {
  clearDropMarks();
  const target = pointerTarget(event);
  if (!target) {
    return;
  }
  const selector =
    target.zone.startsWith("tab-") ? `[role='tab'][data-pane-id='${target.paneId}']` : "";
  const element = selector ? document.querySelector(selector) : null;
  if (element) {
    element.dataset.dropSide = target.zone.replace("tab-", "");
    return;
  }
  const tile = document.querySelector(`article.tile[data-pane-id='${target.paneId}']`);
  if (tile) {
    tile.dataset.dropZone = target.zone;
  }
}

function clearDropMarks() {
  document.querySelectorAll("[data-drop-side]").forEach((item) => delete item.dataset.dropSide);
  document.querySelectorAll("article.tile[data-drop-zone]").forEach((item) => delete item.dataset.dropZone);
}

function movedTooFar(origin, event) {
  return Math.hypot(event.clientX - origin.x, event.clientY - origin.y) > MOVE_TOLERANCE;
}

function isCoarsePointer(event) {
  return event.pointerType === "touch" || event.pointerType === "pen";
}
