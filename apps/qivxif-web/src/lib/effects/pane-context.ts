export function withPaneContext(state, context) {
  if (!context) return state;
  const next = { ...state };
  if (context.targetNodeId && isNodePane(context.paneKind)) {
    next.currentNodeId = context.targetNodeId;
  }
  if (context.targetNodeId && context.paneKind === "graph_map") {
    next.activeGraphMapId = context.targetNodeId;
  }
  return next;
}

function isNodePane(kind) {
  return kind === "text_editor" || kind === "graph_node";
}
