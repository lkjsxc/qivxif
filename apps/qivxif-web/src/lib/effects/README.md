# Effects

## Purpose

Own browser side effects while the controller migrates to typed `AppPorts`.

## Allowed Imports

- `../domain/` types and reducers.
- `../app/ports.ts` contracts.
- `../storage/` typed repositories.
- Optional service API client modules.

## Forbidden Imports

- Svelte components.
- Raw storage calls from action modules.
- Raw worker messages outside storage client.
- Fake product data.

## Owner Files

- `api-client.ts`: HTTP client for the optional sync service.
- `app-actions.ts`: migration action table to be retired by dispatch.
- `sync.ts`: queue flush and pull behavior.
- `state-loader.ts`: local and remote state hydration.
- `tile-actions.ts`: tile layout commands.
- `keyboard.ts`: keyboard shortcut wiring.

## Verification

Run web build and storage, route, and implementation-marker quality gates after changes.
