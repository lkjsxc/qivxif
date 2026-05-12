# Current Slice

This directory owns the implemented authoritative server slice.

## Status

- Status: implemented.
- Runtime: one server process.
- Client: headless protocol client, desktop smoke client, and probe CLI.
- No interactive graphical gameplay client exists.
- Persistence: redb hot database plus local archive manifest smoke path.

## Child Index

- [vertical-loop.md](vertical-loop.md): public path proven by Compose probes.
- [ownership-map.md](ownership-map.md): code owners for each runtime contract.
- [contract-matrix.md](contract-matrix.md): docs-to-code verification matrix.
- [request-replay.md](request-replay.md): duplicate mutating request behavior.

## Implemented Scope

- One authoritative server process.
- One in-process region actor.
- One QUIC connection per probe client.
- One request per reliable bidirectional stream.
- Postcard-encoded public protocol messages.
- redb-backed chunk edit overlays.
- Local object archive manifest write/read/list tests.
- Compose probes for smoke, protocol guards, malformed wire, request replay,
  headless client smoke, desktop smoke, and restart persistence.

## Not Implemented In Current Slice

- Interactive desktop gameplay shell.
- `wgpu` renderer.
- Mobile shell.
- Movement protocol.
- Asset cache.
- Account authentication.
- Character inventory, combat, skills, claims, bases, markets, or mail.
- Multi-region ownership handoff.
- Hosted object archive backends.
