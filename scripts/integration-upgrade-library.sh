#!/usr/bin/env bash
declare WORK BIN data
declare -a env pids
six=$WORK/six-folder
mkdir -p "$six"
fixture=$(dirname "$0")/../tests/fixtures/green-64x32.png
python3 - "$six/six.wpak" "$fixture" <<'PACKAGE'
import json, sys, zipfile
manifest = {"schema": "reiwand.wpak/2", "id": "org.example.six", "version": "1.0.0", "title": "six", "license": "CC0-1.0",
            "kind": "image", "entry": "image.png", "photosensitive": False, "content": {"rating": "general"},
            "origin": {"kind": "original"}, "previews": {"thumbnail": "preview.png"}, "requires": {"features": []}}
with zipfile.ZipFile(sys.argv[1], "w") as out:
    out.writestr("manifest.json", json.dumps(manifest))
    out.write(sys.argv[2], "preview.png")
    out.write(sys.argv[2], "image.png")
PACKAGE
daemon=${pids[-1]}
x old lib add "$six" --wait >/dev/null || fail "the folder was not added"
kill "$daemon"
wait_for 20 daemon_exited "$daemon" || fail "the daemon did not stop"
env "${env[@]}" "$BIN/reiwand-upgrade" client --check --skip-download --data "$data" >"$WORK/check.out" 2>&1 || fail "check: $(cat "$WORK/check.out")"
grep -q "six.wpak" "$WORK/check.out" || fail "the check did not name the package: $(cat "$WORK/check.out")"
unzip -p "$six/six.wpak" manifest.json | grep -q 'wpak/2' || fail "the check wrote"
env "${env[@]}" "$BIN/reiwand-upgrade" client --skip-download --data "$data" >"$WORK/upgrade-library.out" 2>&1 || fail "upgrade: $(cat "$WORK/upgrade-library.out")"
unzip -p "$six/six.wpak" manifest.json | grep -q 'wpak/3' || fail "the package was not converted"
acct_start old
x old lib search --json | jq_ "' '.join(i['title'] for i in r['items'])" | grep -q six || fail "the converted package is not listed: $(x old lib search)"
