# Camera And Controls

## Camera Contract

- Third-person is the default MMORPG camera.
- First-person is a precision mode for building, mining, and aiming.
- Both cameras use the same authoritative movement contract.

## Client Responsibility

- Predict local movement.
- Render camera transitions.
- Send intent frames to the server.

## Server Responsibility

- Accept, reject, or correct movement.
- Own combat and interaction truth.
