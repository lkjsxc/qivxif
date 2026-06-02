export function updatePaneScroll(store, state, paneId, scrollTop) {
  if (!paneId) {
    return;
  }
  const top = Math.max(0, Math.round(Number(scrollTop) || 0));
  if (state.tabScrolls[paneId] === top) {
    return;
  }
  state.tabScrolls[paneId] = top;
  store.put("tab_snapshots", {
    id: scrollKey(paneId),
    kind: "pane_scroll",
    pane_id: paneId,
    scroll_top: top,
  }).catch((error) => {
    state.lastError = String(error);
  });
}

function scrollKey(paneId) {
  return `pane_scroll:${paneId}`;
}
