# Transport

## Canon

qivxif uses QUIC through Quinn.

## Reliable Streams

- Login.
- Join.
- Inventory.
- Crafting.
- Ability casts.
- Chunk bundles.
- Admin probes.

## Datagrams

- Input frames.
- Look deltas.
- Ephemeral entity deltas.
- Effect notifications.

## Initial Slice

The first server slice may use reliable streams only, but it must run through
QUIC so later lanes share the same transport boundary.
