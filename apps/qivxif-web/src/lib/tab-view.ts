export function tabKindToPanel(kind: string) {
  const panels: Record<string, string> = {
    diagnostics: "diagnostics",
    graph_map: "graph-map",
    graph_node: "graph",
    history: "history",
    login: "login",
    media: "media",
    media_asset: "media",
    new_tab: "new-tab",
    publishing: "publish",
    settings: "settings",
    setup: "setup",
    social_feed: "social",
    sync_status: "sync",
    text_editor: "editor",
    welcome: "welcome",
  };
  return panels[kind] ?? "welcome";
}

export function stateForTab(state, tab) {
  if (state.setupRequired) {
    return { ...state, activePaneId: tab.pane_node_id, activeTabId: "setup" };
  }
  let activeTabId = tabKindToPanel(tab.pane_kind);
  if (tab.pane_node_id?.startsWith("local_") && state.activeTabId && state.activeTabId !== "welcome") {
    activeTabId = state.activeTabId;
  }
  const tabState = { ...state, activePaneId: tab.pane_node_id, activeTabId };
  if ((tab.target_node_id || state.currentNodeId) && tabState.activeTabId === "editor") {
    tabState.currentNodeId = tab.target_node_id || state.currentNodeId;
    const snapshot = state.textSnapshots?.[tab.target_node_id];
    const hasDraft = Object.prototype.hasOwnProperty.call(state.tabDrafts ?? {}, tab.pane_node_id);
    tabState.text = hasDraft ? state.tabDrafts[tab.pane_node_id] : snapshot?.state?.content ?? "";
    tabState.textDirty = hasDraft || Boolean(snapshot?.dirty);
  }
  if (tab.target_node_id && tabState.activeTabId === "graph") {
    tabState.currentNodeId = tab.target_node_id;
  }
  if (tab.target_node_id && tabState.activeTabId === "graph-map") {
    tabState.activeGraphMapId = tab.target_node_id;
  }
  return tabState;
}

export function actionsForTab(actions, tab) {
  const context = {
    paneId: tab?.pane_node_id ?? "",
    paneKind: tab?.pane_kind ?? "",
    targetNodeId: tab?.target_node_id ?? "",
  };
  return {
    ...actions,
    addCurrentNodeToGraphMap: () => actions.addCurrentNodeToGraphMap?.(context),
    createGraphMap: () => actions.createGraphMap?.(context),
    openTab: (tabId: string, paneId = tab.pane_node_id) => actions.openTab?.(tabId, paneId),
    saveText: (content: string) => actions.saveText?.(content, tab.target_node_id, tab.pane_node_id),
    updateTextDraft: (content: string) => actions.updateTextDraft?.(tab.pane_node_id, content),
  };
}

export function boundedActive(stack) {
  return Math.max(0, Math.min(stack.active ?? 0, stack.tabs.length - 1));
}

export function tabLabel(tab) {
  const labels: Record<string, string> = {
    graph_map: "Graph Map",
    login: "Login",
    new_tab: "New Tab",
    media: "Media",
    media_asset: "Media",
    profile: "Profile",
    setup: "Setup",
    text_editor: "Editor",
  };
  return labels[tab.pane_kind] ?? tab.title ?? tab.pane_kind;
}
