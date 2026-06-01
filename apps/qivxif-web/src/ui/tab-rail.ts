import { actionButton } from "./dom.ts";

export function renderTabRail(state, actions) {
  const rail = document.createElement("div");
  rail.className = "tab-rail";
  for (const tab of tabsFor(state)) {
    const button = actionButton(tab.label, () => actions.openTab?.(tab.id), "tab");
    button.setAttribute("role", "tab");
    button.setAttribute("aria-selected", String(tab.id === state.activeTabId));
    if (tab.id === state.activeTabId) {
      button.classList.add("active");
    }
    rail.append(button);
  }
  return rail;
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
    { id: "home", label: "Home" },
    { id: "graph", label: "Graph" },
    { id: "editor", label: "Editor" },
    { id: "board", label: "Board" },
    { id: "publish", label: "Publish" },
    { id: "social", label: "Social" },
    { id: "sync", label: "Sync" },
    { id: "history", label: "History" },
    { id: "diagnostics", label: "Diagnostics" },
  ];
}
