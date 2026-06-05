import { isNodeId, isSyncableLayout } from "../domain/tile-layout-validation.ts";
import { activePaneId } from "../domain/tile-tree.ts";
import { reserveActorSeq } from "./actor-seq.ts";
import { edgeCreateEntry, nodeCreateEntry, tileLayoutSetEntry } from "./local-events.ts";

export function stackTile(tabs) {
  return { active: Math.max(0, tabs.length - 1), kind: "stack", tabs };
}

export function tabFor(paneNodeId, targetNodeId, title, paneKind) {
  return { pane_kind: paneKind, pane_node_id: paneNodeId, target_node_id: targetNodeId || null, title };
}

export function tabSpec(tabId, state) {
  const targetNodeId =
    tabId === "editor" || tabId === "graph" ? state.currentNodeId || null : graphMapTarget(tabId, state);
  const specs = {
    "graph-map": ["graph_map", "Graph Map"],
    diagnostics: ["diagnostics", "Diagnostics"],
    editor: ["text_editor", "Text Node"],
    graph: ["graph_node", "Graph Node"],
    history: ["history", "History"],
    media: ["media", "Media"],
    publish: ["publishing", "Publishing"],
    settings: ["settings", "Settings"],
    social: ["social_feed", "Social"],
    sync: ["sync_status", "Sync Status"],
    welcome: ["welcome", "Welcome"],
  };
  const [paneKind, title] = specs[tabId] ?? ["welcome", "Welcome"];
  return { paneKind, targetNodeId, title };
}

export function targetPane(root, paneId) {
  return paneId?.startsWith("nod_") ? paneId : activePaneId(root);
}

export function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}

export async function createPane(store, state, targetNodeId, title, paneKind = "text_editor") {
  const pane = await createNode(store, "pane", { pane_kind: paneKind, title });
  const model = await store.get("tile_layout", "tile_model");
  if (isNodeId(model?.layout_node_id) && isSyncableLayout(model?.layout)) {
    await link(store, model.layout_node_id, pane.node.id, "tile_contains_pane", { slot: title });
  }
  if (targetNodeId) {
    await link(store, pane.node.id, targetNodeId, "pane_views_node", { pane_kind: paneKind });
  }
  return pane;
}

export async function createNode(store, kind, metadata) {
  const created = nodeCreateEntry(await reserveActorSeq(store), kind, metadata);
  await store.put("events", created.entry);
  await store.put("nodes", created.node);
  return created;
}

export async function link(store, fromNode, toNode, kind, metadata) {
  const created = edgeCreateEntry(await reserveActorSeq(store), fromNode, toNode, kind, metadata);
  await store.put("events", created.entry);
  await store.put("edges", created.edge);
  return created;
}

export async function queueLayout(store, state, layoutNodeId, layout) {
  if (!isNodeId(layoutNodeId) || !isSyncableLayout(layout)) {
    throw new Error("tile layout contains local-only pane ids");
  }
  const created = tileLayoutSetEntry(await reserveActorSeq(store), layoutNodeId, layout);
  await store.put("events", created.entry);
  await store.put("tile_layout", created.layoutRecord);
  state.layout = layout;
  state.layoutNodeId = layoutNodeId;
}

function graphMapTarget(tabId, state) {
  if (tabId === "graph-map") {
    return state.activeGraphMapId || state.currentNodeId || null;
  }
  return null;
}

function initialPaneSpec(state) {
  if (state.currentNodeId) {
    return { paneKind: "text_editor", targetNodeId: state.currentNodeId, title: "Text pane" };
  }
  return { paneKind: "welcome", targetNodeId: null, title: "Welcome" };
}

export { initialPaneSpec };
