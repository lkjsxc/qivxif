# Authority Rules

## Server Owns

- Movement acceptance.
- Combat result.
- Inventory state.
- Crafting outcome.
- Claim permissions.
- World mutation.

Player-facing combat, inventory, crafting, and claim behavior is owned by
[../../product/gameplay/combat-abilities.md](../../product/gameplay/combat-abilities.md),
[../../product/gameplay/inventory-crafting.md](../../product/gameplay/inventory-crafting.md),
and [../../product/gameplay/bases-claims.md](../../product/gameplay/bases-claims.md).

## Client Owns

- Input collection.
- Presentation.
- Prediction.
- Local cache.

## Rule

Client data is intent until the server accepts it.
