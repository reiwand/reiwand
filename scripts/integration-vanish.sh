#!/usr/bin/env bash
set -euo pipefail
declare WORK ROOT BIN daemon
from() { python3 -c "import json, sys; r = json.load(sys.stdin); print(eval(sys.argv[1]))" "$1"; }
vanish=$WORK/vanish
mkdir -p "$vanish"
cp "$ROOT/tests/fixtures/magenta-64x32.webm" "$vanish/clip.webm"
cp "$ROOT/tests/fixtures/bands-640x180.png" "$vanish/bands.png"
ctl pack build "$ROOT/scenes/solids" -o "$vanish/solids.wpak" >/dev/null
ctl lib import "$vanish/solids.wpak" --wait >/dev/null
solids=$(ctl lib search "" --json | from "[i['uid'] for i in r['items'] if (i['package'] or {}).get('id') == 'io.github.reiwand.example.solids'][0]")
ctl stream TEST-V 160 90 >/dev/null
answers() { timeout 10 "$BIN/reiwand-ctl" state >/dev/null; }
said() { field "s['outputs']['TEST-V']['error'] or ''" | grep -q "$want"; }
for case in "clip.webm|not an existing local file" "solids|cannot be shown" "bands.png|cannot be shown"; do
  shown=${case%%|*} want=${case#*|}
  if [ "$shown" = solids ]; then
    ctl lib apply "$solids" -o TEST-V
    file=$(ctl lib show "$solids" --json | from "r['item']['path']")
  else
    ctl set "$vanish/$shown" -o TEST-V
    file=$vanish/$shown
  fi
  expect "s['outputs']['TEST-V']['error']" None
  rm -f "$file"
  sleep 2
  kill -0 "$daemon" || fail "the daemon died when $shown went away while shown"
  answers || fail "the daemon stopped answering when $shown went away"
  ctl release TEST-V
  ctl stream TEST-V 160 90 >/dev/null
  sleep 2
  kill -0 "$daemon" || fail "the daemon died when the screen came back without $shown"
  answers || fail "the daemon stopped answering when the screen came back without $shown"
  wait_for 10 said || fail "no message on a screen whose $shown went away: $(ctl state)"
done
ctl set "$ROOT/tests/fixtures/green-64x32.png" -o TEST-V
expect "s['outputs']['TEST-V']['kind'], s['outputs']['TEST-V']['error']" "('image', None)"
ctl release TEST-V
echo "vanished wallpaper checks passed"
