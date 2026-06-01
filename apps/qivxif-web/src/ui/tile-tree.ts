import { activePaneId, visibleRoot } from "../domain/tile-tree.ts";
import { actionButton, text } from "./dom.ts";
import { installDropLayer } from "./drop-layer.ts";
import { renderTabContent } from "./tab-content.ts";
import { renderStackTabRail, tabsFor } from "./tab-rail.ts";

export function renderTileGrid(state, actions) {
  const grid = document.createElement("section");
  grid.className = "tile-grid";
  const root = visibleRoot(state.layout) ?? defaultTile(state);
  grid.append(renderTile(root, state, actions));
  return grid;
}

function renderTile(tile, state, actions) {
  if (tile.kind === "split") {
    const split = document.createElement("section");
    split.className = `tile-split ${tile.axis === "column" ? "column" : "row"}`;
    split.append(renderTile(tile.first, state, actions), renderTile(tile.second, state, actions));
    return split;
  }
  const activeTab = tile.tabs[boundedActive(tile)] ?? null;
  const article = document.createElement("article");
  article.className = "tile";
  if (activeTab?.pane_node_id) {
    article.dataset.paneId = activeTab.pane_node_id;
  }
  installDropLayer(article, activeTab?.pane_node_id, actions);
  article.append(tileHeader(tile, state, actions, activeTab), tabBody(state, actions, activeTab));
  return article;
}

function tileHeader(stack, state, actions, activeTab) {
  const header = document.createElement("div");
  const paneId = activeTab?.pane_node_id ?? activePaneId(stack);
  header.className = "tile-header";
  header.append(renderStackTabRail(stack, actions), tileControls(actions, paneId));
  if (state.tabChooserOpen && state.tabChooserPaneId === paneId) {
    header.append(tabChooser(state, actions, paneId));
  }
  return header;
}

function tileControls(actions, paneId) {
  const controls = document.createElement("div");
  controls.className = "tile-controls";
  controls.append(
    actionButton("+", () => actions.toggleTabChooser?.(paneId), "icon-button tile-add"),
    actionButton("Split pane", () => actions.splitPane?.(paneId), "tile-menu"),
    actionButton("Stack tab", () => actions.stackTab?.(paneId), "tile-menu"),
    actionButton("Maximize pane", () => actions.maximizePane?.(paneId), "tile-menu"),
    actionButton("Close pane", () => actions.closePane?.(paneId), "tile-menu"),
  );
  return controls;
}

function tabChooser(state, actions, paneId) {
  const chooser = document.createElement("div");
  chooser.className = "tab-chooser";
  for (const tab of tabsFor(state)) {
    chooser.append(actionButton(tab.label, () => actions.openTab?.(tab.id, paneId), "chooser-tab"));
  }
  return chooser;
}

function tabBody(state, actions, activeTab) {
  const body = document.createElement("section");
  body.className = "tab-body";
  if (!activeTab) {
    body.append(text("No tab is open."));
    return body;
  }
  body.dataset.paneId = activeTab.pane_node_id;
  body.append(renderTabContent(stateForTab(state, activeTab), actionsForTab(actions, activeTab)));
  return body;
}

function stateForTab(state, tab) {
  const tabState = { ...state, activeTabId: tabKindToPanel(tab.pane_kind) };
  if (tab.target_node_id && tabState.activeTabId === "editor") {
    tabState.currentNodeId = tab.target_node_id;
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
  return {
    ...actions,
    saveText: (content) => actions.saveText?.(content, tab.target_node_id),
  };
}

function tabKindToPanel(kind) {
  const panels = {
    diagnostics: "diagnostics",
    graph_board: "board",
    graph_node: "graph",
    history: "history",
    home: "home",
    login: "login",
    publishing: "publish",
    settings: "diagnostics",
    setup: "setup",
    social_feed: "social",
    sync_status: "sync",
    text_editor: "editor",
  };
  return panels[kind] ?? "home";
}

function boundedActive(stack) {
  return Math.max(0, Math.min(stack.active ?? 0, stack.tabs.length - 1));
}

function defaultTile(state) {
  const panel = state.setupRequired ? "setup" : state.activeTabId;
  const paneKind = paneKindForPanel(panel);
  return {
    active: 0,
    kind: "stack",
    tabs: [
      {
        pane_kind: paneKind,
        pane_node_id: `local_${paneKind}`,
        target_node_id: state.currentNodeId || state.activeBoardId || null,
        title: titleForPanel(panel),
      },
    ],
  };
}

function paneKindForPanel(panel) {
  const kinds = {
    board: "graph_board",
    diagnostics: "diagnostics",
    editor: "text_editor",
    graph: "graph_node",
    history: "history",
    login: "login",
    publish: "publishing",
    setup: "setup",
    social: "social_feed",
    sync: "sync_status",
  };
  return kinds[panel] ?? "home";
}

function titleForPanel(panel) {
  const titles = {
    board: "Board",
    diagnostics: "Diagnostics",
    editor: "Editor",
    graph: "Graph",
    history: "History",
    login: "Login",
    publish: "Publish",
    setup: "Setup",
    social: "Social",
    sync: "Sync",
  };
  return titles[panel] ?? "Home";
}
