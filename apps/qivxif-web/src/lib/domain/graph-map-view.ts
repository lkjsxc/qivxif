export function graphMapItems(state) {
  const graphMapId = state.activeGraphMapId;
  const nodes = state.nodes ?? [];
  const edges = state.edges ?? [];
  const byId = new Map(nodes.map((node: any) => [node.id, node]));
  const latest = new Map<string, any>();
  for (const edge of edges.filter((item: any) => item.kind === "placed_on_graph_map" && item.to_node === graphMapId)) {
    const placement: any = byId.get(edge.from_node);
    if (!placement || placement.kind !== "graph_map_item") continue;
    const itemNodeId = placement.metadata_map?.item_node_id;
    const seq = Number(placement.metadata_map?.placement_seq ?? 0);
    const current = latest.get(itemNodeId);
    if (!current || seq > current.placement_seq) {
      latest.set(itemNodeId, placementRecord(placement, byId.get(itemNodeId), seq));
    }
  }
  return [...latest.values()].sort((left, right) => left.placement_seq - right.placement_seq);
}

export function visibleGraphMapEdges(state, items = graphMapItems(state), enabledKinds = ["links_to"]) {
  const visible = new Set(items.map((item) => item.item_node_id));
  return (state.edges ?? []).filter(
    (edge) =>
      enabledKinds.includes(edge.kind) &&
      visible.has(edge.from_node) &&
      visible.has(edge.to_node) &&
      !edge.tombstone,
  );
}

function placementRecord(placement, target, seq) {
  return {
    item_node_id: placement.metadata_map.item_node_id,
    placement_node_id: placement.id,
    placement_seq: seq,
    target_kind: target?.kind ?? "unknown",
    target_title: target?.metadata_map?.title ?? placement.metadata_map.item_node_id,
    x: Number(placement.metadata_map.x ?? 0),
    y: Number(placement.metadata_map.y ?? 0),
  };
}
