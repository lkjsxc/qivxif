# Chunk Streaming

## Interest Inputs

- Player position.
- Velocity.
- Visual radius.
- Interaction radius.
- Pending requests.

Player-facing world and camera expectations are owned by
[../../product/world/README.md](../../product/world/README.md) and
[../../product/player/camera-controls.md](../../product/player/camera-controls.md).

## Rule

Collision and interaction data have priority over decorative data.

## Initial Slice

The first slice returns one deterministic chunk plus chunk-scoped edit overlays
loaded from the `sections` table.
