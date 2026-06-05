import { STORE_NAMES, type StoreName } from "./types.ts";

export const STORE_TABLES: Record<StoreName, string> = Object.fromEntries(
  STORE_NAMES.map((name) => [name, name]),
) as Record<StoreName, string>;

export function createSchemaSql() {
  return STORE_NAMES.map(
    (name) => `CREATE TABLE IF NOT EXISTS ${STORE_TABLES[name]} (
      id TEXT PRIMARY KEY NOT NULL,
      json TEXT NOT NULL,
      updated_at INTEGER NOT NULL DEFAULT (unixepoch())
    )`,
  ).join(";\n");
}

export const pageCountSql = "PRAGMA page_count";
