# Safe Spawn And Teleport

## Status

- Status: not implemented.
- No spawn or teleport protocol message exists.
- No player position persistence exists.

## Current Facts

- Probe joins with a player label only.
- Server does not assign a spawn position.
- Server does not validate teleport destinations.

## Activation Requirements

- Define player position state.
- Define safe footing checks.
- Define hazard checks.
- Define success and failure protocol messages.
- Add tests that preserve source location on failure.
