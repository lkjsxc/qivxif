# Ownership Map

## Status

- Status: implemented for server slice owners.
- Client owners are dormant because no client crate exists.

## Runtime Owners

| Contract | Owner |
| --- | --- |
| Process startup and QUIC endpoint | `apps/qivxif-serverd::app` |
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
| Public vertical loop | [vertical-loop.md](vertical-loop.md) |
| Request replay | [request-replay.md](request-replay.md) |
| Wire messages | [../network/protocol-messages.md](../network/protocol-messages.md) |
| Session phases | [../network/session-lifecycle.md](../network/session-lifecycle.md) |
| redb hot state | [../persistence/hot-state.md](../persistence/hot-state.md) |
| redb schema | [../persistence/schema-contracts.md](../persistence/schema-contracts.md) |
| Object archive manifests | [../persistence/object-archives.md](../persistence/object-archives.md) |

## Change Rule

- A behavior change updates the owner doc and owner crate together.
- A dormant contract must gain executable verification before it becomes active.
