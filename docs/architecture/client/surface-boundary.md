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
- Controller passes event drafts to effect adapters.
- Controller re-renders through a single subscribe hook in bootstrap.

## AppPorts

Effect adapters implement `AppPorts`:

```typescript
type AppPorts = {
  loadLocalWorkspace(): Promise<WorkspaceState | undefined>;
  saveLocalWorkspace(state: WorkspaceState): Promise<void>;
  getSetupStatus(): Promise<SetupStatus>;
  submitOwnerSetup(input: OwnerSetupInput): Promise<AuthSession>;
  pushEvents(events: EventDraft[]): Promise<PushResult>;
  pullEvents(cursor: SyncCursor): Promise<PullResult>;
  registerServiceWorker(): Promise<void>;
  loadTabSnapshots(): Promise<TabSnapshotMap>;
  saveTabSnapshot(paneId: string, snapshot: TabSnapshot): Promise<void>;
};
```

Ports hide HTTP, IndexedDB, and service worker details from UI and domain code.

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
- Return `{ state, drafts }`.
- Stay pure and deterministic.
- Mirror Rust tile and graph reducers for optimistic shell updates.

## Effect Adapters

Effect adapters:

- Implement `AppPorts`.
- Persist workspace snapshots and dirty event drafts.
- Push and pull accepted events over HTTP sync.
- Register the service worker.
- Load and save tab scroll and draft snapshots.

## Authority

Server reducers are the durable authority. Browser reducers are optimistic
mirrors and must treat server rejection as real.

## Module Layout

See [module-layout.md](module-layout.md) for the browser source tree.
