export function updateTextDraft(store, state, paneId, content) {
  if (!paneId) {
    return;
  }
  state.tabDrafts[paneId] = content;
  store.put("tab_snapshots", {
    content,
    id: draftKey(paneId),
    kind: "text_draft",
    pane_id: paneId,
  }).catch((error) => {
    state.lastError = String(error);
  });
}

export async function clearTextDraft(store, state, paneId) {
  if (!paneId) {
    return;
  }
  delete state.tabDrafts[paneId];
  await store.delete("tab_snapshots", draftKey(paneId));
}

function draftKey(paneId) {
  return `text_draft:${paneId}`;
}
