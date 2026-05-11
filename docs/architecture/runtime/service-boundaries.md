# Service Boundaries

## Status

- Status: implemented as crate and module boundaries.

## Implemented Boundaries

| Boundary | Implementation | Responsibility |
| --- | --- | --- |
| Gateway | `apps/qivxif-serverd::app` | Accept QUIC connections and streams |
| Session | `apps/qivxif-serverd::session` | Track hello, join, and replay state |
| Request | `apps/qivxif-serverd::request` | Map protocol messages to server actions |
| Protocol | `crates/qivxif-protocol` | Define message and error enums |
| Transport helpers | `crates/qivxif-net` | Encode/decode over Quinn streams |
| Region | `crates/qivxif-sim` | Own chunk, mutation, and flush commands |
| Storage | `crates/qivxif-storage` | Own redb and object archive boundaries |
| Probe | `crates/qivxif-probe` | Verify public behavior through QUIC |

## Rule

- Boundaries communicate through typed messages or crate APIs.
- Shared global mutable state is not an active pattern.
