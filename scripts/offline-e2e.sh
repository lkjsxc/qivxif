#!/bin/sh
set -u

repo_dir="${QIVXIF_REPO_DIR:-$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)}"
cd "$repo_dir"

base="${QIVXIF_E2E_BASE:-http://127.0.0.1:8080}"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/qivxif-offline.XXXXXX")"

cleanup() {
  if [ "${server_pid:-}" ]; then
    kill "$server_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$work_dir"
}
trap cleanup EXIT HUP INT TERM

run_stage() {
  stage="$1"
  shift
  log_file="$work_dir/$stage.log"
  "$@" >"$log_file" 2>&1
  status="$?"
  if [ "$status" -eq 0 ]; then
    printf 'offline %s ... ok\n' "$stage"
    return 0
  fi
  printf 'offline %s ... failed\n' "$stage"
  printf -- '----- %s output -----\n' "$stage"
  cat "$log_file"
  exit "$status"
}

browser="${QIVXIF_BROWSER:-}"
if [ -z "$browser" ]; then
  if command -v chromium >/dev/null 2>&1; then
    browser="$(command -v chromium)"
  elif command -v google-chrome >/dev/null 2>&1; then
    browser="$(command -v google-chrome)"
  else
    printf 'chromium browser is required\n' >&2
    exit 1
  fi
fi

rm -rf "$QIVXIF_DATA_DIR"
mkdir -p "$QIVXIF_DATA_DIR"
web_dist="${QIVXIF_WEB_DIST_DIR:-${TMPDIR:-/tmp}/qivxif-web-dist}"
run_stage web-build env QIVXIF_WEB_DIST_DIR="$web_dist" sh scripts/build-web.sh
export QIVXIF_STATIC_DIR="$web_dist"

run_stage server-build cargo build --locked -p qivxif-server
"${CARGO_TARGET_DIR:-target}/debug/qivxif-server" >"$work_dir/server.log" 2>&1 &
server_pid="$!"

for _ in $(seq 1 120); do
  if curl -fsS "$base/health" >"$work_dir/health.json" 2>/dev/null; then
    break
  fi
  sleep 1
done

if ! test -s "$work_dir/health.json"; then
  cat "$work_dir/server.log" >&2
  printf 'server did not become healthy\n' >&2
  exit 1
fi

sleep 2

entry="$(ls "$web_dist"/_app/immutable/entry/start.*.js 2>/dev/null | head -1)"
if [ -z "$entry" ]; then
  printf 'missing vite client bundle in %s\n' "$web_dist" >&2
  exit 1
fi
run_stage client-bundle curl -fsS "$base/_app/immutable/entry/$(basename "$entry")"

export NODE_PATH="$(npm root -g)${NODE_PATH:+:$NODE_PATH}"
export QIVXIF_BROWSER="$browser"
export QIVXIF_E2E_BASE="$base"
run_stage setup-flow node tests/offline/setup-flow.mjs
run_stage proof-slice node tests/offline/proof-slice.mjs
run_stage publish-flow node tests/offline/publish-flow.mjs

printf 'offline e2e pass\n'
