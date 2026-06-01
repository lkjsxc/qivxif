import { actionButton } from "./dom.ts";
import { markDraggableTab } from "./tab-drag.ts";

export function renderTabRail(state, actions) {
  const rail = document.createElement("div");
  rail.className = "tab-rail";
  let activeButton = null;
  for (const tab of tabsFor(state)) {
    const button = actionButton(tab.label, () => actions.openTab?.(tab.id), "tab");
    button.setAttribute("role", "tab");
    button.setAttribute("aria-selected", String(tab.id === state.activeTabId));
    if (tab.id === state.activeTabId) {
      button.classList.add("active");
      activeButton = button;
    }
    rail.append(button);
  }
  finalizeRail(rail, activeButton);
  return rail;
}

export function renderStackTabRail(stack, actions) {
  const rail = document.createElement("div");
  rail.className = "tab-rail";
  let activeButton = null;
  stack.tabs.forEach((tab, index) => {
    const button = actionButton(tabLabel(tab), tabFocus(tab, actions), "tab");
    button.dataset.paneId = tab.pane_node_id;
    markDraggableTab(button, tab.pane_node_id, actions);
    button.setAttribute("role", "tab");
    button.setAttribute("aria-selected", String(index === stack.active));
    if (index === stack.active) {
      button.classList.add("active");
      activeButton = button;
    }
    rail.append(button);
  });
  finalizeRail(rail, activeButton);
  return rail;
}

function tabFocus(tab, actions) {
  if (!tab.pane_node_id?.startsWith("nod_")) {
    return () => {};
  }
  return () => actions.focusPane?.(tab.pane_node_id);
}

function tabLabel(tab) {
  const labels = {
    login: "Login",
    setup: "Setup",
    text_editor: "Editor",
  };
  return labels[tab.pane_kind] ?? tab.title ?? tab.pane_kind;
}

function finalizeRail(rail, activeButton) {
  requestAnimationFrame(() => {
    activeButton?.scrollIntoView({ block: "nearest", inline: "nearest" });
    rail.dataset.overflow = String(rail.scrollWidth > rail.clientWidth + 1);
  });
}

export function tabsFor(state) {
  if (state.setupRequired) {
    return [{ id: "setup", label: "Setup" }];
  }
  if (!state.auth) {
    return [
      { id: "login", label: "Login" },
      { id: "sync", label: "Sync" },
      { id: "diagnostics", label: "Diagnostics" },
    ];
  }
  return [
    { id: "welcome", label: "Welcome" },
    { id: "graph", label: "Graph" },
    { id: "editor", label: "Editor" },
    { id: "board", label: "Board" },
    { id: "publish", label: "Publish" },
    { id: "social", label: "Social" },
    { id: "sync", label: "Sync" },
    { id: "settings", label: "Settings" },
    { id: "history", label: "History" },
    { id: "diagnostics", label: "Diagnostics" },
  ];
}
