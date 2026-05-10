# Compose Stack

## Services

- `server`: authoritative world server.
- `verify`: static build and quality gate.
- `smoke`: live QUIC smoke probe.
- `persist-place`: writes a mutation through the public path.
- `persist-check`: verifies persisted mutation after restart.

## Rule

Probe services own readiness and retry behavior.
