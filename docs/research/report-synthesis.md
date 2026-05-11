# Report Synthesis

## Sources

- `tmp/deep-research-report (47).md`
- `tmp/deep-research-report (49).md`
- `tmp/deep-research-report (50).md`
- `tmp/deep-research-report (51).md`
- `tmp/deep-research-report (53).md`

## Durable Findings

- qivxif already has a coherent docs-first canon, server slice, and headless client.
- Sibling repositories use docs-first workflow, recursive TOCs, line limits, and Compose gates.
- The recommended product is a Rust-native voxel MMORPG survival sandbox.
- The recommended architecture is one authoritative world server with native clients.
- The recommended path is canon, verified server authority, persistence hardening,
  protocol lanes, world depth, then graphical client and gameplay.
- Keep the architecture narrow: one renderer family, one protocol library, one persistence boundary, and region-owned mutation.
- Use `bevy_ecs` inside region actors once entity complexity needs dense data-oriented simulation.
- Use QUIC streams plus datagrams through Quinn for transactional and latest-wins traffic.
- Use `postcard` for compact schema-bound wire messages.
- Use `rkyv` for read-mostly archives and local cache blobs, not client-trusted gameplay truth.
- Use `redb` for hot state and `object_store` for snapshots, replays, crash bundles, and large artifacts.
- Keep security layered: QUIC/TLS for sessions, Argon2id for password-equivalent secrets, Ed25519 for signatures, rcgen only for local certificates, and rustls for auxiliary HTTPS endpoints.
- Use nextest, doctests, insta, proptest, and Criterion as the testing stack.
- Zone-scaled death loss, activity-based claim decay, and regional markets are
  already product canon.
- Windows delivery should start with a headless/protocol client surface before a
  graphical desktop client.
- Linux Docker builds can produce repeatable `x86_64-pc-windows-gnu` artifacts;
  final user-facing Windows releases should use a Windows-native MSVC job.
- Windows-container-first delivery is not the preferred path from a Linux host.
- Local self-signed certificate helpers are acceptable only for local Compose and
  must not become production trust policy.

## Owner Doc Targets

| Finding | Durable owner |
| --- | --- |
| Authoritative server slice | `architecture/current-slice/` |
| QUIC session flow | `architecture/network/session-lifecycle.md` |
| Postcard public messages | `architecture/network/protocol-messages.md` |
| Region-owned mutation | `architecture/simulation/region-ownership.md` |
| ECS boundary | `architecture/simulation/ecs-boundary.md` |
| Hot state | `architecture/persistence/hot-state.md` |
| Cold archives | `architecture/persistence/object-archives.md` |
| Compose acceptance | `operations/verification/compose-pipeline.md` |
| Native client shell | `architecture/client/` |
| LLM authoring rules | `vision/llm-authoring.md` |
| Windows client delivery | `research/client-delivery-references.md` |
| Verification discipline | `research/operations-references.md` |
| Trust boundaries | `research/security-references.md` |

## Implementation Order

1. Keep docs topology and file limits green.
2. Maintain a docs-to-code contract matrix for every active behavior.
3. Harden the server/probe slice before widening gameplay.
4. Prove public behavior through QUIC and postcard, not in-process shortcuts.
5. Persist active terrain edits in `redb` before adding archive workflows.
6. Add `object_store` archive smoke paths only after hot-state behavior is stable.
7. Keep renderer and native-shell work behind the client boundary docs.
8. Prove a Windows headless client artifact before graphical clients.

## Stack Boundaries

| Library | Boundary |
| --- | --- |
| Quinn | Public QUIC transport and stream/datagram lanes |
| `postcard` | Compact public wire messages |
| `redb` | Local hot durable state |
| `object_store` | Cold immutable artifacts and off-host storage |
| `rkyv` | Read-mostly archive/cache blobs after validation |
| `bevy_ecs` | Region-local simulation data layout |
| `wgpu` | Renderer implementation behind client boundary |
| `winit` | Native platform shell lifecycle |
| `tracing` | Runtime, request, storage, and simulation observability |

## Canon Migration

Research facts become durable only when copied into owner docs under `docs/`.
Report wording that implies named product lines or retired-path preservation is
translated into contract wording before it becomes canon.

## Quarantined Wording

- Treat raw report terms as research input until an owner doc translates them.
- Translate retired-shape language into protocol contract, build contract, or schema
  contract decisions.
- Do not copy wording that keeps retired contracts as parallel canon.

## Non-Canon Report Content

- External citations are research evidence, not qivxif authority.
- Sibling repository practices become qivxif canon only when copied into an
  owner doc.
- Any report recommendation that expands scope beyond the current slice must
  land in docs before implementation starts.
