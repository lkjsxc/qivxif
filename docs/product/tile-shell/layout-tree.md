# Layout Tree

## Purpose

This doc owns the canonical N-way tile tree shape used by reducers, event
payloads, and browser rendering.

## Type Shape

```typescript
type TileLayout = {
  root: TileNode;
  maximizedPaneId: PaneId | null;
};

type TileNode = TileStack | TileSplit;

type TileStack = {
  kind: "stack";
  id: TileId;
  activePaneId: PaneId;
  paneIds: PaneId[];
};

type TileSplit = {
  kind: "split";
  id: TileId;
  axis: "row" | "column";
  children: TileNode[];
  weights: number[];
};
```

## Stack

A stack is one pane chrome with a tab rail and one body region. `paneIds` are
visible tab identities. `activePaneId` must be a member of `paneIds`.

## Split

A split arranges N child tiles along one axis.

- `axis` is `row` or `column`.
- `children` is non-empty.
- `weights` length equals `children.length`.
- Weights are normalized positive numbers.
- Minimum child size is 180px height and 260px width.

## Durable Rules

- Stable tile ids identify split and stack nodes.
- Stable pane ids identify visible tabs.
- No DOM id appears in durable state.
- No pane id appears more than once.
- Maximize stores only `maximizedPaneId`; it does not rewrite the tree.

## Smart Split Insertion

Edge-drop and menu split share one insertion primitive:

1. Find the stack containing the target pane.
2. Map left and right to `row`; top and bottom to `column`.
3. If the parent split axis matches, insert a sibling child at the edge index.
4. If the axis differs, wrap the target child and new stack in a split node.
5. Left and top insert before the target child.
6. Right and bottom insert after the target child.
7. Rebalance weights while respecting minimum sizes.

## Resize

- Resize handles sit between adjacent split children.
- Pointermove updates ephemeral sizes only.
- Resize end dispatches `resizeSplit` and persists accepted layout.
- Reducers update adjacent weights while respecting minimum sizes.
- Rejected zero-size results are typed no-ops.

## Collapse Rules

- Removing the last pane from a stack removes that stack node.
- If a split has one child left, the split collapses to that child.
- If a command would remove the last usable workspace pane, reducer returns a no-op.
- Restore clears `maximizedPaneId` without changing the stored tree.

## Invariants

- Every stack has at least one pane id unless it is being eliminated inside a reducer.
- `activePaneId` is valid for every committed stack.
- Split children are non-empty.
- Split weights match child count and sum to a positive value.
- The tree references pane ids only.

## Related Docs

- [tiled-tabs.md](tiled-tabs.md): tab behavior and reducer contract.
- [drag-drop.md](drag-drop.md): edge-drop mapping to smart split.
- [performance.md](performance.md): interaction budgets.
- [../../architecture/schema/event-kinds.md](../../architecture/schema/event-kinds.md):
  `tile.layout_set` payload.
