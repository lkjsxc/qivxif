# WASM Boundary

## Ownership

Svelte owns rendered product UI. Rust/WASM owns deterministic browser kernels:

- tile and workspace reducers,
- graph and event reducers,
- protocol and DTO codecs,
- SQLite row codecs,
- relay or sync planning reducers,
- bounded feed geometry and cache calculations.

Rust/WASM does not own a product UI shell. No Leptos component tree receives new
product work.

## Service Modules

Generated bindings stay behind TypeScript service modules. Svelte components
receive plain view models and callbacks.

Target modules:

- `src/lib/wasm/module-loader.ts`
- `src/lib/wasm/result.ts`
- `src/lib/wasm/workspace-service.ts`
- `src/lib/wasm/storage-codec-service.ts`
- `src/lib/wasm/feed-geometry-service.ts`
- `src/lib/wasm/sync-planning-service.ts`

Only service modules import generated WASM bindings.

## Result Shape

WASM services return discriminated results:

```typescript
type WasmResult<T> =
  | { ok: true; value: T }
  | { ok: false; reason: string; detail?: string };
```

Raw JS exceptions are converted at the service boundary. Product surfaces show
honest unavailable or degraded states when the bridge cannot load.

## Loading

- Load the module lazily on first service call.
- Cache the module handle in one documented loader.
- Expose reset only for tests.
- Keep the Svelte shell usable when WASM load fails.
- Delete duplicated TypeScript reducer logic after WASM parity is proven by
  focused tests.

## Forbidden Paths

- Svelte component imports from generated WASM.
- Leptos product UI crates or mount functions.
- Compatibility aliases for retired UI paths.
- Fake bridge success when the module fails to load.
