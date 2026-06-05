import { serverInfo, setupStatus } from "../effects/api-client.ts";
import { actionsFor } from "../effects/app-actions.ts";
import { installKeyboardShortcuts } from "../effects/keyboard.ts";
import { loadLocalState } from "../effects/state-loader.ts";
import { flushQueue, refreshQueueState } from "../effects/sync.ts";
import { localStoreDiagnostics, openLocalStore } from "../storage/current-store.ts";
import type { AppPorts } from "./ports.ts";

export async function createBrowserPorts(): Promise<AppPorts> {
  const store = await openLocalStore();
  return {
    actions: { forState: (state, notify) => actionsFor(store, state, notify) },
    keyboard: { install: installKeyboardShortcuts },
    localState: { load: (state) => loadLocalState(store, state) },
    server: { info: serverInfo },
    serviceWorker: { register: registerServiceWorker },
    setup: { status: setupStatus },
    storage: { diagnostics: () => localStoreDiagnostics(store), store },
    sync: {
      flush: (state) => flushQueue(store, state),
      refreshQueue: (state) => refreshQueueState(store, state),
    },
  };
}

async function registerServiceWorker() {
  if (!("serviceWorker" in navigator)) return;
  await navigator.serviceWorker.register("/service-worker.js");
}
