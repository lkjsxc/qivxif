# Report Synthesis

## Source

`tmp/deep-research-report (49).md`.

## Durable Findings

- qivxif is greenfield.
- Sibling repositories use docs-first workflow, recursive TOCs, line limits, and Compose gates.
- The recommended product is a Rust-native voxel MMORPG survival sandbox.
- The recommended architecture is one authoritative world server with native clients.
- The recommended initial path is canon, workspace, transport, simulation, world, persistence, then client.
- Keep the architecture narrow: one renderer family, one protocol library, one persistence boundary, and region-owned mutation.
- Use `bevy_ecs` inside region actors once entity complexity needs dense data-oriented simulation.
- Use QUIC streams plus datagrams through Quinn for transactional and latest-wins traffic.
- Use `postcard` for compact schema-bound wire messages.
- Use `rkyv` for read-mostly archives and local cache blobs, not client-trusted gameplay truth.
- Use `redb` for hot state and `object_store` for snapshots, replays, crash bundles, and large artifacts.
- Keep security layered: QUIC/TLS for sessions, Argon2id for password-equivalent secrets, Ed25519 for signatures, rcgen only for local certificates, and rustls for auxiliary HTTPS endpoints.
- Use nextest, doctests, insta, proptest, and Criterion as the testing stack.
- Product rules should close the current gaps with zone-scaled death loss, activity-based claim decay, and regional markets.

## Canon Migration

Research facts become durable only when copied into owner docs under `docs/`.
Report wording that implies named product lines or preserved old paths is
translated into epoch wording before it becomes canon.
