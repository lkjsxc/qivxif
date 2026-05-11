# Compose Stack

## Services

- `server`: authoritative world server.
- `verify`: static build and quality gate.
- `smoke`: live QUIC smoke probe.
- `persist-place`: writes a mutation through the public path.
- `persist-check`: verifies persisted mutation after restart.

## Rule

Probe services own readiness and retry behavior.
Local Compose may use self-signed rcgen certificates and probe-side skipped
verification. Production deployment must provide trust roots and certificate
rotation outside the probe path.
