import { STORE_NAMES, type StoreInventory, type StorageDiagnostics } from "./types.ts";

export function emptyInventory(value = 0): StoreInventory {
  return Object.fromEntries(STORE_NAMES.map((name) => [name, value])) as StoreInventory;
}

export function unavailableDiagnostics(reason: string, detail?: string): StorageDiagnostics {
  return {
    cache: { protected: 0, prunable: 0 },
    inventory: emptyInventory(),
    lastOperationError: detail,
    mode: "unavailable",
    queue: { accepted: 0, dirty: 0, pending: 0, rejected: 0 },
    quota: 0,
    reason,
    stores: emptyInventory(),
    usage: 0,
  };
}

export async function estimateStorage() {
  const estimate = await globalThis.navigator?.storage?.estimate?.().catch(() => ({}));
  return {
    quota: Number(estimate?.quota ?? 0),
    usage: Number(estimate?.usage ?? 0),
  };
}

export function normalizeCount(value: unknown): number {
  const count = Number(value ?? 0);
  return Number.isFinite(count) ? count : 0;
}

export function cacheBytes(records: any[]) {
  return records.reduce(
    (totals, record) => {
      const bytes = normalizeCount(record.bytes ?? record.byte_count ?? record.size);
      if (record.protected || record.kind === "protected") {
        totals.protected += bytes;
      } else {
        totals.prunable += bytes;
      }
      return totals;
    },
    { protected: 0, prunable: 0 },
  );
}
