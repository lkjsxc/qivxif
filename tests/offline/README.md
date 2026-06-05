# Offline Browser Tests

Playwright scripts that run inside the `offline-e2e` Docker Compose service.

## Scripts

- [setup-flow.mjs](setup-flow.mjs): first-run owner setup inside the tile shell.
- [proof-slice.mjs](proof-slice.mjs): core shell, tabs, sync, and layout proof.
- [publish-flow.mjs](publish-flow.mjs): publishing queue and public route checks.
- [browser-helpers.mjs](browser-helpers.mjs): shared page helpers and event capture.
- [drag-helpers.mjs](drag-helpers.mjs): tab drag and drop geometry helpers.
- [publish-helpers.mjs](publish-helpers.mjs): publish form and draft helpers.
- [local-snapshot-helpers.mjs](local-snapshot-helpers.mjs): SQLite worker snapshot checks.

## Run

```bash
docker compose run --rm offline-e2e
```

## Related Docs

- [../../docs/product/setup/first-run.md](../../docs/product/setup/first-run.md)
- [../../docs/product/tile-shell/drag-drop.md](../../docs/product/tile-shell/drag-drop.md)
