#!/usr/bin/env bash
set -euo pipefail
IMAGE=$(realpath "${1:?usage: REIWAND_VERSION=<version> $0 <AppImage>}")
VERSION=${REIWAND_VERSION:?}
WORK=$(mktemp -d)
trap 'echo "FAIL: $(basename "$0") line $LINENO: $BASH_COMMAND" >&2' ERR
trap 'rm -rf --one-file-system "$WORK"' EXIT

offset=$("$IMAGE" --appimage-offset)
test "$(unsquashfs -o "$offset" -mkfs-time "$IMAGE")" = "${SOURCE_DATE_EPOCH:-$(git log -1 --format=%ct)}"
unsquashfs -o "$offset" -lln "$IMAGE" | awk '$2 != "0/0" {print; bad = 1} END {exit bad}'
(cd "$WORK" && "$IMAGE" --appimage-extract >/dev/null)
app=$WORK/squashfs-root
test "$("$app/AppRun" --version)" = "reiwand $VERSION"
test "$("$app/AppRun" reiwand-ctl --version)" = "reiwand-ctl $VERSION"
test -x "$app/update.sh" && test -f "$app/reiwand-update.timer"
cmp "$app/release-key.asc" "$(dirname "$0")/../packaging/flatpak/repo-key.asc"
for p in plasma-wallpaper plasma-tray kwin-pointer; do
  grep -qF "\"Version\": \"${VERSION%%+*}\"" "$app/$p/metadata.json"
done
for bin in bin/reiwand-app bin/reiwand-web qt/lib/qt6/QtWebEngineProcess; do
  core=$(ldd "$app/$bin" | awk '$1 == "libQt6Core.so.6" { print $3 }')
  test "$(realpath "$core")" = "$app/qt/lib/libQt6Core.so.6"
done
bins=("$app"/bin/reiwand*)
if strings "${bins[@]}" | grep -F -e "$HOME" -e "${GITHUB_WORKSPACE:-$PWD}" -e "$(id -un)@"; then exit 1; fi
if strings "${bins[@]}" | grep -E '/(home|root)/'; then exit 1; fi
if readelf -p .comment "${bins[@]}" 2>/dev/null | grep -q '\['; then exit 1; fi
echo "$IMAGE carries version $VERSION and no build machine data"
