# GUI Runtime

## Status

- Status: implemented for desktop smoke.
- Owner: `crates/qivxif-client-core` for protocol runtime and cache.
- Shell owner: `apps/qivxif-client-desktop`.

## First Runtime Loop

1. Connect through the same QUIC endpoint as the headless client.
2. Send `Hello` with the current protocol contract.
3. Send `JoinWorld`.
4. Request a small chunk neighborhood around the origin.
5. Store returned `BlockCell` values in a local cache.
6. Expose a read-only snapshot to renderer, input, and UI code.
7. Send place or remove commands only through `PlaceBlock`.
8. Apply visual block changes from authoritative `MutationAck`.

## Boundaries

- The runtime does not own server truth.
- The runtime does not invent movement protocol messages.
- The runtime may move a local camera for viewing and targeting.
- Removal is `PlaceBlock` with block `0`.

## Verification

- Headless client behavior stays covered by the `client-cli` Compose service.
- Desktop runtime behavior is covered by
  [../../operations/verification/desktop-smoke.md](../../operations/verification/desktop-smoke.md).
