#!/usr/bin/env bash
set -euo pipefail
IMAGE=$(realpath "${1:?usage: $0 <AppImage> [output-dir]}")
OUT=$(realpath -m "${2:-dist}")
ROOT=$(cd "$(dirname "$0")/.." && pwd)
VERSION=$(basename "$IMAGE" | sed -n 's/^reiwand-\(.*\)-x86_64\.AppImage$/\1/p')
PRE=${VERSION#*-}
RELEASE=1
[ "$PRE" = "$VERSION" ] || RELEASE=0.${PRE//-/_}
TOP=$(mktemp -d)
trap 'rm -rf --one-file-system "$TOP"' EXIT
mkdir -p "$TOP/SOURCES" "$OUT"
cp "$IMAGE" "$TOP/SOURCES/"
OFFSET=$("$IMAGE" --appimage-offset)
export SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-$(unsquashfs -o "$OFFSET" -mkfs-time "$IMAGE")}
HOME=$TOP rpmbuild -bb --define "_topdir $TOP" --define "dist %{nil}" --define "reiwand_version $VERSION" \
  --define "reiwand_core ${VERSION%%-*}" --define "reiwand_release $RELEASE" --define "reiwand_offset $OFFSET" --define "_buildhost localhost" \
  --define "use_source_date_epoch_as_buildtime 1" --define "clamp_mtime_to_source_date_epoch 1" \
  "$ROOT/packaging/rpm/reiwand.spec"
find "$TOP/RPMS" -name '*.rpm' -exec cp {} "$OUT/" \;
ls "$OUT"/*.rpm
