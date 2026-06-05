# App Boundary

## Purpose

Create the browser controller and connect Svelte dispatch to ports.

## Allowed Imports

- `../domain/` command, state, reducer, and effect-plan modules.
- `../effects/` only through browser port construction while migration is open.
- `../storage/` repository factories through `browser-ports.ts`.
- `../wasm/` service facades through ports.

## Forbidden Imports

- Svelte components.
- Raw SQL, OPFS, IndexedDB, or direct worker messages.
- Direct `fetch` in `controller.ts`.

## Owner Files

- `controller.ts`: public controller surface.
- `ports.ts`: port contracts.
- `browser-ports.ts`: concrete browser port wiring.
- `effect-runner.ts`: typed effect plan execution.
- `workspace-context.ts`: migration adapter for component dispatch.

## Verification

Run web build and `qivxifctl quality check-lines` after changes.
