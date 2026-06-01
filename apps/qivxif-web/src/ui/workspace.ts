import { renderBoardPane } from "./board.ts";
import { renderPublishPane } from "./publish.ts";
import { renderSyncStatus } from "./sync-status-pane.ts";

export function renderWorkspace(root, state, actions) {
  root.innerHTML = "";
  const workspace = document.createElement("section");
  workspace.className = "workspace";
  workspace.append(
    graphPane(state, actions),
    editorPane(state, actions),
    renderBoardPane(state, actions),
    renderSyncStatus(state),
  );
  root.append(workspace);
}

function graphPane(state, actions) {
  const pane = document.createElement("aside");
  pane.className = "pane";
  pane.append(heading("Graph"));
  if (state.auth) {
    pane.append(text(`Signed in as ${state.auth.user.name}`));
    pane.append(actionButton("Create text node", () => actions.createTextNode?.()));
    pane.append(actionButton("Flush queue", () => actions.sync?.()));
    pane.append(actionButton("Split pane", () => actions.splitPane?.()));
    pane.append(actionButton("Stack tab", () => actions.stackTab?.()));
    pane.append(actionButton("Maximize pane", () => actions.maximizePane?.()));
    pane.append(actionButton("Close pane", () => actions.closePane?.()));
    pane.append(renderPublishPane(state, actions));
    pane.append(openNodeForm(actions));
  } else {
    pane.append(loginForm(actions));
  }
  pane.append(nodeList(state, actions));
  return pane;
}

function editorPane(state, actions) {
  const pane = document.createElement("section");
  pane.className = "pane";
  pane.append(tabbar(), heading("Text Node"));
  if (!state.currentNodeId) {
    pane.append(text("Create or select a text node."));
    return pane;
  }
  pane.append(text(state.currentNodeId));
  if (state.textDirty) {
    pane.append(text("Text has a dirty local operation."));
  }
  const editor = document.createElement("textarea");
  editor.className = "editor";
  editor.value = state.text ?? "";
  const save = actionButton("Save text operation", () => actions.saveText?.(editor.value));
  pane.append(editor);
  pane.append(save);
  pane.append(layoutSummary(state));
  pane.append(historyList(state));
  return pane;
}

function layoutSummary(state) {
  const section = document.createElement("section");
  section.className = "layout-summary";
  const count = state.layout ? paneCount(state.layout.root) : 0;
  section.append(text(`Layout panes: ${count}`));
  section.append(text(`Maximized: ${state.layout?.maximized_pane_id ?? "none"}`));
  if (state.layoutDirty) {
    section.append(text("Layout has a dirty local operation."));
  }
  return section;
}

function loginForm(actions) {
  const form = document.createElement("form");
  form.className = "login";
  const nameLabel = field("Login name", "text");
  const passwordLabel = field("Password", "password");
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

function historyList(state) {
  const history = document.createElement("section");
  history.className = "history";
  history.append(subheading("History"));
  const operations = state.history ?? [];
  if (operations.length === 0) {
    history.append(text("No accepted operations loaded."));
    return history;
  }
  for (const operation of operations) {
    history.append(text(`${operation.kind} #${operation.actor_seq}`));
  }
  return history;
}

function tabbar() {
  const bar = document.createElement("div");
  bar.className = "tabbar";
  const tab = document.createElement("button");
  tab.className = "tab";
  tab.type = "button";
  tab.textContent = "Workspace";
  bar.append(tab);
  return bar;
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

function actionButton(label, handler) {
  const button = document.createElement("button");
  button.className = "command";
  button.type = "button";
  button.textContent = label;
  button.addEventListener("click", handler);
  return button;
}

function heading(value) {
  const element = document.createElement("h1");
  element.textContent = value;
  return element;
}

function subheading(value) {
  const element = document.createElement("h2");
  element.textContent = value;
  return element;
}

function field(label, type) {
  const wrapper = document.createElement("label");
  wrapper.textContent = label;
  const element = document.createElement("input");
  element.autocomplete = type === "password" ? "current-password" : "username";
  element.type = type;
  wrapper.append(element);
  return wrapper;
}

function text(value) {
  const element = document.createElement("p");
  element.textContent = value;
  return element;
}
