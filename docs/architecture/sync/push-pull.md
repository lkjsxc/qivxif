# Push Pull

## HTTP Lane

- `POST /api/sync/push` uploads durable operations.
- `GET /api/sync/pull` returns operations after a cursor.
- Batch limits protect server memory.
- Rejections are structured.

## Live Lane

WebTransport reliable streams carry the same durable message types after authentication.
