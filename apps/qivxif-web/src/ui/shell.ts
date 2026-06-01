import { renderCommandPalette } from "./command-palette.ts";
import { renderHeader } from "./header.ts";
import { renderTileGrid } from "./tile-tree.ts";

export function renderShell(root, state, actions) {
  root.replaceChildren(appShell(state, actions));
}

function appShell(state, actions) {
  const shell = document.createElement("section");
  shell.className = "app-shell workspace";
  shell.append(renderHeader(state, actions), renderTileGrid(state, actions));
  if (state.commandPaletteOpen) {
    shell.append(renderCommandPalette(state, actions));
  }
  return shell;
}
