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
        app-actions.ts
        sync.ts
        tile-actions.ts
      storage/
        README.md
        sqlite-worker-client.ts
        repositories.ts
      wasm/
        README.md
        module-loader.ts
        result.ts
        workspace-service.ts
      workspace/
        README.md
        tab-drop-hit.ts
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
          NewTab.svelte
          FeedTab.svelte
          EditorTab.svelte
      styles/
        base.css
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
| `components/` | controller callbacks, `domain/` types | `effects/`, raw storage, fetch |
| `domain/` | other `domain/` modules | components, `effects/`, DOM |
| `effects/` | `domain/`, `app/ports.ts`, typed repositories | Svelte components |
| `storage/` | worker client, repository DTOs | Svelte components |
| `wasm/` | generated bindings, result helpers | Svelte components |
| `app/` | `domain/`, `effects/`, bootstrap | business rules outside reducers |

## Bootstrap Flow

1. `+page.svelte` mounts `WorkspaceRoot`.
2. Controller starts storage and bridge ports.
3. Controller loads workspace through repositories.
4. Controller subscribes Svelte state.
5. Service worker registers through ports after first render.

## WorkspaceCommand

UI dispatches commands only. Examples: `focusTab`, `openNewTab`, `convertNewTab`,
`closeTab`, `splitPane`, `moveTabToEdge`, `reorderTab`, `resizeSplit`,
`saveTextDraft`, `flushSyncQueue`.

## Retirement

The direct DOM tree under `src/ui/*.ts` is gone. Do not keep parallel renderers.
