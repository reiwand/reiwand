#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
DEST=$(realpath "${1:-.}")
OUT=${OUT:-$ROOT/android/app/build/outputs/apk/release}
mkdir -p "$DEST"
cp "$OUT/app-universal-release.apk" "$DEST/reiwand-android.apk"
"$ROOT/scripts/apk-check.sh" "$DEST/reiwand-android.apk"
for abi in arm64-v8a x86_64; do
  cp "$OUT/app-$abi-release.apk" "$DEST/reiwand-android-$abi.apk"
  WANT="$abi " "$ROOT/scripts/apk-check.sh" "$DEST/reiwand-android-$abi.apk"
done
