import { handleStorageRequest } from "./worker-runtime.ts";
import { failure, success, type StorageWorkerRequest } from "./worker-protocol.ts";

self.addEventListener("message", async (event: MessageEvent<StorageWorkerRequest>) => {
  const request = event.data;
  try {
    const value = await handleStorageRequest(request.kind, request.payload);
    self.postMessage(success(request.id, value));
  } catch (error) {
    self.postMessage(failure(request.id, error?.code ?? "transaction_failed", error));
  }
});
