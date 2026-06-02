# Module Layout

## Browser Source Tree

```text
apps/qivxif-web/
  src/
    app.html
    routes/
      +layout.svelte
      +page.svelte
    lib/
      app/
        README.md
        bootstrap.ts
        controller.ts
        ports.ts
      domain/
        README.md
        workspace-state.ts
        workspace-command.ts
        tile-tree.ts
        tile-move.ts
        drop-resolver.ts
      effects/
        README.md
        api-client.ts
        indexed-db.ts
        sync.ts
        tile-actions.ts
      workspace/
        README.md
        pane-drop-resolve.ts
        tab-drop-hit.ts
        tab-drop-preview.ts
        pointer-tab-drag.ts
        move-tab.ts
      components/
        workspace/
          WorkspaceRoot.svelte
          SplitNode.svelte
          Pane.svelte
          PaneHead.svelte
          TabStrip.svelte
          TabFrame.svelte
          PaneTabStack.svelte
          PaneDropLayer.svelte
          ResizeHandle.svelte
          TileMenu.svelte
        surfaces/
          SetupTab.svelte
          WelcomeTab.svelte
          FeedTab.svelte
          EditorTab.svelte
        ui/
          Button.svelte
          Field.svelte
          Card.svelte
      styles/
        README.md
        tokens.css
        shell.css
        panes.css
        tabs.css
        surfaces.css
  service-worker/
  static/
  svelte.config.js
  vite.config.ts
```

Each directory has one `README.md`. Each source file stays at 200 lines or fewer.

## Layer Rules

| Layer | May import | Must not import |
| --- | --- | --- |
| `components/` | controller callbacks, `domain/` types | `effects/`, IndexedDB |
| `domain/` | other `domain/` modules | components, `effects/`, DOM |
| `effects/` | `domain/`, `app/ports.ts` | Svelte components |
| `app/` | `domain/`, `effects/`, bootstrap | business rules outside reducers |

## Bootstrap Flow

1. `+page.svelte` mounts `WorkspaceRoot`.
2. `bootstrap.ts` builds ports and controller.
3. Controller loads workspace through ports and subscribes Svelte state.
4. Service worker registers through ports after first render.

## WorkspaceCommand

UI dispatches commands only. Examples: `focusTab`, `openTab`, `closeTab`,
`splitPane`, `moveTabToEdge`, `reorderTab`, `resizeSplit`, `saveTextDraft`,
`flushSyncQueue`.

## Retirement

The retired DOM tree under `src/ui/*.ts` is removed after Svelte parity and E2E
gates pass. Do not keep parallel renderers.
