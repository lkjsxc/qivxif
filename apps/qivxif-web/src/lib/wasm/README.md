# WASM Services

## Purpose

Expose Rust/WASM kernels through TypeScript service modules.

## Allowed Imports

- Generated bindings in service modules only.
- Domain DTOs and result helpers.

## Forbidden Imports

- Svelte components importing generated bindings.
- Leptos UI code.
- Fake success when a module load fails.

## Owner Files

- `module-loader.ts`: lazy module loading and test reset.
- `result.ts`: discriminated result helpers.
- `workspace-service.ts`: tile and workspace kernels.
- `storage-codec-service.ts`: durable row codecs.
- `sync-planning-service.ts`: queue ordering and retry planning.
- `feed-geometry-service.ts`: bounded feed geometry kernels.

## Verification

Run Rust tests for kernels and web build for service imports.
