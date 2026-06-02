export function renderSyncStatus(state) {
  const pane = document.createElement("aside");
  pane.className = "pane status";
  pane.append(row("Sync", state.online ? "online" : "offline"));
  pane.append(row("Queued", String(state.queued)));
  pane.append(row("Accepted", String(state.acceptedCount ?? 0)));
  pane.append(row("Rejected", String(state.rejected ?? 0)));
  pane.append(row("Session", state.auth ? state.auth.user.name : "none"));
  pane.append(row("Capabilities", capabilityText(state.capabilities)));
  if (state.lastError) {
    pane.append(row("Last error", state.lastError));
  }
  pane.append(queueList(state.queueEntries ?? []));
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

function queueList(entries) {
  const section = document.createElement("section");
  section.className = "queue-list";
  if (entries.length === 0) {
    section.append(row("Queue entries", "none"));
    return section;
  }
  section.append(row("Queue entries", String(entries.length)));
  for (const entry of entries) {
    section.append(queueEntry(entry));
  }
  return section;
}

function queueEntry(entry) {
  const item = document.createElement("article");
  item.className = `queue-entry ${entry.status}`;
  const id = document.createElement("div");
  id.className = "mono";
  id.textContent = entry.id;
  item.append(
    row("Event", `${entry.kind} ${entry.status}`),
    id,
    row("Target", entry.node_id ?? entry.request?.layout_node_id ?? "—"),
    row("Route", `${entry.route?.method ?? "POST"} ${entry.route?.path ?? ""}`),
  );
  if (entry.last_error) {
    const error = document.createElement("div");
    error.className = "error-text";
    error.textContent = entry.last_error;
    item.append(error);
  }
  return item;
}
