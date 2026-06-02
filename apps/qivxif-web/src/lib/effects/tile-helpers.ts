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
    tabId === "editor" || tabId === "graph" ? state.currentNodeId || null : boardTarget(tabId, state);
  const specs = {
    board: ["graph_board", "Board"],
    diagnostics: ["diagnostics", "Diagnostics"],
    editor: ["text_editor", "Text Node"],
    graph: ["graph_node", "Graph Node"],
    history: ["history", "History"],
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
  if (model?.layout_node_id) {
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
  const created = tileLayoutSetEntry(await reserveActorSeq(store), layoutNodeId, layout);
  await store.put("events", created.entry);
  await store.put("tile_layout", created.layoutRecord);
  state.layout = layout;
  state.layoutNodeId = layoutNodeId;
}

function boardTarget(tabId, state) {
  if (tabId === "board") {
    return state.activeBoardId || state.currentNodeId || null;
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
