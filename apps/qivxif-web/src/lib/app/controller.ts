import { initialWorkspaceState } from "../domain/workspace-state.ts";
import { reduceWorkspace } from "../domain/workspace-reducer.ts";
import type { WorkspaceCommand } from "../domain/workspace-command.ts";
import { createBrowserPorts } from "./browser-ports.ts";
import { runEffectPlans } from "./effect-runner.ts";
import type { AppPorts } from "./ports.ts";

export async function createController(givenPorts?: AppPorts) {
  const state = initialWorkspaceState();
  const ports = givenPorts ?? (await createBrowserPorts());
  state.storageStatus = await ports.storage.diagnostics();
  const listeners = new Set<any>();

  const actionTable = () => ports.actions.forState(state, notify);
  const notify = () => {
    const actions = actionTable();
    for (const listener of listeners) listener(state, actions);
  };

  async function dispatch(command: WorkspaceCommand) {
    if (await runDomainCommand(ports, state, command)) {
      notify();
      return;
    }
    const handler = commandHandlers[command.type];
    if (!handler) return;
    await handler(actionTable(), command);
    notify();
  }

  return {
    async start() {
      notify();
      await refreshSetupStatus(ports, state);
      ports.keyboard.install(actionTable, state);
      await ports.localState.load(state);
      await ports.sync.refreshQueue(state);
      state.storageStatus = await ports.storage.diagnostics();
      selectInitialTab(state);
      await refreshServerInfo(ports, state);
      await registerServiceWorker(ports, state);
      if (state.auth && !state.setupRequired) await ports.sync.flush(state);
      notify();
    },
    dispatch,
    state: () => state,
    subscribe(listener) {
      listeners.add(listener);
      listener(state, actionTable());
      return () => listeners.delete(listener);
    },
  };
}

const commandHandlers: Record<string, (actions: any, command: any) => Promise<void> | void> = {
  closePane: (actions, command) => actions.closePane?.(command.paneId),
  createOwner: (actions, command) => actions.createOwner?.(command.name, command.password),
  focusPane: (actions, command) => actions.focusPane?.(command.paneId),
  login: (actions, command) => actions.login?.(command.name, command.password),
  maximizePane: (actions, command) => actions.maximizePane?.(command.paneId),
  movePane: (actions, command) => actions.movePane?.(command.source, command.target, command.zone),
  openTab: (actions, command) => actions.openTab?.(command.tabId, command.paneId),
  resizeSplit: (actions, command) => actions.resizeSplit?.(command.paneId, command.sizes),
  splitPane: (actions, command) => actions.splitPane?.(command.paneId, command.context),
  stackTab: (actions, command) => actions.stackTab?.(command.paneId, command.context),
  sync: (actions) => actions.sync?.(),
  toggleCommandPalette: (actions, command) => actions.toggleCommandPalette?.(command.open),
  toggleTabChooser: (actions, command) => actions.toggleTabChooser?.(command.paneId),
  updatePaneScroll: (actions, command) => actions.updatePaneScroll?.(command.paneId, command.scrollTop),
  updateTextDraft: (actions, command) => actions.updateTextDraft?.(command.paneId, command.content),
};

async function runDomainCommand(ports: AppPorts, state: any, command: WorkspaceCommand) {
  if (!["bootstrap", "flushSyncQueue", "refreshDiagnostics"].includes(command.type)) return false;
  const reduced = reduceWorkspace(state, command);
  Object.assign(state, reduced.state);
  await runEffectPlans(ports, state, reduced.effects);
  return true;
}

async function refreshSetupStatus(ports: AppPorts, state: any) {
  try {
    const status = await ports.setup.status();
    state.setupChecked = true;
    state.setupRequired = Boolean(status.required);
    state.setupError = "";
    if (state.setupRequired) state.activeTabId = "setup";
  } catch (error) {
    state.online = false;
    state.setupError = String(error);
    state.lastError = String(error);
  }
}

async function refreshServerInfo(ports: AppPorts, state: any) {
  try {
    const payload = await ports.server.info();
    state.capabilities = payload.capabilities?.capabilities ?? [];
    state.online = true;
  } catch (error) {
    state.online = false;
    state.lastError = String(error);
  }
}

function selectInitialTab(state: any) {
  if (state.setupRequired) state.activeTabId = "setup";
  else if (!state.auth) state.activeTabId = "login";
  else if (!state.activeTabId || state.activeTabId === "setup" || state.activeTabId === "login") {
    state.activeTabId = "welcome";
  }
}

async function registerServiceWorker(ports: AppPorts, state: any) {
  try {
    await ports.serviceWorker.register();
    state.serviceWorkerReady = true;
  } catch (error) {
    state.lastError = String(error);
  }
}
