import { renderBoardPane } from "./board.ts";
import { actionButton, field, heading, panel, text } from "./dom.ts";
import { renderPublishPane } from "./publish.ts";
import { renderSetupTab } from "./setup.ts";
import { renderSocialPane } from "./social.ts";
import { renderSyncStatus } from "./sync-status-pane.ts";

export function renderTabContent(state, actions) {
  if (state.setupRequired || state.activeTabId === "setup") {
    return renderSetupTab(state, actions);
  }
  switch (state.activeTabId) {
    case "login":
      return loginPanel(actions);
    case "graph":
      return graphPanel(state, actions);
    case "editor":
      return editorPanel(state, actions);
    case "board":
      return renderBoardPane(state, actions);
    case "publish":
      return renderPublishPane(state, actions);
    case "social":
      return renderSocialPane(state, actions);
    case "sync":
      return renderSyncStatus(state);
    case "history":
      return historyList(state);
    case "diagnostics":
      return diagnosticsPanel(state);
    default:
      return homePanel(state, actions);
  }
}

function homePanel(state, actions) {
  const section = panel("tab-panel home", heading("Home"));
  if (state.auth) {
    section.append(actionButton("Create text node", () => actions.createTextNode?.()));
    section.append(actionButton("Create board", () => actions.createBoard?.()));
    section.append(actionButton("Flush queue", () => actions.sync?.()));
    section.append(openNodeForm(actions), layoutSummary(state), nodeList(state, actions));
  } else {
    section.append(loginPanel(actions));
  }
  return section;
}

function loginPanel(actions) {
  const section = panel("tab-panel login-panel", heading("Login"));
  const form = document.createElement("form");
  form.className = "login";
  const nameLabel = field("Login name", "text", "username");
  const passwordLabel = field("Password", "password", "current-password");
  const name = nameLabel.querySelector("input");
  const password = passwordLabel.querySelector("input");
  const submit = document.createElement("button");
  submit.type = "submit";
  submit.textContent = "Login";
  form.append(nameLabel, passwordLabel, submit);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    actions.login?.(name.value, password.value);
  });
  section.append(form);
  return section;
}

function graphPanel(state, actions) {
  return panel("tab-panel graph", heading("Graph"), openNodeForm(actions), nodeList(state, actions));
}

function editorPanel(state, actions) {
  const section = panel("tab-panel editor-panel", heading("Text Node"));
  if (!state.currentNodeId) {
    section.append(text("Create or select a text node."));
    return section;
  }
  section.append(text(state.currentNodeId));
  if (state.textDirty) {
    section.append(text("Text has a dirty local event."));
  }
  const editor = document.createElement("textarea");
  editor.className = "editor";
  editor.value = state.text ?? "";
  section.append(editor, actionButton("Save text event", () => actions.saveText?.(editor.value)));
  section.append(actionButton("Create board", () => actions.createBoard?.()));
  section.append(layoutSummary(state), historyList(state));
  return section;
}

function openNodeForm(actions) {
  const form = document.createElement("form");
  form.className = "open-node";
  const nodeLabel = field("Server node id", "text");
  const nodeInput = nodeLabel.querySelector("input");
  const submit = document.createElement("button");
  submit.type = "submit";
  submit.textContent = "Open node";
  form.append(nodeLabel, submit);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    actions.openNode?.(nodeInput.value.trim());
  });
  return form;
}

function nodeList(state, actions) {
  const list = document.createElement("div");
  list.className = "node-list";
  const nodes = state.nodes ?? [];
  if (nodes.length === 0) {
    list.append(text("No local nodes."));
    return list;
  }
  for (const node of nodes) {
    const label = `${node.metadata_map?.title ?? node.id}${node.dirty ? " (dirty)" : ""}`;
    const button = actionButton(label, () => actions.selectNode?.(node.id));
    if (node.id === state.currentNodeId) {
      button.classList.add("selected");
    }
    list.append(button);
  }
  return list;
}

function layoutSummary(state) {
  const section = document.createElement("section");
  section.className = "layout-summary";
  const count = state.layout ? paneCount(state.layout.root) : 0;
  section.append(text(`Layout panes: ${count}`));
  section.append(text(`Maximized: ${state.layout?.maximized_pane_id ?? "none"}`));
  if (state.layoutDirty) {
    section.append(text("Layout has a dirty local event."));
  }
  return section;
}

function historyList(state) {
  const history = panel("tab-panel history", heading("History", 2));
  const events = state.history ?? [];
  if (events.length === 0) {
    history.append(text("No accepted events loaded."));
    return history;
  }
  for (const event of events) {
    history.append(text(`${event.kind} #${event.actor_seq}`));
  }
  return history;
}

function diagnosticsPanel(state) {
  const section = panel("tab-panel diagnostics", heading("Diagnostics"));
  section.append(renderSyncStatus(state), layoutSummary(state));
  return section;
}

function paneCount(tile) {
  if (!tile) {
    return 0;
  }
  if (tile.kind === "stack") {
    return tile.tabs.length;
  }
  return paneCount(tile.first) + paneCount(tile.second);
}
