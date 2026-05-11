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
compose build verify server
compose up -d --build server
compose run --rm -T smoke
compose run --rm -T protocol-guards
compose run --rm -T malformed-wire
compose run --rm -T request-replay
compose run --rm -T client-cli
compose run --rm -T persist-place
compose restart server
compose run --rm -T persist-check
cleanup

printf 'verify compose ... ok\n'
