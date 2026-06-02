import { moveTabNearTab, moveTabToEdge, moveTabToStack } from "../domain/tile-move.ts";
import { ensureLayout } from "./tile-actions.ts";
import { queueLayout } from "./tile-helpers.ts";

export async function movePane(store, state, sourcePaneId, targetPaneId, zone) {
  requireAuth(state);
  if (!sourcePaneId || !targetPaneId) {
    return;
  }
  const model = await ensureLayout(store, state);
  const next = nextLayout(model.layout, sourcePaneId, targetPaneId, zone);
  state.activePaneId = sourcePaneId;
  if (sameLayout(model.layout, next)) {
    return;
  }
  await queueLayout(store, state, model.layout_node_id, next);
}

function nextLayout(layout, sourcePaneId, targetPaneId, zone) {
  try {
    if (zone === "tab-before") {
      return moveTabNearTab(layout, sourcePaneId, targetPaneId, "before");
    }
    if (zone === "tab-after") {
      return moveTabNearTab(layout, sourcePaneId, targetPaneId, "after");
    }
    return zone === "center"
      ? moveTabToStack(layout, sourcePaneId, targetPaneId)
      : moveTabToEdge(layout, sourcePaneId, targetPaneId, zone);
  } catch (error) {
    return layout;
  }
}

function sameLayout(left, right) {
  return JSON.stringify(left) === JSON.stringify(right);
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
