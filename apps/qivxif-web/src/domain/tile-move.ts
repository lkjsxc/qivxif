import {
  closePaneInLayout,
  focusPaneInLayout,
  splitPaneInLayout,
  stackTabInLayout,
} from "./tile-tree.ts";

export function moveTabToStack(layout, sourcePaneId, targetPaneId) {
  if (sourcePaneId === targetPaneId) {
    return focusPaneInLayout(layout, targetPaneId);
  }
  const tab = findTab(layout.root, sourcePaneId);
  const withoutSource = closePaneInLayout(layout, sourcePaneId);
  return stackTabInLayout(withoutSource, targetPaneId, tab);
}

export function moveTabToEdge(layout, sourcePaneId, targetPaneId, direction) {
  if (sourcePaneId === targetPaneId) {
    return focusPaneInLayout(layout, targetPaneId);
  }
  const tab = findTab(layout.root, sourcePaneId);
  const withoutSource = closePaneInLayout(layout, sourcePaneId);
  return splitPaneInLayout(withoutSource, targetPaneId, tab, direction);
}

function findTab(tile, paneId) {
  if (tile.kind === "stack") {
    const tab = tile.tabs.find((item) => item.pane_node_id === paneId);
    if (tab) {
      return tab;
    }
    throw new Error("pane missing");
  }
  try {
    return findTab(tile.first, paneId);
  } catch (error) {
    return findTab(tile.second, paneId);
  }
}
