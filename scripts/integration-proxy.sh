#!/usr/bin/env bash
set -euo pipefail
declare WORK daemon
if [ -z "${REIWAND_BUS_PROXY:-}" ]; then
  echo "bus proxy checks skipped: set REIWAND_BUS_PROXY to an xdg-dbus-proxy binary to run them"
else
  stop_daemon "$daemon" "$WORK/reiwandd.log"
  plain=$DBUS_SESSION_BUS_ADDRESS
  "$REIWAND_BUS_PROXY" "$plain" "$WORK/proxy.sock" --filter --own=io.github.reiwand \
    --own=io.github.reiwand.Daemon --talk=org.freedesktop.Notifications >"$WORK/proxy.log" 2>&1 &
  proxy=$!
  pids+=("$proxy")
  wait_for 10 test -S "$WORK/proxy.sock" || fail "the bus proxy did not start: $(tail -n 5 "$WORK/proxy.log")"
  export DBUS_SESSION_BUS_ADDRESS=unix:path=$WORK/proxy.sock REIWAND_PRIVATE_BUS_ADDRESS=unix:path=$WORK/proxy.sock
  start_daemon "$WORK/proxied.log"
  ctl state >/dev/null || fail "the daemon did not answer through the bus proxy"
  ctl lib search "" >/dev/null || fail "the library did not answer through the bus proxy"
  for cycle in $(seq 1 15); do
    stop_daemon "$daemon" "$WORK/proxied.log"
    start_daemon "$WORK/proxied.log"
    ctl state >/dev/null || fail "the daemon did not answer through the bus proxy after restart $cycle"
    if grep -q "Invalid" "$WORK/proxy.log"; then fail "the bus proxy refused a message: $(tail -n 5 "$WORK/proxy.log")"; fi
  done
  export DBUS_SESSION_BUS_ADDRESS=$plain REIWAND_PRIVATE_BUS_ADDRESS=$plain
  stop_daemon "$daemon" "$WORK/proxied.log"
  kill "$proxy" 2>/dev/null || true
  wait "$proxy" 2>/dev/null || true
  if grep -q "Invalid" "$WORK/proxy.log"; then fail "the bus proxy refused a message: $(tail -n 5 "$WORK/proxy.log")"; fi
  start_daemon "$WORK/reiwandd.log"
  echo "bus proxy checks passed"
fi
