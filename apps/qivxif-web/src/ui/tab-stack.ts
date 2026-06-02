import { renderTabContent } from "./tab-content.ts";
import { installPaneScrollSnapshot } from "./tab-scroll.ts";

export function renderTabStack(stack, state, actions) {
  const mount = document.createElement("div");
  mount.className = "tab-stack-mount";
  stack.tabs.forEach((tab, index) => {
    const body = document.createElement("section");
    body.className = "tab-body";
    body.dataset.paneId = tab.pane_node_id;
    const active = index === boundedActive(stack);
    body.hidden = !active;
    body.setAttribute("aria-hidden", active ? "false" : "true");
    body.append(renderTabContent(stateForTab(state, tab), actionsForTab(actions, tab)));
    installPaneScrollSnapshot(body, tab.pane_node_id, state, actions);
    mount.append(body);
  });
  return mount;
}

function stateForTab(state, tab) {
  let activeTabId = tabKindToPanel(tab.pane_kind);
  if (tab.pane_kind === "welcome" && state.currentNodeId && state.activeTabId === "editor") {
    activeTabId = "editor";
  }
  const tabState = { ...state, activePaneId: tab.pane_node_id, activeTabId };
  if ((tab.target_node_id || state.currentNodeId) && tabState.activeTabId === "editor") {
    tabState.currentNodeId = tab.target_node_id || state.currentNodeId;
    tabState.currentNodeId = tab.target_node_id;
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

function actionsForTab(actions, tab) {
  const context = {
    paneId: tab?.pane_node_id ?? "",
    paneKind: tab?.pane_kind ?? "",
    targetNodeId: tab?.target_node_id ?? "",
  };
  return {
    ...actions,
    addCurrentNodeToBoard: () => actions.addCurrentNodeToBoard?.(context),
    createBoard: () => actions.createBoard?.(context),
    saveText: (content) => actions.saveText?.(content, tab.target_node_id, tab.pane_node_id),
    updateTextDraft: (content) => actions.updateTextDraft?.(tab.pane_node_id, content),
  };
}

function tabKindToPanel(kind) {
  const panels = {
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

function boundedActive(stack) {
  return Math.max(0, Math.min(stack.active ?? 0, stack.tabs.length - 1));
}
