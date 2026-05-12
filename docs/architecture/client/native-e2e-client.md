# Native E2E Client

## Status

- Status: active.
- Owner: `apps/qivxif-client-desktop`.
- Runtime owner: `crates/qivxif-client-core`.
- Renderer owner: `crates/qivxif-render`.

## Contract

- Use the public QUIC protocol only.
- Keep `qivxif-client-desktop` as a thin native shell.
- Own connection, cache, command execution, and snapshots in
  `qivxif-client-core`.
- Own camera and pointer-to-cell helpers in `qivxif-input`.
- Own HUD state in `qivxif-ui`.
- Own GPU frame execution in `qivxif-render`.
- Keep `smoke-frame` as deterministic CPU evidence.

## Native Loop

1. Create a Tokio runtime for client networking.
2. Create a `winit` event loop and native window.
3. Spawn the client runtime task.
4. Connect, send `Hello`, and join the world.
5. Fetch the origin chunk neighborhood.
6. Render cached authoritative `BlockCell` data.
7. Convert keyboard and pointer actions into runtime commands.
8. Apply visual state only after authoritative acknowledgements.

## Commands

- `qivxif-client-desktop run` opens an interactive native window.
- `qivxif-client-desktop e2e` opens a native window, runs scripted
  verification, writes evidence, and exits.
- `qivxif-client-desktop smoke-frame` remains the deterministic CPU smoke gate.

## Scripted Acceptance

The e2e command must prove:

- Connected and joined.
- At least nine chunks are cached.
- Cached cells are nonzero.
- GPU frames were submitted.
- The evidence frame is nonblank.
- Block `9` placement at `{ x: 1, y: 3, z: 1 }` is acknowledged and cached.
- Removal through block `0` is acknowledged and reflected in cache.

## Boundaries

- Do not add wire messages in this wave.
- Do not change storage schema.
- Do not add server API shortcuts.
- Use Linux Compose and Xvfb for live native-window proof.
- Use the Windows bundle check for artifact, launcher, manifest, checksum, and
  zip proof.
