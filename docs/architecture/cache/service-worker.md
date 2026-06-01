# Service Worker

## Responsibilities

- Cache app shell assets.
- Serve app shell offline.
- Serve cached public pages when available.
- Provide offline fallback route.
- Report cache status through messages.

## Constraint

The service worker does not own graph sync at first.

## Scope

- Service worker URL is `/service-worker.js`.
- Scope is `/`.
- Offline navigation fallback is `/index.html`.
- API requests under `/api/` bypass app-shell cache.
- Public page responses may be cached only when marked safe by response headers.

## Cache Names

- `qivxif-app-shell`: index, modules, CSS, manifest, icons.
- `qivxif-public-pages`: safe public HTML and public render assets.
- `qivxif-diagnostics`: cache status messages and last cache plan summary.

## Messages

- `cache.installed`
- `cache.updated`
- `cache.offline_fallback_used`
- `cache.usage_report`

## Rules

- Activation claims existing app-shell clients after install.
- Failed API requests surface as network failures.
- The service worker never fabricates graph, auth, sync, or publish success.
