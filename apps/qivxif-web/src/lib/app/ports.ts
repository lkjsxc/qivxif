/** Port surface documented in docs/architecture/client/surface-boundary.md */
export function createPorts(store, api) {
  return {
    loadLocalWorkspace: () => store.get("local_workspace", "workspace"),
    saveLocalWorkspace: (state) =>
      store.put("local_workspace", {
        id: "workspace",
        layout: state.layout,
        layoutNodeId: state.layoutNodeId,
        tabDrafts: state.tabDrafts,
        tabScrolls: state.tabScrolls,
      }),
    getSetupStatus: api.setupStatus,
    submitOwnerSetup: api.createOwner,
    pushEvents: api.sendQueued,
    pullEvents: api.nodeHistory,
    registerServiceWorker: () => navigator.serviceWorker.register("/service-worker.js"),
  };
}
