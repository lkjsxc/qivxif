# Effects

## Contents

- [api-client.ts](api-client.ts): HTTP API client for optional sync service.
- [app-actions.ts](app-actions.ts): action table wired to effects.
- [sync.ts](sync.ts): queue flush and pull.
- [state-loader.ts](state-loader.ts): hydrate workspace from local and remote data.
- [tile-actions.ts](tile-actions.ts): tile layout commands.
- [keyboard.ts](keyboard.ts): keyboard shortcuts.

## Boundary

Effects implement `AppPorts` IO. They do not render DOM. Browser storage access
must move behind typed repositories and the SQLite worker boundary.
