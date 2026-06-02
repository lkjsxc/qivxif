import { initialWorkspaceState } from "../domain/workspace-state.ts";
import { serverInfo, setupStatus } from "../effects/api-client.ts";
import { openLocalStore } from "../effects/indexed-db.ts";
import { actionsFor } from "../effects/app-actions.ts";
import { installKeyboardShortcuts } from "../effects/keyboard.ts";
import { loadLocalState } from "../effects/state-loader.ts";
import { flushQueue, refreshQueueState } from "../effects/sync.ts";

export async function createController() {
  const state = initialWorkspaceState();
  const store = await openLocalStore();
  const listeners = new Set();

  const notify = () => {
    const actions = actionsFor(store, state, notify);
    for (const listener of listeners) {
      listener(state, actions);
    }
  };

  async function dispatch(command) {
    const actions = actionsFor(store, state, notify);
    const handler = commandHandlers[command.type];
    if (!handler) {
      return;
    }
    await handler(actions, command);
    notify();
  }

  return {
    async start() {
      notify();
      await refreshSetupStatus(state);
      installKeyboardShortcuts(() => actionsFor(store, state, notify), state);
      await loadLocalState(store, state);
      await refreshQueueState(store, state);
      selectInitialTab(state);
      await refreshServerInfo(state);
      await registerServiceWorker(state);
      if (state.auth && !state.setupRequired) {
        await flushQueue(store, state);
      }
      notify();
    },
    state: () => state,
    dispatch,
    subscribe(listener) {
      listeners.add(listener);
      listener(state, actionsFor(store, state, notify));
      return () => listeners.delete(listener);
    },
  };
}

const commandHandlers = {
  focusPane: (actions, command) => actions.focusPane?.(command.paneId),
  splitPane: (actions, command) => actions.splitPane?.(command.paneId, command.context),
  stackTab: (actions, command) => actions.stackTab?.(command.paneId, command.context),
  closePane: (actions, command) => actions.closePane?.(command.paneId),
  maximizePane: (actions, command) => actions.maximizePane?.(command.paneId),
  movePane: (actions, command) => actions.movePane?.(command.source, command.target, command.zone),
  resizeSplit: (actions, command) => actions.resizeSplit?.(command.paneId, command.sizes),
  openTab: (actions, command) => actions.openTab?.(command.tabId, command.paneId),
  toggleTabChooser: (actions, command) => actions.toggleTabChooser?.(command.paneId),
  toggleCommandPalette: (actions, command) => actions.toggleCommandPalette?.(command.open),
  createOwner: (actions, command) => actions.createOwner?.(command.name, command.password),
  login: (actions, command) => actions.login?.(command.name, command.password),
  sync: (actions) => actions.sync?.(),
  updateTextDraft: (actions, command) => actions.updateTextDraft?.(command.paneId, command.content),
  updatePaneScroll: (actions, command) => actions.updatePaneScroll?.(command.paneId, command.scrollTop),
};

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

function selectInitialTab(state) {
  if (state.setupRequired) {
    state.activeTabId = "setup";
    return;
  }
  if (!state.auth) {
    state.activeTabId = "login";
    return;
  }
  if (!state.activeTabId || state.activeTabId === "setup" || state.activeTabId === "login") {
    state.activeTabId = "welcome";
  }
}

async function registerServiceWorker(state) {
  if (!("serviceWorker" in navigator)) {
    return;
  }
  try {
    await navigator.serviceWorker.register("/service-worker.js");
    state.serviceWorkerReady = true;
  } catch (error) {
    state.lastError = String(error);
  }
}
