import { actionButton } from "./dom.ts";
import { renderTabContent } from "./tab-content.ts";
import { renderTabRail, tabsFor } from "./tab-rail.ts";

export function renderTileGrid(state, actions) {
  const grid = document.createElement("section");
  grid.className = "tile-grid";
  grid.append(renderTile(state.layout?.root ?? defaultTile(), state, actions, true));
  return grid;
}

function renderTile(tile, state, actions, primary) {
  if (tile.kind === "split") {
    const split = document.createElement("section");
    split.className = `tile-split ${tile.axis === "column" ? "column" : "row"}`;
    split.append(
      renderTile(tile.first, state, actions, primary),
      renderTile(tile.second, state, actions, false),
    );
    return split;
  }
  const article = document.createElement("article");
  article.className = "tile";
  article.append(tileHeader(state, actions, primary), tabBody(state, actions, primary));
  return article;
}

function tileHeader(state, actions, primary) {
  const header = document.createElement("div");
  header.className = "tile-header";
  header.append(renderTabRail(state, actions));
  if (primary) {
    header.append(tileControls(actions));
  }
  if (state.tabChooserOpen) {
    header.append(tabChooser(state, actions));
  }
  return header;
}

function tileControls(actions) {
  const controls = document.createElement("div");
  controls.className = "tile-controls";
  controls.append(
    actionButton("+", () => actions.toggleTabChooser?.(), "icon-button tile-add"),
    actionButton("Split pane", () => actions.splitPane?.(), "tile-menu"),
    actionButton("Stack tab", () => actions.stackTab?.(), "tile-menu"),
    actionButton("Maximize pane", () => actions.maximizePane?.(), "tile-menu"),
    actionButton("Close pane", () => actions.closePane?.(), "tile-menu"),
  );
  return controls;
}

function tabChooser(state, actions) {
  const chooser = document.createElement("div");
  chooser.className = "tab-chooser";
  for (const tab of tabsFor(state)) {
    chooser.append(actionButton(tab.label, () => actions.openTab?.(tab.id), "chooser-tab"));
  }
  return chooser;
}

function tabBody(state, actions, primary) {
  const body = document.createElement("section");
  body.className = "tab-body";
  if (primary) {
    body.append(renderTabContent(state, actions));
  }
  return body;
}

function defaultTile() {
  return {
    active: 0,
    kind: "stack",
    tabs: [
      {
        pane_kind: "home",
        pane_node_id: "local_home",
        target_node_id: null,
        title: "Home",
      },
    ],
  };
}
