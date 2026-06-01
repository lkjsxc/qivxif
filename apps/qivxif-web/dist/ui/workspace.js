import { renderSyncStatus } from "./sync-status-pane.js";

export function renderWorkspace(root, state) {
  root.innerHTML = "";
  const workspace = document.createElement("section");
  workspace.className = "workspace";
  workspace.append(leftPane(), editorPane(), renderSyncStatus(state));
  root.append(workspace);
}

function leftPane() {
  const pane = document.createElement("aside");
  pane.className = "pane";
  pane.append(heading("Graph"), text("No graph node selected."));
  return pane;
}

function editorPane() {
  const pane = document.createElement("section");
  pane.className = "pane";
  pane.append(tabbar(), heading("Text Node"));
  const editor = document.createElement("textarea");
  editor.className = "editor";
  editor.value = "";
  pane.append(editor);
  return pane;
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

function heading(value) {
  const element = document.createElement("h1");
  element.textContent = value;
  return element;
}

function text(value) {
  const element = document.createElement("p");
  element.textContent = value;
  return element;
}
