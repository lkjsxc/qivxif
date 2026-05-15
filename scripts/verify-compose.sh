#!/bin/sh
set -eu

compose() {
  docker compose --ansi never --progress quiet \
    -f docker-compose.yml \
    -f docker-compose.verify.yml "$@"
}

cleanup() {
  compose down -v --remove-orphans >/dev/null 2>&1 || true
}

trap cleanup EXIT HUP INT TERM

cleanup
compose config --quiet
compose run --rm --build -T verify
compose run --rm --build -T superapp-smoke
cleanup

printf 'verify compose ... ok\n'
