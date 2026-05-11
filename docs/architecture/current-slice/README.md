# Current Slice

This directory owns the implemented authoritative server slice.

## Child Index

- [vertical-loop.md](vertical-loop.md): public path proven by Compose probes
- [ownership-map.md](ownership-map.md): code owners for each runtime contract
- [contract-matrix.md](contract-matrix.md): docs-to-code verification matrix
- [request-replay.md](request-replay.md): duplicate mutating request behavior

## Scope

- One authoritative server process.
- One in-process region actor.
- One reliable-stream QUIC request per bidirectional stream.
- Postcard-encoded public messages.
- redb-backed chunk edit overlays.
- Compose probes for smoke and restart persistence.

## Excluded

- Renderer and native client shells.
- Authentication and account recovery.
- Markets, bases, skills, combat, and claims.
- Multi-region handoff.
- Object-store archives.
