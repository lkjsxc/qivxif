const DEFAULT_SIZE = 500;

export function containsPane(tile, paneId) {
  if (!tile || !paneId) {
    return false;
  }
  if (tile.kind === "stack") {
    return tile.tabs.some((tab) => tab.pane_node_id === paneId);
  }
  return (tile.children ?? []).some((child) => containsPane(child, paneId));
}

export function focusTree(tile, paneId) {
  if (tile.kind === "stack") {
    const index = tile.tabs.findIndex((tab) => tab.pane_node_id === paneId);
    return index < 0 ? unchanged(tile) : changed(stackTile(tile.tabs, index));
  }
  const children = tile.children.map((child) => focusTree(child, paneId));
  const index = children.findIndex((child) => child.changed);
  if (index < 0) {
    return unchanged(tile);
  }
  const next = [...tile.children];
  next[index] = children[index].tile;
  return changed(splitTile(tile.axis, next, tile.sizes));
}

export function appendToStack(tile, targetPaneId, tab) {
  if (tile.kind === "stack") {
    if (!containsPane(tile, targetPaneId)) {
      return unchanged(tile);
    }
    return changed(stackTile([...tile.tabs, tab], tile.tabs.length));
  }
  const children = tile.children.map((child) => appendToStack(child, targetPaneId, tab));
  const index = children.findIndex((child) => child.changed);
  if (index < 0) {
    return unchanged(tile);
  }
  const next = [...tile.children];
  next[index] = children[index].tile;
  return changed(splitTile(tile.axis, next, tile.sizes));
}

export function splitStack(tile, targetPaneId, tab, direction) {
  if (tile.kind === "stack" && containsPane(tile, targetPaneId)) {
    return changed(splitForDirection(tile, stackTile([tab], 0), direction));
  }
  if (tile.kind === "stack") {
    return unchanged(tile);
  }
  const children = tile.children.map((child) => splitStack(child, targetPaneId, tab, direction));
  const index = children.findIndex((child) => child.changed);
  if (index < 0) {
    return unchanged(tile);
  }
  const next = [...tile.children];
  next[index] = children[index].tile;
  return changed(splitTile(tile.axis, next, tile.sizes));
}

export function removePane(tile, paneId) {
  if (tile.kind === "stack") {
    const index = tile.tabs.findIndex((tab) => tab.pane_node_id === paneId);
    if (index < 0) {
      return unchanged(tile);
    }
    const tabs = tile.tabs.filter((tab) => tab.pane_node_id !== paneId);
    return {
      changed: true,
      tile: tabs.length ? stackTile(tabs, nextActive(tile.active, index, tabs.length)) : null,
    };
  }
  for (let index = 0; index < tile.children.length; index += 1) {
    const result = removePane(tile.children[index], paneId);
    if (result.changed) {
      const children = [...tile.children];
      const sizes = [...tile.sizes];
      if (result.tile) {
        children[index] = result.tile;
      } else {
        children.splice(index, 1);
        sizes.splice(index, 1);
      }
      return { changed: true, tile: collapseSplit(tile.axis, children, sizes) };
    }
  }
  return unchanged(tile);
}

export function resizeSplit(tile, paneId, sizes) {
  if (tile.kind === "split") {
    if (
      tile.children.some((child) => directChildContains(child, paneId)) &&
      sizes.length === tile.children.length
    ) {
      tile.sizes = sizes;
      return true;
    }
    return tile.children.some((child) => resizeSplit(child, paneId, sizes));
  }
  return false;
}

export function stackContaining(tile, paneId) {
  if (tile.kind === "stack") {
    return containsPane(tile, paneId) ? tile : null;
  }
  for (const child of tile.children ?? []) {
    const found = stackContaining(child, paneId);
    if (found) {
      return found;
    }
  }
  return null;
}

export function replaceRoot(layout, result, clearMaximized) {
  if (!result.changed) {
    throw new Error("pane missing");
  }
  return {
    ...layout,
    maximized_pane_id: clearMaximized ? null : layout.maximized_pane_id,
    root: result.tile,
  };
}

export function assertMissing(tile, paneId) {
  if (containsPane(tile, paneId)) {
    throw new Error("pane already exists");
  }
}

function splitForDirection(existing, created, direction) {
  const axis = direction === "top" || direction === "bottom" ? "column" : "row";
  const children =
    direction === "left" || direction === "top" ? [created, existing] : [existing, created];
  return splitTile(axis, children, equalSizes(2));
}

function collapseSplit(axis, children, sizes) {
  if (!children.length) {
    return null;
  }
  if (children.length === 1) {
    return children[0];
  }
  return splitTile(axis, children, normalizeSizes(sizes, children.length));
}

function splitTile(axis, children, sizes) {
  return { axis, children, kind: "split", sizes: normalizeSizes(sizes, children.length) };
}

function stackTile(tabs, active) {
  return { active, kind: "stack", tabs };
}

function equalSizes(count) {
  return Array.from({ length: count }, () => DEFAULT_SIZE);
}

function normalizeSizes(sizes, count) {
  return sizes.length === count ? sizes : equalSizes(count);
}

function directChildContains(child, paneId) {
  return child.kind === "stack" && child.tabs.some((tab) => tab.pane_node_id === paneId);
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
