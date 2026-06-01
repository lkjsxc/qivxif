#!/bin/sh
set -eu

repo_dir="${QIVXIF_REPO_DIR:-/workspace}"
if [ ! -d "$repo_dir" ]; then
  repo_dir="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
fi
cd "$repo_dir"

npm --prefix apps/qivxif-web run build
cargo fmt --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run --locked -p qivxifctl -- docs validate-topology
cargo run --locked -p qivxifctl -- quality check-lines
cargo run --locked -p qivxifctl -- quality check-wording
cargo run --locked -p qivxifctl -- quality check-retired-canon
cargo run --locked -p qivxifctl -- quality check-public-names
cargo run --locked -p qivxifctl -- quality check-placeholders
cargo run --locked -p qivxifctl -- quality check-workspace
cargo run --locked -p qivxifctl -- quality check-routes
cargo run --locked -p qivxifctl -- quality check-redb-tables
