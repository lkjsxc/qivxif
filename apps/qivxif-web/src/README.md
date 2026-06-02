# Browser Source

The browser source is split so the visible surface can be replaced without
rewriting durable behavior.

## Contents

- [main.ts](main.ts): starts the app shell.
- [ids.ts](ids.ts): browser-side random ID generation for local drafts.
- [actors/](actors/): app orchestration and action handlers.
- [domain/](domain/): pure local reducers and tile-tree operations.
- [http/](http/): API client boundary.
- [store/](store/): IndexedDB adapter.
- [ui/](ui/): DOM rendering and gesture modules.

## Rules

- UI modules do not call HTTP or IndexedDB directly.
- Actors call effects and request re-rendering.
- Domain modules stay deterministic and side-effect free.
- IDs use 32 random bytes rendered as lowercase hex.
