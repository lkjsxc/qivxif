# HTTP Boundary

This directory owns browser calls to the Axum API.

## Contents

- [client.ts](client.ts): setup, auth, graph, text, publish, social, sync, and
  queued-route helpers.

## Rules

- Render modules must not import this directory.
- Actors pass CSRF tokens and queued event records into this boundary.
- API errors keep structured codes when the server returns an envelope.
- Route-specific flushes must create the same durable events as batch sync.
