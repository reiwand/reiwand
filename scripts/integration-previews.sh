#!/usr/bin/env bash
set -euo pipefail
snow=$(example snowfall)
sound=$WORK/sound.webm
ctl unmute && ctl volume 100
hash_of() { "$BIN/probe" "$1" 3 | sed -n 's/.* hash \([0-9a-f]*\)$/\1/p'; }
framed() { "$BIN/probe" "$1" 3 | grep -q "frame 160x90"; }
moves() { local a b; a=$(hash_of "$1") && sleep 0.5 && b=$(hash_of "$1") && [ -n "$a" ] && [ "$a" != "$b" ]; }
read -r _ _ _ card < <(ctl stream preview:card-live 160 90)
for round in 1 2 3; do
  bus Library1 Apply ssss "$snow" preview:card-live "" "" >/dev/null
  retry 20 framed "$card" || fail "round $round: a library card of a scene with sound shows no frames: $("$BIN/probe" "$card" 3 2>&1)"
  retry 20 moves "$card" || fail "round $round: the card of a scene with sound stands still"
  ctl set "$sound" -o preview:card-live
  retry 20 framed "$card" || fail "round $round: a library card of a video shows no frames: $("$BIN/probe" "$card" 3 2>&1)"
done
ctl release preview:card-live
for n in 1 2 3 4 5 6; do
  ctl stream "preview:card-$n" 160 90 >/dev/null
  ctl set "$sound" -o "preview:card-$n"
  sleep 0.3
  ctl release "preview:card-$n"
done
read -r _ _ _ screen < <(ctl stream TEST-17 160 90)
ctl set "$sound" -o TEST-17 && consume "$screen"
retry 20 framed "$screen" || fail "the video with sound shows no frames: $("$BIN/probe" "$screen" 3 2>&1)"
expect "s['outputs']['TEST-17']['playing']" True
ctl lib apply "$snow" -o TEST-17
shows() { [ "$(field "$1")" = "$2" ]; }
wait_for 3 shows "s['outputs']['TEST-17']['item']" "$snow" || fail "applying a scene over a playing video did not take effect within 3 s: $(ctl state)"
retry 6 moves "$screen" || fail "the scene applied over a playing video does not draw"
ctl set "$sound" -o TEST-17
wait_for 3 shows "s['outputs']['TEST-17']['image'].endswith('sound.webm')" True ||
  fail "applying a video over a playing scene did not take effect within 3 s: $(ctl state)"
retry 20 framed "$screen" || fail "the video applied over a scene shows no frames"
ctl set "$ROOT/tests/fixtures/green-64x32.png" -o TEST-17
wait_for 3 shows "s['outputs']['TEST-17']['image'].endswith('green-64x32.png')" True ||
  fail "applying a picture over a playing video did not take effect within 3 s: $(ctl state)"
ctl mute
ctl release TEST-17 && expect "list(s['outputs'])" "[]"
echo "library card previews and replacing playing wallpapers checks passed"
