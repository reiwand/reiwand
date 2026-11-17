#!/usr/bin/env bash
set -euo pipefail

BIN=$(realpath "${1:?usage: $0 <bin-dir>}")
ROOT=$(cd "$(dirname "$0")/.." && pwd)
WORK=$(mktemp -d /tmp/reiwand-carry.XXXXXX)
export PIPEWIRE_RUNTIME_DIR=$WORK/run
mkdir -p "$PIPEWIRE_RUNTIME_DIR" && chmod 700 "$PIPEWIRE_RUNTIME_DIR"
log=${REIWAND_IT_LOG:-info},naga=warn,wgpu_core=warn,wgpu_hal=warn
pids=()
cleanup() {
  for p in "${pids[@]}"; do kill "$p" 2>/dev/null || true; done
  for _ in $(seq 50); do
    alive=0
    for p in "${pids[@]}"; do kill -0 "$p" 2>/dev/null && alive=1; done
    [ "$alive" = 0 ] && break
    sleep 0.1
  done
  if [ "${status:-1}" != 0 ]; then
    for f in "$WORK"/*.log; do echo "== $f"; tail -n 40 "$f"; done
  fi
  discard "$WORK"
}
trap cleanup EXIT
wait_for() {
  local seconds=$1; shift
  for _ in $(seq $((seconds * 10))); do "$@" >/dev/null 2>&1 && return 0; sleep 0.1; done
  return 1
}
fail() { echo "FAIL: $*"; exit 1; }
jq_() { python3 -c "import json, sys; r = json.load(sys.stdin); print(eval(sys.argv[1]))" "$1"; }
source "$(dirname "$0")"/integration-daemon.sh

carry_home() {
  local dir=$WORK/$1
  mkdir -p "$dir/run" && chmod 700 "$dir/run"
  cat >"$dir/bus.conf" <<CONF
<busconfig><type>session</type><listen>unix:path=$dir/bus</listen><auth>EXTERNAL</auth>
<policy context="default"><allow send_destination="*" eavesdrop="true"/><allow eavesdrop="true"/><allow own="*"/></policy></busconfig>
CONF
  pids+=("$(dbus-daemon --config-file="$dir/bus.conf" --fork --print-pid)")
}
carry_env() {
  local dir=$WORK/$1
  printf '%s\n' DBUS_SESSION_BUS_ADDRESS="unix:path=$dir/bus" XDG_RUNTIME_DIR="$dir/run" PIPEWIRE_RUNTIME_DIR="$PIPEWIRE_RUNTIME_DIR" XDG_CONFIG_HOME="$dir/config" \
    XDG_DATA_HOME="$dir/data" XDG_CACHE_HOME="$dir/cache" REIWAND_PRIVATE_BUS_ONLY=1 \
    REIWAND_PRIVATE_BUS_ADDRESS="unix:path=$dir/bus"
}
carry_start() {
  local name=$1 env
  mapfile -t env < <(carry_env "$name")
  DBUS_SESSION_BUS_ADDRESS="unix:path=$WORK/$name/bus" wait_for 10 name_free || fail "daemon $name: the bus name is owned"
  env "${env[@]}" RUST_LOG="$log" "$BIN/reiwandd" >>"$WORK/reiwandd-$name.log" 2>&1 &
  pids+=($!)
  await_daemon "$!" "$WORK/reiwandd-$name.log" env "${env[@]}" "$BIN/reiwand-ctl" state
}
x() { local env; mapfile -t env < <(carry_env "$1"); shift; env "${env[@]}" "$BIN/reiwand-ctl" "$@"; }
XDG_RUNTIME_DIR=$WORK/run pipewire >"$WORK/pipewire.log" 2>&1 & pids+=($!)
wait_for 10 test -S "$PIPEWIRE_RUNTIME_DIR/pipewire-0" || fail "PipeWire did not start"
XDG_RUNTIME_DIR=$WORK/run wireplumber >"$WORK/wireplumber.log" 2>&1 & pids+=($!)
carry_home src
carry_home dst
carry_start src
carry_start dst
x src provider store file 2>/dev/null
x dst provider store file 2>/dev/null

site=$WORK/site
mkdir -p "$site/src"
cp "$ROOT/tests/fixtures/green-64x32.png" "$site/src/image.png"
cp "$ROOT/tests/fixtures/green-64x32.png" "$site/src/preview.png"
cp "$ROOT/tests/fixtures/green-64x32.png" "$WORK/mine.png"
cat >"$site/src/manifest.json" <<'MANIFEST'
{"schema": "reiwand.wpak/2", "id": "org.example.dawn", "version": "1.0.0", "title": "Dawn",
 "content": {"rating": "general"}, "photosensitive": false, "origin": {"kind": "original"},
 "license": "CC0-1.0", "kind": "image", "entry": "image.png", "previews": {"thumbnail": "preview.png"}, "tags": ["sky"]}
MANIFEST
(
  cd "$site"
  x src repo keygen root --no-password >/dev/null
  x src repo keygen online --no-password >/dev/null
  x src pack build src -o dawn.wpak >/dev/null
  x src repo init repo --name Example --root-key root.pub --online-key online.pub >/dev/null 2>&1
  x src repo sign-root repo --key root.key >/dev/null
  x src repo add repo dawn.wpak >/dev/null
  x src repo publish repo --key online.key >/dev/null
)
x src remote add "$site/repo" --wait >/dev/null
dawn=$(x src pkg install org.example.dawn --wait --json | jq_ "r['output']['item']")
x src lib import "$WORK/mine.png" --copy --wait >/dev/null
mine=$(x src lib search "sort:title" --json | jq_ "[i['uid'] for i in r['items'] if i['uid'] != '$dawn'][0]")
x src lib fav "$dawn" "$mine"
x src lib rate 4 "$dawn"
x src lib tag "$mine" --add keep
x src playlist new Mix >/dev/null
x src playlist add Mix "$dawn" "$mine"
x src provider add nas webdav url=http://nas.example.org/dav user=reader refresh=0s --no-secret >/dev/null
printf 'carry-secret-value\n' | x src provider secret nas --secret-stdin
x src config library.hide_mature true

file=$WORK/plain.reiwand
x src export "$file" --own >/dev/null
[ -s "$file" ] || fail "the export wrote no file"
if x src export "$file" 2>/dev/null; then fail "an export replaced an existing file"; fi
if grep -a -q carry-secret-value "$file"; then fail "a secret is in a file exported without secrets"; fi
x src jobs --json | jq_ "[j['kind'] for j in r['jobs']].count('carry_export')" | grep -qx 1 ||
  fail "the export is not listed as a job: $(x src jobs)"
preview=$(x dst import "$file" --list)
grep -q "lists" <<<"$preview" || fail "the preview names no lists: $preview"
[ "$(x dst remote ls --json | jq_ "len(r['repos'])")" = 0 ] || fail "the preview changed the destination"

x dst import "$file" >"$WORK/first.out" || fail "import: $(cat "$WORK/first.out")"
imported() { [ "$(x dst playlist ls --json | jq_ "sorted(p['name'] for p in r['playlists'])")" = "['Mix']" ]; }
wait_for 20 imported || fail "the list was not imported: $(x dst playlist ls)"
[ "$(x dst remote ls --json | jq_ "[x['name'] for x in r['repos']]")" = "['Example']" ] ||
  fail "the repository was not added: $(x dst remote ls)"
[ "$(x dst pkg ls --json | jq_ "[p['id'] for p in r['packages']]")" = "['org.example.dawn']" ] ||
  fail "the package was not installed again: $(x dst pkg ls)"
[ "$(x dst provider ls --json | jq_ "[(p['id'], p['has_secret']) for p in r['providers']]")" = "[('nas', False)]" ] ||
  fail "providers: $(x dst provider ls)"
[ "$(x dst lib search "fav:true" --json | jq_ "r['total']")" = 2 ] || fail "favourites: $(x dst lib search fav:true)"
[ "$(x dst lib search "tag:keep" --json | jq_ "r['total']")" = 1 ] || fail "tags: $(x dst lib search tag:keep)"
[ "$(x dst playlist show Mix --json | jq_ "r['total']")" = 2 ] || fail "list items: $(x dst playlist show Mix)"
[ "$(x dst config library.hide_mature)" = "library.hide_mature = true" ] || fail "a portable setting was not imported"

x dst import "$file" >/dev/null
[ "$(x dst playlist ls --json | jq_ "len(r['playlists'])")" = 1 ] || fail "a second import duplicated a list"
[ "$(x dst playlist show Mix --json | jq_ "r['total']")" = 2 ] || fail "a second import duplicated items"
[ "$(x dst lib search "" --json | jq_ "r['total']")" = 2 ] || fail "a second import duplicated items: $(x dst lib search '')"

x dst import "$file" --lists both --suffix " (importada)" >/dev/null
[ "$(x dst playlist ls --json | jq_ "sorted(p['name'] for p in r['playlists'])")" = "['Mix', 'Mix (importada)']" ] ||
  fail "list mode both: $(x dst playlist ls)"

printf 'correct horse\n' >"$WORK/pass"
printf 'wrong horse\n' >"$WORK/wrong"
x src export "$WORK/sealed.reiwand" --secrets --password-file "$WORK/pass" >/dev/null
if grep -a -q carry-secret-value "$WORK/sealed.reiwand"; then fail "a secret is readable in the encrypted file"; fi
if x dst import "$WORK/sealed.reiwand" --list --password-file "$WORK/wrong" 2>"$WORK/wrong.err"; then
  fail "a wrong password opened the file"
fi
grep -qi "password" "$WORK/wrong.err" || fail "the wrong password is not said: $(cat "$WORK/wrong.err")"
x dst import "$WORK/sealed.reiwand" --take providers --take secrets --password-file "$WORK/pass" >/dev/null
[ "$(x dst provider ls --json | jq_ "[p['has_secret'] for p in r['providers']]")" = "[True]" ] ||
  fail "the key of the provider was not imported: $(x dst provider ls)"
status=0
