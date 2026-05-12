# qivxif

qivxif is a Rust-native voxel MMORPG survival sandbox.

The project is built for LLM-maintained development. Durable truth belongs in
`docs/`, then implementation follows that canon.

## Start Here

- [docs/README.md](docs/README.md): documentation canon and reading order
- [docs/vision/purpose.md](docs/vision/purpose.md): product purpose
- [docs/product/playable-target.md](docs/product/playable-target.md): target play shape
- [docs/architecture/README.md](docs/architecture/README.md): system architecture
- [docs/operations/verification/compose-pipeline.md](docs/operations/verification/compose-pipeline.md): acceptance gate

## Current Shape

- One implemented authoritative world server.
- One implemented headless protocol client.
- One implemented desktop smoke client that renders a deterministic frame from
  public chunk data.
- Compose probes for smoke, protocol guards, malformed wire input, request
  replay, client CLI behavior, desktop smoke, and persistence restart behavior.
- A portable unsigned Windows demo bundle for internal server and client smoke
  runs.

## Product Goals

- Native full-parity clients for desktop, Android, and iOS.
- Open frontier PvP outside protected sanctuary and starter spaces.
- Permanent accepted terrain edits.
- Player-crafted economy.
- Zone-scaled death loss.
- Activity-based claim decay.
- Regional markets.

## Verification

Use the full Docker Compose acceptance flow:

```bash
./scripts/verify-compose.sh
```

The script runs the static gate, starts the server, runs smoke and persistence
probes, restarts the server, and verifies the persisted mutation.

Build the Windows demo bundle with:

```bash
./scripts/verify-windows-demo-bundle.sh
```
