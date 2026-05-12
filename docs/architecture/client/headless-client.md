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

## Windows Demo Bundle

- The raw Windows CLI executable artifact is historical.
- The current Windows artifact is a portable internal demo bundle.
- The bundle includes the headless client, server, local config, data
  placeholder, and `.cmd` launchers.
- Bundle construction and limits are owned by
  [../../operations/verification/windows-demo-bundle.md](../../operations/verification/windows-demo-bundle.md).

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
- Docker Compose builds the Windows demo bundle before graphical client work.
