export async function reserveActorSeq(store) {
  const current = await store.get("sync_cursors", "actor_seq");
  const value = (current?.value ?? 0) + 1;
  await store.put("sync_cursors", { id: "actor_seq", value });
  return value;
}
