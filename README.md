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

- One authoritative world server.
- Native full-parity clients for desktop, Android, and iOS.
- Open frontier PvP outside protected sanctuary and starter spaces.
- Permanent accepted terrain edits.
- Player-crafted economy.
- Zone-scaled death loss.
- Activity-based claim decay.
- Regional markets.

## Verification

Use Docker Compose for acceptance:

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --build -T verify
```
