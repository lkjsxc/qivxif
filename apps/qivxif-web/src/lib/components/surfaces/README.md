# Tab Surfaces

## Purpose

Render product tab bodies inside pane stacks.

## Allowed Imports

- Plain view state and dispatch callbacks from parent components.
- Shared styles and pure domain view helpers.

## Forbidden Imports

- `../../effects/`, `../../storage/`, raw network calls, SQL, IndexedDB, OPFS, or
  direct workers.
- Invented product content or hidden sample data.
- Accepted event construction inside components.

## Owner Files

- `TabSurface.svelte`: routes tab kind to surface component.
- `SetupTab.svelte`, `LoginTab.svelte`, `WelcomeTab.svelte`: entry surfaces.
- `NewTab.svelte`: chooser tab that converts in place.
- `EditorTab.svelte`, `GraphTab.svelte`, `GraphMapTab.svelte`: knowledge surfaces.
- `MediaTab.svelte`: media import and inspection surface.
- `FeedTab.svelte`, `PublishTab.svelte`: social and publishing surfaces.
- `SyncTab.svelte`, `SettingsTab.svelte`, `DiagnosticsTab.svelte`: status surfaces.

## Verification

Run web build, implementation-marker gate, and offline checks after product surface edits.
