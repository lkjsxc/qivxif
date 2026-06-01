# IndexedDB

## Stores

- nodes
- edges
- ops
- text snapshots
- sync cursors
- cache entries
- cache journal
- feed windows

## Rule

IndexedDB stores structured qivxif data. The Cache API stores app shell and safe HTTP responses.

## Queue Store

The `ops` store is keyed by operation id. Queue records include:

- documented operation kind
- dirty or rejected state
- target node id when present
- route method and path
- route request body
- client display timestamp
- last structured rejection when present

The sync actor is the only browser component that sends queued records to the network. UI components request queue writes through actor messages.
