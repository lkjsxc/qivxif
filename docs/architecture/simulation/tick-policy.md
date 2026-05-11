# Tick Policy

## Status

- Status: no fixed tick loop implemented.
- Current actor processes mailbox commands sequentially.

## Implemented Facts

- Region work runs in a Tokio task.
- Chunk generation is called synchronously inside the region command handler.
- redb flush is called only from explicit `Flush` commands.
- Current probes call flush explicitly before restart-sensitive checks.

## Active Rule

- Do not claim a fixed-step simulation tick exists in the current slice.

## Future Constraints

- Tick work must not block on network backpressure.
- Expensive jobs must leave the tick path through bounded queues.
- Full queues must shed, delay, or reject work explicitly.
