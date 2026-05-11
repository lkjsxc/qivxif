# Soak And Load

LLM purpose: reserve long-running operational checks for authoritative tick
pressure.

## Purpose

Soak and load checks protect the authoritative tick path.

## Deferred Scope

The initial server slice keeps this as a documented gate without long-running
automation.

## Future Checks

- Many sessions.
- Chunk churn.
- Mutation bursts.
- Persistence flush pressure.
