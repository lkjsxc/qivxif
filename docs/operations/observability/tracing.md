# Tracing

## Required Events

- Startup.
- Shutdown.
- Connection accepted.
- Connection closed.
- Request handled.
- Chunk request completed.
- Mutation accepted.
- Mutation rejected.
- Persistence flushed.
- Persistence flush rejected.
- Probe result.

## Dormant Events

- Archive manifest written.
- Archive manifest listed.
- Region tick completed.
- Request replayed.

## Probe Output

- `qivxifctl` emits `probe <name> ... ok` after successful probes.
- Probe output is part of the observability surface for Compose acceptance.

## Rule

Routine acceptance logs stay compact.
