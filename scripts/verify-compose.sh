#!/bin/sh
set -eu

compose() {
  docker compose --ansi never --progress quiet \
    -f docker-compose.yml \
    -f docker-compose.verify.yml "$@"
}

cleanup() {
  compose down -v >/dev/null 2>&1 || true
}

trap cleanup EXIT HUP INT TERM

compose down -v
compose run --rm --build -T verify
compose up -d --build server
compose run --rm -T smoke
compose run --rm -T request-replay
compose run --rm -T persist-place
compose restart server
compose run --rm -T persist-check
compose down -v

printf 'verify compose ... ok\n'
