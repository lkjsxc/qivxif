import { neighborhood, node, nodeHistory, text } from "./api-client.ts";
import { isNodeId, isSyncableLayout } from "../domain/tile-layout-validation.ts";
import { activePaneId, containsPane } from "../domain/tile-tree.ts";

export async function loadLocalState(store, state) {
  const auth = await store.get("sync_cursors", "auth");
  const current = await store.get("tile_layout", "current_node");
  const currentBlog = await store.get("tile_layout", "current_blog_post");
  const graphMap = await store.get("tile_layout", "active_graph_map");
  const layout = await store.get("tile_layout", "tile_model");
  const publicRoute = await store.get("sync_cursors", "last_public_route");
  state.auth = auth?.auth ?? state.auth;
  state.edges = await store.all("edges");
  state.feedItems = await store.all("feed_windows");
  state.mediaAssets = await store.all("media_assets");
  state.nodes = await store.all("nodes");
  const tabSnapshots = await store.all("tab_snapshots");
  state.tabDrafts = Object.fromEntries(
    tabSnapshots.filter((entry) => entry.kind === "text_draft").map((entry) => [entry.pane_id, entry.content]),
  );
  state.tabScrolls = Object.fromEntries(
    tabSnapshots.filter((entry) => entry.kind === "pane_scroll").map((entry) => [entry.pane_id, entry.scroll_top]),
  );
  state.textSnapshots = Object.fromEntries(
    (await store.all("text_snapshots")).map((entry) => [entry.id, entry]),
  );
  state.currentNodeId = current?.node_id ?? state.currentNodeId;
  state.currentBlogPostId = currentBlog?.node_id ?? state.currentBlogPostId;
  state.currentBlogPost =
    state.nodes.find((node) => node.id === state.currentBlogPostId) ?? null;
  state.lastPublicRoute = publicRoute?.path ?? state.lastPublicRoute;
  state.activeGraphMapId = graphMap?.node_id ?? state.activeGraphMapId;
  if (isNodeId(layout?.layout_node_id) && isSyncableLayout(layout?.layout)) {
    state.layout = layout.layout;
    state.layoutDirty = layout.dirty ?? false;
    state.layoutNodeId = layout.layout_node_id;
  } else if (layout) {
    state.layout = null;
    state.layoutDirty = false;
    state.layoutNodeId = "";
    await store.delete("tile_layout", "tile_model");
  } else {
    state.layoutDirty = false;
  }
  if (state.layout && !containsPane(state.layout.root, state.activePaneId)) {
    state.activePaneId = activePaneId(state.layout.root);
  }
  const textState = state.currentNodeId ? state.textSnapshots[state.currentNodeId] : null;
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
  if (nodePayload.projection.node.kind === "graph_map") {
    state.activeTabId = "graph-map";
    state.activeGraphMapId = state.currentNodeId;
    await store.put("tile_layout", { id: "active_graph_map", node_id: state.currentNodeId });
    await refreshNeighborhood(store, state.currentNodeId);
  }
  if (nodePayload.projection.node.kind === "tile_layout") {
    const layoutJson = nodePayload.projection.node.metadata_map?.layout_json;
    if (layoutJson) {
      const parsedLayout = JSON.parse(layoutJson);
      if (isSyncableLayout(parsedLayout)) {
        state.layout = parsedLayout;
        state.layoutNodeId = state.currentNodeId;
        await store.put("tile_layout", {
          dirty: false,
          id: "tile_model",
          layout: state.layout,
          layout_node_id: state.currentNodeId,
        });
      }
    }
  }
  if (nodePayload.projection.node.kind === "blog_post") {
    state.currentBlogPostId = state.currentNodeId;
    state.currentBlogPost = nodePayload.projection.node;
    await store.put("tile_layout", {
      id: "current_blog_post",
      node_id: state.currentNodeId,
    });
  }
  const historyPayload = await nodeHistory(state.currentNodeId);
  state.history = historyPayload.events;
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
