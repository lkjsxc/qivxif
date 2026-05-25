#!/bin/sh
set -eu

root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
artifact_root="$root/dist/windows"
bundle="${1:-$artifact_root/qivxif-superapp-windows-x86_64}"
zip_path="${2:-$artifact_root/qivxif-superapp-windows-x86_64.zip}"

fail() {
  printf 'verify package-windows ... failed: %s\n' "$1" >&2
  exit 1
}

[ -d "$bundle" ] || fail "missing bundle directory"
[ -f "$bundle/qivxif-superapp.exe" ] || fail "missing executable"
[ -f "$bundle/run.bat" ] || fail "missing launcher"
[ -f "$bundle/LICENSE" ] || fail "missing license"
[ -f "$bundle/README.md" ] || fail "missing readme"
[ -s "$zip_path" ] || fail "missing zip"

dll_count="$(find "$bundle" -maxdepth 1 -type f -name '*.dll' | wc -l | tr -d ' ')"
if [ "${QIVXIF_REQUIRE_WINDOWS_DLLS:-0}" = 1 ] && [ "$dll_count" -eq 0 ]; then
  fail "no runtime dlls copied"
fi

if command -v unzip >/dev/null 2>&1; then
  unzip -Z1 "$zip_path" | grep -q 'qivxif-superapp.exe' || fail "zip missing executable"
  unzip -Z1 "$zip_path" | grep -q 'run.bat' || fail "zip missing launcher"
fi

printf 'verify package-windows ... ok bundle=%s zip=%s dlls=%s\n' \
  "$bundle" "$zip_path" "$dll_count"
