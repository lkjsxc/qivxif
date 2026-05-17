#!/bin/sh
set -eu

root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
target_dir="${CARGO_TARGET_DIR:-$root/target}"
artifact_root="$root/dist/linux"
bundle="$artifact_root/qivxif-superapp-linux-x86_64"
binary="$target_dir/release/qivxif-superapp"
cargo_bin="${CARGO:-cargo}"

if ! command -v "$cargo_bin" >/dev/null 2>&1 && [ -x /usr/local/cargo/bin/cargo ]; then
  cargo_bin=/usr/local/cargo/bin/cargo
fi

cd "$root"
"$cargo_bin" build --locked --release -p qivxif-superapp

rm -rf "$bundle"
mkdir -p "$bundle"
cp "$binary" "$bundle/qivxif-superapp"
cp LICENSE "$bundle/LICENSE"

cat >"$bundle/run.sh" <<'EOF'
#!/bin/sh
set -eu
DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
exec "$DIR/qivxif-superapp" run
EOF
chmod +x "$bundle/run.sh"

cat >"$bundle/README.md" <<'EOF'
# qivxif Super App

Run:

```bash
./run.sh
```

Direct command:

```bash
./qivxif-superapp run
```

State is stored in `.qivxif-state` under the current working directory unless
`QIVXIF_STATE_DIR` is set.
EOF

(
  cd "$artifact_root"
  tar -czf qivxif-superapp-linux-x86_64.tar.gz qivxif-superapp-linux-x86_64
)

printf '%s\n' "$bundle"
printf '%s\n' "$artifact_root/qivxif-superapp-linux-x86_64.tar.gz"
