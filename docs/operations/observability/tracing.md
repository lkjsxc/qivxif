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
- Mutation persisted.
- Probe request.

## Probe Output

- `qivxifctl` emits `probe <name> ... ok` after successful probes.
- Probe output is part of the observability surface for Compose acceptance.

## Rule

Routine acceptance logs stay compact.
