# Browser Source

SvelteKit client for qivxif.

## Layout

- [routes/](routes/): SvelteKit pages and layout.
- [lib/app/](lib/app/): controller and workspace context.
- [lib/domain/](lib/domain/): pure reducers and drop resolver.
- [lib/effects/](lib/effects/): IndexedDB, sync, and API adapters.
- [lib/components/](lib/components/): workspace shell and tab surfaces.
- [lib/styles/](lib/styles/): design tokens and CSS modules.

## Build

```bash
npm run build
```

Output is written to `dist/`.
