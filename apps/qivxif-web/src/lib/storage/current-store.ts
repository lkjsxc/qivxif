import { unavailableDiagnostics } from "./diagnostics.ts";
import { openSqliteWorkerStore } from "./sqlite-worker-client.ts";
import type { JsonRecord, LocalStore, StoreName, StorageDiagnostics } from "./types.ts";

export async function openLocalStore(): Promise<LocalStore> {
  try {
    return exposeStorageDebug(await openSqliteWorkerStore());
  } catch (error) {
    return exposeStorageDebug(unavailableStore(error?.message ?? String(error)));
  }
}

export async function saveLocalWorkspace(store: LocalStore, state: any) {
  await store.put("local_workspace", {
    id: "workspace",
    layout: state.layout,
    layoutNodeId: state.layoutNodeId,
    tabDrafts: state.tabDrafts,
    tabScrolls: state.tabScrolls,
  });
}

export function loadLocalWorkspace(store: LocalStore) {
  return store.get("local_workspace", "workspace");
}

export function localStoreDiagnostics(store: LocalStore): Promise<StorageDiagnostics> {
  return store.diagnostics();
}

function exposeStorageDebug(store: LocalStore): LocalStore {
  if (typeof window !== "undefined") {
    (window as any).__qivxifStorageDebug = {
      all: store.all,
      diagnostics: store.diagnostics,
      get: store.get,
    };
  }
  return store;
}

function unavailableStore(reason: string): LocalStore {
  const diagnostics = unavailableDiagnostics("sqlite worker is unavailable", reason);
  const fail = () => Promise.reject(new Error(diagnostics.reason));
  return {
    all: (_name: StoreName) => Promise.resolve([]),
    count: (_name: StoreName) => Promise.resolve(0),
    delete: fail,
    diagnostics: () => Promise.resolve(diagnostics),
    get: (_name: StoreName, _id: string) => Promise.resolve(undefined),
    put: (_name: StoreName, _value: JsonRecord) => fail(),
  };
}
