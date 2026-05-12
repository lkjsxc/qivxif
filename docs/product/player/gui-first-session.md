# GUI First Session

## Status

- Status: implemented for desktop smoke.
- Owner: desktop GUI client over the implemented protocol loop.

## Player Flow

Implemented desktop smoke flow:

1. Start the desktop client with server address, server name, TLS mode, and
   player label.
2. Connect to the server.
3. Join the world.
4. Load a small origin chunk neighborhood.
5. Place one deterministic block through the authoritative server path.
6. Render one deterministic frame from acknowledged `BlockCell` data.

Later interactive flow:

1. Move a local camera for inspection and targeting.
2. Place or remove blocks through the authoritative server path.
3. See acknowledged terrain changes in the local view.

## Excluded

- Account login.
- Inventory.
- Combat.
- Interactive camera controls.
- Player movement authority.
- Mobile lifecycle.
- Remote asset streaming.
