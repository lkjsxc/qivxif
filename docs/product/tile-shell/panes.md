# Panes

## Structure

```text
.pane
  .pane-head
    TileMenu
    .tab-strip [role=tablist]
      .tab-frame [role=tab] [data-pane-id]
    NewTabButton
  PaneDropLayer (active while dragging-tab)
  PaneTabStack
    .tab-body (mounted; inactive hidden)
```

## Pane Kinds

Product tabs map to pane kinds: setup, welcome, graph, text editor, board, feed,
publish, history, sync, settings, diagnostics.

## Rules

- Each pane has a stable pane ID on `data-pane-id`.
- Pane chrome height 36–40px; no separate title bar above the tab rail.
- Components emit `WorkspaceCommand` only.
- Inactive tab bodies stay mounted with `hidden` and `aria-hidden="true"`.
- Last tab leaving a stack collapses the empty pane in the layout tree.

## Selectors

- `role="tablist"` on the strip; `role="tab"` on frames.
- `data-pane-id` and `data-tab-kind` on tab frames for tests.
- `.pane` and `.tile` class names are equivalent for E2E (`article.tile`).
