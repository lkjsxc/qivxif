# WebTransport

## Handshake

1. Client opens WebTransport URL.
2. Client sends session proof on control stream.
3. Server validates session against redb.
4. Server sends accepted capabilities and cursor summary.
5. Client sends local cursor summary.
6. Peers exchange missing operation batches.

## Rule

WebTransport improves live sync but does not replace HTTP fallback.
