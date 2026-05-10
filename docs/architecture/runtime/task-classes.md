# Task Classes

## Authoritative Tick Work

- Region simulation.
- Movement acceptance.
- Combat resolution.
- World mutation.

## Async Orchestration

- QUIC I/O.
- Session lifecycle.
- Timers.
- Persistence handoff.

## Bulk Work

- World generation.
- Compression.
- Asset or mesh preparation.
- Snapshot upload.

Bulk work must return immutable results to the authoritative owner.
