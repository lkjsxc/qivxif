# Quickstart

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
