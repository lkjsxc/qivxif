# Death And Respawn

## Death

- Server owns lethal damage.
- Client may predict presentation only.
- Death loss scales by zone.
- Bound identity items, quest-critical items, and equipped recovery tools are
  preserved unless a specific event overrides them.

## Zone Loss

- Sanctuary: no inventory loss and no durability loss.
- Starter basin: no item drop; minor durability loss on damaged gear.
- Frontier: carried raw resources, extracted valuables, and unbound trade goods
  can drop or become corpse-bound; gear takes durability loss.
- Contested event: event-tagged loot can be escrowed, dropped, or claimed by
  event rules after the server records the death.

## Recovery

- Corpse or cache recovery is allowed only where zone rules permit it.
- Recovery timers are server-owned.
- Logged-out or disconnected clients do not pause exposed death caches.

## Respawn

- Respawn uses sanctuary, starter basin, base bed, or explicit anchor rules.
- Destination checks use the same safety style as starter teleport.
- Unsafe anchors fall back to the nearest valid sanctuary or starter basin.
