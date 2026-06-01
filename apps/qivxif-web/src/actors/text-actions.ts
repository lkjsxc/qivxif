import { generateId } from "../ids.ts";
import { refreshCurrentNode } from "./state-loader.ts";
import { reserveActorSeq } from "./actor-seq.ts";
import { textNodeCreateEntry, textRestoreEntry } from "./local-operations.ts";

export async function createTextNode(store, state) {
  requireAuth(state);
  const actorSeq = await reserveActorSeq(store);
  const created = textNodeCreateEntry(actorSeq);
  await store.put("ops", created.entry);
  await store.put("nodes", created.node);
  await store.put("tile_layout", { id: "current_node", node_id: created.node.id });
  state.currentNodeId = created.node.id;
  state.activeTabId = "editor";
}

export async function saveText(store, state, content) {
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
  await store.put("text_snapshots", {
    dirty: true,
    doc_id: docId,
    id: state.currentNodeId,
    state: { content },
  });
  state.text = content;
}

export async function openNode(store, state, nodeId) {
  if (!nodeId) {
    throw new Error("node id is required");
  }
  state.currentNodeId = nodeId;
  await store.put("tile_layout", { id: "current_node", node_id: nodeId });
  await refreshCurrentNode(store, state);
}

export async function selectNode(store, state, nodeId) {
  const node = await store.get("nodes", nodeId);
  if (node?.kind === "blog_post") {
    await selectBlogNode(store, state, node, nodeId);
    return;
  }
  state.currentNodeId = nodeId;
  await store.put("tile_layout", { id: "current_node", node_id: nodeId });
}

async function selectBlogNode(store, state, node, nodeId) {
  state.currentBlogPostId = nodeId;
  await store.put("tile_layout", { id: "current_blog_post", node_id: nodeId });
  const bodyNodeId = node.metadata_map?.body_node_id;
  if (bodyNodeId) {
    state.currentNodeId = bodyNodeId;
    await store.put("tile_layout", { id: "current_node", node_id: bodyNodeId });
  }
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
