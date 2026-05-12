# LOD And Summaries

## Status

- Status: not implemented.
- `qivxif-render` exists for deterministic smoke output and native GPU frames.
- No mesh or summary storage exists.

## Current Facts

- Server returns exact `BlockCell` lists for requested chunks.
- Gameplay does not depend on GPU features.

## Activation Requirements

- Define mesh and summary ownership in the renderer boundary.
- Define mesh or summary payloads.
- Define cache ownership.
- Add verification for near-field correctness.

## Rule

- Do not describe LOD as active until chunk summaries or renderer code exists.
