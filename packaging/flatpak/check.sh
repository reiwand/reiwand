#!/bin/sh
set -eu
bundle=${1:?usage: REIWAND_VERSION=<version> $0 <bundle>}
version=${REIWAND_VERSION:?}
app=io.github.reiwand.Reiwand
flatpak install -y --noninteractive --bundle "$bundle"
run() { cmd=$1 && shift && flatpak run --env=QT_QPA_PLATFORM=offscreen --env=REIWAND_APP_CHECK_SHARDS="${SHARDS:-3}" --command="$cmd" "$@"; }
test "$(run reiwand-ctl $app --version)" = "reiwand-ctl $version"
test "$(run reiwandd $app --version)" = "reiwandd $version"
run reiwand-app $app --check
if said=$(run reiwand-web $app --check 2>&1); then echo "the runtime unexpectedly carries WPE WebKit"; exit 1; fi
case $said in *libWPEWebKit*) echo "web wallpapers refused here, as expected" ;; *) echo "reiwand-web --check failed for another reason: $said"; exit 1 ;; esac
files=$(flatpak info -l $app)/files
exports=${files%%/app/*}/exports/share
run reiwand-plasma $app > plasma.sh
sh -n plasma.sh
for p in wallpapers/io.github.reiwand plasmoids/io.github.reiwand.tray; do
  grep -qF "\"Version\": \"${version%%+*}\"" "$files/share/plasma/$p/metadata.json"
  grep -F -e "--install '$files/share/plasma/$p'" plasma.sh
done
grep -e "^mkdir " -e "^chmod " plasma.sh | sh
test "$(stat -c %a "${XDG_DATA_HOME:-$HOME/.local/share}/reiwand/edits")" = 700
dir=$(mktemp -d "$HOME/.reiwand-check-XXXXXX")
cp "$(dirname "$0")/../../scenes/pond/preview.jpg" "$dir/"
flatpak run --filesystem="$dir:ro" --command=reiwand-ctl $app edit set "$dir/preview.jpg" ev=0.5
grep -l '"ev"' "${XDG_DATA_HOME:-$HOME/.local/share}"/reiwand/edits/*.json
test ! -e "$dir/preview.jpg.reiwand.json"
rm -r "$dir"
test -f "$files/share/reiwand/kwin-pointer/contents/code/main.js"
grep -qF "\"Version\": \"${version%%+*}\"" "$files/share/reiwand/kwin-pointer/metadata.json"
for name in .Daemon:reiwandd :reiwand-app; do
  grep -x "Exec=.* --command=/app/bin/${name#*:} $app" "$exports/dbus-1/services/io.github.reiwand${name%%:*}.service"
done
desktop=$exports/applications/$app.desktop
grep -x "Exec=.* --command=/app/bin/reiwand-app --file-forwarding $app @@u %U @@" "$desktop"
grep -x "Icon=$app" "$desktop"
if grep -q '^DBusActivatable=' "$desktop"; then exit 1; fi
test -f "$exports/icons/hicolor/scalable/apps/$app.svg"
grep -qF "content_attribute id=\"sex-nudity\"" "$files/share/metainfo/$app.metainfo.xml"
test -s "$files/share/licenses/$app/THIRD-PARTY-NOTICES"
grep -qF "<release version=\"${version%%+*}\"" "$files/share/metainfo/$app.metainfo.xml"
ls "$files/bin/reiwandd" "$files/bin/reiwand-ctl" "$files/bin/reiwand-app" "$files/bin/reiwand-web"
if grep -l -E '/run/build|/(home|root)/' "$files"/bin/reiwand*; then exit 1; fi
