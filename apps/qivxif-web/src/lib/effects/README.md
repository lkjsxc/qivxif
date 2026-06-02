# Effects

## Contents

- [api-client.ts](api-client.ts): HTTP API client.
- [indexed-db.ts](indexed-db.ts): IndexedDB persistence.
- [app-actions.ts](app-actions.ts): action table wired to effects.
- [sync.ts](sync.ts): queue flush and pull.
- [state-loader.ts](state-loader.ts): hydrate workspace from local and remote.
- [tile-actions.ts](tile-actions.ts): tile layout commands.
- [keyboard.ts](keyboard.ts): keyboard shortcuts.

## Boundary

Effects implement `AppPorts` IO. They do not render DOM.
