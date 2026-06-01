import { login, serverInfo } from "../api/client.ts";
import { generateId } from "../ids.ts";
import { openLocalStore } from "../store/indexed-db.ts";
import { renderWorkspace } from "../ui/workspace.ts";
import { flushQueue, refreshQueueState } from "./sync.ts";

export async function startAppShell(root) {
  if (!root) {
    return;
  }
  const state = {
    online: navigator.onLine,
    capabilities: [],
    nodes: [],
    queued: 0,
    rejected: 0,
    lastError: "",
    auth: null,
    currentNodeId: "",
    text: "",
    textDirty: false,
  };
  renderWorkspace(root, state, {});
  const store = await openLocalStore();
  const actions = actionsFor(root, store, state);
  await loadLocalState(store, state);
  await refreshQueueState(store, state);
  try {
    const payload = await serverInfo();
    state.capabilities = payload.capabilities?.capabilities ?? [];
    state.online = true;
  } catch (error) {
    state.online = false;
    state.lastError = String(error);
  }
  await registerServiceWorker(state);
  renderWorkspace(root, state, actions);
}

async function registerServiceWorker(state) {
  if (!("serviceWorker" in navigator)) {
    return;
  }
  try {
    await navigator.serviceWorker.register("/service-worker.js");
  } catch (error) {
    state.lastError = String(error);
  }
}

function actionsFor(root, store, state) {
  return {
    createTextNode: () => run(root, store, state, () => createTextNode(store, state)),
    login: (name, password) => run(root, store, state, () => loginUser(store, state, name, password)),
    saveText: (content) => run(root, store, state, () => saveText(store, state, content)),
    selectNode: (nodeId) => run(root, store, state, () => selectNode(store, state, nodeId)),
    sync: () => run(root, store, state, () => flushQueue(store, state)),
  };
}

async function run(root, store, state, action) {
  const actions = actionsFor(root, store, state);
  try {
    await action();
    await loadLocalState(store, state);
    await refreshQueueState(store, state);
    await flushQueue(store, state);
    await loadLocalState(store, state);
  } catch (error) {
    state.lastError = error.api?.code ?? String(error);
  }
  renderWorkspace(root, state, actions);
}

async function loginUser(store, state, name, password) {
  const payload = await login(name, password);
  state.auth = payload;
  await store.put("sync_cursors", { id: "auth", auth: payload });
}

async function createTextNode(store, state) {
  requireAuth(state);
  const actorSeq = await reserveActorSeq(store);
  const nodeId = generateId("nod");
  const opId = generateId("op");
  const request = {
    actor_seq: actorSeq,
    kind: "text",
    metadata_map: { title: "Untitled text" },
    node_id: nodeId,
    op_id: opId,
    visibility: "private",
  };
  await store.put("ops", queueEntry(opId, "node.create", actorSeq, nodeId, "/api/nodes", request));
  await store.put("nodes", { id: nodeId, kind: "text", metadata_map: request.metadata_map, dirty: true });
  await store.put("workspace_layout", { id: "current_node", node_id: nodeId });
  state.currentNodeId = nodeId;
}

async function saveText(store, state, content) {
  requireAuth(state);
  if (!state.currentNodeId) {
    throw new Error("select a text node first");
  }
  const actorSeq = await reserveActorSeq(store);
  const opId = generateId("op");
  const docId = await textDocId(store, state.currentNodeId);
  const request = {
    actor_seq: actorSeq,
    operation: {
      doc_id: docId,
      edit: {
        actor_id: state.auth.user.actor_id,
        content,
        first_seq: actorSeq * 1000000,
        kind: "restore",
      },
      op_id: opId,
    },
  };
  const path = `/api/text/${state.currentNodeId}/ops`;
  await store.put("ops", queueEntry(opId, "text.restore", actorSeq, state.currentNodeId, path, request));
  await store.put("text_snapshots", { id: state.currentNodeId, doc_id: docId, state: { content }, dirty: true });
  state.text = content;
}

async function selectNode(store, state, nodeId) {
  state.currentNodeId = nodeId;
  await store.put("workspace_layout", { id: "current_node", node_id: nodeId });
}

async function loadLocalState(store, state) {
  const auth = await store.get("sync_cursors", "auth");
  const current = await store.get("workspace_layout", "current_node");
  state.auth = auth?.auth ?? state.auth;
  state.nodes = await store.all("nodes");
  state.currentNodeId = current?.node_id ?? state.currentNodeId;
  const text = state.currentNodeId ? await store.get("text_snapshots", state.currentNodeId) : null;
  state.text = text?.state?.content ?? "";
  state.textDirty = text?.dirty ?? false;
}

async function reserveActorSeq(store) {
  const current = await store.get("sync_cursors", "actor_seq");
  const value = (current?.value ?? 0) + 1;
  await store.put("sync_cursors", { id: "actor_seq", value });
  return value;
}

async function textDocId(store, nodeId) {
  const key = `text_doc:${nodeId}`;
  const current = await store.get("sync_cursors", key);
  if (current?.doc_id) {
    return current.doc_id;
  }
  const docId = generateId("txt");
  await store.put("sync_cursors", { id: key, doc_id: docId });
  return docId;
}

function queueEntry(id, kind, actorSeq, nodeId, path, request) {
  return {
    id,
    actor_seq: actorSeq,
    created_at: new Date().toISOString(),
    kind,
    node_id: nodeId,
    request,
    route: { method: "POST", path },
    status: "dirty",
  };
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
