import { generateId } from "../ids.ts";

export function textNodeCreateEntry(actorSeq) {
  return nodeCreateEntry(actorSeq, "text", { title: "Untitled text" });
}

export function blogPostCreateEntry(actorSeq, bodyNodeId, title) {
  return nodeCreateEntry(actorSeq, "blog_post", {
    body_node_id: bodyNodeId,
    publication_state: "draft",
    title,
  });
}

export function nodeCreateEntry(actorSeq, kind, metadataMap) {
  const nodeId = generateId("nod");
  const opId = generateId("op");
  const request = {
    actor_seq: actorSeq,
    kind,
    metadata_map: metadataMap,
    node_id: nodeId,
    op_id: opId,
    visibility: "private",
  };
  return {
    entry: queueEntry(opId, "node.create", actorSeq, nodeId, "/api/nodes", request),
    node: {
      id: nodeId,
      dirty: true,
      kind,
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

export function edgeCreateEntry(actorSeq, fromNode, toNode, kind, metadataMap) {
  const edgeId = generateId("edg");
  const opId = generateId("op");
  const request = {
    actor_seq: actorSeq,
    edge_id: edgeId,
    from_node: fromNode,
    kind,
    metadata_map: metadataMap,
    op_id: opId,
    to_node: toNode,
  };
  return {
    edge: {
      id: edgeId,
      dirty: true,
      from_node: fromNode,
      kind,
      metadata_map: metadataMap,
      to_node: toNode,
    },
    entry: queueEntry(opId, "edge.create", actorSeq, fromNode, "/api/edges", request),
  };
}

export function workspaceLayoutSetEntry(actorSeq, layoutNodeId, layout) {
  const opId = generateId("op");
  const request = {
    actor_seq: actorSeq,
    layout,
    layout_node_id: layoutNodeId,
    op_id: opId,
  };
  return {
    entry: queueEntry(
      opId,
      "workspace.layout_set",
      actorSeq,
      layoutNodeId,
      "/api/workspace/layout",
      request,
    ),
    layoutRecord: {
      dirty: true,
      id: "workspace_model",
      layout,
      layout_node_id: layoutNodeId,
    },
  };
}

export function publishPostEntry(actorSeq, postNodeId, slug, summary) {
  const opId = generateId("op");
  const request = { actor_seq: actorSeq, op_id: opId, slug, summary };
  return {
    entry: queueEntry(opId, "publish.post", actorSeq, postNodeId, `/api/publish/${postNodeId}`, request),
  };
}

export function shortPostCreateEntry(actorSeq, user, body) {
  const nodeId = generateId("nod");
  const opId = generateId("op");
  const request = {
    actor_seq: actorSeq,
    body,
    node_id: nodeId,
    op_id: opId,
    reply_to: null,
    visibility: "public",
  };
  return {
    entry: queueEntry(opId, "social.short_post_create", actorSeq, nodeId, "/api/social/short-posts", request),
    feedItem: {
      author_name: user.name,
      author_user_id: user.user_id,
      body,
      created_at: new Date().toISOString(),
      operation_id: opId,
      post_node_id: nodeId,
      visibility: "public",
    },
    node: {
      id: nodeId,
      dirty: true,
      kind: "short_post",
      metadata_map: { author_name: user.name, body, social_state: "posted" },
    },
  };
}

export function unpublishPostEntry(actorSeq, postNodeId, reason) {
  const opId = generateId("op");
  const request = { actor_seq: actorSeq, op_id: opId, reason };
  return {
    entry: queueEntry(
      opId,
      "publish.unpublish",
      actorSeq,
      postNodeId,
      `/api/unpublish/${postNodeId}`,
      request,
    ),
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
