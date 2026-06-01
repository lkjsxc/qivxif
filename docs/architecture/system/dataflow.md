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

1. Persist local event in IndexedDB.
2. Apply local projection reducer.
3. Upload event over reliable lane.
4. Receive acceptance or rejection.
5. Pull remote events.
6. Replay into projections.
