# Compose Pipeline

LLM purpose: define the canonical full acceptance command sequence and expected
evidence shape.

## Owner Scope

This file owns the Compose acceptance order. Individual probe contracts are in
[protocol-probes.md](protocol-probes.md). Static gate details are in
[static-gates.md](static-gates.md).

## Canonical Commands

Run the full acceptance pipeline through the repository wrapper:

```bash
./scripts/verify-compose.sh
```

The wrapper executes this Compose sequence:

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml down -v
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml config --quiet
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --build -T verify
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T smoke
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T protocol-guards
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T malformed-wire
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T request-replay
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T client-cli
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T persist-place
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml restart server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T persist-check
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml down -v --remove-orphans
```

## Required Behavior

| Step | Required behavior |
| --- | --- |
| `config` | Validates the merged Compose configuration before building. |
| `verify` | Runs formatting, Clippy, nextest, doctests, optimized build, docs topology, and line limits. |
| `smoke` | Verifies connect, hello, join, chunk request, and ping. |
| `protocol-guards` | Verifies session-phase and hello rejection codes through public QUIC requests. |
| `malformed-wire` | Verifies decode failures return `BadRequest` and do not advance session phase. |
| `request-replay` | Verifies duplicate mutating identifiers return the first response without applying or flushing again. |
| `client-cli` | Verifies the headless protocol client through the public server path. |
| `persist-place` | Mutates a block through the public path. |
| `persist-check` | Verifies the mutation after restart. |

Non-zero exit blocks acceptance.

## Evidence

- Agents save task verification logs under `.sisyphus/evidence/`.
- Evidence files use task-scoped names such as `task-4-compose-docs.txt`.
- A passing full run ends with `verify compose ... ok`.

## Safety Notes

- The pipeline uses `down -v --remove-orphans` and deletes Compose project volumes.
- Do not use Docker Compose dry-run with `run` in this repository.
- `tmp/` research files are not part of acceptance.
