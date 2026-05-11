# Contract Matrix

This matrix maps active docs canon to implementation owners and executable
checks. Implementation behavior outside these rows is not protected.

## Acceptance Rule

- Update the owner doc before changing implementation behavior.
- Keep the implementation owner and verification path in the same coherent
  batch.
- Prefer public QUIC probes over in-process shortcuts for server acceptance.

## Active Contracts

| Contract | Owner doc | Implementation | Verification |
| --- | --- | --- | --- |
| Public vertical loop | `architecture/current-slice/vertical-loop.md` | `apps/qivxif-serverd`, `apps/qivxifctl`, `crates/qivxif-probe` | `scripts/verify-compose.sh` smoke and persistence probes |
| Session phases | `architecture/network/session-lifecycle.md` | `apps/qivxif-serverd/src/session.rs` | server session tests and `protocol-guards` probe |
| Request replay | `architecture/current-slice/request-replay.md` | `apps/qivxif-serverd/src/request.rs` | `request-replay` probe and request tests |
| Public messages | `architecture/network/protocol-messages.md` | `crates/qivxif-protocol` | protocol roundtrip and guard tests |
| Codec boundary | `architecture/network/protocol-codecs.md` | `crates/qivxif-protocol`, `crates/qivxif-net` | malformed-wire probe and protocol tests |
| QUIC transport | `architecture/network/transport.md` | `crates/qivxif-net`, `apps/qivxif-serverd` | smoke probe through Compose network |
| Region mutation authority | `architecture/simulation/region-ownership.md` | `crates/qivxif-sim` | region tests and mutation probe |
| ECS boundary | `architecture/simulation/ecs-boundary.md` | `crates/qivxif-sim` | dependency audit and region tests |
| Tick policy | `architecture/simulation/tick-policy.md` | `crates/qivxif-sim` | simulation tick tests |
| Coordinate model | `architecture/world/coordinate-model.md` | `crates/qivxif-world`, `crates/qivxif-core` | world tests |
| Chunk streaming | `architecture/world/chunk-streaming.md` | `crates/qivxif-world`, `crates/qivxif-probe` | chunk probe path |
| Hot state | `architecture/persistence/hot-state.md` | `crates/qivxif-storage` | hot write/read/reopen storage tests and `persist-check` probe |
| Storage schema | `architecture/persistence/schema-contracts.md` | `crates/qivxif-storage` | storage schema and commit-boundary tests |
| Cold archives | `architecture/persistence/object-archives.md` | `crates/qivxif-storage` | object_store manifest smoke tests |
| Compose acceptance | `operations/verification/compose-pipeline.md` | `scripts/verify-compose.sh`, Compose files | `scripts/verify-compose.sh` |
| Static gates | `operations/verification/static-gates.md` | `scripts/verify-static.sh`, `crates/qivxif-quality` | `verify` Compose service |
| Protocol probes | `operations/verification/protocol-probes.md` | `crates/qivxif-probe`, `apps/qivxifctl` | probe services in Compose |
| Docs topology | `repository/rules/line-limits.md` | `apps/qivxifctl`, `crates/qivxif-quality` | `qivxifctl docs validate-topology` |
| Source line limits | `repository/rules/line-limits.md` | `crates/qivxif-quality` | `qivxifctl quality check-lines` |
| LLM authoring | `vision/llm-authoring.md` | repository docs structure | docs topology and line-limit gates |

## Dormant Contracts

| Contract | Owner doc | Activation rule |
| --- | --- | --- |
| Native client shells | `architecture/client/` | Add workspace crates only after shell docs define public checks |
| Renderer family | `architecture/client/renderer.md` | Add renderer code only after client shell boundary is active |
| Markets and claims | `product/gameplay/`, `product/social/` | Add gameplay code after server authority and persistence checks pass |

## Workspace Members

| Member | Role |
| --- | --- |
| `apps/qivxif-serverd` | Authoritative server process |
| `apps/qivxifctl` | Agent-friendly quality and probe CLI |
| `crates/qivxif-core` | Shared primitive types |
| `crates/qivxif-protocol` | Public protocol catalog and postcard encoding |
| `crates/qivxif-net` | QUIC and certificate helpers |
| `crates/qivxif-world` | Coordinates and chunk generation |
| `crates/qivxif-sim` | Region-local mutation authority |
| `crates/qivxif-storage` | Hot durable state and archive boundary |
| `crates/qivxif-quality` | Repository quality checks |
| `crates/qivxif-probe` | Public probe scenarios |
