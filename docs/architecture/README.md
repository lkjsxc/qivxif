# Architecture

LLM retrieval scope: architecture contracts for the current qivxif codebase.

## Current Implementation

- The implemented slice is server-only.
- No native client crate exists.
- One authoritative server accepts QUIC requests.
- Public messages are postcard-encoded Rust enums.
- One region actor owns terrain mutation.
- redb stores world metadata and chunk edit overlays.
- object_store is implemented only for local archive manifest smoke tests.

## Directory Index

- [current-slice/README.md](current-slice/README.md): implemented server slice.
- [runtime/README.md](runtime/README.md): server process and task boundaries.
- [network/README.md](network/README.md): QUIC transport, sessions, and protocol.
- [simulation/README.md](simulation/README.md): region actor and authority rules.
- [world/README.md](world/README.md): coordinates and deterministic chunk cells.
- [persistence/README.md](persistence/README.md): redb hot state and archive manifests.
- [client/README.md](client/README.md): dormant client architecture notes.

## Reading Order

1. Start with [current-slice/vertical-loop.md](current-slice/vertical-loop.md).
2. Check owners in [current-slice/ownership-map.md](current-slice/ownership-map.md).
3. Check protocol facts in [network/protocol-messages.md](network/protocol-messages.md).
4. Check persistence facts in [persistence/schema-contracts.md](persistence/schema-contracts.md).
