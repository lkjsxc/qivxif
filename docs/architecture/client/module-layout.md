# Module Layout

## Browser Source Tree

```text
apps/qivxif-web/src/
  README.md
  main.ts
  app/
    README.md
    bootstrap.ts
    controller.ts
    ports.ts
  domain/
    README.md
    workspace-state.ts
    workspace-command.ts
    workspace-reducer.ts
    tile-tree.ts
    tile-move.ts
    drop-resolver.ts
    tab-drag-state.ts
    tab-state.ts
  effects/
    README.md
    api-client.ts
    indexed-db.ts
    sync-actor.ts
    pointer-drag.ts
    native-drag.ts
    keyboard.ts
    service-worker-client.ts
  ui/
    README.md
    app-shell.ts
    app-header.ts
    split-node.ts
    pane.ts
    pane-head.ts
    tab-strip.ts
    tab-frame.ts
    tab-stack.ts
    drop-layer.ts
    resize-handle.ts
    command-palette.ts
    setup-tab.ts
    welcome-tab.ts
    graph-tab.ts
    text-tab.ts
    board-tab.ts
    sync-tab.ts
    settings-tab.ts
    diagnostics-tab.ts
    dom.ts
  styles/
    README.md
    reset.css
    tokens.css
    shell.css
    panes.css
    tabs.css
    content.css
    forms.css
```

Each directory has one `README.md`. Each source file stays at 200 lines or fewer.

## Layer Rules

| Layer | May import | Must not import |
| --- | --- | --- |
| `ui/` | `domain/` types only through controller callbacks | `effects/`, HTTP, IndexedDB |
| `domain/` | other `domain/` modules | `ui/`, `effects/`, DOM APIs |
| `effects/` | `domain/`, `app/ports.ts` | `ui/` |
| `app/` | `domain/`, `effects/`, `ui/` bootstrap | business rules outside reducers |

## Bootstrap Flow

1. `main.ts` mounts the root element.
2. `bootstrap.ts` builds ports and controller.
3. Controller loads local workspace and setup status through ports.
4. Controller subscribes UI render to state changes.
5. Service worker registration runs through ports after first render.

## WorkspaceCommand Examples

- `focusTab`
- `openTab`
- `closeTab`
- `splitPane`
- `stackTab`
- `moveTabToStack`
- `moveTabToEdge`
- `reorderTab`
- `resizeSplit`
- `maximizePane`
- `restorePane`
- `createOwner`
- `saveTextDraft`
- `flushSyncQueue`

Commands carry pane IDs and pane context. UI never embeds event envelope details.

## Retirement

The legacy `src/actors/` tree is removed after controller migration and E2E gates
pass. Do not keep parallel action tables.
