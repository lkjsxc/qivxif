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

export function moveTabNearTab(layout, sourcePaneId, targetPaneId, side) {
  if (sourcePaneId === targetPaneId) {
    return focusPaneInLayout(layout, targetPaneId);
  }
  const tab = findTab(layout.root, sourcePaneId);
  const withoutSource = closePaneInLayout(layout, sourcePaneId);
  const result = insertNear(withoutSource.root, targetPaneId, tab, side);
  if (!result.changed) {
    throw new Error("pane missing");
  }
  return { ...withoutSource, maximized_pane_id: null, root: result.tile };
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

function insertNear(tile, targetPaneId, tab, side) {
  if (tile.kind === "stack") {
    const index = tile.tabs.findIndex((item) => item.pane_node_id === targetPaneId);
    if (index < 0) {
      return unchanged(tile);
    }
    const tabs = [...tile.tabs];
    const insertAt = side === "after" ? index + 1 : index;
    tabs.splice(insertAt, 0, tab);
    return changed({ ...tile, active: insertAt, tabs });
  }
  const first = insertNear(tile.first, targetPaneId, tab, side);
  if (first.changed) {
    return changed({ ...tile, first: first.tile });
  }
  const second = insertNear(tile.second, targetPaneId, tab, side);
  return second.changed ? changed({ ...tile, second: second.tile }) : unchanged(tile);
}

function changed(tile) {
  return { changed: true, tile };
}

function unchanged(tile) {
  return { changed: false, tile };
}
