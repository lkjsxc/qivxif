# Node Canvas

## Behavior

- Board positions are stored as records linked to board and node IDs.
- Users can expand graph depth from any board node.
- The canvas shows node kind, title, sync state, and conflict state.
- Dragging updates local board operations.

## Initial Controls

The first canvas may use explicit controls before pointer dragging:

- Add current text node to the active board.
- Move the selected board item by appending a placement record.
- Link two board items with a typed graph edge.
- Open a board from its node ID and derive items from neighborhood edges.

## Constraints

- Large neighborhoods are bounded by depth and cache budget.
- Layout updates are debounced but never lost.
- Local dirty placement records are rendered before server acceptance.
