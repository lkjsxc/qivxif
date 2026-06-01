const STORE_NAMES = [
  "nodes",
  "edges",
  "ops",
  "text_snapshots",
  "sync_cursors",
  "cache_entries",
  "cache_journal",
  "feed_windows",
  "workspace_layout",
];

export function openLocalStore() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open("qivxif", 1);
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
    count(name) {
      return new Promise((resolve, reject) => {
        const tx = db.transaction(name, "readonly");
        const request = tx.objectStore(name).count();
        request.onerror = () => reject(request.error);
        request.onsuccess = () => resolve(request.result);
      });
    },
  };
}
