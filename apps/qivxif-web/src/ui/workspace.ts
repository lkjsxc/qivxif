import { renderSyncStatus } from "./sync-status-pane.ts";

export function renderWorkspace(root, state, actions) {
  root.innerHTML = "";
  const workspace = document.createElement("section");
  workspace.className = "workspace";
  workspace.append(graphPane(state, actions), editorPane(state, actions), renderSyncStatus(state));
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
  return pane;
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
