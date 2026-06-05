# Service Worker

This directory owns the app-shell cache worker source.

## Contents

- [service-worker.ts](service-worker.ts): install, activate, navigation fallback,
  and static asset cache handling.

## Rules

- API requests always go to the network path.
- Navigation falls back to cached `index.html` when offline.
- Static assets are cached from `asset-manifest.json` plus the core shell files.
- Durable data remains in browser SQLite, OPFS media chunks, and server events, not Cache API entries.
