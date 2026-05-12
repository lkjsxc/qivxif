# Ownership Map

## Status

- Status: implemented for server slice owners.
- Desktop smoke and native e2e client owners are active.
- Mobile shell owners are dormant.

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
| Headless protocol client | `crates/qivxif-client-core`, `apps/qivxif-client-cli` |
| Desktop smoke client | `apps/qivxif-client-desktop`, `crates/qivxif-client-core` |
| Native e2e client | `apps/qivxif-client-desktop`, `crates/qivxif-client-core` |
| Desktop smoke world cache | `crates/qivxif-client-core::WorldCache` |
| Deterministic smoke renderer | `crates/qivxif-render`, `crates/qivxif-assets` |
| Native GPU renderer | `crates/qivxif-render`, `crates/qivxif-assets` |
| GUI smoke status model | `crates/qivxif-ui` |
| Native input model | `crates/qivxif-input` |
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
| Headless protocol client | [../client/headless-client.md](../client/headless-client.md) |
| Desktop smoke client | [../../operations/verification/desktop-smoke.md](../../operations/verification/desktop-smoke.md) |
| Native e2e client | [../client/native-e2e-client.md](../client/native-e2e-client.md) |
| GUI runtime | [../client/gui-runtime.md](../client/gui-runtime.md) |
| Renderer boundary | [../client/renderer.md](../client/renderer.md) |

## Change Rule

- A behavior change updates the owner doc and owner crate together.
- A dormant contract must gain executable verification before it becomes active.
