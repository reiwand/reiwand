#!/usr/bin/env bash
set -euo pipefail
declare daemon
python3 - "$WORK/strata" <<'PNG'
import struct, sys, zlib
def chunk(kind, data):
    return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))
def save(path, kind, row):
    with open(path, "wb") as out:
        out.write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 1024, 576, 8, kind, 0, 0, 0)))
        out.write(chunk(b"IDAT", zlib.compress((b"\0" + row) * 576)) + chunk(b"IEND", b""))
for n in range(4):
    save(f"{sys.argv[1]}-{n}.png", 2, b"".join(bytes((x % 251, (x * 3 + n * 40) % 256, 90)) for x in range(1024)))
    save(f"{sys.argv[1]}-{n}-depth.png", 0, bytes((x // (128 - n * 16)) * 40 % 256 for x in range(1024)))
PNG
ctl config playback.software_parallax true
outputs=()
shows() { read -r _ _ _ node < <(ctl stream "TEST-2$1" 160 90) && consume "$node" && outputs+=("TEST-2$1"); }
for n in 0 1 2 3; do
  ctl edit depth import "$WORK/strata-$n.png" "$WORK/strata-$n-depth.png" >/dev/null
  ctl edit parallax "$WORK/strata-$n.png" --source auto --steps 8
done
still() { field "s['outputs']['$1'].get('still')"; }
is() { [ "$(still "$1")" = "$2" ]; }
jobs_of() { ctl jobs --json | python3 -c "import json, sys; print(sum(j['kind'] == 'layers' and j['target'].endswith(sys.argv[1]) for j in json.load(sys.stdin)['jobs']))" "$1"; }
idle() { ! ctl jobs | grep -Eiq "queued|running"; }
job_of() { ctl jobs --json | python3 -c "import json, sys; print(*[j['id'] for j in json.load(sys.stdin)['jobs'] if j['target'].endswith(sys.argv[1]) and j['state'] in ('queued', 'running')])" "$1"; }
shows 0 && ctl set "$WORK/strata-0.png" -o TEST-20 --fit fill
wait_for 60 is TEST-20 None || fail "the daemon did not build the layers: $(ctl jobs)"
leave && outputs=()
find "$XDG_CACHE_HOME" -name "*.layers.png" | while read -r set; do printf "not a png" >"$set"; done
stop_daemon "$daemon" && start_daemon "$WORK/restarted.log"
shows 0 && ctl set "$WORK/strata-0.png" -o TEST-20 --fit fill
wait_for 30 is TEST-20 failed || fail "a set that cannot be read did not fail: $(ctl state)"
ctl screen TEST-20 --size 160x90 --density 2 && sleep 2
[ "$(still TEST-20)" = failed ] || fail "a set that cannot be read is not failed any more"
[ "$(jobs_of strata-0.png)" = 0 ] || fail "a set that cannot be read was built again: $(ctl jobs)"
ctl retry-layers "$WORK/strata-0.png"
wait_for 60 is TEST-20 None || fail "trying again did not build the layers: $(ctl jobs)"
[ "$(jobs_of strata-0.png)" = 1 ] || fail "trying again did not build the layers once: $(ctl jobs)"
for n in 1 2 3; do shows "$n"; done
for n in 1 2 3; do ctl set "$WORK/strata-$n.png" -o "TEST-2$n" --fit fill; done
job=$(job_of strata-3.png)
[ -n "$job" ] || fail "the last picture has no layer job: $(ctl jobs)"
ctl cancel "$job"
wait_for 30 is TEST-23 cancelled || fail "a cancelled layer job is not reported: $(ctl state)"
wait_for 120 idle || fail "the layer jobs did not end: $(ctl jobs)"
ctl screen TEST-23 --size 160x90 --density 2 && sleep 2
[ "$(jobs_of strata-3.png)" = 1 ] || fail "a cancelled layer job started again: $(ctl jobs)"
[ "$(still TEST-23)" = cancelled ] || fail "a cancelled layer job is not cancelled any more"
ctl set "$WORK/strata-3.png" -o TEST-23 --fit fill
wait_for 60 is TEST-23 None || fail "applying again did not build the layers: $(ctl jobs)"
leave
for output in "${outputs[@]}"; do ctl release "$output"; done
for n in 0 1 2 3; do ctl edit parallax "$WORK/strata-$n.png" --off; done
ctl config playback.software_parallax false
echo "depth layer state checks passed"
