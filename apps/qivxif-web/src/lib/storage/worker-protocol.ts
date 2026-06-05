import type { JsonRecord, StoreName, StorageDiagnostics } from "./types.ts";

export type StorageWorkerRequestKind =
  | "open"
  | "diagnostics.read"
  | "inventory.read"
  | "record.all"
  | "record.count"
  | "record.delete"
  | "record.get"
  | "record.put"
  | "workspace.load"
  | "workspace.save_snapshot"
  | "queue.append_dirty_event"
  | "queue.list_all"
  | "queue.list_non_accepted"
  | "queue.mark_pending"
  | "queue.mark_accepted"
  | "queue.mark_rejected";

export type StorageWorkerRequest = {
  id: string;
  kind: StorageWorkerRequestKind;
  payload?: any;
};

export type StorageWorkerError = {
  code: "startup_failed" | "not_open" | "transaction_failed" | "invalid_request";
  message: string;
  detail?: string;
};

export type StorageWorkerResponse =
  | { id: string; ok: true; value: any }
  | { id: string; ok: false; error: StorageWorkerError };

export type RecordRequest = {
  name: StoreName;
  id?: string;
  value?: JsonRecord;
};

export type OpenResult = {
  diagnostics: StorageDiagnostics;
};

export function failure(id: string, code: StorageWorkerError["code"], error: any): StorageWorkerResponse {
  return {
    error: {
      code,
      detail: error?.stack ?? error?.detail,
      message: error?.message ?? String(error),
    },
    id,
    ok: false,
  };
}

export function success(id: string, value: any): StorageWorkerResponse {
  return { id, ok: true, value };
}
