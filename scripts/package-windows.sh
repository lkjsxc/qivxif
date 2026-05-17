#!/bin/sh
set -eu

root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
target_triple="${QIVXIF_WINDOWS_TARGET:-x86_64-pc-windows-gnu}"
target_dir="${CARGO_TARGET_DIR:-$root/target}"
artifact_root="$root/dist/windows"
bundle="$artifact_root/qivxif-superapp-windows-x86_64"
zip_path="$artifact_root/qivxif-superapp-windows-x86_64.zip"
binary="$target_dir/$target_triple/release/qivxif-superapp.exe"
cargo_bin="${CARGO:-cargo}"

if ! command -v "$cargo_bin" >/dev/null 2>&1 && [ -x /usr/local/cargo/bin/cargo ]; then
  cargo_bin=/usr/local/cargo/bin/cargo
fi

cd "$root"
if command -v rustup >/dev/null 2>&1; then
  rustup target add "$target_triple"
fi
"$cargo_bin" build --locked --release --target "$target_triple" -p qivxif-superapp

rm -rf "$bundle" "$zip_path"
mkdir -p "$bundle"
cp "$binary" "$bundle/qivxif-superapp.exe"
cp LICENSE "$bundle/LICENSE"

cat >"$bundle/run.bat" <<'EOF'
@echo off
set "DIR=%~dp0"
"%DIR%qivxif-superapp.exe" run
EOF

cat >"$bundle/README.md" <<'EOF'
# qivxif Super App

Run:

```powershell
.\run.bat
```

Direct command:

```powershell
.\qivxif-superapp.exe run
```

Smoke command:

```powershell
.\qivxif-superapp.exe smoke
```

State is stored in `.qivxif-state` under the current working directory unless
`QIVXIF_STATE_DIR` is set.
EOF

if command -v x86_64-w64-mingw32-objdump >/dev/null 2>&1; then
  x86_64-w64-mingw32-objdump -p "$binary" \
    | awk -F': ' '/DLL Name:/ {print $2}' \
    | while IFS= read -r dll; do
        case "$(printf '%s' "$dll" | tr '[:upper:]' '[:lower:]')" in
          api-ms-*.dll|ext-ms-*.dll)
            continue
            ;;
          advapi32.dll|bcrypt.dll|bcryptprimitives.dll)
            continue
            ;;
          cfgmgr32.dll|comctl32.dll|comdlg32.dll|crypt32.dll)
            continue
            ;;
          d3d12.dll|d3dcompiler_47.dll|dwmapi.dll|dxgi.dll|gdi32.dll)
            continue
            ;;
          imm32.dll|kernel32.dll|msvcrt.dll|ntdll.dll|ole32.dll|oleaut32.dll)
            continue
            ;;
          opengl32.dll|propsys.dll|shell32.dll|shlwapi.dll|user32.dll)
            continue
            ;;
          uiautomationcore.dll|userenv.dll|uxtheme.dll)
            continue
            ;;
          windowscodecs.dll|winmm.dll|ws2_32.dll)
            continue
            ;;
        esac
        found="$(find /usr -type f -name "$dll" -print -quit || true)"
        if [ -z "$found" ]; then
          printf 'missing required runtime DLL: %s\n' "$dll" >&2
          exit 1
        fi
        cp "$found" "$bundle/$dll"
      done
fi

(
  cd "$artifact_root"
  zip -qr "$(basename "$zip_path")" "$(basename "$bundle")"
)

printf '%s\n' "$bundle"
printf '%s\n' "$zip_path"
