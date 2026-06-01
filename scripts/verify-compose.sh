#!/bin/sh
set -eu

if command -v docker-compose >/dev/null 2>&1; then
  compose="docker-compose"
else
  compose="docker compose"
fi

run() {
  printf 'compose %s ...\n' "$*"
  $compose "$@"
}

run -f docker-compose.verify.yml run --rm --build -T verify
run run --rm --build -T server-smoke
run run --rm --build -T api-test
run run --rm --build -T offline-e2e
