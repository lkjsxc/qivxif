import { actionButton, heading, panel, text } from "./dom.ts";

export function renderSettingsPane(state, actions) {
  const section = panel("tab-panel settings", heading("Settings"));
  section.append(row("Account", state.auth?.user?.name ?? "signed out"));
  if (state.auth?.user?.actor_id) {
    section.append(row("Actor", state.auth.user.actor_id));
  }
  if (state.auth?.user?.profile_node_id) {
    section.append(row("Profile", state.auth.user.profile_node_id));
  }
  section.append(row("Network", state.online ? "online" : "offline"));
  section.append(row("Queued events", String(state.queued ?? 0)));
  section.append(row("Rejected events", String(state.rejected ?? 0)));
  section.append(row("Capabilities", capabilityText(state.capabilities)));
  if (state.lastError) {
    section.append(row("Last error", state.lastError));
  }
  if (state.auth) {
    section.append(actionButton("Flush queue", () => actions.sync?.()));
  }
  return section;
}

function row(label, value) {
  return text(`${label}: ${value}`);
}

function capabilityText(capabilities) {
  if (!capabilities || capabilities.length === 0) {
    return "none";
  }
  return capabilities.join(", ");
}
