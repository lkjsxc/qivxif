# Headless Client

## Status

- Status: implemented.
- Owners: `crates/qivxif-client-core` and `apps/qivxif-client-cli`.

## Role

- Provides reusable headless client session behavior.
- Provides a protocol-facing client executable.
- Connects through QUIC with explicit TLS mode and server name.
- Sends `Hello`, `JoinWorld`, `ChunkRequest`, `PlaceBlock`, and
  `FlushPersistence`.
- Prints compact command output for agent and artifact smoke checks.

## Current Limits

- No renderer.
- No input loop.
- No local gameplay cache.
- No account authentication.
- No production release signing policy.

## Verification

- Docker Compose runs `client-cli` against the local server.
- The CLI uses `--tls local-compose` only for local Compose.
- Default CLI TLS mode is `verified`.

## Next Activation

- Add Windows artifact build verification before graphical client work.
