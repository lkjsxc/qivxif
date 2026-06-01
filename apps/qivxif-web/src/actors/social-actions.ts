import { reserveActorSeq } from "./actor-seq.ts";
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

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
