# Cross-Region Handoff

## Status

- Status: not implemented.
- Current implementation has one region actor.

## Current Boundary

- `RegionHandle` hides actor internals behind async methods.
- This boundary can become a handoff message boundary later.

## Activation Requirements

- Define region partitioning.
- Define source-region validation.
- Define destination-region acceptance or rejection.
- Define session routing updates.
- Add executable multi-region tests.

## Rule

- Do not document handoff behavior as active until more than one region actor exists.
