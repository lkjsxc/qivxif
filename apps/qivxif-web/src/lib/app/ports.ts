import type { LocalStore, StorageDiagnostics } from "../storage/types.ts";

export type AppPorts = {
  actions: {
    forState(state: any, notify: () => void): Record<string, any>;
  };
  keyboard: {
    install(actions: () => Record<string, any>, state: any): void;
  };
  localState: {
    load(state: any): Promise<void>;
  };
  server: {
    info(): Promise<any>;
  };
  serviceWorker: {
    register(): Promise<void>;
  };
  setup: {
    status(): Promise<any>;
  };
  storage: {
    diagnostics(): Promise<StorageDiagnostics>;
    store: LocalStore;
  };
  sync: {
    flush(state: any): Promise<void>;
    refreshQueue(state: any): Promise<void>;
  };
};
