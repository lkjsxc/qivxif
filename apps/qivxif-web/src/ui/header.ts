import { actionButton } from "./dom.ts";

export function renderHeader(state, actions) {
  const header = document.createElement("header");
  header.className = "app-header";
  header.append(brand(), statusGroup(state), commandGroup(state, actions));
  return header;
}

function brand() {
  const element = document.createElement("div");
  element.className = "brand";
  element.textContent = "qivxif";
  return element;
}

function statusGroup(state) {
  const group = document.createElement("div");
  group.className = "header-status";
  group.append(
    status(`Sync: ${state.online ? "online" : "offline"}`),
    status(authLabel(state)),
    status(`Queued: ${state.queued}`),
    status(`Rejected: ${state.rejected ?? 0}`),
    status(`Capabilities: ${capabilityText(state.capabilities)}`),
  );
  return group;
}

function commandGroup(state, actions) {
  const group = document.createElement("div");
  group.className = "header-actions";
  group.append(
    actionButton("Commands", () => actions.toggleCommandPalette?.(), "header-button"),
    actionButton("New tab", () => actions.toggleTabChooser?.(), "header-button"),
  );
  if (state.auth) {
    group.append(actionButton("Sync", () => actions.sync?.(), "header-button"));
  }
  return group;
}

function authLabel(state) {
  if (state.setupRequired) {
    return "Setup required";
  }
  if (state.auth) {
    return `Signed in as ${state.auth.user.name}`;
  }
  return "Signed out";
}

function capabilityText(capabilities) {
  if (!capabilities || capabilities.length === 0) {
    return "none";
  }
  return capabilities.join(", ");
}

function status(value) {
  const item = document.createElement("span");
  item.textContent = value;
  return item;
}
