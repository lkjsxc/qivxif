import { reserveActorSeq } from "./actor-seq.ts";
import { edgeCreateEntry, nodeCreateEntry } from "./local-events.ts";

export async function createBoard(store, state) {
  requireAuth(state);
  const board = nodeCreateEntry(await reserveActorSeq(store), "graph_board", {
    title: "Graph board",
  });
  await store.put("events", board.entry);
  await store.put("nodes", board.node);
  await store.put("tile_layout", { id: "active_board", node_id: board.node.id });
  state.activeBoardId = board.node.id;
  state.activeTabId = "board";
}

export async function addCurrentNodeToBoard(store, state) {
  requireAuth(state);
  const boardId = state.activeBoardId;
  const targetId = currentBoardTarget(state);
  if (!boardId || !targetId) {
    throw new Error("board and target node are required");
  }
  await appendPlacement(store, boardId, targetId, nextPlacementSeq(state), 120, 120);
}

export async function moveBoardItem(store, state) {
  requireAuth(state);
  const item = boardItems(state)[0];
  if (!state.activeBoardId || !item) {
    throw new Error("board item is required");
  }
  await appendPlacement(
    store,
    state.activeBoardId,
    item.item_node_id,
    nextPlacementSeq(state),
    item.x + 40,
    item.y + 24,
  );
}

export async function linkBoardNodes(store, state) {
  requireAuth(state);
  const items = boardItems(state);
  if (items.length < 2) {
    throw new Error("two board items are required");
  }
  const edge = edgeCreateEntry(
    await reserveActorSeq(store),
    items[0].item_node_id,
    items[1].item_node_id,
    "links_to",
    { source: "board" },
  );
  await store.put("events", edge.entry);
  await store.put("edges", edge.edge);
}

export function boardItems(state) {
  const boardId = state.activeBoardId;
  const nodes = state.nodes ?? [];
  const edges = state.edges ?? [];
  const byId = new Map(nodes.map((node: any) => [node.id, node]));
  const latest = new Map<string, any>();
  for (const edge of edges.filter((item: any) => item.kind === "placed_on_board" && item.to_node === boardId)) {
    const placement: any = byId.get(edge.from_node);
    if (!placement || placement.kind !== "board_item") {
      continue;
    }
    const itemNodeId = placement.metadata_map?.item_node_id;
    const seq = Number(placement.metadata_map?.placement_seq ?? 0);
    const current = latest.get(itemNodeId);
    if (!current || seq > current.placement_seq) {
      latest.set(itemNodeId, placementRecord(placement, byId.get(itemNodeId), seq));
    }
  }
  return [...latest.values()].sort((left, right) => left.placement_seq - right.placement_seq);
}

async function appendPlacement(store, boardId, itemNodeId, seq, x, y) {
  const placement = nodeCreateEntry(await reserveActorSeq(store), "board_item", {
    item_node_id: itemNodeId,
    placement_seq: String(seq),
    title: "Board item",
    x: String(x),
    y: String(y),
  });
  await store.put("events", placement.entry);
  await store.put("nodes", placement.node);
  const boardEdge = edgeCreateEntry(
    await reserveActorSeq(store),
    placement.node.id,
    boardId,
    "placed_on_board",
    { placement_seq: String(seq) },
  );
  await store.put("events", boardEdge.entry);
  await store.put("edges", boardEdge.edge);
  const targetEdge = edgeCreateEntry(
    await reserveActorSeq(store),
    placement.node.id,
    itemNodeId,
    "contains",
    { relation: "board_item_target" },
  );
  await store.put("events", targetEdge.entry);
  await store.put("edges", targetEdge.edge);
}

function currentBoardTarget(state) {
  if (state.currentNodeId && state.currentNodeId !== state.activeBoardId) {
    return state.currentNodeId;
  }
  return state.nodes?.find((node) => node.kind === "text")?.id;
}

function nextPlacementSeq(state) {
  return boardItems(state).reduce((max, item) => Math.max(max, item.placement_seq), 0) + 1;
}

function placementRecord(placement, target, seq) {
  return {
    item_node_id: placement.metadata_map.item_node_id,
    placement_node_id: placement.id,
    placement_seq: seq,
    target_title: target?.metadata_map?.title ?? placement.metadata_map.item_node_id,
    x: Number(placement.metadata_map.x ?? 0),
    y: Number(placement.metadata_map.y ?? 0),
  };
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
