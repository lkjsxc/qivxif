# Layout Tree

## Purpose

This doc owns the canonical N-way tile tree shape used by reducers, event
payloads, and browser rendering.

## Node Kinds

### Stack

A stack is a tab group inside one pane.

```json
{
  "kind": "stack",
  "active": 0,
  "tabs": [
    {
      "pane_node_id": "nod_<64 hex>",
      "pane_kind": "editor",
      "target_node_id": "nod_<64 hex>",
      "title": "Text Node"
    }
  ]
}
```

### Split

A split arranges N child tiles along one axis.

```json
{
  "kind": "split",
  "axis": "row",
  "children": [
    { "kind": "stack", "active": 0, "tabs": [] }
  ],
  "sizes": [600, 400]
}
```

- `axis`: `row` or `column`.
- `children`: non-empty array of stack or split nodes.
- `sizes`: pixel weights for each child; length must equal `children.length`.
- Minimum child size: 180px height and 260px width.

## Tile Layout Snapshot

```json
{
  "root": { "kind": "stack", "active": 0, "tabs": [] },
  "maximized_pane_id": null
}
```

## Smart Split Insertion

Edge-drop and menu split share one insertion primitive:

1. Find the stack containing the target pane.
2. If the parent split axis matches the requested edge axis, insert a sibling
   child at the requested index and rebalance `sizes`.
3. If the axis differs, wrap the target child and new stack in a new split node.
4. Left and top edges insert before the target child.
5. Right and bottom edges insert after the target child.
6. Left and right map to `row`; top and bottom map to `column`.

## Resize

- Resize handles sit between adjacent split children.
- Dragging a handle updates two adjacent entries in `sizes`.
- Reducers enforce minimum child sizes and reject zero-width results.
- Resize emits `tile.layout_set` with the full tree snapshot.

## Collapse Rules

- Removing the last tab from a stack removes that stack node.
- If a split has one child left, the split collapses to that child.
- Maximizing sets `maximized_pane_id`; restore clears it without changing the
  stored tree.

## Invariants

- Every `pane_node_id` appears at most once in the tree.
- Stack `active` is always a valid tab index.
- Split `sizes` sum to a positive total and match child count.
- The tree references pane node IDs, never DOM IDs.

## Related Docs

- [tiled-tabs.md](tiled-tabs.md): tab behavior and reducer contract.
- [drag-drop.md](drag-drop.md): edge-drop mapping to smart split.
- [../../architecture/schema/event-kinds.md](../../architecture/schema/event-kinds.md):
  `tile.layout_set` payload.
