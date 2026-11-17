#!/usr/bin/env bash
set -euo pipefail
declare WORK BIN green
declare -a pids
mkdir -p "$WORK/strip"
solid() {
  python3 - "$1" "$2" <<'PNG'
import struct, sys, zlib
def chunk(kind, data):
    return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))
pixels = (b"\0" + bytes.fromhex(sys.argv[2]) * 64) * 32
with open(sys.argv[1], "wb") as out:
    out.write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 64, 32, 8, 2, 0, 0, 0)))
    out.write(chunk(b"IDAT", zlib.compress(pixels)) + chunk(b"IEND", b""))
PNG
}
solid "$WORK/strip/a-red.png" ff0000
solid "$WORK/strip/b-green.png" 00ff00
solid "$WORK/strip/c-blue.png" 0000ff
solid "$WORK/strip/d-yellow.png" ffff00
ctl lib add "$WORK/strip" --wait >/dev/null
ctl lib search "" --json >"$WORK/strip-all.json"
uid() {
  python3 - "$WORK/strip-all.json" "$1" <<'PY'
import json, sys
print(next(i['uid'] for i in json.load(open(sys.argv[1]))['items'] if i['title'] == sys.argv[2]))
PY
}
red=$(uid a-red) green_item=$(uid b-green) blue_item=$(uid c-blue)
item="s['outputs']['HEADLESS-1']['item']"
ctl lib apply "$red" -o HEADLESS-1
expect "$item" "$red"
check HEADLESS-1 "0.5,0.5,#ff0000"

behind=b30000
lineup() {
  ctl browse show -o HEADLESS-1 --json >"$WORK/strip-show.json"
  python3 - "$WORK/strip-show.json" "$WORK/strip-all.json" <<'PY'
import json, sys
colors = {"a-red": "ff0000", "b-green": "00ff00", "c-blue": "0000ff", "d-yellow": "ffff00"}
items = [i for i in json.load(open(sys.argv[2]))["items"] if i["title"] in colors]
uids = [i["uid"] for i in items]
by = {i["uid"]: colors[i["title"]] for i in items}
at = uids.index(json.load(open(sys.argv[1]))["candidate"]["uid"])
print(*(by[uids[(at + d) % len(uids)]] for d in (-1, 0, 1)))
PY
}
strip_is() {
  local want; want=$(lineup)
  local left middle right
  read -r left middle right <<<"$want"
  shows HEADLESS-1 "0.22,0.46,#$left" "0.5,0.46,#$middle" "0.78,0.46,#$right" "0.02,0.15,#$behind"
}
wait_strip() { wait_for 10 strip_is || { strip_is; fail "the strip is not drawn on the wallpaper: $(lineup)"; }; }

ctl next -o HEADLESS-1 --preview --from library >/dev/null
wait_strip
expect "$item" "$red"
check HEADLESS-2 "0.5,0.5,$green"
timeout 5 grim -o HEADLESS-1 "$WORK/strip.png"
[ -z "${REIWAND_STRIP_SHOT:-}" ] || cp "$WORK/strip.png" "$REIWAND_STRIP_SHOT"
ctl next -o HEADLESS-1 --preview >/dev/null
wait_strip
ctl prev -o HEADLESS-1 --preview >/dev/null
wait_strip
expect "$item" "$red"

mkfifo "$WORK/strip-pointer.in"
"$BIN/examples/pointer" <"$WORK/strip-pointer.in" >"$WORK/strip-pointer.log" 2>&1 & pids+=($!)
exec 8>"$WORK/strip-pointer.in"
wait_for 10 grep -q ready "$WORK/strip-pointer.log" || fail "no virtual pointer for the strip"
touch_at() { printf '%s %s 1067 360\n' "$1" "$2" >&8; sleep 0.3; }
press() { printf '1 1\n1 0\n' >&8; sleep 0.5; }
spin() { printf 'wheel %s\n' "$1" >&8; sleep 0.5; }
touch_at 320 90
before=$(lineup)
spin 1
wait_for 10 test "$(lineup)" != "$before" || fail "the wheel did not move the strip"
wait_strip
spin -1
wait_for 10 test "$(lineup)" = "$before" || fail "the wheel did not move the strip back"
wait_strip
touch_at 498 166
press
wait_for 10 test "$(lineup)" != "$before" || fail "a click on a side card did not select it"
wait_strip
expect "$item" "$red"
touch_at 320 166
selected=$(lineup | cut -d' ' -f2)
press
case "$selected" in
  00ff00) expect "$item" "$green_item" ;;
  0000ff) expect "$item" "$blue_item" ;;
  *) wait_for 10 test "$(field "$item")" != "$red" || fail "a click on the middle card did not apply it" ;;
esac
touch_at 600 340
check HEADLESS-1 "0.5,0.5,#$selected" "0.02,0.15,#$selected"
[ "$(ctl browse show -o HEADLESS-1 --json | python3 -c "import json, sys; print(json.load(sys.stdin)['active'])")" = False ] || fail "the session stayed after applying"

now=$(field "$item")
behind=0000b3
ctl next -o HEADLESS-1 --preview --from library >/dev/null
wait_strip
ctl browse cancel -o HEADLESS-1 >/dev/null
expect "$item" "$now"
check HEADLESS-1 "0.5,0.5,#$selected" "0.02,0.15,#$selected" "0.22,0.46,#$selected"
exec 8>&-
echo "wallpaper strip checks passed"
