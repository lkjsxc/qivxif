import { reserveActorSeq } from "./actor-seq.ts";
import { edgeCreateEntry, nodeCreateEntry, tileLayoutSetEntry } from "./local-operations.ts";

export async function splitPane(store, state) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const pane = await createPane(store, state, state.currentNodeId, "Split pane");
  const next = {
    maximized_pane_id: null,
    root: {
      axis: "row",
      first: model.layout.root,
      kind: "split",
      ratio_percent: 50,
      second: stackTile([tabFor(pane.node.id, state.currentNodeId, "Split pane")]),
    },
  };
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function stackTab(store, state) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const pane = await createPane(store, state, state.currentNodeId, "Stacked pane");
  const next = {
    ...model.layout,
    root: appendTab(
      model.layout.root,
      tabFor(pane.node.id, state.currentNodeId, "Stacked pane"),
    ),
  };
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function maximizePane(store, state) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const paneId = firstPaneId(model.layout.root);
  const next = { ...model.layout, maximized_pane_id: paneId };
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function closePane(store, state) {
  requireAuth(state);
  const model = await ensureLayout(store, state);
  const next = {
    ...model.layout,
    maximized_pane_id: null,
    root: removeFirstPane(model.layout.root),
  };
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
  const pane = await createPane(store, state, state.currentNodeId, "Text pane");
  const initial = {
    maximized_pane_id: null,
    root: stackTile([tabFor(pane.node.id, state.currentNodeId, "Text pane")]),
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
  return record;
}

async function createPane(store, state, targetNodeId, title) {
  const pane = await createNode(store, "pane", { pane_kind: "text_editor", title });
  const model = await store.get("tile_layout", "tile_model");
  if (model?.layout_node_id) {
    await link(store, model.layout_node_id, pane.node.id, "tile_contains_pane", { slot: title });
  }
  if (targetNodeId) {
    await link(store, pane.node.id, targetNodeId, "pane_views_node", { pane_kind: "text_editor" });
  }
  return pane;
}

async function createNode(store, kind, metadata) {
  const created = nodeCreateEntry(await reserveActorSeq(store), kind, metadata);
  await store.put("ops", created.entry);
  await store.put("nodes", created.node);
  return created;
}

async function link(store, fromNode, toNode, kind, metadata) {
  const created = edgeCreateEntry(await reserveActorSeq(store), fromNode, toNode, kind, metadata);
  await store.put("ops", created.entry);
  await store.put("edges", created.edge);
  return created;
}

async function queueLayout(store, state, layoutNodeId, layout) {
  const created = tileLayoutSetEntry(await reserveActorSeq(store), layoutNodeId, layout);
  await store.put("ops", created.entry);
  await store.put("tile_layout", created.layoutRecord);
  state.layout = layout;
  state.layoutNodeId = layoutNodeId;
}

function appendTab(tile, tab) {
  if (tile.kind === "stack") {
    return { ...tile, active: tile.tabs.length, tabs: [...tile.tabs, tab] };
  }
  return { ...tile, second: appendTab(tile.second, tab) };
}

function removeFirstPane(tile) {
  if (tile.kind === "stack") {
    return stackTile(tile.tabs.slice(1));
  }
  return { ...tile, first: removeFirstPane(tile.first) };
}

function firstPaneId(tile) {
  if (tile.kind === "stack") {
    return tile.tabs[0]?.pane_node_id ?? null;
  }
  return firstPaneId(tile.first);
}

function stackTile(tabs) {
  return { active: Math.max(0, tabs.length - 1), kind: "stack", tabs };
}

function tabFor(paneNodeId, targetNodeId, title) {
  return {
    pane_kind: "text_editor",
    pane_node_id: paneNodeId,
    target_node_id: targetNodeId,
    title,
  };
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
