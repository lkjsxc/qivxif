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

## Rule

A behavior change updates the owner doc and owner crate together in the same
coherent batch.
