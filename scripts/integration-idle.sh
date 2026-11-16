#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")"/integration-env.sh "$@"
limits=$ROOT/apps/reiwand-ctl/tests/net/idle.txt
out=TEST-IDLE
start_daemon "$WORK/reiwandd.log"
read -r _ _ _ serial < <(ctl stream "$out" 160 90)
"$BIN/probe" "$serial" 120 --hold >/dev/null 2>&1 & pids+=($!)

drawn() { field "s['outputs']['$out']['frames']"; }
moves() { local before; before=$(drawn); sleep 1; [ "$(drawn)" -gt "$before" ]; }
wakes() { { cat /proc/"$daemon"/task/*/status 2>/dev/null || true; } | awk '/ctxt_switches/ { s += $2 } END { print s + 0 }'; }
fresh=()
idle() {
  local name=$1 frames wakeups from woke most_frames most_wakes
  sleep 1
  from=$(drawn) woke=$(wakes)
  sleep 3
  wakeups=$(($(wakes) - woke)) frames=$(($(drawn) - from))
  echo "idle $name: $frames frames drawn and $wakeups wake-ups in 3 s"
  if [ -n "${REIWAND_BLESS:-}" ]; then
    fresh+=("$name $frames $((wakeups * 2 + 20))")
    return
  fi
  read -r _ most_frames most_wakes < <(grep "^$name " "$limits") || fail "no stored idle numbers for $name; refresh them with scripts/regression.sh --bless"
  [ "$frames" -le "$most_frames" ] || fail "$name: $frames frames drawn in 3 s, at most $most_frames"
  [ "$wakeups" -le "$most_wakes" ] || fail "$name: the daemon woke $wakeups times in 3 s, at most $most_wakes"
}

ctl set "$ROOT/tests/fixtures/bands-640x180.png" >/dev/null
shown() { [ "$(drawn)" -gt 0 ]; }
wait_for 20 shown || fail "the still picture was never drawn"
idle still

ctl pack build "$ROOT/scenes/snowfall" -o "$WORK/snowfall.wpak" >/dev/null
ctl lib import "$WORK/snowfall.wpak" --wait >/dev/null
snowfall=$(ctl lib search "" --json | python3 -c "import json, sys
print([i['uid'] for i in json.load(sys.stdin)['items'] if (i['package'] or {}).get('id', '').endswith('.snowfall')][0])")
ctl lib apply "$snowfall" -o "$out" >/dev/null
expect "s['outputs']['$out']['kind']" scene
wait_for 20 moves || fail "the snowfall scene was not drawn while shown"

ctl covered "$out" true && expect "s['outputs']['$out']['active']" False
idle covered
ctl covered "$out" false
wait_for 20 moves || fail "the snowfall scene did not draw again once shown"
ctl pause && expect "s['paused']" True
idle paused

if [ -n "${REIWAND_BLESS:-}" ]; then
  printf '%s\n' "${fresh[@]}" >"$limits"
  echo "stored $limits"
fi
status=0
echo "idle checks passed"
