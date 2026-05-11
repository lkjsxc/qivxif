# Compose Pipeline

## Canonical Commands

```bash
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml down -v
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm --build -T verify
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml up -d --build server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T smoke
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T persist-place
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml restart server
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml run --rm -T persist-check
docker compose --ansi never --progress quiet -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Behavior

- `verify` runs formatting, Clippy, nextest, doctests, optimized build, docs topology, and line limits.
- `smoke` verifies connect, hello, join, chunk request, and ping.
- `persist-place` mutates a block through the public path.
- `persist-check` verifies the mutation after restart.
- Non-zero exit blocks acceptance.
