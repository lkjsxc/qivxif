export function insertTabAfterPaneInLayout(layout, targetPaneId, tab) {
  if (containsPane(layout.root, tab.pane_node_id)) {
    throw new Error("pane already exists");
  }
  const result = insertAfterPane(layout.root, targetPaneId, tab);
  if (!result.changed) {
    throw new Error("pane missing");
  }
  return { ...layout, maximized_pane_id: null, root: result.tile };
}

export function replaceTabInLayout(layout, paneId, spec) {
  const result = replaceTab(layout.root, paneId, spec);
  if (!result.changed) {
    throw new Error("pane missing");
  }
  return { ...layout, root: result.tile };
}

export function tabKindForPane(layout, paneId) {
  const tab = findTab(layout.root, paneId);
  return tab?.pane_kind ?? "";
}

function insertAfterPane(tile, targetPaneId, tab) {
  if (tile.kind === "stack") {
    const index = tile.tabs.findIndex((item) => item.pane_node_id === targetPaneId);
    if (index < 0) {
      return unchanged(tile);
    }
    const tabs = [...tile.tabs];
    const insertAt = index + 1;
    tabs.splice(insertAt, 0, tab);
    return changed({ ...tile, active: insertAt, tabs });
  }
  const children = tile.children.map((child) => insertAfterPane(child, targetPaneId, tab));
  const hit = children.findIndex((child) => child.changed);
  if (hit < 0) {
    return unchanged(tile);
  }
  const next = [...tile.children];
  next[hit] = children[hit].tile;
  return changed({ ...tile, children: next });
}

function replaceTab(tile, paneId, spec) {
  if (tile.kind === "stack") {
    const index = tile.tabs.findIndex((tab) => tab.pane_node_id === paneId);
    if (index < 0) {
      return unchanged(tile);
    }
    const tabs = [...tile.tabs];
    tabs[index] = { ...tabs[index], ...spec, pane_node_id: paneId };
    return changed({ ...tile, active: index, tabs });
  }
  const children = tile.children.map((child) => replaceTab(child, paneId, spec));
  const hit = children.findIndex((child) => child.changed);
  if (hit < 0) {
    return unchanged(tile);
  }
  const next = [...tile.children];
  next[hit] = children[hit].tile;
  return changed({ ...tile, children: next });
}

function findTab(tile, paneId) {
  if (tile.kind === "stack") {
    return tile.tabs.find((tab) => tab.pane_node_id === paneId) ?? null;
  }
  for (const child of tile.children ?? []) {
    const found = findTab(child, paneId);
    if (found) {
      return found;
    }
  }
  return null;
}

function containsPane(tile, paneId) {
  return Boolean(findTab(tile, paneId));
}

function changed(tile) {
  return { changed: true, tile };
}

function unchanged(tile) {
  return { changed: false, tile };
}
