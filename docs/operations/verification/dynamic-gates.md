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
- Dirty local operation survives cache planning.

## API Proof Service

`docker compose run --rm api-test` exercises the current durable API proof:

1. Build the browser app shell.
2. Bootstrap an admin into an empty redb store.
3. Start the Axum server.
4. Login and capture the session cookie plus CSRF token.
5. Create two text nodes.
6. Apply a text restore operation.
7. Create a typed edge between the nodes.
8. Read node edges.
9. Pull accepted operations through HTTP sync.
10. Inspect node history.
11. Load the app shell route.
12. Logout and confirm the session is rejected.
