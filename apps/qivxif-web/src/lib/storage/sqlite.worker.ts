import { handleStorageRequest } from "./worker-runtime.ts";
import { failure, success, type StorageWorkerRequest } from "./worker-protocol.ts";

let requestChain = Promise.resolve();

self.addEventListener("message", (event: MessageEvent<StorageWorkerRequest>) => {
  const request = event.data;
  requestChain = requestChain.then(
    () => respond(request),
    () => respond(request),
  );
});

async function respond(request: StorageWorkerRequest) {
  try {
    const value = await handleStorageRequest(request.kind, request.payload);
    self.postMessage(success(request.id, value));
  } catch (error) {
    self.postMessage(failure(request.id, error?.code ?? "transaction_failed", error));
  }
}
