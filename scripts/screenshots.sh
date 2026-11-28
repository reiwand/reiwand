#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

usage() {
  echo "usage: scripts/screenshots.sh [--size WxH] [--bin DIR] [--wallpaper SCENE] [OUT]" >&2
  echo "Captures the desktop app as a new user sees it, on a private X server and session bus," >&2
  echo "into OUT (docs/images by default): one PNG per line of SHOTS." >&2
  exit 2
}

SHOTS=(
  "library library"
  "explore repos"
  "sources providers"
  "lists playlists"
  "settings settings"
)

size=1600x900
bin=target/debug
wallpaper=dusk
out=docs/images
while [ $# -gt 0 ]; do
  case $1 in
    --size) size=$2; shift 2 ;;
    --bin) bin=$2; shift 2 ;;
    --wallpaper) wallpaper=$2; shift 2 ;;
    -h | --help) usage ;;
    -*) usage ;;
    *) out=$1; shift ;;
  esac
done

for tool in Xvfb dbus-run-session xdotool jq import convert; do
  command -v "$tool" >/dev/null || { echo "screenshots: $tool is missing" >&2; exit 1; }
done
for program in reiwandd reiwand-app reiwand-ctl; do
  [ -x "$bin/$program" ] || { echo "screenshots: build $program first (cargo build), or pass --bin" >&2; exit 1; }
done

home=$(mktemp -d /tmp/rw-shots.XXXXXX)
trap 'kill "${server:-0}" 2>/dev/null || true; rm -rf "$home"' EXIT
mkdir -p "$out" "$home/run"
chmod 700 "$home/run"
printf '%s\n' "${SHOTS[@]}" >"$home/shots"
display=:$((90 + RANDOM % 9))
Xvfb "$display" -screen 0 "${size}x24" -nolisten tcp >/dev/null 2>&1 &
server=$!
sleep 1

cat >"$home/session.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
export DISPLAY=$display QT_QPA_PLATFORM=xcb LANG=C.UTF-8 LC_ALL=C.UTF-8 LANGUAGE=en
export HOME=$home XDG_CONFIG_HOME=$home/config XDG_DATA_HOME=$home/data XDG_CACHE_HOME=$home/cache
export XDG_STATE_HOME=$home/state XDG_RUNTIME_DIR=$home/run
command -v pipewire >/dev/null && { pipewire >/dev/null 2>&1 & sleep 1; }
"$PWD/$bin/reiwandd" >"$home/daemon.log" 2>&1 &
for _ in \$(seq 50); do "$PWD/$bin/reiwand-ctl" state >/dev/null 2>&1 && break; sleep 0.2; done
mkdir -p "$home/packages"
for scene in "$PWD"/scenes/*/; do
  "$PWD/$bin/reiwand-ctl" pack build -o "$home/packages/\$(basename "\$scene").wpak" "\$scene" >/dev/null
done
"$PWD/$bin/reiwand-ctl" pkg import --wait "$home"/packages/*.wpak >/dev/null
"$PWD/$bin/reiwand-ctl" lib search --json --limit 500 >"$home/items.json"
uid=\$(jq -r '.items[] | select(.package.id == "io.github.reiwand.example.$wallpaper") | .uid' "$home/items.json")
"$PWD/$bin/reiwand-ctl" lib apply "\$uid" >/dev/null
while read -r name page; do
  "$PWD/$bin/reiwand-app" --page "\$page" >"$home/app-\$name.log" 2>&1 &
  app=\$!
  sleep 6
  window=\$(xdotool search --pid \$app --onlyvisible | tail -1)
  xdotool windowmove "\$window" 0 0 windowsize "\$window" ${size%x*} ${size#*x}
  sleep 2
  import -window root "$home/\$name.png"
  kill \$app; wait \$app 2>/dev/null || true
done <"$home/shots"
EOF
chmod +x "$home/session.sh"
dbus-run-session -- "$home/session.sh"

for shot in "${SHOTS[@]}"; do
  name=${shot%% *}
  convert "$home/$name.png" -strip -define png:compression-level=9 "$out/$name.png"
  echo "$out/$name.png"
done
