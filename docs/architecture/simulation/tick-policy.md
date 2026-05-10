# Tick Policy

## Rules

- Authoritative simulation uses fixed steps.
- Tick work never blocks on file I/O.
- Tick work never waits on network backpressure.
- Expensive jobs leave the tick path through bounded queues.

## Failure

If queues are full, the server must shed, delay, or reject work explicitly.
