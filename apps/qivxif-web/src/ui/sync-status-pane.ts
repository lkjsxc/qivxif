export function renderSyncStatus(state) {
  const pane = document.createElement("aside");
  pane.className = "pane status";
  pane.append(row("Sync", state.online ? "online" : "offline"));
  pane.append(row("Queued", String(state.queued)));
  pane.append(row("Rejected", String(state.rejected ?? 0)));
  pane.append(row("Session", state.auth ? state.auth.user.name : "none"));
  pane.append(row("Capabilities", capabilityText(state.capabilities)));
  if (state.lastError) {
    pane.append(row("Last error", state.lastError));
  }
  return pane;
}

function capabilityText(capabilities) {
  if (!capabilities || capabilities.length === 0) {
    return "none";
  }
  return capabilities.map((capability) => capability.toString()).join(", ");
}

function row(label, value) {
  const element = document.createElement("div");
  element.textContent = `${label}: ${value}`;
  return element;
}
