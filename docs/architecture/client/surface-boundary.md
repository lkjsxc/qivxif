# Surface Boundary

## Rule

Browser UI render modules are replaceable. Durable behavior belongs to domain
reducers, effect adapters, and server reducers.

## AppController

`AppController` owns workspace state and is the only entry point for UI-driven
changes.

```typescript
type AppController = {
  state(): WorkspaceState;
  dispatch(command: WorkspaceCommand): Promise<void>;
  subscribe(listener: (state: WorkspaceState) => void): () => void;
};
```

- UI calls `dispatch` with `WorkspaceCommand` values.
- Controller runs pure reducers, updates state, notifies subscribers.
- Controller sends typed `EffectPlan` values to the effect runner.
- Controller re-renders through a single subscribe hook in bootstrap.
- `actionsFor` is retired when dispatch migration is complete.

## WorkspaceCommand

`WorkspaceCommand` is an algebraic union. Controller internals do not dispatch
untyped string commands after the migration.

Initial commands include `bootstrap`, `focusPane`, `openNewTab`,
`convertNewTab`, `closePane`, `splitPane`, `stackTab`, `maximizePane`,
`restorePane`, `movePane`, `reorderTab`, `resizeSplit`, `createOwner`, `login`,
`logout`, `createTextNode`, `openNode`, `saveTextDraft`, `saveText`,
`updatePaneScroll`, `createGraphMap`, `addCurrentNodeToGraphMap`,
`pinGraphMapNode`, `linkGraphNodes`, `createMediaUpload`, `updateProfile`,
`issueInvite`, `issueApiKey`, `createShortPost`, `followProfile`,
`clearSocialEdge`, `createBlogDraft`, `publishBlogPost`, `unpublishBlogPost`,
`flushSyncQueue`, `refreshDiagnostics`, and `toggleCommandPalette`.

## AppPorts

Effect adapters implement `AppPorts`:

```typescript
type AppPorts = {
  storage: LocalRepositories;
  setup: SetupPort;
  auth: AuthPort;
  sync: SyncPort;
  serviceWorker: ServiceWorkerPort;
  diagnostics: DiagnosticsPort;
  wasm: WasmKernelPort;
};
```

Ports hide HTTP, SQLite worker, WASM bridge, service worker, storage diagnostics,
and sync actor details from UI and domain code.

## EffectPlan

Reducers return typed effect plans such as `PersistWorkspace`,
`AppendDirtyEvent`, `FlushQueue`, `RegisterServiceWorker`,
`LoadStorageDiagnostics`, `FetchServerInfo`, and `StartSyncActor`.

The effect runner executes plans through `AppPorts` and returns typed results
that the controller feeds into follow-up commands or state updates.

## UI Modules

UI modules:

- Render `WorkspaceState`.
- Emit `WorkspaceCommand` through controller dispatch.
- Hold only ephemeral DOM details such as drag arming and focus.
- Do not import ports directly.
- Do not build accepted events.

## Domain Reducers

Domain reducers:

- Accept `WorkspaceCommand` and current `WorkspaceState`.
- Return `{ state, effects }`.
- Stay pure and deterministic.
- Do not read clocks, random values, DOM, storage, HTTP, workers, or globals.
- Mirror Rust tile and graph reducers for optimistic shell updates until WASM
  parity allows duplicate TypeScript logic to be deleted.

## Effect Adapters

Effect adapters:

- Implement `AppPorts`.
- Persist workspace snapshots and dirty event drafts through typed repositories.
- Push and pull accepted events over HTTP sync when a service is available.
- Register the service worker.
- Load storage diagnostics.
- Load and save tab scroll and draft snapshots.

## Authority

Server reducers are the durable authority. Browser reducers are optimistic
mirrors and must treat server rejection as real.

## Module Layout

See [module-layout.md](module-layout.md) for the browser source tree.
