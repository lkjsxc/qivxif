import { graphMapItems } from "../domain/graph-map-view.ts";
import { reserveActorSeq } from "./actor-seq.ts";
import { edgeCreateEntry, nodeCreateEntry } from "./local-events.ts";

export async function createGraphMap(store, state) {
  requireAuth(state);
  const graphMap = nodeCreateEntry(await reserveActorSeq(store), "graph_map", {
    dimension_state: JSON.stringify(defaultDimensions()),
    query_shape: JSON.stringify({ depth: 1, limit: 100 }),
    title: "Graph Map",
  });
  await store.put("events", graphMap.entry);
  await store.put("nodes", graphMap.node);
  await store.put("tile_layout", { id: "active_graph_map", node_id: graphMap.node.id });
  state.activeGraphMapId = graphMap.node.id;
  state.activeTabId = "graph-map";
}

export async function addCurrentNodeToGraphMap(store, state) {
  requireAuth(state);
  const graphMapId = state.activeGraphMapId;
  const targetId = currentGraphMapTarget(state);
  if (!graphMapId || !targetId) throw new Error("Graph Map and target node are required");
  await appendPlacement(store, graphMapId, targetId, nextPlacementSeq(state), 120, 120);
}

export async function moveGraphMapItem(store, state) {
  requireAuth(state);
  const item = graphMapItems(state)[0];
  if (!state.activeGraphMapId || !item) throw new Error("Graph Map item is required");
  await appendPlacement(
    store,
    state.activeGraphMapId,
    item.item_node_id,
    nextPlacementSeq(state),
    item.x + 40,
    item.y + 24,
  );
}

export async function linkGraphMapNodes(store, state) {
  requireAuth(state);
  const items = graphMapItems(state);
  if (items.length < 2) throw new Error("two Graph Map items are required");
  const edge = edgeCreateEntry(
    await reserveActorSeq(store),
    items[0].item_node_id,
    items[1].item_node_id,
    "links_to",
    { source: "graph_map" },
  );
  await store.put("events", edge.entry);
  await store.put("edges", edge.edge);
}

async function appendPlacement(store, graphMapId, itemNodeId, seq, x, y) {
  const placement = nodeCreateEntry(await reserveActorSeq(store), "graph_map_item", {
    item_node_id: itemNodeId,
    placement_seq: String(seq),
    position_key: String(seq).padStart(8, "0"),
    title: "Graph Map item",
    x: String(x),
    y: String(y),
  });
  await store.put("events", placement.entry);
  await store.put("nodes", placement.node);
  await putEdge(store, placement.node.id, graphMapId, "placed_on_graph_map", { placement_seq: String(seq) });
  await putEdge(store, placement.node.id, itemNodeId, "contains", { relation: "graph_map_item_target" });
}

async function putEdge(store, fromNode, toNode, kind, metadata) {
  const edge = edgeCreateEntry(await reserveActorSeq(store), fromNode, toNode, kind, metadata);
  await store.put("events", edge.entry);
  await store.put("edges", edge.edge);
}

function currentGraphMapTarget(state) {
  if (state.currentNodeId && state.currentNodeId !== state.activeGraphMapId) return state.currentNodeId;
  return state.nodes?.find((node) => node.kind === "text")?.id;
}

function nextPlacementSeq(state) {
  return graphMapItems(state).reduce((max, item) => Math.max(max, item.placement_seq), 0) + 1;
}

function defaultDimensions() {
  return { links_to: true, references: true, tags: true, profiles: true, media: true };
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) throw new Error("login is required");
}
