# Report Synthesis

## Source

`tmp/deep-research-report (50).md`.

## Durable Findings

- qivxif already has a coherent docs-first canon and an initial server slice.
- Sibling repositories use docs-first workflow, recursive TOCs, line limits, and Compose gates.
- The recommended product is a Rust-native voxel MMORPG survival sandbox.
- The recommended architecture is one authoritative world server with native clients.
- The recommended path is canon, verified server authority, persistence hardening,
  protocol lanes, world depth, then client and gameplay.
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

## Canon Migration

Research facts become durable only when copied into owner docs under `docs/`.
Report wording that implies named product lines or retired-path preservation is
translated into epoch wording before it becomes canon.

## Quarantined Wording

- Treat raw report terms as research input until an owner doc translates them.
- Replace compatibility or legacy-shape language with protocol epoch, build
  epoch, or schema epoch decisions.
- Do not copy wording that preserves retired contracts as parallel canon.
