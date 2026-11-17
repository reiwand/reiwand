# shellcheck shell=bash
mkdir -p "$WORK/out"
mkdir -p "$WORK/local" && cp "$ROOT/tests/fixtures/green-64x32.png" "$WORK/local/image.png"
cat >"$WORK/local/manifest.json" <<'LOCAL'
{"schema": "reiwand.wpak/2", "id": "org.example.hush", "version": "1.2.0", "title": "Hush", "license": "CC0-1.0",
 "kind": "image", "entry": "image.png", "origin": {"kind": "original"}, "content": {"rating": "general"},
 "photosensitive": false}
LOCAL
ctl pack build "$WORK/local" -o "$WORK/out/hush-1.2.wpak" >/dev/null
sed -i 's/"1.2.0"/"1.1.0"/' "$WORK/local/manifest.json"
ctl pack build "$WORK/local" -o "$WORK/out/hush-1.1.wpak" >/dev/null
sed -i 's/"1.1.0"/"1.3.0"/' "$WORK/local/manifest.json"
ctl pack build "$WORK/local" -o "$WORK/out/hush-1.3.wpak" >/dev/null
printf 'not a package' >"$WORK/out/broken.wpak"
ctl pkg import "$WORK/out/hush-1.2.wpak" --wait >/dev/null || fail "pkg import"
[ "$(reply "r['total']" Library1 Search suu "local:yes" 0 0)" = 1 ] || fail "the imported package is not in the Local filter"
[ "$(reply "r['total']" Library1 Search suu "local:yes kind:package" 0 0)" = 1 ] || fail "the Local filter lost its kind"
ctl pkg import "$WORK/out/hush-1.2.wpak" --wait >/dev/null || fail "importing the same version again"
[ "$(reply "r['total']" Library1 Search suu "local:yes" 0 0)" = 1 ] || fail "the same version made a second item"
if ctl pkg import "$WORK/out/broken.wpak" --wait >"$WORK/out/broken.out" 2>&1; then fail "a broken package was imported"; fi
hush=$(reply "r['items'][0]['uid']" Library1 Search suu "local:yes" 0 1)
ctl stream TEST-7 160 90 >/dev/null
ctl lib apply "$hush" -o TEST-7
expect "s['outputs']['TEST-7']['item']" "$hush"
ctl pkg import "$WORK/out/hush-1.3.wpak" --wait >/dev/null || fail "newer version"
expect "s['outputs']['TEST-7']['item'], s['outputs']['TEST-7']['error']" "('$hush', None)"
ctl release TEST-7
[ "$(reply "r['total']" Library1 Search suu "local:yes" 0 0)" = 1 ] || fail "the newer version made a second item"
if ctl pkg import "$WORK/out/hush-1.1.wpak" --wait >/dev/null 2>&1; then fail "an older version replaced a newer one"; fi
ctl pkg import "$WORK/out/hush-1.1.wpak" --allow-older --wait >/dev/null || fail "older version with the flag"
[ "$(reply "r['total']" Library1 Search suu "local:yes" 0 0)" = 1 ] || fail "the older version made a second item"
bus Library1 Remove asb 1 "$hush" true >/dev/null
[ "$(reply "r['total']" Library1 Search suu "" 0 0)" = 0 ] || fail "the imported package was not removed"
[ "$(find "$XDG_DATA_HOME/reiwand/files" -name 'hush*' 2>/dev/null | wc -l)" = 0 ] || fail "the imported copy was kept"
