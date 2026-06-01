#!/bin/sh
set -eu

repo_dir="${QIVXIF_REPO_DIR:-$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)}"
cd "$repo_dir"

base="http://127.0.0.1:8080"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/qivxif-api-proof.XXXXXX")"
cookies="$work_dir/cookies.txt"

cleanup() {
  if [ "${server_pid:-}" ]; then
    kill "$server_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$work_dir"
}

trap cleanup EXIT HUP INT TERM

make_id() {
  node -e '
const crypto = require("crypto");
console.log(`${process.argv[1]}_${crypto.randomBytes(32).toString("hex")}`);
' "$1"
}

json_value() {
  node -e '
const fs = require("fs");
const data = JSON.parse(fs.readFileSync(process.argv[1], "utf8"));
let value = data;
for (const key of process.argv[2].split(".")) value = value[key];
if (typeof value === "object") console.log(JSON.stringify(value));
else console.log(value);
' "$1" "$2"
}

json_check() {
  file="$1"
  script="$2"
  shift 2
  node -e '
const fs = require("fs");
const data = JSON.parse(fs.readFileSync(process.argv[1], "utf8"));
'"$script" "$file" "$@"
}

post_json() {
  path="$1"
  body="$2"
  out="$3"
  curl -fsS -b "$cookies" -c "$cookies" \
    -H "content-type: application/json" \
    -H "x-qivxif-csrf: $csrf" \
    -d @"$body" \
    "$base$path" >"$out"
}

post_public_json() {
  path="$1"
  body="$2"
  out="$3"
  curl -fsS -b "$cookies" -c "$cookies" \
    -H "content-type: application/json" \
    -d @"$body" \
    "$base$path" >"$out"
}

get_json() {
  path="$1"
  out="$2"
  curl -fsS -b "$cookies" "$base$path" >"$out"
}

rm -rf "$QIVXIF_DATA_DIR"
mkdir -p "$QIVXIF_DATA_DIR"
web_dist="${QIVXIF_WEB_DIST_DIR:-${TMPDIR:-/tmp}/qivxif-web-dist}"
QIVXIF_WEB_DIST_DIR="$web_dist" npm --prefix apps/qivxif-web run build
export QIVXIF_STATIC_DIR="$web_dist"

cargo run --locked -p qivxif-server >"$work_dir/server.log" 2>&1 &
server_pid="$!"

for _ in $(seq 1 120); do
  if curl -fsS "$base/health" >"$work_dir/health.json"; then
    break
  fi
  sleep 1
done

if ! test -s "$work_dir/health.json"; then
  cat "$work_dir/server.log" >&2
  printf 'server did not become healthy\n' >&2
  exit 1
fi

get_json "/api/setup" "$work_dir/setup-open.json"
json_check "$work_dir/setup-open.json" '
if (!data.payload.required || !data.payload.owner_creation_open) {
  throw new Error("setup was not open");
}
'

cat >"$work_dir/setup-body.json" <<'JSON'
{"name":"admin","password":"secret"}
JSON
post_public_json "/api/setup/owner" "$work_dir/setup-body.json" "$work_dir/setup-owner.json"
csrf="$(json_value "$work_dir/setup-owner.json" payload.csrf_token)"
actor_id="$(json_value "$work_dir/setup-owner.json" payload.user.actor_id)"
get_json "/api/me" "$work_dir/me-after-setup.json"
json_check "$work_dir/me-after-setup.json" '
if (data.payload.user.name !== "admin") throw new Error("setup session missing");
'
status="$(curl -sS -o "$work_dir/setup-conflict.json" -w "%{http_code}" \
  -b "$cookies" -c "$cookies" -H "content-type: application/json" \
  -d @"$work_dir/setup-body.json" "$base/api/setup/owner")"
test "$status" = "409"
get_json "/api/setup" "$work_dir/setup-closed.json"
json_check "$work_dir/setup-closed.json" '
if (data.payload.required || data.payload.owner_creation_open) {
  throw new Error("setup stayed open");
}
'

node_a="$(make_id nod)"
node_b="$(make_id nod)"

cat >"$work_dir/node-a.json" <<JSON
{"event_id":"$(make_id evt)","actor_seq":1,"node_id":"$node_a","kind":"text","visibility":"private","metadata_map":{"title":"API proof A"}}
JSON
post_json "/api/nodes" "$work_dir/node-a.json" "$work_dir/node-a-out.json"

cat >"$work_dir/node-b.json" <<JSON
{"event_id":"$(make_id evt)","actor_seq":2,"node_id":"$node_b","kind":"text","visibility":"private","metadata_map":{"title":"API proof B"}}
JSON
post_json "/api/nodes" "$work_dir/node-b.json" "$work_dir/node-b-out.json"

cat >"$work_dir/text.json" <<JSON
{"actor_seq":3,"event":{"event_id":"$(make_id evt)","doc_id":"$(make_id txt)","edit":{"kind":"restore","content":"compose proof text","actor_id":"$actor_id","first_seq":3000000}}}
JSON
post_json "/api/text/$node_a/events" "$work_dir/text.json" "$work_dir/text-out.json"

cat >"$work_dir/edge.json" <<JSON
{"event_id":"$(make_id evt)","actor_seq":4,"edge_id":"$(make_id edg)","from_node":"$node_a","to_node":"$node_b","kind":"links_to","metadata_map":{}}
JSON
post_json "/api/edges" "$work_dir/edge.json" "$work_dir/edge-out.json"

get_json "/api/nodes/$node_a" "$work_dir/node-read.json"
get_json "/api/nodes/$node_a/edges" "$work_dir/edges.json"
get_json "/api/graph/neighborhood?node_id=$node_a&depth=1&limit=10" "$work_dir/neighborhood.json"
get_json "/api/sync/pull?limit=10&scope=graph" "$work_dir/pull.json"
get_json "/api/nodes/$node_a/history" "$work_dir/history.json"

json_check "$work_dir/node-read.json" 'if (data.payload.projection.node.id !== process.argv[2]) throw new Error("node read mismatch");' "$node_a"
json_check "$work_dir/edges.json" 'if (data.payload.outgoing.length !== 1) throw new Error("edge read mismatch");'
json_check "$work_dir/neighborhood.json" 'if (data.payload.projection.nodes.length !== 2) throw new Error("neighborhood mismatch");'
json_check "$work_dir/pull.json" 'if (data.payload.events.length < 4) throw new Error("sync pull missed events");'
json_check "$work_dir/history.json" '
const kinds = data.payload.events.map((event) => event.kind);
for (const kind of ["node.create", "text.restore", "edge.create"]) {
  if (!kinds.includes(kind)) throw new Error(`history missed ${kind}`);
}
'

curl -fsS "$base/" >"$work_dir/app-shell.html"
grep -q 'id="app"' "$work_dir/app-shell.html"

curl -fsS -b "$cookies" -c "$cookies" -H "x-qivxif-csrf: $csrf" \
  -X POST "$base/api/auth/logout" >"$work_dir/logout.json"
status="$(curl -sS -o "$work_dir/me-after-logout.json" -w "%{http_code}" -b "$cookies" "$base/api/me")"
test "$status" = "401"

printf 'api proof pass\n'
