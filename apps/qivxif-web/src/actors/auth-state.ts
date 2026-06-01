export async function storeAuthPayload(store, state, payload) {
  state.auth = payload;
  await store.put("sync_cursors", { id: "auth", auth: payload });
  await store.put("sync_cursors", {
    id: "actor_seq",
    value: Math.max(0, (payload.next_actor_seq ?? 1) - 1),
  });
}
