#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
OUT=${1:-$ROOT/dist}
export REIWAND_VERSION=${REIWAND_VERSION:-$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -1)-local}
VERSION=${REIWAND_VERSION%%+*}
ARCH=$(uname -m)
NAME=reiwand-$VERSION-$ARCH-linux
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
D=$STAGE/$NAME
mkdir -p "$SRC" "$D/bin" "$D/packaging"
git -C "$ROOT" archive HEAD integrations packaging scenes scripts/plasma-assets.sh apps/reiwand-app/look apps/reiwand-app/qml/facade/Look.qml LICENSE README.md | tar -x -C "$SRC"
for bin in reiwandd reiwand-ctl reiwand-app reiwand-web reiwand-upgrade; do
  install -m755 "$BIN/$bin" "$D/bin/"
  objcopy --remove-section=.comment "$D/bin/$bin"
done
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
cp -r "$SRC/packaging/systemd" "$SRC/packaging/dbus" "$SRC/packaging/desktop" "$SRC/packaging/icons" "$D/packaging/"
install -m755 "$SRC/packaging/install.sh" "$SRC/packaging/uninstall.sh" "$D/"
cp "$SRC/LICENSE" "$SRC/README.md" "$SRC/packaging/THIRD-PARTY-NOTICES" "$D/"

mkdir -p "$OUT"
tar -C "$STAGE" --sort=name --owner=0 --group=0 --numeric-owner --mtime="@$SOURCE_DATE_EPOCH" \
  --pax-option=exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime -cf - "$NAME" | gzip -n -9 > "$OUT/$NAME.tar.gz"
echo "$OUT/$NAME.tar.gz"
