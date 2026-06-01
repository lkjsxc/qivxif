import { neighborhood, node, nodeHistory, text } from "../http/client.ts";

export async function loadLocalState(store, state) {
  const auth = await store.get("sync_cursors", "auth");
  const current = await store.get("workspace_layout", "current_node");
  const currentBlog = await store.get("workspace_layout", "current_blog_post");
  const board = await store.get("workspace_layout", "active_board");
  const layout = await store.get("workspace_layout", "workspace_model");
  const publicRoute = await store.get("sync_cursors", "last_public_route");
  state.auth = auth?.auth ?? state.auth;
  state.edges = await store.all("edges");
  state.feedItems = await store.all("feed_windows");
  state.nodes = await store.all("nodes");
  state.currentNodeId = current?.node_id ?? state.currentNodeId;
  state.currentBlogPostId = currentBlog?.node_id ?? state.currentBlogPostId;
  state.currentBlogPost =
    state.nodes.find((node) => node.id === state.currentBlogPostId) ?? null;
  state.lastPublicRoute = publicRoute?.path ?? state.lastPublicRoute;
  state.activeBoardId = board?.node_id ?? state.activeBoardId;
  state.layout = layout?.layout ?? state.layout;
  state.layoutDirty = layout?.dirty ?? false;
  state.layoutNodeId = layout?.layout_node_id ?? state.layoutNodeId;
  const textState = state.currentNodeId
    ? await store.get("text_snapshots", state.currentNodeId)
    : null;
  state.text = textState?.state?.content ?? "";
  state.textDirty = textState?.dirty ?? false;
}

export async function refreshCurrentNode(store, state) {
  const nodePayload = await node(state.currentNodeId);
  await store.put("nodes", { ...nodePayload.projection.node, dirty: false });
  for (const edge of [...nodePayload.projection.outgoing, ...nodePayload.projection.incoming]) {
    await store.put("edges", { ...edge, dirty: false });
  }
  if (nodePayload.projection.node.kind === "text") {
    state.activeTabId = "editor";
    const textPayload = await text(state.currentNodeId);
    await store.put("text_snapshots", {
      dirty: false,
      id: state.currentNodeId,
      state: textPayload.state,
    });
  }
  if (nodePayload.projection.node.kind === "graph_board") {
    state.activeTabId = "board";
    state.activeBoardId = state.currentNodeId;
    await store.put("workspace_layout", { id: "active_board", node_id: state.currentNodeId });
    await refreshNeighborhood(store, state.currentNodeId);
  }
  if (nodePayload.projection.node.kind === "workspace_layout") {
    const layoutJson = nodePayload.projection.node.metadata_map?.layout_json;
    if (layoutJson) {
      state.layout = JSON.parse(layoutJson);
      state.layoutNodeId = state.currentNodeId;
      await store.put("workspace_layout", {
        dirty: false,
        id: "workspace_model",
        layout: state.layout,
        layout_node_id: state.currentNodeId,
      });
    }
  }
  if (nodePayload.projection.node.kind === "blog_post") {
    state.currentBlogPostId = state.currentNodeId;
    state.currentBlogPost = nodePayload.projection.node;
    await store.put("workspace_layout", {
      id: "current_blog_post",
      node_id: state.currentNodeId,
    });
  }
  const historyPayload = await nodeHistory(state.currentNodeId);
  state.history = historyPayload.operations;
}

async function refreshNeighborhood(store, nodeId) {
  const payload = await neighborhood(nodeId);
  for (const projection of payload.projection.nodes) {
    await store.put("nodes", { ...projection.node, dirty: false });
    for (const edge of [...projection.outgoing, ...projection.incoming]) {
      await store.put("edges", { ...edge, dirty: false });
    }
  }
}
