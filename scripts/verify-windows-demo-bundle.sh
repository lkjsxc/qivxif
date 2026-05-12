#!/bin/sh
set -eu

root_dir="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
bundle_dir="$root_dir/dist/windows/demo"
zip_file="$root_dir/dist/windows/qivxif-demo-windows-x86_64.zip"

expected_files="
qivxif-serverd.exe
qivxif-client-cli.exe
config/server.toml
data/.keep
start-server.cmd
run-client-demo.cmd
README.md
manifest.json
checksums.txt
"

clean_windows_dist() {
  if rm -rf "$root_dir/dist/windows" 2>/dev/null; then
    return
  fi
  docker compose --ansi never --progress quiet \
    -f docker-compose.windows.yml \
    run --rm --build -T --user 0:0 --entrypoint sh windows-demo-bundle \
    -c 'rm -rf /dist/windows/* /dist/windows/.[!.]* /dist/windows/..?*'
  rm -rf "$root_dir/dist/windows"
}

clean_windows_dist
mkdir -p "$root_dir/dist/windows"

cd "$root_dir"
docker compose --ansi never --progress quiet \
  -f docker-compose.windows.yml \
  config --quiet

QIVXIF_BUNDLE_UID="$(id -u)" \
QIVXIF_BUNDLE_GID="$(id -g)" \
  docker compose --ansi never --progress quiet \
  -f docker-compose.windows.yml \
  run --rm --build -T windows-demo-bundle

test -s "$bundle_dir/qivxif-serverd.exe"
test -s "$bundle_dir/qivxif-client-cli.exe"

for file in $expected_files; do
  test -e "$bundle_dir/$file"
done

for file in \
  qivxif-serverd.exe \
  qivxif-client-cli.exe \
  config/server.toml \
  start-server.cmd \
  run-client-demo.cmd
do
  grep -F "  $file" "$bundle_dir/checksums.txt" >/dev/null
done

(cd "$bundle_dir" && sha256sum -c checksums.txt >/dev/null)

test -s "$zip_file"
for file in $expected_files; do
  zipinfo -1 "$zip_file" | grep -Fx "demo/$file" >/dev/null
done

printf 'verify windows demo bundle ... ok\n'
