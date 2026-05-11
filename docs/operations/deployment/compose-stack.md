# Compose Stack

LLM purpose: identify the local Compose services used for runtime and
acceptance probes.

## Owner Scope

This file owns local Compose service roles only. Probe behavior is defined in
[../verification/protocol-probes.md](../verification/protocol-probes.md) and
[../verification/compose-pipeline.md](../verification/compose-pipeline.md).

## Services

| Service | Role |
| --- | --- |
| `server` | Authoritative world server. |
| `verify` | Static build and quality gate. |
| `smoke` | Live QUIC smoke probe. |
| `protocol-guards` | Session guard rejection probe. |
| `malformed-wire` | Decode failure probe. |
| `request-replay` | Mutating request replay probe. |
| `client-cli` | Headless protocol client smoke path. |
| `persist-place` | Writes a mutation through the public path. |
| `persist-check` | Verifies persisted mutation after restart. |

## Rule

Probe services own readiness and retry behavior.
Local Compose may use self-signed rcgen certificates and probe-side skipped
verification. Production deployment must provide trust roots and certificate
rotation outside the probe path.
