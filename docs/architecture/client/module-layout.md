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
        browser-ports.ts
        controller.ts
        effect-runner.ts
        ports.ts
        workspace-context.ts
      domain/
        README.md
        effect-plan.ts
        workspace-state.ts
        workspace-command.ts
        workspace-reducer.ts
        tile-tree.ts
        tile-move.ts
        drop-resolver.ts
      effects/
        README.md
        api-client.ts
        sync.ts
        tile-actions.ts
      storage/
        README.md
        current-store.ts
        diagnostics.ts
        repositories.ts
        sqlite-schema.ts
        sqlite-statements.ts
        sqlite-worker-client.ts
        sqlite.worker.ts
        types.ts
        worker-protocol.ts
        worker-runtime.ts
      wasm/
        README.md
        module-loader.ts
        result.ts
        workspace-service.ts
        storage-codec-service.ts
        sync-planning-service.ts
        feed-geometry-service.ts
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
          GraphMapTab.svelte
          MediaTab.svelte
          ProfileTab.svelte
          AdminTab.svelte
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
| `components/` | controller dispatch, `domain/` types | `effects/`, raw storage, fetch, workers |
| `domain/` | other `domain/` modules | components, `effects/`, DOM, storage |
| `effects/` | `domain/`, `app/ports.ts`, typed repositories | Svelte components |
| `storage/` | worker client, repository DTOs | Svelte components, raw UI state |
| `wasm/` | generated bindings, result helpers | Svelte components |
| `app/` | `domain/`, effects through `AppPorts`, bootstrap | business rules outside reducers |

## Bootstrap Flow

1. `+page.svelte` mounts `WorkspaceRoot`.
2. Controller starts storage and bridge ports.
3. Controller loads workspace through repositories.
4. Controller subscribes Svelte state.
5. Service worker registers through ports after first render.

## WorkspaceCommand

UI dispatches commands only. Examples: `bootstrap`, `focusPane`, `openNewTab`,
`convertNewTab`, `closePane`, `splitPane`, `stackTab`, `maximizePane`,
`restorePane`, `movePane`, `reorderTab`, `resizeSplit`, `createTextNode`,
`createGraphMap`, `saveTextDraft`, `saveText`, `createMediaUpload`,
`updateProfile`, `issueInvite`, `issueApiKey`, `flushSyncQueue`, and
`refreshDiagnostics`.

## Retirement

The direct DOM tree under `src/ui/*.ts` is gone. Do not keep parallel renderers.
