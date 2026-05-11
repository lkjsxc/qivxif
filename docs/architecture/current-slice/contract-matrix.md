# Contract Matrix

## Status

- Status: implemented where verification is listed.
- Dormant rows are explicitly marked.

## Acceptance Rule

- Update the owner doc before changing implementation behavior.
- Keep implementation owner and verification in one coherent change.
- Prefer public QUIC probes over in-process shortcuts for server acceptance.

## Active Contracts

| Contract | Owner doc | Implementation | Verification |
| --- | --- | --- | --- |
| Public vertical loop | [vertical-loop.md](vertical-loop.md) | `apps/qivxif-serverd`, `apps/qivxifctl`, `crates/qivxif-probe` | `scripts/verify-compose.sh` |
| Session phases | [../network/session-lifecycle.md](../network/session-lifecycle.md) | `apps/qivxif-serverd::session`, `apps/qivxif-serverd::request` | `protocol-guards` probe; session tests |
| Request replay | [request-replay.md](request-replay.md) | `apps/qivxif-serverd::session`, `apps/qivxif-serverd::request` | `request-replay` probe; request tests |
| Public messages | [../network/protocol-messages.md](../network/protocol-messages.md) | `crates/qivxif-protocol` | protocol tests |
| Codec boundary | [../network/protocol-codecs.md](../network/protocol-codecs.md) | `crates/qivxif-protocol`, `crates/qivxif-net` | malformed-wire probe; protocol tests |
| QUIC transport | [../network/transport.md](../network/transport.md) | `crates/qivxif-net`, `apps/qivxif-serverd` | smoke probe through Compose network |
| Region authority | [../simulation/region-ownership.md](../simulation/region-ownership.md) | `crates/qivxif-sim` | region tests; mutation probe |
| ECS boundary | [../simulation/ecs-boundary.md](../simulation/ecs-boundary.md) | `crates/qivxif-sim` | region tests |
| Coordinate model | [../world/coordinate-model.md](../world/coordinate-model.md) | `crates/qivxif-core`, `crates/qivxif-world` | world tests |
| Chunk cells | [../world/chunk-streaming.md](../world/chunk-streaming.md) | `crates/qivxif-world`, `crates/qivxif-probe` | chunk probe path |
| Hot state | [../persistence/hot-state.md](../persistence/hot-state.md) | `crates/qivxif-storage` | storage tests; `persist-check` probe |
| Storage schema | [../persistence/schema-contracts.md](../persistence/schema-contracts.md) | `crates/qivxif-storage` | storage tests |
| Object archive manifests | [../persistence/object-archives.md](../persistence/object-archives.md) | `crates/qivxif-storage::ArchiveStore` | archive manifest tests |
| Headless client smoke | [../client/headless-client.md](../client/headless-client.md) | `crates/qivxif-client-core`, `apps/qivxif-client-cli` | `client-cli` Compose service |
| Docs topology | [../../operations/quality/documentation-topology.md](../../operations/quality/documentation-topology.md) | `crates/qivxif-quality` | `qivxifctl docs validate-topology` |
| Line limits | [../../repository/rules/line-limits.md](../../repository/rules/line-limits.md) | `crates/qivxif-quality` | `qivxifctl quality check-lines` |

## Dormant Contracts

| Contract | Owner doc | Current state |
| --- | --- | --- |
| Native client shells | [../client/platform-shells.md](../client/platform-shells.md) | No shell crate exists |
| Renderer family | [../client/renderer.md](../client/renderer.md) | No renderer crate exists |
| Multi-region handoff | [../simulation/cross-region-handoff.md](../simulation/cross-region-handoff.md) | One region actor only |
| Gameplay systems | `docs/product/gameplay/` | No server gameplay crates beyond terrain mutation |

## Workspace Members

| Member | Role |
| --- | --- |
| `apps/qivxif-serverd` | Authoritative server process |
| `apps/qivxif-client-cli` | Headless protocol client |
| `apps/qivxifctl` | Agent-friendly quality and probe CLI |
| `crates/qivxif-core` | Shared primitive types and config |
| `crates/qivxif-client-core` | Reusable headless client session support |
| `crates/qivxif-protocol` | Public protocol catalog and postcard helpers |
| `crates/qivxif-net` | QUIC and certificate helpers |
| `crates/qivxif-world` | Chunk coordinates and generated cells |
| `crates/qivxif-sim` | Region actor and mutation authority |
| `crates/qivxif-storage` | redb hot state and archive boundary |
| `crates/qivxif-quality` | Repository quality checks |
| `crates/qivxif-probe` | Public probe scenarios |
