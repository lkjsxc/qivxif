import { renderHeader } from "./header.ts";
import { renderTileGrid } from "./tile-tree.ts";

export function renderShell(root, state, actions) {
  root.replaceChildren(appShell(state, actions));
}

function appShell(state, actions) {
  const shell = document.createElement("section");
  shell.className = "app-shell workspace";
  shell.append(renderHeader(state, actions), renderTileGrid(state, actions));
  return shell;
}
