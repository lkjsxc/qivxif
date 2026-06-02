import { isNodeId, isSyncableLayout } from "../domain/tile-layout-validation.ts";
import {
  activePaneId,
  closePaneInLayout,
  focusPaneInLayout,
  maximizePaneInLayout,
  resizeSplitInLayout,
  splitPaneInLayout,
  stackTabInLayout,
} from "../domain/tile-tree.ts";
import {
  createNode,
  createPane,
  initialPaneSpec,
  link,
  queueLayout,
  requireAuth,
  stackTile,
  tabFor,
  tabSpec,
  targetPane,
} from "./tile-helpers.ts";

export async function focusPane(store, state, paneId) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const target = targetPane(model.layout.root, paneId);
  state.activePaneId = target;
  await queueLayout(store, state, model.layout_node_id, focusPaneInLayout(model.layout, target));
}

export async function splitPane(store, state, paneId) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const target = targetPane(model.layout.root, paneId);
  const pane = await createPane(store, state, state.currentNodeId, "Split pane", "text_editor");
  const tab = tabFor(pane.node.id, state.currentNodeId, "Split pane", "text_editor");
  const next = splitPaneInLayout(model.layout, target, tab, "right");
  state.activePaneId = pane.node.id;
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function stackTab(store, state, paneId) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const target = targetPane(model.layout.root, paneId);
  const pane = await createPane(store, state, state.currentNodeId, "Stacked pane", "text_editor");
  const tab = tabFor(pane.node.id, state.currentNodeId, "Stacked pane", "text_editor");
  const next = stackTabInLayout(model.layout, target, tab);
  state.activePaneId = pane.node.id;
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function openProductTab(store, state, paneId, tabId) {
  if (!state.auth?.user?.actor_id) {
    state.activeTabId = tabId;
    state.tabChooserOpen = false;
    return;
  }
  const model = await ensureLayout(store, state);
  const spec = tabSpec(tabId, state);
  const target = targetPane(model.layout.root, paneId);
  const pane = await createPane(store, state, spec.targetNodeId, spec.title, spec.paneKind);
  const tab = tabFor(pane.node.id, spec.targetNodeId, spec.title, spec.paneKind);
  const next = stackTabInLayout(model.layout, target, tab);
  state.activePaneId = pane.node.id;
  state.tabChooserOpen = false;
  state.tabChooserPaneId = "";
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function maximizePane(store, state, paneId) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const target = targetPane(model.layout.root, paneId);
  const next =
    model.layout.maximized_pane_id === target
      ? { ...model.layout, maximized_pane_id: null }
      : maximizePaneInLayout(model.layout, target);
  state.activePaneId = target;
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function closePane(store, state, paneId) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const target = targetPane(model.layout.root, paneId);
  const next = closePaneInLayout(model.layout, target);
  state.activePaneId = target === state.activePaneId ? activePaneId(next.root) : state.activePaneId;
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function resizeSplit(store, state, paneId, sizes) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const target = targetPane(model.layout.root, paneId);
  const next = resizeSplitInLayout(model.layout, target, sizes);
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function ensureLayout(store, state) {
  const current = await store.get("tile_layout", "tile_model");
  if (isNodeId(current?.layout_node_id) && isSyncableLayout(current?.layout)) {
    state.layout = current.layout;
    state.layoutNodeId = current.layout_node_id;
    return current;
  }
  const layout = await createNode(store, "tile_layout", { title: "Tile layout" });
  const spec = initialPaneSpec(state);
  const pane = await createPane(store, state, spec.targetNodeId, spec.title, spec.paneKind);
  const initial = {
    maximized_pane_id: null,
    root: stackTile([tabFor(pane.node.id, spec.targetNodeId, spec.title, spec.paneKind)]),
  };
  const record = {
    dirty: true,
    id: "tile_model",
    layout: initial,
    layout_node_id: layout.node.id,
  };
  await link(store, layout.node.id, pane.node.id, "tile_contains_pane", { slot: "root" });
  await store.put("tile_layout", record);
  state.layout = initial;
  state.layoutNodeId = layout.node.id;
  state.activePaneId = pane.node.id;
  return record;
}
