# Combat And Abilities

## Scope

This file owns combat authority and ability result semantics.

## Combat Facts

- Server owns hit validation and damage.
- Client predicts visuals.
- Open frontier combat is allowed unless a protected zone disables it.

## Ability Facts

- Ability casts are reliable intent messages.
- Server returns accepted outcome, rejected outcome, cooldown, or correction.
- Visual effects may travel over latest-wins lanes.

## Cross-References

- Zone PvP constraints are defined in [../world/zones.md](../world/zones.md).
- Death after lethal damage is defined in [../player/death-respawn.md](../player/death-respawn.md).
- Camera input responsibilities are defined in [../player/camera-controls.md](../player/camera-controls.md).
