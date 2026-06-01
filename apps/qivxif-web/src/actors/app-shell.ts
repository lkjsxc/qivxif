import { login, serverInfo } from "../http/client.ts";
import { generateId } from "../ids.ts";
import { openLocalStore } from "../store/indexed-db.ts";
import { renderWorkspace } from "../ui/workspace.ts";
import { addCurrentNodeToBoard, createBoard, linkBoardNodes, moveBoardItem } from "./board-actions.ts";
import { reserveActorSeq } from "./actor-seq.ts";
import { textNodeCreateEntry, textRestoreEntry } from "./local-operations.ts";
import { loadLocalState, refreshCurrentNode } from "./state-loader.ts";
import { flushQueue, refreshQueueState } from "./sync.ts";
import { closePane, maximizePane, splitPane, stackTab } from "./workspace-actions.ts";

export async function startAppShell(root) {
  if (!root) {
    return;
  }
  const state = {
    online: navigator.onLine,
    capabilities: [],
    edges: [],
    nodes: [],
    queued: 0,
    rejected: 0,
    lastError: "",
    auth: null,
    currentNodeId: "",
    history: [],
    text: "",
    textDirty: false,
    activeBoardId: "",
    layout: null,
    layoutDirty: false,
    layoutNodeId: "",
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
    openNode: (nodeId) => run(root, store, state, () => openNode(store, state, nodeId)),
    saveText: (content) => run(root, store, state, () => saveText(store, state, content)),
    selectNode: (nodeId) => run(root, store, state, () => selectNode(store, state, nodeId)),
    sync: () => run(root, store, state, () => flushQueue(store, state)),
    addCurrentNodeToBoard: () => run(root, store, state, () => addCurrentNodeToBoard(store, state)),
    closePane: () => run(root, store, state, () => closePane(store, state)),
    createBoard: () => run(root, store, state, () => createBoard(store, state)),
    linkBoardNodes: () => run(root, store, state, () => linkBoardNodes(store, state)),
    maximizePane: () => run(root, store, state, () => maximizePane(store, state)),
    moveBoardItem: () => run(root, store, state, () => moveBoardItem(store, state)),
    splitPane: () => run(root, store, state, () => splitPane(store, state)),
    stackTab: () => run(root, store, state, () => stackTab(store, state)),
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
    if (state.online && state.currentNodeId) {
      await refreshCurrentNode(store, state);
    }
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
  const created = textNodeCreateEntry(actorSeq);
  await store.put("ops", created.entry);
  await store.put("nodes", created.node);
  await store.put("workspace_layout", { id: "current_node", node_id: created.node.id });
  state.currentNodeId = created.node.id;
}

async function saveText(store, state, content) {
  requireAuth(state);
  if (!state.currentNodeId) {
    throw new Error("select a text node first");
  }
  const actorSeq = await reserveActorSeq(store);
  const docId = await textDocId(store, state.currentNodeId);
  const restored = textRestoreEntry(
    actorSeq,
    state.currentNodeId,
    docId,
    state.auth.user.actor_id,
    content,
  );
  await store.put("ops", restored.entry);
  await store.put("text_snapshots", { id: state.currentNodeId, doc_id: docId, state: { content }, dirty: true });
  state.text = content;
}

async function openNode(store, state, nodeId) {
  if (!nodeId) {
    throw new Error("node id is required");
  }
  state.currentNodeId = nodeId;
  await store.put("workspace_layout", { id: "current_node", node_id: nodeId });
  await refreshCurrentNode(store, state);
}

async function selectNode(store, state, nodeId) {
  state.currentNodeId = nodeId;
  await store.put("workspace_layout", { id: "current_node", node_id: nodeId });
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

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
