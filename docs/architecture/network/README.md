# Network Architecture

Use this subtree for implemented QUIC transport, sessions, and public protocol.

## Current Implementation

- Transport uses Quinn QUIC.
- Server accepts bidirectional streams.
- Each probe request uses one bidirectional stream.
- Public payloads use postcard.
- Public datagram payloads are not implemented.

## Child Index

- [transport.md](transport.md): QUIC transport contract.
- [session-lifecycle.md](session-lifecycle.md): connection phases and guards.
- [message-lanes.md](message-lanes.md): active and dormant lane facts.
- [replication.md](replication.md): current chunk response facts.
- [protocol-epoch.md](protocol-epoch.md): epoch and build gating.
- [protocol-codecs.md](protocol-codecs.md): wire codec ownership.
- [protocol-messages.md](protocol-messages.md): public message catalog.
- [security.md](security.md): implemented transport security facts.
