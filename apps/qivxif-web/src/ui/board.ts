import { boardItems } from "../effects/board-actions.ts";

export function renderBoardPane(state, actions) {
  const pane = document.createElement("section");
  pane.className = "pane board";
  pane.append(heading("Board"));
  if (!state.auth) {
    pane.append(text("Login to use boards."));
    return pane;
  }
  pane.append(actionButton("Create board", () => actions.createBoard?.()));
  pane.append(actionButton("Add current node to board", () => actions.addCurrentNodeToBoard?.()));
  pane.append(actionButton("Move board item", () => actions.moveBoardItem?.()));
  pane.append(actionButton("Link board nodes", () => actions.linkBoardNodes?.()));
  pane.append(text(`Active board: ${state.activeBoardId || "none"}`));
  const items = boardItems(state);
  pane.append(text(`Board items: ${items.length}`));
  for (const item of items) {
    pane.append(text(`${item.target_title} @ ${item.x},${item.y}`));
  }
  pane.append(layoutSummary(state));
  return pane;
}

function actionButton(label, handler) {
  const button = document.createElement("button");
  button.className = "command";
  button.type = "button";
  button.textContent = label;
  button.addEventListener("click", handler);
  return button;
}

function heading(value) {
  const element = document.createElement("h1");
  element.textContent = value;
  return element;
}

function text(value) {
  const element = document.createElement("p");
  element.textContent = value;
  return element;
}

function layoutSummary(state) {
  const section = document.createElement("section");
  section.className = "layout-summary";
  section.append(text(`Layout panes: ${paneCount(state.layout?.root)}`));
  section.append(text(`Maximized: ${state.layout?.maximized_pane_id ?? "none"}`));
  if (state.layoutDirty) {
    section.append(text("Layout has a dirty local event."));
  }
  return section;
}

function paneCount(tile) {
  if (!tile) {
    return 0;
  }
  if (tile.kind === "stack") {
    return tile.tabs.length;
  }
  return (tile.children ?? []).reduce((sum, child) => sum + paneCount(child), 0);
}
