# Task Classes

## Status

- Status: implemented for async orchestration and one actor task.
- No fixed authoritative tick loop exists yet.

## Implemented Async Work

- QUIC endpoint accept loop.
- Connection task per accepted connection.
- Request handling per accepted bidirectional stream.
- Probe connection retry loop.
- Archive manifest operations through async object_store APIs.

## Implemented Actor Work

- Region mailbox receives `Chunk`, `PlaceBlock`, and `Flush` commands.
- Region command handlers run sequentially in one actor task.
- Command replies use `oneshot` channels.

## Synchronous Work Inside Actor

- Chunk cell generation.
- Dirty overlay merge.
- redb flush on explicit `Flush` command.

## Not Implemented

- Fixed-step simulation tick.
- Bulk world generation workers.
- Mesh or asset preparation workers.
- Snapshot upload tasks.
