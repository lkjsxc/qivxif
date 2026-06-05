export const STORE_NAMES = [
  "accepted_events",
  "cache_entries",
  "cache_journal",
  "dirty_events",
  "edges",
  "events",
  "feed_windows",
  "local_workspace",
  "media_assets",
  "media_chunks",
  "media_uploads",
  "nodes",
  "profile_snapshots",
  "resource_entries",
  "resource_jobs",
  "resource_journal",
  "resource_leases",
  "sync_cursors",
  "tab_snapshots",
  "text_snapshots",
  "tile_layout",
] as const;

export type StoreName = (typeof STORE_NAMES)[number];
export type StorageMode = "opfs" | "memory" | "unavailable";
export type JsonRecord = { id: string; [key: string]: any };

export type StoreInventory = Record<StoreName, number>;

export type EventQueueCounts = {
  accepted: number;
  dirty: number;
  pending: number;
  rejected: number;
};

export type CacheByteCounts = {
  protected: number;
  prunable: number;
};

export type StorageDiagnostics = {
  mode: StorageMode;
  reason: string;
  quota: number;
  usage: number;
  pageCount?: number;
  stores: StoreInventory;
  inventory: StoreInventory;
  queue: EventQueueCounts;
  cache: CacheByteCounts;
  lastOperationError?: string;
};

export type LocalStore = {
  all(name: StoreName): Promise<JsonRecord[]>;
  count(name: StoreName): Promise<number>;
  delete(name: StoreName, id: string): Promise<void>;
  diagnostics(): Promise<StorageDiagnostics>;
  get(name: StoreName, id: string): Promise<JsonRecord | undefined>;
  put(name: StoreName, value: JsonRecord): Promise<void>;
};

export function isStoreName(value: string): value is StoreName {
  return (STORE_NAMES as readonly string[]).includes(value);
}
