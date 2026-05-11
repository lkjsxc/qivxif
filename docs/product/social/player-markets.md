# Player Markets

## Scope

This file owns regional market behavior and transaction settlement.

## Purpose

Markets connect crafting, logistics, and frontier extraction.

## Rules

- Players list crafted or extracted goods.
- Server owns transaction settlement.
- Market records must be durable.
- NPC participation is limited to basic seeding and recovery.

## Regional Markets

- Markets are regional, not one instant global exchange.
- Listings belong to a settlement, sanctuary, starter basin, or frontier hub.
- Buyers can see remote summaries when discovery rules allow it.
- Physical delivery, courier contracts, or caravan events move goods between
  regions.
- Transport risk is part of the economy outside protected zones.

## Settlement

- The server locks listed goods before publishing an order.
- The server releases goods only after payment and delivery rules succeed.
- Failed or expired orders return goods to the owning regional inventory.

## Cross-References

- Economy direction is defined in [../gameplay/economy.md](../gameplay/economy.md).
- Zone transport risk is defined in [../world/zones.md](../world/zones.md).
- Guild market identity is defined in [parties-guilds.md](parties-guilds.md).
