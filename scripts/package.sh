#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
OUT=${1:-$ROOT/dist}
export REIWAND_VERSION=${REIWAND_VERSION:-$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -1)-local}
VERSION=${REIWAND_VERSION%%+*}
ARCH=$(uname -m)
NAME=reiwand-$VERSION-$ARCH
STAGE=$(mktemp -d)
trap 'rm -rf --one-file-system "$STAGE"' EXIT

BIN=${REIWAND_BIN_DIR:-$ROOT/target/release}
if [ -z "${REIWAND_BIN_DIR:-}" ]; then
  "$ROOT/scripts/build-release.sh"
else
  chmod +x "$BIN"/*
fi
export SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-$(git -C "$ROOT" log -1 --format=%ct 2>/dev/null || echo 0)}

SRC=$STAGE/src
D=$STAGE/AppDir
mkdir -p "$SRC" "$D/bin" "$D/packaging"
git -C "$ROOT" archive HEAD integrations packaging scenes scripts/plasma-assets.sh apps/reiwand-app/look apps/reiwand-app/qml/facade/Look.qml LICENSE README.md | tar -x -C "$SRC"
for bin in reiwandd reiwand-ctl reiwand-app reiwand-web; do
  install -m755 "$BIN/$bin" "$D/bin/"
  objcopy --remove-section=.comment "$D/bin/$bin"
done
python3 "$ROOT/scripts/qt-bundle.py" "${REIWAND_QT_PREFIX:?the Qt prefix made by scripts/qt-prefix.sh}" "$D"
cp -r "$SRC/integrations/plasma-wallpaper" "$D/plasma-wallpaper"
cp -r "$SRC/integrations/plasma-tray" "$D/plasma-tray"
"$SRC/scripts/plasma-assets.sh" "$D/plasma-tray"
cp -r "$SRC/integrations/kwin-pointer" "$D/kwin-pointer"
sed -i "s/\"Version\": \"[^\"]*\"/\"Version\": \"$VERSION\"/" "$D"/*/metadata.json
"$ROOT/scripts/translations.sh" build "$D"
mkdir -p "$D/examples"
for scene in "$SRC"/scenes/*/; do
  "$BIN/reiwand-ctl" pack build "$scene" -o "$D/examples/$(basename "$scene").wpak" >/dev/null
done
cp "$SRC/scenes/LICENSE" "$D/examples/"
cp -r "$SRC/packaging/systemd" "$SRC/packaging/dbus" "$SRC/packaging/desktop" "$SRC/packaging/icons" "$SRC/packaging/mime" "$D/packaging/"
install -m755 "$SRC/packaging/appimage/AppRun" "$SRC/packaging/appimage/integrate.sh" "$SRC/packaging/appimage/remove.sh" \
  "$SRC/packaging/appimage/update.sh" "$D/"
install -m644 "$SRC/packaging/appimage/reiwand-update.service" "$SRC/packaging/appimage/reiwand-update.timer" "$D/"
install -m644 "$SRC/packaging/flatpak/repo-key.asc" "$D/release-key.asc"
echo "$REIWAND_VERSION" >"$D/VERSION"
sed -e 's|@BINDIR@/reiwand-app|reiwand|' -e '/^DBusActivatable=/d' "$SRC/packaging/desktop/io.github.reiwand.desktop" >"$D/io.github.reiwand.desktop"
cp "$SRC/packaging/icons/io.github.reiwand.svg" "$D/"
cp "$SRC/packaging/icons/hicolor/256x256/apps/io.github.reiwand.png" "$D/.DirIcon"
cp "$SRC/LICENSE" "$SRC/README.md" "$SRC/packaging/THIRD-PARTY-NOTICES" "$D/"

RUNTIME=$("$ROOT/scripts/tools.sh" appimage-runtime "$STAGE/tools")
mksquashfs "$D" "$STAGE/payload" -quiet -no-progress -noappend -comp zstd -Xcompression-level 19 -b 1M -all-root -no-xattrs
mkdir -p "$OUT"
cat "$RUNTIME" "$STAGE/payload" >"$OUT/$NAME.AppImage"
chmod 755 "$OUT/$NAME.AppImage"
echo "$OUT/$NAME.AppImage"
