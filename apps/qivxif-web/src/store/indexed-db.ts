const STORE_NAMES = [
  "nodes",
  "edges",
  "ops",
  "text_snapshots",
  "sync_cursors",
  "cache_entries",
  "cache_journal",
  "feed_windows",
  "tile_layout",
];

export function openLocalStore() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open("qivxif", 2);
    request.onupgradeneeded = () => {
      const db = request.result;
      for (const name of STORE_NAMES) {
        if (!db.objectStoreNames.contains(name)) {
          db.createObjectStore(name, { keyPath: "id" });
        }
      }
    };
    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(wrapDb(request.result));
  });
}

function wrapDb(db) {
  return {
    all(name) {
      return request(db, name, "readonly", (store) => store.getAll());
    },
    count(name) {
      return request(db, name, "readonly", (store) => store.count());
    },
    delete(name, id) {
      return request(db, name, "readwrite", (store) => store.delete(id));
    },
    get(name, id) {
      return request(db, name, "readonly", (store) => store.get(id));
    },
    put(name, value) {
      return request(db, name, "readwrite", (store) => store.put(value));
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
