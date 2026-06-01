import { sendQueued } from "../api/client.ts";

export async function refreshQueueState(store, state) {
  const entries = await store.all("ops");
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
  await store.put("ops", { ...entry, status: "pending_validation" });
  try {
    const payload = await sendQueued(entry, state.auth.csrf_token);
    if (payload.operation?.op_id !== entry.id) {
      await store.put("ops", {
        ...entry,
        status: "dirty",
        last_error: "operation acceptance mismatch",
      });
      state.lastError = "operation acceptance mismatch";
      return "network_failed";
    }
    await acceptEntry(store, state, entry, payload);
    return "accepted";
  } catch (error) {
    if (!error.api) {
      await store.put("ops", { ...entry, status: "dirty", last_error: String(error) });
      state.online = false;
      state.lastError = String(error);
      return "network_failed";
    }
    await store.put("ops", {
      ...entry,
      status: "rejected",
      last_error: error.api.code,
    });
    state.lastError = error.api.message ?? error.api.code;
    return "rejected";
  }
}

async function acceptEntry(store, state, entry, payload) {
  await store.delete("ops", entry.id);
  if (entry.kind === "node.create") {
    await store.put("nodes", { ...payload.node, dirty: false });
    state.currentNodeId = payload.node.id;
  }
  if (entry.kind.startsWith("text.")) {
    await store.put("text_snapshots", {
      id: entry.node_id,
      state: payload.state,
      updated_by: entry.id,
    });
    state.text = payload.state.content;
  }
  state.online = true;
  state.lastError = "";
}
