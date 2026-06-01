# Surface Boundary

## Rule

Browser UI render modules are replaceable. Durable behavior belongs to domain
reducers, effect adapters, and server reducers.

## UI Modules

UI modules:

- Render state.
- Emit `WorkspaceCommand`.
- Hold only ephemeral DOM details.
- Do not call redb.
- Do not write IndexedDB directly.
- Do not call API clients directly.
- Do not build accepted events.

## Domain Reducers

Domain reducers:

- Accept commands and current workspace state.
- Return next workspace state.
- Return event drafts when durable changes are needed.
- Stay pure and deterministic.
- Do not read clocks, storage, cookies, or network state.

## Effect Adapters

Effect adapters:

- Persist local workspace state.
- Store dirty event drafts.
- Push events to HTTP sync.
- Pull accepted events.
- Register the service worker.
- Load and save tab snapshots.

## Authority

Server reducers are the durable authority. Browser reducers are optimistic
mirrors and must treat server rejection as real.
