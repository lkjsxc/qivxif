#!/bin/sh
set -eu

out_dir="dist/windows/client-cli"
exe="$out_dir/qivxif-client-cli.exe"

rm -rf "$out_dir"
mkdir -p "$out_dir"

docker compose --ansi never --progress quiet \
  -f docker-compose.windows.yml \
  run --rm --build -T windows-client-cli

test -s "$exe"
printf 'verify windows client cli ... ok\n'
