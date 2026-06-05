import type {
  OpenResult,
  StorageWorkerRequest,
  StorageWorkerRequestKind,
  StorageWorkerResponse,
} from "./worker-protocol.ts";
import type { JsonRecord, LocalStore, StoreName, StorageDiagnostics } from "./types.ts";

const STARTUP_TIMEOUT_MS = 15000;
let nextRequestId = 1;

export async function openSqliteWorkerStore(): Promise<LocalStore> {
  const client = new StorageWorkerClient();
  await client.open();
  return client.store();
}

class StorageWorkerClient {
  private readonly pending = new Map<string, PendingRequest>();
  private readonly worker: Worker;

  constructor() {
    this.worker = new Worker(new URL("./sqlite.worker.ts", import.meta.url), { type: "module" });
    this.worker.addEventListener("message", (event: MessageEvent<StorageWorkerResponse>) => {
      this.complete(event.data);
    });
  }

  async open() {
    await this.request<OpenResult>("open", {}, STARTUP_TIMEOUT_MS);
  }

  store(): LocalStore {
    return {
      all: (name) => this.request<JsonRecord[]>("record.all", { name }),
      count: (name) => this.request<number>("record.count", { name }),
      delete: (name, id) => this.request<void>("record.delete", { id, name }),
      diagnostics: () => this.request<StorageDiagnostics>("diagnostics.read"),
      get: (name, id) => this.request<JsonRecord | undefined>("record.get", { id, name }),
      put: (name, value) => this.request<void>("record.put", { name, value }),
    };
  }

  private request<T>(kind: StorageWorkerRequestKind, payload?: any, timeoutMs = 30000) {
    const id = `stor_${nextRequestId++}`;
    const message: StorageWorkerRequest = { id, kind, payload };
    return new Promise<T>((resolve, reject) => {
      const timer = setTimeout(() => {
        this.pending.delete(id);
        reject(new Error(`storage request timed out: ${kind}`));
      }, timeoutMs);
      this.pending.set(id, { reject, resolve, timer });
      this.worker.postMessage(message);
    });
  }

  private complete(response: StorageWorkerResponse) {
    const pending = this.pending.get(response.id);
    if (!pending) return;
    clearTimeout(pending.timer);
    this.pending.delete(response.id);
    if (response.ok) {
      pending.resolve(response.value);
    } else {
      pending.reject(Object.assign(new Error(response.error.message), { storage: response.error }));
    }
  }
}

type PendingRequest = {
  reject(error: any): void;
  resolve(value: any): void;
  timer: ReturnType<typeof setTimeout>;
};
