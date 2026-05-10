# Message Lanes

## Reliable Request Lane

Use for durable requests that require clear success or failure.

## Reliable Bulk Lane

Use for chunk bundles and larger payloads.

## Latest-Wins Lane

Use datagrams for realtime intent and ephemeral state after the initial slice.

## Repair

Reliable keyframes repair lost latest-wins state.
