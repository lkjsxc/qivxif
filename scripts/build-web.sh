#!/bin/sh
set -eu

repo_dir="${QIVXIF_REPO_DIR:-$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)}"
cd "$repo_dir"

web_dist="${QIVXIF_WEB_DIST_DIR:-${TMPDIR:-/tmp}/qivxif-web-dist}"
web_build="${TMPDIR:-/tmp}/qivxif-web-build"

rm -rf "$web_build"
cp -a apps/qivxif-web "$web_build"
(
  cd "$web_build"
  npm ci
  env VITE_CACHE_DIR="${TMPDIR:-/tmp}/qivxif-vite-cache" npm run build
  rm -rf "$web_dist"
  mkdir -p "$web_dist"
  cp -a dist/. "$web_dist/"
)
