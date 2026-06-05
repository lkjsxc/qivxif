# Offline Web

## Facts

- Service workers intercept requests and can serve cached app shell assets.
- Cache API stores app shell and safe HTTP responses.
- OPFS can host browser-owned byte storage.
- SQLite WASM can use OPFS for durable structured data in a worker.

## Implication

qivxif stores graph records, dirty events, accepted events, text snapshots,
cursors, cache metadata, media metadata, and resource planner rows in browser
SQLite. OPFS also stores media chunks and previews outside raw SQLite rows.
