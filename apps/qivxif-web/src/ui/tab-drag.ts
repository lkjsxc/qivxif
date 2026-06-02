import { tabInsertSide } from "../domain/drop-resolver.ts";
import { measurePaneRects, resolvePaneDrop } from "../domain/drop-resolver.ts";

const TAB_MIME = "application/x-qivxif-pane";
const LONG_PRESS_MS = 250;
const MOVE_TOLERANCE = 6;
const COARSE_CANCEL = 8;

export function markDraggableTab(button, paneId, actions) {
  if (!paneId?.startsWith("nod_")) {
    return;
  }
  button.className = "tab-frame tab";
  button.dataset.paneId = paneId;
  button.setAttribute("role", "tab");
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
  button.dataset.dropSide = tabInsertSide(button, event.clientX);
}

function tabDrop(button, targetPaneId, actions, event) {
  const sourcePaneId = draggedPaneId(event);
  const side = tabInsertSide(button, event.clientX);
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
      /* headless tests may not support capture */
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
    if (!armed && movedTooFar(origin, event, COARSE_CANCEL)) {
      cleanup();
      return;
    }
    if (!armed) {
      return;
    }
    event.preventDefault();
    markPointerTarget(event, paneId);
  };
  const end = (event) => {
    const target = armed ? pointerTarget(event, paneId) : null;
    cleanup();
    if (target) {
      event.preventDefault();
      actions.movePane?.(paneId, target.paneId, target.zone);
    }
  };
  return { cancel: cleanup, end, move };
}

function pointerTarget(event, sourcePaneId) {
  const element = document.elementFromPoint(event.clientX, event.clientY);
  const tab = element?.closest?.(".tab-frame[data-pane-id]");
  if (tab) {
    return { paneId: tab.dataset.paneId, zone: `tab-${tabInsertSide(tab, event.clientX)}` };
  }
  const pane = element?.closest?.("article.pane[data-pane-id]");
  if (!pane) {
    return null;
  }
  const head = pane.querySelector(".pane-head");
  const body = pane.querySelector(".pane-stack");
  if (!head || !body) {
    return null;
  }
  const rects = measurePaneRects(pane, head, body);
  const result = resolvePaneDrop({
    clientX: event.clientX,
    clientY: event.clientY,
    inSourceStrip: event.clientY <= rects.stripBottom,
    rects,
    targetPane: pane,
    targetPaneId: pane.dataset.paneId,
  });
  if (result.kind === "rail") {
    return { paneId: result.targetPaneId, zone: `tab-${result.insertSide}` };
  }
  return { paneId: result.targetPaneId, zone: result.zone };
}

function markPointerTarget(event, sourcePaneId) {
  clearDropMarks();
  const target = pointerTarget(event, sourcePaneId);
  if (!target) {
    return;
  }
  if (target.zone.startsWith("tab-")) {
    const tab = document.querySelector(`.tab-frame[data-pane-id='${target.paneId}']`);
    if (tab) {
      tab.dataset.dropSide = target.zone.replace("tab-", "");
    }
    return;
  }
  const pane = document.querySelector(`article.pane[data-pane-id='${target.paneId}']`);
  const body = pane?.querySelector(".pane-stack");
  if (body) {
    body.dataset.dropZone = target.zone;
  }
}

function clearDropMarks() {
  document.querySelectorAll("[data-drop-side]").forEach((item) => delete item.dataset.dropSide);
  document.querySelectorAll("[data-drop-zone]").forEach((item) => delete item.dataset.dropZone);
}

function movedTooFar(origin, event, tolerance) {
  return Math.hypot(event.clientX - origin.x, event.clientY - origin.y) > tolerance;
}

function isCoarsePointer(event) {
  return event.pointerType === "touch" || event.pointerType === "pen";
}
