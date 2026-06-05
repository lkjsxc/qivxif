import sqlite3InitModule from "@sqlite.org/sqlite-wasm";
import { cacheBytes, emptyInventory, estimateStorage, normalizeCount } from "./diagnostics.ts";
import { createSchemaSql, pageCountSql } from "./sqlite-schema.ts";
import {
  deleteSql,
  selectAllSql,
  selectCountSql,
  selectOneSql,
  upsertSql,
} from "./sqlite-statements.ts";
import { isStoreName, STORE_NAMES, type JsonRecord, type StoreName } from "./types.ts";

let db: any;
let mode: "opfs" | "memory" | "unavailable" = "unavailable";
let reason = "worker has not opened sqlite";
let lastOperationError = "";

export async function handleStorageRequest(kind: string, payload: any = {}) {
  if (kind === "open") return openDatabase();
  if (!db) throw Object.assign(new Error(reason), { code: "not_open" });
  try {
    return await dispatch(kind, payload);
  } catch (error) {
    lastOperationError = error?.message ?? String(error);
    throw error;
  }
}

async function openDatabase() {
  if (db) return { diagnostics: await diagnostics() };
  const sqlite3 = await sqlite3InitModule().catch((error) => {
    mode = "unavailable";
    reason = "sqlite wasm failed to load";
    lastOperationError = error?.message ?? String(error);
    throw error;
  });
  db = openOpfs(sqlite3) ?? openMemory(sqlite3);
  db.exec(createSchemaSql());
  return { diagnostics: await diagnostics() };
}

function openOpfs(sqlite3: any) {
  if (!("opfs" in sqlite3)) return undefined;
  try {
    const connection = new sqlite3.oo1.OpfsDb("/qivxif.sqlite3", "c");
    mode = "opfs";
    reason = "opfs sqlite storage is active";
    return connection;
  } catch (error) {
    lastOperationError = error?.message ?? String(error);
    return undefined;
  }
}

function openMemory(sqlite3: any) {
  mode = "memory";
  reason = "opfs unavailable; memory sqlite storage is active";
  return new sqlite3.oo1.DB("/qivxif-memory.sqlite3", "ct");
}

async function dispatch(kind: string, payload: any) {
  if (kind === "diagnostics.read") return diagnostics();
  if (kind === "inventory.read") return inventory();
  if (kind === "workspace.load") return getRecord("local_workspace", "workspace");
  if (kind === "workspace.save_snapshot") return putRecord("local_workspace", payload);
  if (kind === "queue.append_dirty_event") return putRecord("events", payload);
  if (kind === "queue.list_all") return allRecords("events");
  if (kind === "queue.list_non_accepted") return nonAcceptedEvents();
  if (kind === "queue.mark_pending") return markEvent(payload.eventId, "pending_validation");
  if (kind === "queue.mark_rejected") return markRejected(payload.eventId, payload.error);
  if (kind === "queue.mark_accepted") return markAccepted(payload.eventId, payload.payload);
  if (kind === "record.all") return allRecords(recordName(payload));
  if (kind === "record.count") return countRecords(recordName(payload));
  if (kind === "record.delete") return deleteRecord(recordName(payload), payload.id);
  if (kind === "record.get") return getRecord(recordName(payload), payload.id);
  if (kind === "record.put") return putRecord(recordName(payload), payload.value);
  throw Object.assign(new Error(`invalid storage request ${kind}`), { code: "invalid_request" });
}

function recordName(payload: { name?: string }): StoreName {
  if (!payload?.name || !isStoreName(payload.name)) {
    throw Object.assign(new Error("invalid store name"), { code: "invalid_request" });
  }
  return payload.name;
}

function allRecords(name: StoreName): JsonRecord[] {
  return db.selectObjects(selectAllSql(name)).map((row) => JSON.parse(String(row.json)));
}

function countRecords(name: StoreName): number {
  const row = db.selectObject(selectCountSql(name));
  return normalizeCount(row?.count);
}

function getRecord(name: StoreName, id: string): JsonRecord | undefined {
  const row = db.selectObject(selectOneSql(name), [id]);
  return row ? JSON.parse(String(row.json)) : undefined;
}

function putRecord(name: StoreName, value: JsonRecord) {
  if (!value?.id) throw Object.assign(new Error("record id is required"), { code: "invalid_request" });
  transaction(() => db.exec({ bind: [value.id, JSON.stringify(value)], sql: upsertSql(name) }));
}

function deleteRecord(name: StoreName, id: string) {
  transaction(() => db.exec({ bind: [id], sql: deleteSql(name) }));
}

function markEvent(eventId: string, status: string) {
  const entry = getRecord("events", eventId);
  if (entry) putRecord("events", { ...entry, status });
}

function markRejected(eventId: string, error: any) {
  const entry = getRecord("events", eventId);
  if (entry) putRecord("events", { ...entry, last_error: error, status: "rejected" });
}

function markAccepted(eventId: string, payload: any) {
  const entry = getRecord("events", eventId);
  if (!entry) return;
  transaction(() => {
    const accepted = { ...entry, acceptance: payload, status: "accepted" };
    db.exec({ bind: [eventId, JSON.stringify(accepted)], sql: upsertSql("accepted_events") });
    db.exec({ bind: [eventId], sql: deleteSql("dirty_events") });
    db.exec({ bind: [eventId], sql: deleteSql("events") });
  });
}

function nonAcceptedEvents() {
  return allRecords("events")
    .filter((entry) => entry.status !== "accepted")
    .sort((left, right) => normalizeCount(left.actor_seq) - normalizeCount(right.actor_seq));
}

async function diagnostics() {
  const counts = inventory();
  const events = allRecords("events");
  const storage = await estimateStorage();
  return {
    cache: cacheBytes(allRecords("cache_entries")),
    inventory: counts,
    lastOperationError: lastOperationError || undefined,
    mode,
    pageCount: normalizeCount(db.selectValue(pageCountSql)),
    queue: queueCounts(events),
    quota: storage.quota,
    reason,
    stores: counts,
    usage: storage.usage,
  };
}

function inventory() {
  return Object.fromEntries(STORE_NAMES.map((name) => [name, countRecords(name)])) || emptyInventory();
}

function queueCounts(events: JsonRecord[]) {
  return {
    accepted: countRecords("accepted_events"),
    dirty: events.filter((entry) => entry.status === "dirty").length,
    pending: events.filter((entry) => entry.status === "pending_validation").length,
    rejected: events.filter((entry) => entry.status === "rejected").length,
  };
}

function transaction(run: () => void) {
  db.exec("BEGIN IMMEDIATE");
  try {
    run();
    db.exec("COMMIT");
  } catch (error) {
    db.exec("ROLLBACK");
    throw Object.assign(error, { code: "transaction_failed" });
  }
}
