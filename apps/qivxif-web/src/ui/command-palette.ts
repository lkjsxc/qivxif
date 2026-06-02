import { boardItems } from "../effects/board-actions.ts";
import { visibleRoot } from "../domain/tile-tree.ts";
import { actionButton, heading, panel, text } from "./dom.ts";

export function renderCommandPalette(state, actions) {
  const backdrop = document.createElement("div");
  backdrop.className = "command-palette-backdrop";
  backdrop.addEventListener("click", (event) => {
    if (event.target === backdrop) {
      actions.toggleCommandPalette?.(false);
    }
  });
  const dialog = panel("command-palette");
  dialog.setAttribute("role", "dialog");
  dialog.setAttribute("aria-label", "Command palette");
  const search = commandSearch();
  const empty = text("No matching commands.", "command-empty");
  const list = commandList(state, actions);
  empty.hidden = true;
  search.addEventListener("input", () => filterCommands(list, empty, search.value));
  dialog.append(heading("Command palette", 2), search, list, empty);
  backdrop.append(dialog);
  requestAnimationFrame(() => search.focus());
  return backdrop;
}

function commandSearch() {
  const input = document.createElement("input");
  input.className = "command-search";
  input.type = "search";
  input.autocomplete = "off";
  input.setAttribute("aria-label", "Search commands");
  return input;
}

function commandList(state, actions) {
  const list = document.createElement("div");
  list.className = "command-list";
  for (const command of commands(state, actions)) {
    const button = actionButton(command.label, () => runCommand(command, actions), "command");
    button.disabled = !command.enabled;
    button.dataset.search = `${command.label} ${command.reason}`.toLowerCase();
    if (command.reason) {
      button.title = command.reason;
      button.append(text(command.reason, "command-reason"));
    }
    list.append(button);
  }
  return list;
}

function filterCommands(list, empty, query) {
  const needle = query.trim().toLowerCase();
  let visible = 0;
  for (const button of list.querySelectorAll(".command")) {
    const matched = !needle || button.dataset.search.includes(needle);
    button.hidden = !matched;
    visible += matched ? 1 : 0;
  }
  empty.hidden = visible > 0;
}

async function runCommand(command, actions) {
  if (!command.enabled) {
    return;
  }
  await command.run();
  actions.toggleCommandPalette?.(false);
}

function commands(state, actions) {
  const tab = activeTab(state.layout, state.activePaneId);
  const paneId = tab?.pane_node_id ?? "";
  const context = paneContext(tab);
  const authed = Boolean(state.auth);
  const items = boardItems(state);
  const hasBoardTarget = Boolean(state.activeBoardId && (state.currentNodeId || textNode(state)));
  return [
    command("Open graph", true, () => actions.openTab?.("graph", paneId)),
    command("Create text node", authed, () => actions.createTextNode?.(), "login required"),
    command("Create board", authed, () => actions.createBoard?.(context), "login required"),
    command("Split pane", authed && paneId, () => actions.splitPane?.(paneId, context), "pane required"),
    command("Stack tab", authed && paneId, () => actions.stackTab?.(paneId, context), "pane required"),
    command("Maximize pane", authed && paneId, () => actions.maximizePane?.(paneId), "pane required"),
    command(
      "Add current node to board",
      authed && hasBoardTarget,
      () => actions.addCurrentNodeToBoard?.(),
      authed ? boardReason(state) : "login required",
    ),
    command("Move board item", authed && items.length > 0, () => actions.moveBoardItem?.(), boardItemReason(authed)),
    command("Link board nodes", authed && items.length > 1, () => actions.linkBoardNodes?.(), linkReason(authed)),
    command("Open sync status", true, () => actions.openTab?.("sync", paneId)),
    command("Open history", true, () => actions.openTab?.("history", paneId)),
    command("Open settings", true, () => actions.openTab?.("settings", paneId)),
    command("Open publishing tools", true, () => actions.openTab?.("publish", paneId)),
    command("Open social tools", true, () => actions.openTab?.("social", paneId)),
    command("Flush queue", authed, () => actions.sync?.(), "login required"),
  ];
}

function boardReason(state) {
  return state.activeBoardId ? "target node required" : "board required";
}

function boardItemReason(authed) {
  return authed ? "board item required" : "login required";
}

function linkReason(authed) {
  return authed ? "two board items required" : "login required";
}

function textNode(state) {
  return state.nodes?.find((node) => node.kind === "text") ?? null;
}

function command(label, enabled, run, reason) {
  return { enabled: Boolean(enabled), label, reason: enabled ? "" : reason, run };
}

function activeTab(layout, paneId) {
  const root = visibleRoot(layout);
  const focused = tabByPane(root, paneId);
  if (focused) {
    return focused;
  }
  const stack = firstStack(root);
  return stack?.tabs[Math.max(0, Math.min(stack.active ?? 0, stack.tabs.length - 1))] ?? null;
}

function tabByPane(tile, paneId) {
  if (!tile || !paneId) {
    return null;
  }
  if (tile.kind === "stack") {
    return tile.tabs.find((tab) => tab.pane_node_id === paneId) ?? null;
  }
  for (const child of tile.children ?? []) {
    const found = tabByPane(child, paneId);
    if (found) {
      return found;
    }
  }
  return null;
}

function firstStack(tile) {
  if (!tile) {
    return null;
  }
  if (tile.kind === "stack") {
    return tile;
  }
  for (const child of tile.children ?? []) {
    const found = firstStack(child);
    if (found) {
      return found;
    }
  }
  return null;
}

function paneContext(tab) {
  return {
    paneId: tab?.pane_node_id ?? "",
    paneKind: tab?.pane_kind ?? "",
    targetNodeId: tab?.target_node_id ?? "",
  };
}
