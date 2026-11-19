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
if ! command -v dbus-run-session >/dev/null; then
  echo "dbus-run-session is needed" >&2
  exit 1
fi
dbus-run-session -- flatpak run --device=dri --socket=wayland --env=QT_QUICK_BACKEND=software --env=LIBGL_ALWAYS_SOFTWARE=1 --command=reiwand-web "$app" --check | grep -q "^Qt [0-9]"
if dbus-run-session -- flatpak run --command=unshare "$app" --user --pid --net --fork true 2>"$dir/unshare.log"; then
  echo "web sandbox inside the Flatpak: full (user, pid and network namespaces can be created)"
else
  echo "web sandbox inside the Flatpak: reduced (the namespaces cannot be created; the web helper turns the Chromium sandbox off)"
  head -n 3 "$dir/unshare.log"
fi
