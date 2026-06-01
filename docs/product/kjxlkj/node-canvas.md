# Node Canvas

## Behavior

- Board positions are stored as records linked to board and node IDs.
- Users can expand graph depth from any board node.
- The canvas shows node kind, title, sync state, and conflict state.
- Dragging updates local board operations.

## Constraints

- Large neighborhoods are bounded by depth and cache budget.
- Layout updates are debounced but never lost.
