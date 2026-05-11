# Inventory And Crafting

## Scope

This file owns item identity, inventory reconciliation, and crafting dependencies.

## Inventory Facts

- Server owns item identity, count, durability, and location.
- Client transactions use stable request identifiers.
- Server acknowledgements reconcile UI state.

## Crafting Facts

- Player-crafted supply is central.
- Recipes can depend on skills, stations, resources, and zone materials.

## Cross-References

- Economy direction is defined in [economy.md](economy.md).
- Skill unlock shape is defined in [../player/progression.md](../player/progression.md).
- Starter resources are introduced by [../player/onboarding.md](../player/onboarding.md).
