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
  const eventId = generateId("evt");
  const request = {
    actor_seq: actorSeq,
    kind,
    metadata_map: metadataMap,
    node_id: nodeId,
    event_id: eventId,
    visibility: "private",
  };
  return {
    entry: queueEntry(eventId, "node.create", actorSeq, nodeId, "/api/nodes", request),
    node: {
      id: nodeId,
      dirty: true,
      kind,
      metadata_map: request.metadata_map,
    },
  };
}

export function textRestoreEntry(actorSeq, nodeId, docId, actorId, content) {
  const eventId = generateId("evt");
  const path = `/api/text/${nodeId}/events`;
  const request = {
    actor_seq: actorSeq,
    event: {
      doc_id: docId,
      edit: {
        actor_id: actorId,
        content,
        first_seq: actorSeq * 1000000,
        kind: "restore",
      },
      event_id: eventId,
    },
  };
  return {
    entry: queueEntry(eventId, "text.restore", actorSeq, nodeId, path, request),
    request,
  };
}

export function edgeCreateEntry(actorSeq, fromNode, toNode, kind, metadataMap) {
  const edgeId = generateId("edg");
  const eventId = generateId("evt");
  const request = {
    actor_seq: actorSeq,
    edge_id: edgeId,
    from_node: fromNode,
    kind,
    metadata_map: metadataMap,
    event_id: eventId,
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
    entry: queueEntry(eventId, "edge.create", actorSeq, fromNode, "/api/edges", request),
  };
}

export function tileLayoutSetEntry(actorSeq, layoutNodeId, layout) {
  const eventId = generateId("evt");
  const request = {
    actor_seq: actorSeq,
    layout,
    layout_node_id: layoutNodeId,
    event_id: eventId,
  };
  return {
    entry: queueEntry(
      eventId,
      "tile.layout_set",
      actorSeq,
      layoutNodeId,
      "/api/tile-layout",
      request,
    ),
    layoutRecord: {
      dirty: true,
      id: "tile_model",
      layout,
      layout_node_id: layoutNodeId,
    },
  };
}

export function publishPostEntry(actorSeq, postNodeId, slug, summary) {
  const eventId = generateId("evt");
  const request = { actor_seq: actorSeq, event_id: eventId, slug, summary };
  return {
    entry: queueEntry(eventId, "publish.post", actorSeq, postNodeId, `/api/publish/${postNodeId}`, request),
  };
}

export function shortPostCreateEntry(actorSeq, user, body) {
  const nodeId = generateId("nod");
  const eventId = generateId("evt");
  const request = {
    actor_seq: actorSeq,
    body,
    node_id: nodeId,
    event_id: eventId,
    reply_to: null,
    visibility: "public",
  };
  return {
    entry: queueEntry(eventId, "social.short_post_create", actorSeq, nodeId, "/api/social/short-posts", request),
    feedItem: {
      author_name: user.name,
      author_user_id: user.user_id,
      body,
      created_at: new Date().toISOString(),
      event_id: eventId,
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
  const eventId = generateId("evt");
  const request = { actor_seq: actorSeq, event_id: eventId, reason };
  return {
    entry: queueEntry(
      eventId,
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
