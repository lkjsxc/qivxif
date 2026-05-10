# Chunk Streaming

## Interest Inputs

- Player position.
- Velocity.
- Visual radius.
- Interaction radius.
- Pending requests.

## Rule

Collision and interaction data have priority over decorative data.

## Initial Slice

The first slice returns one deterministic chunk plus persisted block overrides.
