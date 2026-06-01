const TAB_MIME = "application/x-qivxif-pane";

export function markDraggableTab(button, paneId) {
  if (!paneId?.startsWith("nod_")) {
    return;
  }
  button.draggable = true;
  button.addEventListener("dragstart", (event) => {
    event.dataTransfer?.setData(TAB_MIME, paneId);
    event.dataTransfer?.setData("text/plain", paneId);
    event.dataTransfer.effectAllowed = "move";
  });
}

export function draggedPaneId(event) {
  return event.dataTransfer?.getData(TAB_MIME) || event.dataTransfer?.getData("text/plain") || "";
}
