import { sendQueued } from "../http/client.ts";

export async function refreshQueueState(store, state) {
  const entries = await store.all("events");
  state.queued = entries.filter((entry) => entry.status !== "accepted").length;
  state.rejected = entries.filter((entry) => entry.status === "rejected").length;
  return entries;
}

export async function flushQueue(store, state) {
  if (!state.auth?.csrf_token) {
    await refreshQueueState(store, state);
    return;
  }
  const entries = await refreshQueueState(store, state);
  const dirty = entries
    .filter((entry) => entry.status === "dirty")
    .sort((left, right) => left.actor_seq - right.actor_seq);
  for (const entry of dirty) {
    const result = await flushEntry(store, state, entry);
    if (result === "network_failed") {
      break;
    }
  }
  await refreshQueueState(store, state);
}

async function flushEntry(store, state, entry) {
  await store.put("events", { ...entry, status: "pending_validation" });
  try {
    const payload = await sendQueued(entry, state.auth.csrf_token);
    if (payload.event?.event_id !== entry.id) {
      await store.put("events", {
        ...entry,
        status: "dirty",
        last_error: "event acceptance mismatch",
      });
      state.lastError = "event acceptance mismatch";
      return "network_failed";
    }
    await acceptEntry(store, state, entry, payload);
    return "accepted";
  } catch (error) {
    if (!error.api) {
      await store.put("events", { ...entry, status: "dirty", last_error: String(error) });
      state.online = false;
      state.lastError = String(error);
      return "network_failed";
    }
    await store.put("events", {
      ...entry,
      status: "rejected",
      last_error: error.api.code,
    });
    state.lastError = error.api.message ?? error.api.code;
    return "rejected";
  }
}

async function acceptEntry(store, state, entry, payload) {
  await store.delete("events", entry.id);
  if (entry.kind === "node.create") {
    await store.put("nodes", { ...payload.node, dirty: false });
    if (payload.node.kind === "text") {
      state.currentNodeId = payload.node.id;
    }
    if (payload.node.kind === "graph_board") {
      state.activeBoardId = payload.node.id;
      await store.put("tile_layout", { id: "active_board", node_id: payload.node.id });
    }
  }
  if (entry.kind === "edge.create") {
    await store.put("edges", { ...payload.edge, dirty: false });
  }
  if (entry.kind.startsWith("text.")) {
    await store.put("text_snapshots", {
      id: entry.node_id,
      state: payload.state,
      updated_by: entry.id,
    });
    state.text = payload.state.content;
  }
  if (entry.kind === "tile.layout_set") {
    await store.put("nodes", { ...payload.layout_node, dirty: false });
    await store.put("tile_layout", {
      dirty: false,
      id: "tile_model",
      layout: entry.request.layout,
      layout_node_id: entry.request.layout_node_id,
    });
    state.layout = entry.request.layout;
    state.layoutNodeId = entry.request.layout_node_id;
  }
  if (entry.kind.startsWith("publish.")) {
    await store.put("nodes", { ...payload.post, dirty: false });
    state.currentBlogPost = payload.post;
    state.currentBlogPostId = payload.post.id;
    await store.put("tile_layout", {
      id: "current_blog_post",
      node_id: payload.post.id,
    });
  }
  if (entry.kind === "social.short_post_create") {
    await store.put("nodes", { ...payload.post, dirty: false });
    await store.put("feed_windows", {
      dirty: false,
      id: payload.feed_item.event_id,
      item: payload.feed_item,
    });
  }
  if (entry.kind.startsWith("social.") && payload.edge) {
    await store.put("edges", { ...payload.edge, dirty: false });
  }
  state.online = true;
  state.lastError = "";
}
