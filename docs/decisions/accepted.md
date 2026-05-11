# Accepted Decisions

## Product

- Native full-parity clients.
- Dual camera.
- Open frontier PvP.
- Permanent accepted terrain edits.
- Player-crafted economy.
- Zone-scaled death loss.
- Activity-based claim decay.
- Regional player markets.

## Architecture

- Rust workspace.
- One authoritative server process.
- QUIC transport.
- Region-owned mutation.
- `bevy_ecs` inside region actors.
- redb hot state.
- `object_store` cold artifacts.
- `postcard` protocol messages.
- `rkyv` read-mostly archives and caches.
- Layered security with QUIC/TLS, Argon2id, Ed25519, rcgen for local certificates, and rustls for HTTPS endpoints.
- nextest, doctests, insta, proptest, and Criterion for the test stack.
- Docker Compose acceptance.
