# Tracing

LLM purpose: list stable runtime events that agents can search in logs.

## Owner Scope

This file owns required and dormant event names. Probe behavior is owned by
[../verification/protocol-probes.md](../verification/protocol-probes.md).

## Required Events

- `server starting`.
- `server listening`.
- `server shutdown`.
- `connection accepted`.
- `connection closed`.
- `request handled`.
- `chunk request completed`.
- `mutation accepted`.
- `mutation rejected`.
- `persistence flushed`.
- `persistence flush rejected`.
- `probe <name> ... ok`.

## Dormant Events

- `archive manifest written`.
- `archive manifest listed`.
- `region tick completed`.
- `request replayed`.

## Probe Output

- `qivxifctl` emits `probe <name> ... ok` after successful probes.
- Probe output is part of the observability surface for Compose acceptance.

## Rule

Routine acceptance logs stay compact.
