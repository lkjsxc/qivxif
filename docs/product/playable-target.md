# Playable Target

## Initial Target

The initial server slice proves the smallest durable loop:

1. Client establishes a secure connection to the world.
2. Client joins the world.
3. Client requests a chunk.
4. Client places a block.
5. Server persists the mutation.
6. Server restarts.
7. Probe verifies the mutation remains.

Network transport is owned by [../architecture/network/transport.md](../architecture/network/transport.md).

## Excluded From Initial Target

- Renderer.
- Mobile shells.
- Combat.
- Claim UI.
- Economy UI.
