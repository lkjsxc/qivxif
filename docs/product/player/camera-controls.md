# Camera And Controls

## Scope

This file owns camera modes and player movement responsibility.

## Facts

- Third-person is the default MMORPG camera.
- First-person is a precision mode for building, mining, and aiming.
- Both cameras use the same authoritative movement contract.

## Current Implementation

- No graphical client exists yet.
- No movement protocol exists yet.
- The desktop GUI first session may move a local camera without claiming player
  movement authority.

## Future Client Responsibility

- Predict local movement.
- Render camera transitions.
- Send intent frames to the server.

## Future Server Responsibility

- Accept, reject, or correct movement.
- Own combat and interaction truth.

## Cross-References

- Combat authority is defined in [../gameplay/combat-abilities.md](../gameplay/combat-abilities.md).
- Terrain editing use cases are defined in [../world/terrain-editing.md](../world/terrain-editing.md).
