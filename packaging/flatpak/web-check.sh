#!/bin/sh
set -eu
app=io.github.reiwand.Reiwand
dir=$(mktemp -d "${TMPDIR:-/tmp}/reiwand-web-check-XXXXXX")
export XDG_RUNTIME_DIR="$dir/run"
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"
export WLR_BACKENDS=headless WLR_RENDERER=pixman WLR_HEADLESS_OUTPUTS=1 WLR_LIBINPUT_NO_DEVICES=1
unset WAYLAND_DISPLAY DISPLAY SWAYSOCK
printf 'xwayland disable\n' >"$dir/sway.conf"
sway -d -c "$dir/sway.conf" >"$dir/sway.log" 2>&1 &
sway=$!
trap 'kill "$sway" 2>/dev/null || true; rm -rf "$dir"' EXIT
socket=
for _ in $(seq 100); do
  socket=$(basename "$(find "$XDG_RUNTIME_DIR" -maxdepth 1 -name 'wayland-[0-9]' -print -quit)")
  [ "$socket" != . ] && [ -n "$socket" ] && break
  sleep 0.2
done
if [ -z "$socket" ] || [ "$socket" = . ]; then
  echo "the headless compositor did not start"
  tail -n 20 "$dir/sway.log"
  exit 1
fi
export WAYLAND_DISPLAY="$socket"
flatpak run --device=dri --socket=wayland --command=reiwand-web "$app" --check
