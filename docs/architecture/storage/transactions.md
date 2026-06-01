# Transactions

## Write Pattern

1. Validate request outside the write transaction.
2. Open write transaction.
3. Read required current records.
4. Validate causal and auth conditions.
5. Check event idempotency by `EventId` and `(ActorId, actor_seq)`.
6. Apply pure reducers to in-memory state.
7. Insert event envelope.
8. Update primary records.
9. Update secondary indexes.
10. Write commit group changes when present.
11. Update sync markers.
12. Commit.
13. Emit post-commit notifications.

## Boundaries

- `user.bootstrap_owner` writes user, profile node, session-independent auth state, and event records atomically.
- `node.create` writes event and node in one transaction.
- `edge.create` writes event, edge, endpoint indexes, and target indexes in one transaction.
- `text.insert` and `text.delete` write event and text projection in one transaction.
- Feed index updates happen in the same transaction as the event that creates feed visibility.
- Sync pull is read-only and must pass auth context into the store before event envelopes leave storage.

## Failure Rule

If any primary record, index record, or commit group write fails, the whole transaction aborts and no post-commit notification is emitted.

## Prohibited

- Network IO inside write transactions.
- Large hashing inside write transactions.
- Markdown rendering inside write transactions.
- WebTransport fanout inside write transactions.
