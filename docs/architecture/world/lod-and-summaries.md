# LOD And Summaries

## Status

- Status: not implemented.
- No renderer crate exists.
- No mesh or summary storage exists.

## Current Facts

- Server returns exact `BlockCell` lists for requested chunks.
- Gameplay does not depend on GPU features.

## Activation Requirements

- Define a client renderer crate.
- Define mesh or summary payloads.
- Define cache ownership.
- Add verification for near-field correctness.

## Rule

- Do not describe LOD as active until chunk summaries or renderer code exists.
