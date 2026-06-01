# IndexedDB

## Stores

- nodes
- edges
- dirty_events
- accepted_events
- text snapshots
- sync cursors
- cache entries
- cache journal
- feed windows

## Rule

IndexedDB stores structured qivxif data. The Cache API stores app shell and safe HTTP responses.

## Queue Store

The `dirty_events` store is keyed by event id. Queue records include:

- documented event kind
- dirty or rejected state
- target node id when present
- route method and path
- route request body
- client display timestamp
- last structured rejection when present

The sync actor is the only browser component that sends queued records to the network. UI components request queue writes through actor messages.
