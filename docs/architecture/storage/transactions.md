# Transactions

## Write Pattern

1. Validate request outside the write transaction.
2. Open write transaction.
3. Read required current records.
4. Validate causal and auth conditions.
5. Insert operation envelope.
6. Update primary records.
7. Update indexes.
8. Update sync markers.
9. Commit.
10. Emit post-commit notifications.

## Prohibited

- Network IO inside write transactions.
- Large hashing inside write transactions.
- Markdown rendering inside write transactions.
- WebTransport fanout inside write transactions.
