import { openLocalStore } from "../store/indexed-db.js";
import { renderWorkspace } from "../ui/workspace.js";

export async function startAppShell(root) {
  if (!root) {
    return;
  }
  const state = {
    online: navigator.onLine,
    capabilities: [],
    queued: 0,
    lastError: "",
  };
  renderWorkspace(root, state);
  const store = await openLocalStore();
  state.queued = await store.count("ops");
  try {
    const response = await fetch("/api/server-info");
    const envelope = await response.json();
    state.capabilities = envelope.payload?.capabilities?.capabilities ?? [];
    state.online = true;
  } catch (error) {
    state.online = false;
    state.lastError = String(error);
  }
  await registerServiceWorker(state);
  renderWorkspace(root, state);
}

async function registerServiceWorker(state) {
  if (!("serviceWorker" in navigator)) {
    return;
  }
  try {
    await navigator.serviceWorker.register("/service-worker.js");
  } catch (error) {
    state.lastError = String(error);
  }
}
