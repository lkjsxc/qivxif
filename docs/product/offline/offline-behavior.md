# Offline Behavior

## Works Offline

- Create nodes.
- Edit text.
- Draft posts.
- Change layouts.
- Browse cached neighborhoods.
- Inspect cached history.

## Rules

- Local operations are stored before UI marks them queued.
- Dirty local operations are never evicted.
- Server validation happens when connectivity returns.
- Create and edit commands keep working without network after the app shell and IndexedDB open.
- The UI may show local projections from dirty operations, but it labels them dirty until server acceptance.
- Login, logout, publishing, ACL changes, and slug checks require a server response.

## Browser Proof Slice

The first offline proof slice uses route-backed queue entries:

- A text node command records `node.create` locally.
- A text save command records a text operation locally.
- Refresh reloads queued operations from IndexedDB.
- Reconnect flushes queued operations in creation order.
- A successful durable route response clears dirty state for that operation.
- A structured route error keeps the operation visible as rejected.
