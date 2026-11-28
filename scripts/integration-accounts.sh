#!/usr/bin/env bash
set -euo pipefail

BIN=$(realpath "${1:?usage: $0 <bin-dir>}")
source "$(dirname "$0")"/integration-complete.sh
WORK=$(mktemp -d /tmp/reiwand-accounts.XXXXXX)
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

acct_home() {
  local dir=$WORK/$1
  mkdir -p "$dir/run" && chmod 700 "$dir/run"
  cat >"$dir/bus.conf" <<CONF
<busconfig><type>session</type><listen>unix:path=$dir/bus</listen><auth>EXTERNAL</auth>
<policy context="default"><allow send_destination="*" eavesdrop="true"/><allow eavesdrop="true"/><allow own="*"/></policy></busconfig>
CONF
  pids+=("$(dbus-daemon --config-file="$dir/bus.conf" --fork --print-pid)")
}
acct_env() {
  local dir=$WORK/$1
  printf '%s\n' DBUS_SESSION_BUS_ADDRESS="unix:path=$dir/bus" XDG_RUNTIME_DIR="$dir/run" PIPEWIRE_RUNTIME_DIR="$PIPEWIRE_RUNTIME_DIR" XDG_CONFIG_HOME="$dir/config" \
    XDG_DATA_HOME="$dir/data" XDG_CACHE_HOME="$dir/cache" REIWAND_PRIVATE_BUS_ONLY=1 LANGUAGE=en LC_ALL=C.UTF-8 \
    REIWAND_PRIVATE_BUS_ADDRESS="unix:path=$dir/bus" ${ACCT_EXTRA:+"$ACCT_EXTRA"}
}
acct_start() {
  local name=$1 env
  mapfile -t env < <(acct_env "$name")
  DBUS_SESSION_BUS_ADDRESS="unix:path=$WORK/$name/bus" wait_for 10 name_free || fail "daemon $name: the bus name is owned"
  env "${env[@]}" RUST_LOG="$log" "$BIN/reiwandd" >>"$WORK/reiwandd-$name.log" 2>&1 &
  pids+=($!)
  await_daemon "$!" "$WORK/reiwandd-$name.log" env "${env[@]}" "$BIN/reiwand-ctl" state
}
x() { local env; mapfile -t env < <(acct_env "$1"); shift; env "${env[@]}" "$BIN/reiwand-ctl" "$@"; }
XDG_RUNTIME_DIR=$WORK/run pipewire >"$WORK/pipewire.log" 2>&1 & pids+=($!)
wait_for 10 test -S "$PIPEWIRE_RUNTIME_DIR/pipewire-0" || fail "PipeWire did not start"
XDG_RUNTIME_DIR=$WORK/run wireplumber >"$WORK/wireplumber.log" 2>&1 & pids+=($!)
acct_home old
data=$WORK/old/data/reiwand
mkdir -p "$data" "$WORK/old/config"
printf '{}' >"$data/sync.json"

mapfile -t env < <(acct_env old)
env "${env[@]}" RUST_LOG="$log" "$BIN/reiwandd" >"$WORK/reiwandd-refused.log" 2>&1 &
refused=$!
wait_for 20 daemon_exited "$refused" || fail "the daemon started on data of another version"
grep -q "which this version does not read; nothing was changed" "$WORK/reiwandd-refused.log" ||
  fail "the refusal does not say why: $(tail -n 5 "$WORK/reiwandd-refused.log")"
[ -e "$data/sync.json" ] || fail "the refused data was changed"
if [ -x "$BIN/reiwand-server" ]; then
  source "$(dirname "$0")"/integration-accounts-servers.sh
  source "$(dirname "$0")"/integration-accounts-flows.sh
else
  echo "SKIP: no reiwand-server in $BIN"
fi
status=0
echo "accounts checks passed"
