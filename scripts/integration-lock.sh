#!/usr/bin/env bash
set -euo pipefail
lk=$WORK/lock
mkdir -p "$lk"
lone=$WORK/lock-imported
mkdir -p "$lone"
png "$lone/lock-red.png" ff0000
png "$lk/lock-blue.png" 0000ff
png "$lk/lock-green.png" 00ff00
ctl lib add "$lk" --wait >/dev/null
ctl lib import "$lone/lock-red.png" --wait >/dev/null
lock_red=$(ctl lib show "$lone/lock-red.png" --json | js "r['item']['uid']")
lock() { ctl lock show --json | js "$1"; }
conf=$XDG_CONFIG_HOME/reiwand/config.toml

ctl set "$lk/lock-green.png" -o TEST-L1
read -r _ _ _ desk < <(ctl stream TEST-L1 160 90)
read -r _ _ _ locked < <(ctl stream lock:TEST-L1 160 90)
center() { "$BIN/probe" "$1" 3 | grep "center bgra \[$2\]" >/dev/null; }
green="0, 255, 0, 255" red="0, 0, 255, 255" blue="255, 0, 0, 255"
[ "$(ctl lock show)" = "every screen: like the desktop" ] || fail "a lock choice before any: $(ctl lock show)"
retry 10 center "$locked" "$green" || fail "the lock screen does not follow the desktop: $("$BIN/probe" "$locked" 3)"
! grep -q '^\[lock\]' "$conf" || fail "the lock block was written with nothing chosen"

ctl lock set "$lone/lock-red.png" >/dev/null
[ "$(lock "r['every']['item'], r['every']['playlist']")" = "('$lock_red', None)" ] || fail "the lock choice: $(ctl lock show --json)"
retry 10 center "$locked" "$red" || fail "the lock screen does not show its own wallpaper: $("$BIN/probe" "$locked" 3)"
retry 10 center "$desk" "$green" || fail "the desktop took the lock wallpaper: $("$BIN/probe" "$desk" 3)"
grep -q '^\[lock\]' "$conf" || fail "the lock choice was not saved: $(cat "$conf")"
[ "$(field "list(s['outputs'])")" = "['TEST-L1']" ] || fail "the lock stream is listed as a screen"

ctl playlist new LockList >/dev/null
ctl playlist add LockList "$lk/lock-blue.png"
ctl lock playlist LockList -o TEST-L1 >/dev/null
[ "$(lock "list(r['outputs']), r['outputs']['TEST-L1']['name']")" = "(['TEST-L1'], 'LockList')" ] ||
  fail "the lock playlist of one screen: $(ctl lock show --json)"
retry 10 center "$locked" "$blue" || fail "the lock playlist does not show: $("$BIN/probe" "$locked" 3)"
ctl lock follow -o TEST-L1 >/dev/null
retry 10 center "$locked" "$red" || fail "the screen did not take the choice of every screen: $("$BIN/probe" "$locked" 3)"
ctl lock playlist LockList -o TEST-L1 >/dev/null
ctl playlist rm LockList
[ "$(lock "list(r['outputs'])")" = "[]" ] || fail "a deleted playlist stayed on the lock screen: $(ctl lock show --json)"
retry 10 center "$locked" "$red" || fail "the lock screen after its playlist was deleted: $("$BIN/probe" "$locked" 3)"
ctl lock desktop -o TEST-L1 >/dev/null
[ "$(lock "r['outputs']['TEST-L1']['desktop']")" = "True" ] || fail "the desktop mark of one screen: $(ctl lock show --json)"
retry 10 center "$locked" "$green" || fail "the screen marked to keep its desktop: $("$BIN/probe" "$locked" 3)"
ctl lock follow -o TEST-L1 >/dev/null
retry 10 center "$locked" "$red" || fail "the screen did not take every screen's choice again: $("$BIN/probe" "$locked" 3)"

ctl lib rm "$lock_red" >/dev/null
[ "$(lock "r['every']")" = "None" ] || fail "a removed item stayed on the lock screen: $(ctl lock show --json)"
retry 10 center "$locked" "$green" || fail "the lock screen did not go back to the desktop: $("$BIN/probe" "$locked" 3)"
! grep -q '^\[lock\]' "$conf" || fail "the lock block stayed with nothing chosen: $(cat "$conf")"
if ctl lock set "$lk/lock-blue.png" -o lock:TEST-L1 2>/dev/null; then fail "a lock choice for a lock stream"; fi
ctl lock set "$lk/lock-blue.png" -o TEST-L1 >/dev/null && ctl lock follow >/dev/null
[ "$(lock "r['every'], r['outputs']")" = "(None, {})" ] || fail "follow left a choice: $(ctl lock show --json)"
ctl release lock:TEST-L1 && ctl release TEST-L1
echo "lock screen checks passed"
