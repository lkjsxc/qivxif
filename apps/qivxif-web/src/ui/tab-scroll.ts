export function installPaneScrollSnapshot(body, paneId, state, actions) {
  if (!paneId) {
    return;
  }
  const top = Math.max(0, Number(state.tabScrolls?.[paneId] ?? 0));
  requestAnimationFrame(() => {
    if (body.isConnected) {
      body.scrollTop = top;
    }
  });
  body.addEventListener(
    "scroll",
    () => {
      actions.updatePaneScroll?.(paneId, body.scrollTop);
    },
    { passive: true },
  );
}
