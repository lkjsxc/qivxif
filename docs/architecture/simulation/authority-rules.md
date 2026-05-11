# Authority Rules

## Status

- Status: implemented for terrain mutation only.
- Owner: `apps/qivxif-serverd::request` and `crates/qivxif-sim`.

## Server Owns Now

- Session phase acceptance.
- Chunk request handling.
- Terrain mutation acceptance.
- Persistence flush acceptance.

## Client Or Probe Owns Now

- Request construction.
- Local transport connection.
- Probe assertions.

## Not Implemented

- Movement simulation.
- Combat results.
- Inventory state.
- Crafting outcomes.
- Claim permissions.

## Product Links

- Combat: [../../product/gameplay/combat-abilities.md](../../product/gameplay/combat-abilities.md).
- Inventory and crafting: [../../product/gameplay/inventory-crafting.md](../../product/gameplay/inventory-crafting.md).
- Bases and claims: [../../product/gameplay/bases-claims.md](../../product/gameplay/bases-claims.md).

## Rule

- Client data is intent until the server accepts it.
