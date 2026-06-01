#!/bin/sh
set -eu

cd /workspace
web_dist="${QIVXIF_WEB_DIST_DIR:-${TMPDIR:-/tmp}/qivxif-web-dist}"
QIVXIF_WEB_DIST_DIR="$web_dist" npm --prefix apps/qivxif-web run build
export QIVXIF_STATIC_DIR="$web_dist"
cargo run --locked -p qivxifctl -- store health --store "$QIVXIF_DATABASE_FILE"
cargo run --locked -p qivxif-server &
server_pid="$!"

cleanup() {
  kill "$server_pid" >/dev/null 2>&1 || true
}

trap cleanup EXIT HUP INT TERM

for _ in 1 2 3 4 5 6 7 8 9 10; do
  if curl -fsS "http://127.0.0.1:8080/health" >/tmp/qivxif-health.json; then
    cat /tmp/qivxif-health.json
    exit 0
  fi
  sleep 1
done

printf 'server did not become healthy\n' >&2
exit 1
