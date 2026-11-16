#!/usr/bin/env bash
set -euo pipefail
TARBALL=$(realpath "${1:?usage: $0 <release-tarball> [output-dir]}")
OUT=$(realpath -m "${2:-dist}")
ROOT=$(cd "$(dirname "$0")/.." && pwd)
VERSION=$(basename "$TARBALL" | sed -n 's/^reiwand-\(.*\)-x86_64-linux\.tar\.gz$/\1/p')
PRE=${VERSION#*-}
RELEASE=1
[ "$PRE" = "$VERSION" ] || RELEASE=0.${PRE//-/_}
TOP=$(mktemp -d)
trap 'rm -rf --one-file-system "$TOP"' EXIT
mkdir -p "$TOP/SOURCES" "$OUT"
cp "$TARBALL" "$TOP/SOURCES/"
STAMP=$(TZ=UTC tar --full-time -tvzf "$TARBALL" | awk 'NR == 1 {print $4 " " $5}')
export SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-$(TZ=UTC date -d "$STAMP" +%s)}
HOME=$TOP rpmbuild -bb --define "_topdir $TOP" --define "dist %{nil}" --define "reiwand_version $VERSION" \
  --define "reiwand_core ${VERSION%%-*}" --define "reiwand_release $RELEASE" --define "_buildhost localhost" \
  --define "use_source_date_epoch_as_buildtime 1" --define "clamp_mtime_to_source_date_epoch 1" \
  "$ROOT/packaging/rpm/reiwand.spec"
find "$TOP/RPMS" -name '*.rpm' -exec cp {} "$OUT/" \;
ls "$OUT"/*.rpm
