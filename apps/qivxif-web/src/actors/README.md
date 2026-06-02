# Browser Actors

Actors own browser orchestration between UI actions, local stores, HTTP routes,
and shell state.

## Contents

- `app-shell.ts`: startup sequence and first render.
- `app-actions.ts`: action table passed into render modules.
- `tile-actions.ts` and `tile-move-actions.ts`: layout commands and local queueing.
- `local-events.ts`: route-specific dirty event records.
- `sync.ts`: dirty queue flush and accepted-result handling.
- Other files own auth, board, publish, social, text, scroll, and keyboard actions.

## Rules

- Actors may call HTTP and IndexedDB adapters.
- Actors must route visible changes back through state plus render.
- Actors do not mutate DOM elements directly except keyboard listener setup.
- Durable mutations become dirty event records before network delivery.
