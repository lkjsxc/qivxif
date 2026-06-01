#!/bin/sh
set -u

log_dir="$(mktemp -d "${TMPDIR:-/tmp}/qivxif-verify.XXXXXX")"

cleanup() {
  rm -rf "$log_dir"
}

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

run_stage web-build env QIVXIF_WEB_DIST_DIR="$web_dist" npm --prefix apps/qivxif-web run build
run_stage web-syntax find "$web_dist" -name '*.js' -exec node --check {} ';'
run_stage fmt cargo fmt -- --check
run_stage clippy cargo clippy --locked --workspace --all-targets -- -D warnings
run_stage test cargo nextest run --locked --workspace
run_stage doctest cargo test --locked --doc --workspace
run_stage build cargo build --locked --release --workspace
run_stage docs-topology cargo run --locked --bin qivxifctl -- docs validate-topology
run_stage line-limits cargo run --locked --bin qivxifctl -- quality check-lines
run_stage wording cargo run --locked --bin qivxifctl -- quality check-wording
run_stage retired-canon cargo run --locked --bin qivxifctl -- quality check-retired-canon
run_stage implementation-markers cargo run --locked --bin qivxifctl -- quality check-placeholders
run_stage workspace-match cargo run --locked --bin qivxifctl -- quality check-workspace
run_stage route-match cargo run --locked --bin qivxifctl -- quality check-routes

printf 'verify pass\n'
