import { serverInfo, setupStatus } from "../http/client.ts";
import { openLocalStore } from "../store/indexed-db.ts";
import { renderShell } from "../ui/shell.ts";
import { actionsFor } from "./app-actions.ts";
import { initialState } from "./app-state.ts";
import { installKeyboardShortcuts } from "./keyboard-shortcuts.ts";
import { loadLocalState } from "./state-loader.ts";
import { flushQueue, refreshQueueState } from "./sync.ts";

export async function startAppShell(root) {
  if (!root) {
    return;
  }
  const state = initialState();
  renderShell(root, state, {});
  await refreshSetupStatus(state);
  const store = await openLocalStore();
  installKeyboardShortcuts(() => actionsFor(root, store, state), state);
  await loadLocalState(store, state);
  await refreshQueueState(store, state);
  selectInitialTab(state);
  await refreshServerInfo(state);
  await registerServiceWorker(state);
  if (state.auth && !state.setupRequired) {
    await flushQueue(store, state);
  }
  renderShell(root, state, actionsFor(root, store, state));
}

async function refreshSetupStatus(state) {
  try {
    const status = await setupStatus();
    state.setupChecked = true;
    state.setupRequired = Boolean(status.required);
    state.setupError = "";
    if (state.setupRequired) {
      state.activeTabId = "setup";
    }
  } catch (error) {
    state.online = false;
    state.setupError = String(error);
    state.lastError = String(error);
  }
}

async function refreshServerInfo(state) {
  try {
    const payload = await serverInfo();
    state.capabilities = payload.capabilities?.capabilities ?? [];
    state.online = true;
  } catch (error) {
    state.online = false;
    state.lastError = String(error);
  }
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

function selectInitialTab(state) {
  if (state.setupRequired) {
    state.activeTabId = "setup";
  } else if (!state.auth) {
    state.activeTabId = "login";
  } else if (state.activeBoardId) {
    state.activeTabId = "board";
  }
}
