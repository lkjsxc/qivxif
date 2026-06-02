import { activePaneId, visibleRoot } from "../domain/tile-tree.ts";
import { actionButton, text } from "./dom.ts";
import { installDropLayer } from "./drop-layer.ts";
import { renderTabStack } from "./tab-stack.ts";
import { renderStackTabRail, tabsFor } from "./tab-rail.ts";
import { installResizeHandle } from "./resize-handle.ts";

export function renderTileGrid(state, actions) {
  const grid = document.createElement("section");
  grid.className = "tile-grid workspace-main";
  const root = visibleRoot(state.layout) ?? defaultTile(state);
  grid.append(renderTile(root, state, actions));
  return grid;
}

function renderTile(tile, state, actions) {
  if (tile.kind === "split") {
    return renderSplit(tile, state, actions);
  }
  return renderPane(tile, state, actions);
}

function renderSplit(tile, state, actions) {
  const split = document.createElement("section");
  split.className = `tile-split ${tile.axis === "column" ? "column" : "row"}`;
  const sizes = tile.sizes?.length === tile.children.length ? tile.sizes : tile.children.map(() => 500);
  tile.children.forEach((child, index) => {
    const childWrap = document.createElement("div");
    childWrap.className = "tile-split-child";
    childWrap.style.flex = `${sizes[index]} 1 0`;
    childWrap.append(renderTile(child, state, actions));
    split.append(childWrap);
    if (index < tile.children.length - 1) {
      split.append(
        installResizeHandle(tile, index, sizes, (nextSizes) => {
          const paneId = activePaneId(tile.children[index]);
          actions.resizeSplit?.(paneId, nextSizes);
        }),
      );
    }
  });
  return split;
}

function renderPane(stack, state, actions) {
  const activeTab = stack.tabs[boundedActive(stack)] ?? null;
  const article = document.createElement("article");
  article.className = "pane tile";
  if (activeTab?.pane_node_id) {
    article.dataset.paneId = activeTab.pane_node_id;
  }
  const head = document.createElement("header");
  head.className = "pane-head tile-header";
  head.append(renderStackTabRail(stack, actions), tileControls(actions, activeTab));
  if (state.tabChooserOpen && state.tabChooserPaneId === (activeTab?.pane_node_id ?? "")) {
    head.append(tabChooser(state, actions, activeTab?.pane_node_id));
  }
  const body = document.createElement("section");
  body.className = "pane-stack";
  installDropLayer(article, activeTab?.pane_node_id, actions, head, body);
  body.append(renderTabStack(stack, state, actions));
  article.append(head, body);
  return article;
}

function tileControls(actions, activeTab) {
  const controls = document.createElement("div");
  const paneId = activeTab?.pane_node_id ?? "";
  const context = paneContext(activeTab);
  controls.className = "tile-controls";
  controls.append(
    actionButton("+", () => actions.toggleTabChooser?.(paneId), "icon-button tile-add"),
    actionButton("Split pane", () => actions.splitPane?.(paneId, context), "tile-menu"),
    actionButton("Stack tab", () => actions.stackTab?.(paneId, context), "tile-menu"),
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

function paneContext(tab) {
  return {
    paneId: tab?.pane_node_id ?? "",
    paneKind: tab?.pane_kind ?? "",
    targetNodeId: tab?.target_node_id ?? "",
  };
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
    settings: "settings",
    social: "social_feed",
    sync: "sync_status",
    welcome: "welcome",
  };
  return kinds[panel] ?? "welcome";
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
    settings: "Settings",
    social: "Social",
    sync: "Sync",
    welcome: "Welcome",
  };
  return titles[panel] ?? "Welcome";
}
