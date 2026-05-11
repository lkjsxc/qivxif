# Deployment

LLM purpose: deployment contracts for running the current server slice.

## Scope

- Local Docker Compose runtime.
- Explicit runtime configuration fields.
- Durable state and backup handling.

## Not Scope

- Protocol behavior; see [../../architecture/network/README.md](../../architecture/network/README.md).
- Gameplay state rules; see [../../architecture/world/README.md](../../architecture/world/README.md).

## Child Index

- [compose-stack.md](compose-stack.md): local Compose runtime
- [runtime-config.md](runtime-config.md): config file contract
- [state-and-backups.md](state-and-backups.md): state volumes and backups
