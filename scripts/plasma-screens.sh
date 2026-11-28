#!/usr/bin/env bash
# A screen that Plasma shows again at another size or scale, then loses, gives its stream back:
# the plugin releases every claim it took, so the daemon closes the stream of the removed screen.
# After a daemon restart the plugin asks again without giving back the claim the old daemon held.
set -euo pipefail

OUT=$1
ctl() { "$HOME/.local/bin/reiwand-ctl" "$@"; }
fail() { echo "FAIL: $*"; exit 1; }
screens() { kscreen-doctor "$@" >>"$OUT/kscreen.log" 2>&1; }
second=Virtual-1
streams() { ctl state | jq -r '.outputs | keys | join(",")'; }
first() { [ -n "$(streams)" ] && [ "$(streams)" != "$second" ]; }
streamed() { ctl state | jq -e --arg o "${1:-$second}" '.outputs[$o] != null'; }
gone() { ! streamed; }
calls() { grep -A 1 -E "member=$1\$" "$OUT/dbus-monitor.log" | grep -c "string \"${2:-$second}\"" || true; }
asked() { [ "$(calls RequestStream "${1:-$second}")" -gt "$requested" ]; }

echo "===== a second screen, resized and rescaled, then removed"
if ! kscreen-doctor -o 2>/dev/null | sed 's/\x1b\[[0-9;]*m//g' | grep -q " $second "; then
  echo "no $second output (not the virtual backend): skipped"
  exit 0
fi
screens "output.$second.enable" "output.$second.priority.2"
wait_for 60 streamed || fail "the wallpaper of the second screen never asked for its stream: $(streams)"
requested=$(calls RequestStream)
screens "output.$second.scale.1.5"
wait_for 20 asked || fail "the plugin did not ask again after a scale change"
sleep 3
screens "output.$second.scale.1"
sleep 5
echo "requests: $(calls RequestStream), releases: $(calls ReleaseStream), streams: $(streams)"
screens "output.$second.disable"
wait_for 20 gone || fail "the removed screen $second keeps a ghost stream: $(ctl state | jq -c --arg o "$second" '.outputs[$o] | {clients, width, height}')"
echo "requests: $(calls RequestStream), releases: $(calls ReleaseStream), streams: $(streams)"
wait_for 20 first || fail "the first screen lost its stream: $(streams)"

echo "===== the daemon restarts"
main=$(streams)
requested=$(calls RequestStream "$main")
pkill -x reiwandd
wait_for 20 eval '! bus_has io.github.reiwand.Daemon' || fail "reiwandd did not quit"
RUST_LOG=info "$HOME/.local/bin/reiwandd" >"$OUT/reiwandd-again.log" 2>&1 &
wait_for 30 asked "$main" || fail "the plugin did not ask for $main again after the daemon restarted"
sleep 5
streamed "$main" >/dev/null || fail "the plugin gave back the new stream of $main after the daemon restarted: $(streams)"

echo "===== a second screen turned on without a priority"
# Plasma may make it the primary one and swap the desktops of the two screens; each wallpaper then
# asks for its new screen before giving back the old one, so the first screen keeps its stream.
ready() { grep "stream ready" "$OUT/reiwandd-again.log" | grep -c "$main" || true; }
before=$(ready)
screens "output.$second.enable"
wait_for 60 streamed || fail "the second screen turned on without a priority never got its stream: $(streams)"
sleep 5
[ "$(ready)" = "$before" ] || fail "turning on a screen without a priority started the stream of $main again ($before -> $(ready) times)"
streamed "$main" >/dev/null || fail "$main lost its stream when $second came on: $(streams)"
screens "output.$second.disable"
wait_for 20 gone || fail "the second screen kept its stream once off: $(streams)"
echo "plasma screens check passed"
