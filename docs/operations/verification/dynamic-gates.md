# Dynamic Gates

## Required

- Server starts with empty data directory.
- Admin bootstrap works.
- Login and logout work.
- Node create and read work.
- Edge create and read work.
- Text edit works.
- Push and pull sync work.
- App shell loads.
- Offline reload works.
- Dirty local event survives cache planning.

## API Proof Service

`docker compose run --rm api-test` exercises the current durable API proof:

1. Build the SvelteKit browser app (`npm run build` → `dist/`).
2. Bootstrap an admin into an empty redb store.
3. Start the Axum server.
4. Login and capture the session cookie plus CSRF token.
5. Create two text nodes.
6. Apply a text restore event.
7. Create a typed edge between the nodes.
8. Read node edges.
9. Read a bounded graph neighborhood.
10. Pull accepted events through HTTP sync.
11. Inspect node history.
12. Load the app shell route.
13. Logout and confirm the session is rejected.

## Offline Browser Service

`docker compose run --rm offline-e2e` exercises the browser proof slice:

1. Build the SvelteKit browser app (`npm run build` → `dist/`).
2. Bootstrap an admin into an empty redb store.
3. Start the Axum server.
4. Load `/` in a real headless browser.
5. Confirm the service worker becomes ready.
6. Login through the browser UI.
7. Switch the browser context offline.
8. Create a text node and save text.
9. Confirm the SQLite worker repository has dirty events through the typed test
   diagnostics bridge.
10. Reload offline and confirm shell, node, text, and queue survive.
11. Confirm the server does not yet have the local node.
12. Reconnect, flush, and confirm the queue clears only after acceptance.
13. Open a clean browser context, login, open the node, and inspect text plus history.
