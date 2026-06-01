import { moveTabToEdge, moveTabToStack } from "../domain/tile-move.ts";
import { reserveActorSeq } from "./actor-seq.ts";
import { tileLayoutSetEntry } from "./local-events.ts";
import { ensureLayout } from "./tile-actions.ts";

export async function movePane(store, state, sourcePaneId, targetPaneId, zone) {
  requireAuth(state);
  if (!sourcePaneId || !targetPaneId) {
    return;
  }
  const model = await ensureLayout(store, state);
  const next =
    zone === "center"
      ? moveTabToStack(model.layout, sourcePaneId, targetPaneId)
      : moveTabToEdge(model.layout, sourcePaneId, targetPaneId, zone);
  const created = tileLayoutSetEntry(await reserveActorSeq(store), model.layout_node_id, next);
  await store.put("events", created.entry);
  await store.put("tile_layout", created.layoutRecord);
  state.layout = next;
  state.layoutNodeId = model.layout_node_id;
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
