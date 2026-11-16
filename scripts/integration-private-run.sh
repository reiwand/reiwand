#!/usr/bin/env bash
set -euo pipefail
declare BIN ROOT WORK
pids=()
fail() { echo "FAIL: $*"; exit 1; }
wait_for() {
  local seconds=$1; shift
  for _ in $(seq $((seconds * 10))); do "$@" >/dev/null 2>&1 && return 0; sleep 0.1; done
  return 1
}
ctl() { "$BIN/reiwand-ctl" "$@"; }
finish() { for p in "${pids[@]}"; do kill "$p" 2>/dev/null || true; wait "$p" 2>/dev/null || true; done; }
trap finish EXIT
source "$ROOT/scripts/integration-daemon.sh"

here=$WORK/private
: >"$here/hits"
cat >"$here/count.py" <<'EOF'
import socket, sys
s = socket.socket()
s.bind(("127.0.0.1", 0))
s.listen(16)
print(s.getsockname()[1], flush=True)
while True:
    c, _ = s.accept()
    with open(sys.argv[1], "a") as f:
        f.write("hit\n")
    c.sendall(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
    c.close()
EOF
coproc listener { exec python3 "$here/count.py" "$here/hits"; }
pids+=("$listener_PID")
read -r port <&"${listener[0]}"
hits() { wc -l <"$here/hits"; }
sent() { awk '$1 == "out0:" { print $11 }' /proc/self/net/dev; }
reached() { [ "$(hits)" -ge 1 ]; }

start_daemon "$here/reiwandd.log"
ctl private on >/dev/null
ctl private status | grep -q 'private mode: on' || fail "the status does not say private mode is on: $(ctl private status)"
before=$(sent)
refused() {
  local name=$1
  shift
  if "$@" >"$here/out" 2>&1 </dev/null; then fail "$name worked in private mode: $(cat "$here/out")"; fi
  grep -q 'private mode is on' "$here/out" || fail "$name did not say private mode is on: $(tail -n 3 "$here/out")"
  echo "private: refused $name"
}
refused "adding a repository by link" ctl remote add "http://127.0.0.1:$port/" --wait
refused "adding a repository by name" ctl remote add "https://reiwand-private.invalid:$port/" --wait
ctl provider store file >/dev/null
printf 'hunter2\n' | ctl provider add dav webdav "url=http://127.0.0.1:$port/dav/" user=me refresh=0s cache_mb=16 --secret-stdin >/dev/null ||
  fail "a source could not be added in private mode"
refused "updating a source" ctl provider sync dav --wait
refused "updating every source" ctl provider sync --wait
refused "adding the server" ctl server add "https://127.0.0.1:$port" --wait
refused "adding the server by name" ctl server add "https://reiwand-private.invalid:$port" --wait
refused "moderating a server" env REIWAND_SERVER_TOKEN=t "$BIN/reiwand-ctl" moderate --server "http://127.0.0.1:$port" hidden
refused "downloading the depth model" ctl models download depth-anything-v2-small --yes
sleep 1
[ "$(hits)" = 0 ] || fail "$(hits) connections reached the listener in private mode"
[ "$(sent)" = "$before" ] || fail "$(($(sent) - before)) packets left the computer in private mode"
echo "private: on, $(hits) connections reached the listener and $(($(sent) - before)) packets left"

ctl private off >/dev/null
ctl private status | grep -q 'private mode: off' || fail "the status does not say private mode is off"
ctl provider sync dav --wait >"$here/out" 2>&1 || true
wait_for 10 reached || fail "a source did not reach its listener with private mode off: $(tail -n 3 "$here/out")"
mark=$(sent)
left() { [ "$(sent)" -gt "$mark" ]; }
timeout 20 "$BIN/reiwand-ctl" remote add "https://reiwand-private.invalid:$port/" --wait >/dev/null 2>&1 || true
wait_for 10 left || fail "a name lookup did not leave with private mode off, so the packet count proves nothing"
echo "private: off, $(hits) connections reached the listener and $(($(sent) - mark)) packets left"
stop_daemon "$daemon" "$here/reiwandd.log"
