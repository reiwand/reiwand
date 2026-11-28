#!/usr/bin/env bash
set -euo pipefail

WORK=$1
OUT=$2
ctl() { "$HOME/.local/bin/reiwand-ctl" "$@"; }
fail() { echo "FAIL: $*"; exit 1; }
lock_bus() { dbus-send --session --print-reply --dest=org.freedesktop.ScreenSaver /ScreenSaver "org.freedesktop.ScreenSaver.$1"; }
locked() { lock_bus GetActive | grep -q "boolean true"; }
requested() { grep -A 1 "member=RequestStream" "$OUT/dbus-monitor.log" | grep -c "string \"$1" || true; }
greeter() { pgrep -f kscreenlocker_greet >/dev/null; }
asked() { [ "$(requested "$1")" -gt "$2" ]; }

echo "===== the lock screen follows the desktop on its stream, then shows its own wallpaper"
mkdir -p "$HOME/.config"
printf '[Greeter]\nWallpaperPlugin=io.github.reiwand\n' >"$HOME/.config/kscreenlockerrc"
ctl set "$WORK/green.png" --fit stretch
ctl lock follow >/dev/null
screen=$(ctl state | jq -r '.outputs | keys[0]')
desktop=$(requested "$screen") own=$(requested lock:)
lock_bus Lock >/dev/null || fail "the session could not be locked"
wait_for 30 greeter || fail "the lock screen did not start"
wait_for 30 locked || fail "the session does not report itself locked"
wait_for 60 asked "$screen" "$desktop" || fail "the lock screen did not ask for the stream of $screen"
sleep 5
[ "$(requested lock:)" = "$own" ] || fail "the lock screen asked for a stream of its own while following the desktop"
spectacle --background --nonotify --fullscreen --output "$OUT/lock-desktop.png" >"$OUT/spectacle-lock.log" 2>&1 || true
share() { magick "$1" -fuzz 10% -fill white -opaque "$2" -fill black +opaque white -format "%[fx:mean]" info:; }
if wait_for 20 test -s "$OUT/lock-desktop.png"; then
  green=$(share "$OUT/lock-desktop.png" "#00ff00")
  echo "share of the desktop's green on the lock screen: $green"
  awk -v g="$green" 'BEGIN { exit !(g > 0.2) }' || fail "the lock screen does not show the desktop's wallpaper"
fi

magick -size 64x32 xc:"#ff0000" "$WORK/lock-red.png"
ctl lib import "$WORK/lock-red.png" --wait >/dev/null
ctl lock set "$WORK/lock-red.png" >/dev/null
wait_for 60 asked lock: "$own" || fail "the lock screen did not switch to its own stream: $(ctl lock show --json)"
grep -A 1 "member=RequestStream" "$OUT/dbus-monitor.log" | grep -o 'string "lock:[^"]*"' | sort -u
sleep 5
spectacle --background --nonotify --fullscreen --output "$OUT/lock.png" >>"$OUT/spectacle-lock.log" 2>&1 || true
wait_for 20 test -s "$OUT/lock.png" || true
if [ -s "$OUT/lock.png" ]; then
  magick "$OUT/lock.png" -format "lock screenshot %wx%h\n" info:
  magick "$OUT/lock.png" -format %c histogram:info:- | sort -rn | sed -n 1,5p
  red=$(share "$OUT/lock.png" "#ff0000")
  echo "share of the lock wallpaper's red: $red"
  awk -v r="$red" 'BEGIN { exit !(r > 0.2) }' || fail "the lock screenshot does not show the lock wallpaper"
else
  echo "no screenshot while locked: $(tail -n 3 "$OUT/spectacle-lock.log")"
fi
echo "plasma lock checks passed"
