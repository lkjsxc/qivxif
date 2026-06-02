import {
  appendToStack,
  assertMissing,
  containsPane,
  focusTree,
  removePane,
  replaceRoot,
  resizeSplit,
  splitStack,
  stackContaining,
} from "./tile-mutations.ts";

export function activePaneId(tile) {
  if (!tile) {
    return "";
  }
  if (tile.kind === "stack") {
    return tile.tabs[boundedActive(tile)]?.pane_node_id ?? "";
  }
  for (const child of tile.children ?? []) {
    const pane = activePaneId(child);
    if (pane) {
      return pane;
    }
  }
  return "";
}

export function visibleRoot(layout) {
  if (!layout?.maximized_pane_id) {
    return layout?.root ?? null;
  }
  return stackContaining(layout.root, layout.maximized_pane_id) ?? layout.root;
}

export function focusPaneInLayout(layout, paneId) {
  return replaceRoot(layout, focusTree(layout.root, paneId), false);
}

export function stackTabInLayout(layout, targetPaneId, tab) {
  assertMissing(layout.root, tab.pane_node_id);
  return replaceRoot(layout, appendToStack(layout.root, targetPaneId, tab), true);
}

export function splitPaneInLayout(layout, targetPaneId, tab, direction = "right") {
  assertMissing(layout.root, tab.pane_node_id);
  return replaceRoot(layout, splitStack(layout.root, targetPaneId, tab, direction), true);
}

export function closePaneInLayout(layout, paneId) {
  const result = removePane(layout.root, paneId);
  if (!result.changed) {
    throw new Error("pane missing");
  }
  if (!result.tile) {
    throw new Error("tile layout would be empty");
  }
  return {
    ...layout,
    maximized_pane_id: layout.maximized_pane_id === paneId ? null : layout.maximized_pane_id,
    root: result.tile,
  };
}

export function maximizePaneInLayout(layout, paneId) {
  if (!containsPane(layout.root, paneId)) {
    throw new Error("pane missing");
  }
  return { ...layout, maximized_pane_id: paneId };
}

export function resizeSplitInLayout(layout, paneId, sizes) {
  const root = structuredClone(layout.root);
  if (!resizeSplit(root, paneId, sizes)) {
    throw new Error("pane missing");
  }
  return { ...layout, root };
}

export { containsPane };

function boundedActive(tile) {
  return Math.max(0, Math.min(tile.active ?? 0, tile.tabs.length - 1));
}
