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
  dialog.append(heading("Command palette", 2), commandList(state, actions));
  backdrop.append(dialog);
  requestAnimationFrame(() => dialog.querySelector("button:not(:disabled)")?.focus());
  return backdrop;
}

function commandList(state, actions) {
  const list = document.createElement("div");
  list.className = "command-list";
  for (const command of commands(state, actions)) {
    const button = actionButton(command.label, () => runCommand(command, actions), "command");
    button.disabled = !command.enabled;
    if (command.reason) {
      button.title = command.reason;
      button.append(text(command.reason, "command-reason"));
    }
    list.append(button);
  }
  return list;
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
  return [
    command("Create text node", authed, () => actions.createTextNode?.(), "login required"),
    command("Create board", authed, () => actions.createBoard?.(context), "login required"),
    command("Split pane", authed && paneId, () => actions.splitPane?.(paneId, context), "pane required"),
    command("Stack tab", authed && paneId, () => actions.stackTab?.(paneId, context), "pane required"),
    command("Open sync status", true, () => actions.openTab?.("sync", paneId)),
    command("Open history", true, () => actions.openTab?.("history", paneId)),
    command("Open settings", true, () => actions.openTab?.("settings", paneId)),
    command("Flush queue", authed, () => actions.sync?.(), "login required"),
  ];
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
  return tabByPane(tile.first, paneId) ?? tabByPane(tile.second, paneId);
}

function firstStack(tile) {
  if (!tile) {
    return null;
  }
  return tile.kind === "stack" ? tile : firstStack(tile.first) ?? firstStack(tile.second);
}

function paneContext(tab) {
  return {
    paneId: tab?.pane_node_id ?? "",
    paneKind: tab?.pane_kind ?? "",
    targetNodeId: tab?.target_node_id ?? "",
  };
}
