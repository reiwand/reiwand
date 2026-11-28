#!/usr/bin/env bash
set -euo pipefail

IMAGE=$(realpath "${1:?usage: $0 <AppImage> <output-dir>}")
OUT=$(mkdir -p "${2:?usage: $0 <AppImage> <output-dir>}" && realpath "$2")
ROOT=$(cd "$(dirname "$0")/.." && pwd)
TEST_USER=plasmatest

id "$TEST_USER" >/dev/null 2>&1 || useradd -m "$TEST_USER"
HOME_DIR=$(getent passwd "$TEST_USER" | cut -d: -f6)
WORK=$HOME_DIR/test
source "$ROOT/scripts/discard.sh"
discard "$WORK"
mkdir -p "$WORK"
install -m755 "$IMAGE" "$WORK/reiwand.AppImage"
cp "$ROOT/tests/fixtures/green-64x32.png" "$WORK/green.png"
cp "$ROOT/tests/fixtures/magenta-64x32.webm" "$WORK/magenta.webm"
cp "$ROOT/tests/tray-row.qml" "$ROOT/tests/tray-left-out.qml" "$ROOT/tests/tray-consent.qml" "$WORK/"
mkdir -p "$WORK/app" && cp "$ROOT/apps/reiwand-app/qml/Thumbnail.qml" "$WORK/app/"
cp "$ROOT"/scripts/plasma-{session,config,checks,screens,features,lock}.sh "$WORK/"
chown -R "$TEST_USER:" "$WORK"
chmod 777 "$OUT"
chmod 666 /dev/dri/* 2>/dev/null || true

if command -v setcap >/dev/null 2>&1; then
  setcap -r "$(command -v kwin_wayland)" 2>/dev/null || true
fi

status=0
su -l "$TEST_USER" -s /bin/bash -c "bash '$WORK/plasma-session.sh' '$WORK' '$OUT'" || status=$?
exit "$status"
