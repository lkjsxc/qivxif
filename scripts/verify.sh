#!/bin/sh
set -u

repo_dir="${QIVXIF_REPO_DIR:-/workspace}"
if [ ! -d "$repo_dir" ]; then
  repo_dir="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
fi
cd "$repo_dir"

log_dir="$(mktemp -d "${TMPDIR:-/tmp}/qivxif-verify.XXXXXX")"
cleanup() { rm -rf "$log_dir"; }
trap cleanup EXIT HUP INT TERM

run_stage() {
  stage="$1"
  shift
  log_file="$log_dir/$stage.log"
  "$@" >"$log_file" 2>&1
  status="$?"
  if [ "$status" -eq 0 ]; then
    printf 'verify %s ... ok\n' "$stage"
    return 0
  fi
  printf 'verify %s ... failed\n' "$stage"
  printf -- '----- %s output -----\n' "$stage"
  cat "$log_file"
  exit "$status"
}

web_dist="${QIVXIF_WEB_DIST_DIR:-${TMPDIR:-/tmp}/qivxif-web-dist}"
run_stage web-build env QIVXIF_WEB_DIST_DIR="$web_dist" sh scripts/build-web.sh
run_stage fmt cargo fmt --check
run_stage clippy cargo clippy --locked --workspace --all-targets -- -D warnings
run_stage test cargo test --locked --workspace
run_stage docs-topology cargo run --locked -p qivxifctl -- docs validate-topology
run_stage line-limits cargo run --locked -p qivxifctl -- quality check-lines
run_stage wording cargo run --locked -p qivxifctl -- quality check-wording
run_stage retired-canon cargo run --locked -p qivxifctl -- quality check-retired-canon
run_stage public-names cargo run --locked -p qivxifctl -- quality check-public-names
run_stage implementation-markers cargo run --locked -p qivxifctl -- quality check-placeholders
run_stage workspace-match cargo run --locked -p qivxifctl -- quality check-workspace
run_stage browser-storage cargo run --locked -p qivxifctl -- quality check-browser-storage
run_stage route-match cargo run --locked -p qivxifctl -- quality check-routes
run_stage redb-table-match cargo run --locked -p qivxifctl -- quality check-redb-tables

printf 'verify pass\n'
