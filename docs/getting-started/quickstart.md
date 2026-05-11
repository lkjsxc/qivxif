# Quickstart

## Assumption

Use repository scripts and Docker Compose as the repeatable path. Host commands
are diagnostics only unless an owner doc explicitly says otherwise.

## Full Acceptance

```bash
./scripts/verify-compose.sh
```

## Static Gate

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --build -T verify
```

## Run Server

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
```

## Run Smoke Probe

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T smoke
```

## Evidence

Save task-specific command output under `.sisyphus/evidence/` when an agent is
asked to prove completion.
