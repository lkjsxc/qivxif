import { reserveActorSeq } from "./actor-seq.ts";
import { generateId } from "../ids.ts";
import { shortPostCreateEntry } from "./local-operations.ts";

export async function createShortPost(store, state, body) {
  requireAuth(state);
  const safeBody = body.trim();
  if (!safeBody) {
    throw new Error("short post body is required");
  }
  const actorSeq = await reserveActorSeq(store);
  const created = shortPostCreateEntry(actorSeq, state.auth.user, safeBody);
  await store.put("ops", created.entry);
  await store.put("nodes", created.node);
  await store.put("feed_windows", {
    dirty: true,
    id: created.feedItem.operation_id,
    item: created.feedItem,
  });
}

export async function followProfile(store, state, targetProfileNodeId) {
  await queueProfileEdge(store, state, targetProfileNodeId, "social.follow", "/api/social/follow", "follows");
}

export async function muteProfile(store, state, targetProfileNodeId) {
  await queueProfileEdge(store, state, targetProfileNodeId, "social.mute", "/api/social/mute", "mutes");
}

export async function blockProfile(store, state, targetProfileNodeId) {
  await queueProfileEdge(store, state, targetProfileNodeId, "social.block", "/api/social/block", "blocks");
}

export async function clearSocialEdge(store, state, edgeId, kind) {
  requireAuth(state);
  const route = clearRoute(kind);
  const actorSeq = await reserveActorSeq(store);
  const opId = generateId("op");
  const request = { actor_seq: actorSeq, edge_id: edgeId, op_id: opId };
  await store.put("ops", queueEntry(opId, kind, actorSeq, edgeId, route, request));
  const edge = await store.get("edges", edgeId);
  if (edge) {
    await store.put("edges", {
      ...edge,
      dirty: true,
      tombstone: { by: state.auth.user.actor_id, reason: kind },
    });
  }
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}

async function queueProfileEdge(store, state, targetProfileNodeId, kind, path, edgeKind) {
  requireAuth(state);
  if (!state.auth.user.profile_node_id) {
    throw new Error("profile node id is required");
  }
  const target = targetProfileNodeId.trim();
  if (!target) {
    throw new Error("target profile node id is required");
  }
  const actorSeq = await reserveActorSeq(store);
  const edgeId = generateId("edg");
  const opId = generateId("op");
  const request = {
    actor_seq: actorSeq,
    edge_id: edgeId,
    op_id: opId,
    target_profile_node_id: target,
  };
  await store.put("ops", queueEntry(opId, kind, actorSeq, target, path, request));
  await store.put("edges", {
    id: edgeId,
    dirty: true,
    from_node: state.auth.user.profile_node_id,
    kind: edgeKind,
    metadata_map: { social_state: kind },
    to_node: target,
  });
}

function clearRoute(kind) {
  if (kind === "social.unfollow") {
    return "/api/social/unfollow";
  }
  if (kind === "social.unmute") {
    return "/api/social/unmute";
  }
  return "/api/social/unblock";
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
