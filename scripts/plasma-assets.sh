#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
TARGET=${1:?usage: $0 <plasmoid folder>}
LOOK=$ROOT/apps/reiwand-app/look
UI=$TARGET/contents/ui
OUT=$TARGET/contents/look

rm -rf --one-file-system "$OUT"
mkdir -p "$OUT/fonts" "$OUT/icons/regular" "$OUT/icons/fill" "$OUT/icons/bold"
cp "$LOOK"/fonts/*.ttf "$OUT/fonts/"
sed -e '/^import io.github.reiwand.app$/d' \
  -e '/readonly property Looks bridge/d' \
  -e '/onTonesChanged: bridge.paint/d' \
  -e '/^    Component.onCompleted: {$/,/^    }$/d' \
  -e '/^    id: look$/a\
\
    readonly property FontLoader textLoader: FontLoader {\
        source: Qt.resolvedUrl("fonts/Onest.ttf")\
    }\
\
    readonly property FontLoader titleLoader: FontLoader {\
        source: Qt.resolvedUrl("fonts/RedHatDisplay.ttf")\
    }' \
  "$ROOT/apps/reiwand-app/qml/facade/Look.qml" >"$OUT/Look.qml"
printf 'singleton Look 1.0 Look.qml\n' >"$OUT/qmldir"
for weight in regular fill bold; do
  for file in "$LOOK/icons/$weight"/*.svg; do
    base=$(basename "$file" .svg)
    name=$base
    [ "$weight" = regular ] || name=$base-$weight
    if grep -qF "\"$name\"" "$UI"/*.qml; then
      cp "$file" "$OUT/icons/$weight/"
    fi
  done
done
