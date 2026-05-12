# GUI First Session

## Status

- Status: implemented for desktop smoke.
- Owner: desktop GUI client over the implemented protocol loop.

## Player Flow

1. Start the desktop client with server address, server name, TLS mode, and
   player label.
2. Connect to the server.
3. Join the world.
4. Load a small origin chunk neighborhood.
5. See terrain generated from server `BlockCell` data.
6. Move a local camera for inspection and targeting.
7. Place or remove one block through the authoritative server path.
8. See the acknowledged terrain change in the local view.

## Excluded

- Account login.
- Inventory.
- Combat.
- Player movement authority.
- Mobile lifecycle.
- Remote asset streaming.
