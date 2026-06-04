const STORE_NAMES = [
  "accepted_events",
  "cache_entries",
  "cache_journal",
  "dirty_events",
  "edges",
  "events",
  "feed_windows",
  "local_workspace",
  "nodes",
  "sync_cursors",
  "tab_snapshots",
  "text_snapshots",
  "tile_layout",
];

function openWithTimeout(ms) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("indexeddb open timeout")), ms);
    const request = indexedDB.open("qivxif", 4);
    request.onupgradeneeded = () => {
      const db = request.result;
      for (const name of STORE_NAMES) {
        if (!db.objectStoreNames.contains(name)) {
          db.createObjectStore(name, { keyPath: "id" });
        }
      }
    };
    request.onerror = () => {
      clearTimeout(timer);
      reject(request.error);
    };
    request.onsuccess = () => {
      clearTimeout(timer);
      resolve(wrapDb(request.result));
    };
  });
}

export function openLocalStore() {
  return openWithTimeout(15000);
}

export async function saveLocalWorkspace(store, state) {
  await store.put("local_workspace", {
    id: "workspace",
    layout: state.layout,
    layoutNodeId: state.layoutNodeId,
    tabDrafts: state.tabDrafts,
    tabScrolls: state.tabScrolls,
  });
}

export async function loadLocalWorkspace(store) {
  return store.get("local_workspace", "workspace");
}

export async function localStoreDiagnostics(store) {
  const stores = Object.fromEntries(
    await Promise.all(STORE_NAMES.map(async (name) => [name, await countOrMinusOne(store, name)])),
  );
  const estimate: any = await navigator.storage?.estimate?.().catch(() => ({}));
  return {
    mode: "indexeddb",
    reason: "sqlite worker is not active",
    quota: estimate?.quota ?? 0,
    stores,
    usage: estimate?.usage ?? 0,
  };
}

async function countOrMinusOne(store, name) {
  try {
    return await store.count(name);
  } catch {
    return -1;
  }
}

function wrapDb(db) {
  return {
    all(name) {
      return request(db, name, "readonly", (store) => store.getAll());
    },
    count(name) {
      return request(db, name, "readonly", (store) => store.count());
    },
    get(name, id) {
      return request(db, name, "readonly", (store) => store.get(id));
    },
    put(name, value) {
      return request(db, name, "readwrite", (store) => store.put(value));
    },
    delete(name, id) {
      return request(db, name, "readwrite", (store) => store.delete(id));
    },
  };
}

function request(db, name, mode, command) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction(name, mode);
    const store = tx.objectStore(name);
    const call = command(store);
    call.onerror = () => reject(call.error);
    call.onsuccess = () => resolve(call.result);
    tx.onerror = () => reject(tx.error);
    tx.onabort = () => reject(tx.error);
  });
}
