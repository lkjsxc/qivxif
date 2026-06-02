export function tabKindToPanel(kind: string) {
  const panels: Record<string, string> = {
    diagnostics: "diagnostics",
    graph_board: "board",
    graph_node: "graph",
    history: "history",
    login: "login",
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
  if (tab.pane_kind === "welcome" && state.activeTabId && state.activeTabId !== "welcome") {
    activeTabId = state.activeTabId;
  }
  if (tab.pane_kind === "welcome" && state.currentNodeId && state.activeTabId === "editor") {
    activeTabId = "editor";
  }
  const tabState = { ...state, activePaneId: tab.pane_node_id, activeTabId };
  if ((tab.target_node_id || state.currentNodeId) && tabState.activeTabId === "editor") {
    tabState.currentNodeId = tab.target_node_id || state.currentNodeId;
    if (tab.target_node_id) {
      tabState.currentNodeId = tab.target_node_id;
    }
    const snapshot = state.textSnapshots?.[tab.target_node_id];
    const hasDraft = Object.prototype.hasOwnProperty.call(state.tabDrafts ?? {}, tab.pane_node_id);
    tabState.text = hasDraft ? state.tabDrafts[tab.pane_node_id] : snapshot?.state?.content ?? "";
    tabState.textDirty = hasDraft || Boolean(snapshot?.dirty);
  }
  if (tab.target_node_id && tabState.activeTabId === "graph") {
    tabState.currentNodeId = tab.target_node_id;
  }
  if (tab.target_node_id && tabState.activeTabId === "board") {
    tabState.activeBoardId = tab.target_node_id;
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
    addCurrentNodeToBoard: () => actions.addCurrentNodeToBoard?.(context),
    createBoard: () => actions.createBoard?.(context),
    saveText: (content: string) => actions.saveText?.(content, tab.target_node_id, tab.pane_node_id),
    updateTextDraft: (content: string) => actions.updateTextDraft?.(tab.pane_node_id, content),
  };
}

export function boundedActive(stack) {
  return Math.max(0, Math.min(stack.active ?? 0, stack.tabs.length - 1));
}

export function tabLabel(tab) {
  const labels: Record<string, string> = {
    login: "Login",
    setup: "Setup",
    text_editor: "Editor",
  };
  return labels[tab.pane_kind] ?? tab.title ?? tab.pane_kind;
}
