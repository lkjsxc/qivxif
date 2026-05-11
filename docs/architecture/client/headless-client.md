# Headless Client

## Status

- Status: implemented.
- Owner: `apps/qivxif-client-cli`.
- Shared client core is not extracted yet.

## Role

- Provides a protocol-facing client executable.
- Connects through QUIC.
- Sends `Hello`, `JoinWorld`, `ChunkRequest`, `PlaceBlock`, and
  `FlushPersistence`.
- Prints compact command output for agent and artifact smoke checks.

## Current Limits

- No renderer.
- No input loop.
- No local gameplay cache.
- No account authentication.
- No production release trust policy.

## Verification

- Docker Compose runs `client-cli` against the local server.
- The client uses local Compose trust only when explicitly configured for that
  environment.

## Next Activation

- Extract shared client session behavior into `crates/qivxif-client-core`.
- Add Windows artifact build verification before graphical client work.
