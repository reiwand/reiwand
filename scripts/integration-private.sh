#!/usr/bin/env bash
set -euo pipefail
if ! declare -F ctl >/dev/null; then
  source "$(dirname "$0")"/integration-env.sh "$@"
  trap 'status=$?; cleanup' EXIT
fi
command -v strace >/dev/null ||
  echo "strace is missing: the proof counts the packets of a network namespace whose only way out is a dummy interface"
command -v unshare >/dev/null || fail "the private mode check needs unshare"
command -v ip >/dev/null || fail "the private mode check needs ip"
shared=${daemon:-}
[ -z "$shared" ] || stop_daemon "$shared" "$WORK/reiwandd.log"
export BIN ROOT WORK log OUTER_UID=$UID OUTER_GID
OUTER_GID=$(id -g)
mkdir -p "$WORK/private"
unshare --user --map-root-user --net --mount bash "$ROOT/scripts/integration-private-ns.sh" >"$WORK/private.log" 2>&1 ||
  { tail -n 30 "$WORK/private.log"; fail "private mode let the program reach the network"; }
grep '^private:' "$WORK/private.log"
[ -z "$shared" ] || start_daemon "$WORK/reiwandd.log"
