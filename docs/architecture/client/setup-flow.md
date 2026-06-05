# Setup Flow

## State

The app shell owns setup state:

- `setupChecked`
- `setupRequired`
- `setupError`
- active tab id

## Startup

1. Render header plus tile frame.
2. Call `GET /api/setup`.
3. If setup is required, activate Setup and skip login-only actions.
4. Fetch server info when reachable.
5. Open the SQLite worker repository boundary.
6. Register the service worker.
7. Start sync when setup or login state allows it.

## Setup Submit

1. The Setup tab sends name and password to `POST /api/setup/owner`.
2. The response is stored through the same auth helper used by login.
3. The actor sequence cursor is persisted before queued mutations start.
4. `setupRequired` becomes false.
5. The active tab becomes Welcome or Graph.

## Render Rules

- Setup is a tab inside the tile shell.
- Login is not the primary surface while setup is required.
- Header status says setup required until owner creation succeeds.
- No sample data or local-only owner state is created.
