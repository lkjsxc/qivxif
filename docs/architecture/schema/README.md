# Schema Contracts

This directory owns durable schema names used by storage, API DTOs, reducers, sync, and browser persistence.

## Contents

- [ids.md](ids.md): typed identifier forms.
- [node-kinds.md](node-kinds.md): accepted node kind names.
- [edge-kinds.md](edge-kinds.md): accepted edge kind names.
- [operation-kinds.md](operation-kinds.md): accepted durable operation names.
- [api-envelope.md](api-envelope.md): shared API response shape.
- [error-codes.md](error-codes.md): shared API and sync error codes.

## Rules

- Durable names are introduced here before implementation.
- Unknown node, edge, and operation kinds are rejected.
- Public JSON names match these files exactly.
- redb tables store typed bytes owned by Rust structs, not loose public JSON.
