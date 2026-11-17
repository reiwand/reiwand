#!/usr/bin/env bash
set -euo pipefail

PREFIX=${PREFIX:-$HOME/.local}
CONFIG_HOME=${XDG_CONFIG_HOME:-$HOME/.config}
DATA_HOME=${XDG_DATA_HOME:-$HOME/.local/share}

if command -v systemctl >/dev/null 2>&1 && systemctl --user show-environment >/dev/null 2>&1; then
  systemctl --user disable --now reiwandd.service 2>/dev/null || true
fi
if command -v kpackagetool6 >/dev/null 2>&1; then
  kpackagetool6 --type Plasma/Wallpaper --remove io.github.reiwand 2>/dev/null || true
  kpackagetool6 --type Plasma/Applet --remove io.github.reiwand.tray 2>/dev/null || true
fi
rm -f "$PREFIX/bin/reiwandd" "$PREFIX/bin/reiwand-ctl" "$PREFIX/bin/reiwand-app" "$PREFIX/bin/reiwand-web" \
  "$CONFIG_HOME/systemd/user/reiwandd.service" \
  "$DATA_HOME/dbus-1/services/io.github.reiwand.Daemon.service" \
  "$DATA_HOME/dbus-1/services/io.github.reiwand.service" \
  "$DATA_HOME/applications/io.github.reiwand.desktop" \
  "$DATA_HOME/mime/packages/io.github.reiwand.xml" \
  "$DATA_HOME/icons/hicolor/scalable/apps/io.github.reiwand.svg" \
  "$DATA_HOME/icons/hicolor/symbolic/apps/io.github.reiwand-symbolic.svg" \
  "$DATA_HOME/icons/hicolor/symbolic/apps/io.github.reiwand-paused-symbolic.svg" \
  "$DATA_HOME"/icons/hicolor/*x*/apps/io.github.reiwand.png
rm -rf "$DATA_HOME/reiwand/kwin-pointer" "$DATA_HOME/reiwand/examples"
rm -f "$DATA_HOME/reiwand/THIRD-PARTY-NOTICES"
update-mime-database "$DATA_HOME/mime" >/dev/null 2>&1 || true
dbus-send --session --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus \
  org.freedesktop.DBus.ReloadConfig >/dev/null 2>&1 || true
if command -v systemctl >/dev/null 2>&1 && systemctl --user show-environment >/dev/null 2>&1; then
  systemctl --user daemon-reload
fi
if [ "${1:-}" = "--purge" ]; then
  rm -rf "$CONFIG_HOME/reiwand"
fi
echo "Reiwand removed."
