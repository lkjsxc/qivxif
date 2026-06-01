import { reserveActorSeq } from "./actor-seq.ts";
import {
  activePaneId,
  closePaneInLayout,
  focusPaneInLayout,
  maximizePaneInLayout,
  splitPaneInLayout,
  stackTabInLayout,
} from "../domain/tile-tree.ts";
import { edgeCreateEntry, nodeCreateEntry, tileLayoutSetEntry } from "./local-events.ts";

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

export async function ensureLayout(store, state) {
  const current = await store.get("tile_layout", "tile_model");
  if (current?.layout_node_id && current?.layout) {
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

async function createPane(store, state, targetNodeId, title, paneKind = "text_editor") {
  const pane = await createNode(store, "pane", { pane_kind: paneKind, title });
  const model = await store.get("tile_layout", "tile_model");
  if (model?.layout_node_id) {
    await link(store, model.layout_node_id, pane.node.id, "tile_contains_pane", { slot: title });
  }
  if (targetNodeId) {
    await link(store, pane.node.id, targetNodeId, "pane_views_node", { pane_kind: paneKind });
  }
  return pane;
}

async function createNode(store, kind, metadata) {
  const created = nodeCreateEntry(await reserveActorSeq(store), kind, metadata);
  await store.put("events", created.entry);
  await store.put("nodes", created.node);
  return created;
}

async function link(store, fromNode, toNode, kind, metadata) {
  const created = edgeCreateEntry(await reserveActorSeq(store), fromNode, toNode, kind, metadata);
  await store.put("events", created.entry);
  await store.put("edges", created.edge);
  return created;
}

async function queueLayout(store, state, layoutNodeId, layout) {
  const created = tileLayoutSetEntry(await reserveActorSeq(store), layoutNodeId, layout);
  await store.put("events", created.entry);
  await store.put("tile_layout", created.layoutRecord);
  state.layout = layout;
  state.layoutNodeId = layoutNodeId;
}

function stackTile(tabs) {
  return { active: Math.max(0, tabs.length - 1), kind: "stack", tabs };
}

function tabFor(paneNodeId, targetNodeId, title, paneKind) {
  return {
    pane_kind: paneKind,
    pane_node_id: paneNodeId,
    target_node_id: targetNodeId || null,
    title,
  };
}

function initialPaneSpec(state) {
  if (state.currentNodeId) {
    return { paneKind: "text_editor", targetNodeId: state.currentNodeId, title: "Text pane" };
  }
  return { paneKind: "home", targetNodeId: null, title: "Home" };
}

function tabSpec(tabId, state) {
  const targetNodeId =
    tabId === "editor" || tabId === "graph" ? state.currentNodeId || null : boardTarget(tabId, state);
  const specs = {
    board: ["graph_board", "Board"],
    diagnostics: ["diagnostics", "Diagnostics"],
    editor: ["text_editor", "Text Node"],
    graph: ["graph_node", "Graph Node"],
    history: ["history", "History"],
    home: ["home", "Home"],
    publish: ["publishing", "Publishing"],
    settings: ["settings", "Settings"],
    social: ["social_feed", "Social"],
    sync: ["sync_status", "Sync Status"],
  };
  const [paneKind, title] = specs[tabId] ?? ["home", "Home"];
  return { paneKind, targetNodeId, title };
}

function targetPane(root, paneId) {
  return paneId?.startsWith("nod_") ? paneId : activePaneId(root);
}

function boardTarget(tabId, state) {
  if (tabId === "board") {
    return state.activeBoardId || state.currentNodeId || null;
  }
  return null;
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
