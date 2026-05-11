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

run_stage fmt cargo fmt -- --check
run_stage clippy cargo clippy --workspace --all-targets -- -D warnings
run_stage test cargo test --workspace
run_stage doctest cargo test --doc --workspace
run_stage build cargo build --release --workspace
run_stage docs-topology cargo run --bin qivxifctl -- docs validate-topology
run_stage line-limits cargo run --bin qivxifctl -- quality check-lines

printf 'verify pass\n'
