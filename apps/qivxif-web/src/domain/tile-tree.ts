export function activePaneId(tile) {
  if (!tile) {
    return "";
  }
  if (tile.kind === "stack") {
    return tile.tabs[boundedActive(tile)]?.pane_node_id ?? "";
  }
  return activePaneId(tile.first) || activePaneId(tile.second);
}

export function focusPaneInLayout(layout, paneId) {
  const result = focusTree(layout.root, paneId);
  return replaceRoot(layout, result, false);
}

export function stackTabInLayout(layout, targetPaneId, tab) {
  assertMissing(layout.root, tab.pane_node_id);
  const result = appendToStack(layout.root, targetPaneId, tab);
  return replaceRoot(layout, result, true);
}

export function splitPaneInLayout(layout, targetPaneId, tab, direction = "right") {
  assertMissing(layout.root, tab.pane_node_id);
  const result = splitStack(layout.root, targetPaneId, tab, direction);
  return replaceRoot(layout, result, true);
}

export function closePaneInLayout(layout, paneId) {
  const result = removePane(layout.root, paneId);
  if (!result.changed) {
    throw new Error("pane missing");
  }
  if (!result.tile) {
    throw new Error("tile layout would be empty");
  }
  const maximized = layout.maximized_pane_id === paneId ? null : layout.maximized_pane_id;
  return { ...layout, maximized_pane_id: maximized, root: result.tile };
}

export function maximizePaneInLayout(layout, paneId) {
  if (!containsPane(layout.root, paneId)) {
    throw new Error("pane missing");
  }
  return { ...layout, maximized_pane_id: paneId };
}

export function visibleRoot(layout) {
  if (!layout?.maximized_pane_id) {
    return layout?.root ?? null;
  }
  return stackContaining(layout.root, layout.maximized_pane_id) ?? layout.root;
}

export function containsPane(tile, paneId) {
  if (!tile || !paneId) {
    return false;
  }
  if (tile.kind === "stack") {
    return tile.tabs.some((tab) => tab.pane_node_id === paneId);
  }
  return containsPane(tile.first, paneId) || containsPane(tile.second, paneId);
}

function focusTree(tile, paneId) {
  if (tile.kind === "stack") {
    const index = tile.tabs.findIndex((tab) => tab.pane_node_id === paneId);
    return index < 0 ? unchanged(tile) : changed(stackTile(tile.tabs, index));
  }
  const first = focusTree(tile.first, paneId);
  if (first.changed) {
    return changed(splitTile(tile, first.tile, tile.second));
  }
  const second = focusTree(tile.second, paneId);
  return second.changed ? changed(splitTile(tile, tile.first, second.tile)) : unchanged(tile);
}

function appendToStack(tile, targetPaneId, tab) {
  if (tile.kind === "stack") {
    if (!containsPane(tile, targetPaneId)) {
      return unchanged(tile);
    }
    return changed(stackTile([...tile.tabs, tab], tile.tabs.length));
  }
  const first = appendToStack(tile.first, targetPaneId, tab);
  if (first.changed) {
    return changed(splitTile(tile, first.tile, tile.second));
  }
  const second = appendToStack(tile.second, targetPaneId, tab);
  return second.changed ? changed(splitTile(tile, tile.first, second.tile)) : unchanged(tile);
}

function splitStack(tile, targetPaneId, tab, direction) {
  if (tile.kind === "stack") {
    return containsPane(tile, targetPaneId)
      ? changed(splitForDirection(tile, stackTile([tab], 0), direction))
      : unchanged(tile);
  }
  const first = splitStack(tile.first, targetPaneId, tab, direction);
  if (first.changed) {
    return changed(splitTile(tile, first.tile, tile.second));
  }
  const second = splitStack(tile.second, targetPaneId, tab, direction);
  return second.changed ? changed(splitTile(tile, tile.first, second.tile)) : unchanged(tile);
}

function removePane(tile, paneId) {
  if (tile.kind === "stack") {
    const index = tile.tabs.findIndex((tab) => tab.pane_node_id === paneId);
    if (index < 0) {
      return unchanged(tile);
    }
    const tabs = tile.tabs.filter((tab) => tab.pane_node_id !== paneId);
    return { changed: true, tile: tabs.length ? stackTile(tabs, nextActive(tile.active, index, tabs.length)) : null };
  }
  const first = removePane(tile.first, paneId);
  if (first.changed) {
    return changed(joinTile(tile, first.tile, tile.second));
  }
  const second = removePane(tile.second, paneId);
  return second.changed ? changed(joinTile(tile, tile.first, second.tile)) : unchanged(tile);
}

function stackContaining(tile, paneId) {
  if (tile.kind === "stack") {
    return containsPane(tile, paneId) ? tile : null;
  }
  return stackContaining(tile.first, paneId) ?? stackContaining(tile.second, paneId);
}

function splitForDirection(existing, created, direction) {
  const axis = direction === "top" || direction === "bottom" ? "column" : "row";
  return direction === "left" || direction === "top"
    ? newSplit(axis, created, existing)
    : newSplit(axis, existing, created);
}

function replaceRoot(layout, result, clearMaximized) {
  if (!result.changed) {
    throw new Error("pane missing");
  }
  return {
    ...layout,
    maximized_pane_id: clearMaximized ? null : layout.maximized_pane_id,
    root: result.tile,
  };
}

function assertMissing(tile, paneId) {
  if (containsPane(tile, paneId)) {
    throw new Error("pane already exists");
  }
}

function joinTile(template, first, second) {
  if (!first) {
    return second;
  }
  if (!second) {
    return first;
  }
  return splitTile(template, first, second);
}

function splitTile(template, first, second) {
  return { ...template, first, second };
}

function newSplit(axis, first, second) {
  return { axis, first, kind: "split", ratio_percent: 50, second };
}

function stackTile(tabs, active) {
  return { active, kind: "stack", tabs };
}

function boundedActive(tile) {
  return Math.max(0, Math.min(tile.active ?? 0, tile.tabs.length - 1));
}

function nextActive(active, removedIndex, length) {
  return removedIndex < active ? active - 1 : Math.min(active, length - 1);
}

function changed(tile) {
  return { changed: true, tile };
}

function unchanged(tile) {
  return { changed: false, tile };
}
