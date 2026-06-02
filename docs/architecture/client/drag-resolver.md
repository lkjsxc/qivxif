# Drag Resolver

## Purpose

This doc owns browser-side tab drag resolution. Pointer drag and native HTML5
drag must call the same pure resolver.

## Modules

| Module | Role |
| --- | --- |
| `domain/drop-resolver.ts` | Pure zone and insertion resolution |
| `domain/tab-drag-state.ts` | Shared active target store |
| `effects/pointer-drag.ts` | Long-press, threshold, pointer capture |
| `effects/native-drag.ts` | HTML5 drag payload and window listeners |
| `ui/drop-layer.ts` | Overlay rendering from resolver output |
| `ui/tab-strip.ts` | Rail reorder hit testing |

UI modules emit commands. They do not mutate layout directly.

## Measurement Helpers

```typescript
type PaneRects = {
  paneRect: DOMRect;
  bodyRect: DOMRect;
  stripBottom: number;
  bodyOffsetTop: number;
};
```

- `paneRect`: full pane element bounds.
- `bodyRect`: `.pane-stack` bounds.
- `stripBottom`: bottom of the source `.tab-strip` in viewport coordinates.
- `bodyOffsetTop`: body top minus pane top for preview placement.

## Resolver Input

```typescript
type DropResolveInput = {
  clientX: number;
  clientY: number;
  sourcePaneId: string;
  targetPane: HTMLElement;
  rects: PaneRects;
  inSourceStrip: boolean;
};
```

## Resolver Output

```typescript
type DropResolveResult =
  | { kind: "rail"; insertSide: "before" | "after"; targetPaneId: string }
  | { kind: "pane"; zone: "center" | "top" | "bottom" | "left" | "right"; targetPaneId: string }
  | { kind: "none" };
```

## Resolution Order

1. If pointer is over a tab frame in the target rail, return rail insertion.
2. If `inSourceStrip` is true, force center on the source stack.
3. If pointer is in target chrome, return center.
4. If pointer is in target body edge corridor, return that edge zone.
5. Otherwise return center for the target body.

## Shared Drag State

`TabDragState` holds:

- `sourcePaneId`
- active `DropResolveResult`
- drag phase: `idle`, `arming`, `dragging`

`tab-strip.ts` and `drop-layer.ts` subscribe to the same store so previews stay
consistent across pointer and native paths.

## Command Mapping

| Result | WorkspaceCommand |
| --- | --- |
| Rail before/after | `reorderTab` |
| Pane center | `moveTabToStack` |
| Pane edge | `moveTabToEdge` |
| None | no dispatch |

## Preview Rules

- Center preview covers the body rect only.
- Edge preview covers half the body on that edge.
- Rail preview marks the target tab frame side.
- Previews never cover pane chrome.

## Tests

- Chrome drop resolves to center, not edge split.
- Source strip priority suppresses edge split while reordering.
- Edge corridors use body dimensions, not full pane dimensions.
- Native and pointer paths produce identical commands for the same geometry.
