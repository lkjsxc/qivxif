#!/bin/sh
set -eu

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
QIVXIF_WEB_DIST_DIR="$web_dist" npm --prefix apps/qivxif-web run build
export QIVXIF_STATIC_DIR="$web_dist"

cargo run --locked -p qivxif-server >"$work_dir/server.log" 2>&1 &
server_pid="$!"

for _ in $(seq 1 120); do
  if curl -fsS "$base/health" >"$work_dir/health.json"; then
    break
  fi
  sleep 1
done

if ! test -s "$work_dir/health.json"; then
  cat "$work_dir/server.log" >&2
  printf 'server did not become healthy\n' >&2
  exit 1
fi

export NODE_PATH="$(npm root -g)${NODE_PATH:+:$NODE_PATH}"
export QIVXIF_BROWSER="$browser"
export QIVXIF_E2E_BASE="$base"
node tests/offline/setup-flow.mjs
node tests/offline/proof-slice.mjs
node tests/offline/publish-flow.mjs

printf 'offline e2e pass\n'
