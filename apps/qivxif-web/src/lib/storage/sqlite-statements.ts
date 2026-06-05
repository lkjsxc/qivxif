import { STORE_TABLES } from "./sqlite-schema.ts";
import type { StoreName } from "./types.ts";

function table(name: StoreName) {
  return STORE_TABLES[name];
}

export function selectAllSql(name: StoreName) {
  return `SELECT json FROM ${table(name)} ORDER BY id`;
}

export function selectCountSql(name: StoreName) {
  return `SELECT count(*) AS count FROM ${table(name)}`;
}

export function selectOneSql(name: StoreName) {
  return `SELECT json FROM ${table(name)} WHERE id = ?`;
}

export function upsertSql(name: StoreName) {
  return `INSERT INTO ${table(name)} (id, json, updated_at)
    VALUES (?, ?, unixepoch())
    ON CONFLICT(id) DO UPDATE SET json = excluded.json, updated_at = excluded.updated_at`;
}

export function deleteSql(name: StoreName) {
  return `DELETE FROM ${table(name)} WHERE id = ?`;
}
