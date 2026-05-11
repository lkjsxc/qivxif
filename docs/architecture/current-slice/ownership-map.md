# Ownership Map

## Runtime Owners

| Contract | Owner |
| --- | --- |
| Process startup and QUIC endpoint | `apps/qivxif-serverd` |
| Session phase state | `apps/qivxif-serverd::session` |
| Request translation | `apps/qivxif-serverd::request` |
| Public message catalog | `crates/qivxif-protocol` |
| Public transport helpers | `crates/qivxif-net` |
| Region mutation authority | `crates/qivxif-sim` |
| Deterministic chunk generation | `crates/qivxif-world` |
| Hot edit overlays | `crates/qivxif-storage` |
| Public probe scenarios | `crates/qivxif-probe` |
| Static repository gates | `crates/qivxif-quality` |

## Doc Owners

| Contract | Owner doc |
| --- | --- |
| Public vertical loop | `architecture/current-slice/vertical-loop.md` |
| Mutating replay guard | `architecture/current-slice/request-replay.md` |
| Wire messages | `architecture/network/protocol-messages.md` |
| Session phases | `architecture/network/session-lifecycle.md` |
| redb hot state | `architecture/persistence/hot-state.md` |
| redb tables | `architecture/persistence/schema-contracts.md` |
| Public probes | `operations/verification/protocol-probes.md` |
| Compose acceptance | `operations/verification/compose-pipeline.md` |

## Rule

A behavior change updates the owner doc and owner crate together in the same
coherent batch.
