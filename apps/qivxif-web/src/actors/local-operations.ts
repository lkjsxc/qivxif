import { generateId } from "../ids.ts";

export function textNodeCreateEntry(actorSeq) {
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
  return {
    entry: queueEntry(opId, "node.create", actorSeq, nodeId, "/api/nodes", request),
    node: {
      id: nodeId,
      dirty: true,
      kind: "text",
      metadata_map: request.metadata_map,
    },
  };
}

export function textRestoreEntry(actorSeq, nodeId, docId, actorId, content) {
  const opId = generateId("op");
  const path = `/api/text/${nodeId}/ops`;
  const request = {
    actor_seq: actorSeq,
    operation: {
      doc_id: docId,
      edit: {
        actor_id: actorId,
        content,
        first_seq: actorSeq * 1000000,
        kind: "restore",
      },
      op_id: opId,
    },
  };
  return {
    entry: queueEntry(opId, "text.restore", actorSeq, nodeId, path, request),
    request,
  };
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
