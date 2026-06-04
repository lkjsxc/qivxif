# Browser Source

SvelteKit client for qivxif.

## Layout

- [routes/](routes/): SvelteKit pages and layout.
- [lib/app/](lib/app/): controller and workspace context.
- [lib/domain/](lib/domain/): pure reducers and drop resolver.
- [lib/effects/](lib/effects/): action table, sync, and API adapters.
- [lib/components/](lib/components/): workspace shell and tab surfaces.
- [lib/styles/](lib/styles/): design tokens and CSS modules.

## Target Boundaries

- Svelte components render UI and emit commands.
- Storage belongs behind typed repositories and a SQLite worker.
- WASM services expose Rust kernels through result unions.

## Build

```bash
npm run build
```

Output is written to `dist/`.
