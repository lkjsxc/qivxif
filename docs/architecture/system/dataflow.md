# Dataflow

## Durable Write

1. Validate request outside a write transaction.
2. Read current records inside a short write transaction.
3. Validate auth and causal state.
4. Insert event envelope.
5. Update records and indexes.
6. Commit.
7. Emit post-commit notifications.

## Client Sync

1. Persist local event in the SQLite worker repository.
2. Apply local projection reducer.
3. Upload event over the reliable lane when sync is available.
4. Receive acceptance or rejection.
5. Pull remote events.
6. Replay into projections.

## UI Dataflow

1. Svelte component emits a command.
2. Controller runs pure reducers.
3. Effects call typed repositories or service ports.
4. State snapshot updates the Svelte store.
5. Components render the new snapshot.
