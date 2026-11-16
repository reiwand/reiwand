#!/usr/bin/env bash
set -euo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
PREFIX=${PREFIX:-$HOME/.local}
BINDIR=$PREFIX/bin
CONFIG_HOME=${XDG_CONFIG_HOME:-$HOME/.config}
DATA_HOME=${XDG_DATA_HOME:-$HOME/.local/share}
UNIT_DIR=$CONFIG_HOME/systemd/user
DBUS_DIR=$DATA_HOME/dbus-1/services
APP_ID=io.github.reiwand
EXAMPLES=1
reload_bus() {
  dbus-send --session --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus \
    org.freedesktop.DBus.ReloadConfig >/dev/null 2>&1 ||
    busctl --user call org.freedesktop.DBus /org/freedesktop/DBus org.freedesktop.DBus ReloadConfig \
      >/dev/null 2>&1 ||
    echo "the session bus learns about Reiwand at the next login"
}
for arg in "$@"; do
  case $arg in
    --no-examples) EXAMPLES=0 ;;
    *) echo "usage: $0 [--no-examples]" >&2 && exit 2 ;;
  esac
done

gst=$(ldd "$HERE/bin/reiwandd" 2>/dev/null | awk '$1 == "libgstreamer-1.0.so.0" { print $3 }' || true)
gst=$(readlink -f "$gst" 2>/dev/null || true) && gst=${gst##*/libgstreamer-1.0.so.0.} && gst=${gst%%.*}
[[ $gst =~ ^[0-9]+$ ]] && ((gst >= 2400)) ||
  echo "WARNING: GStreamer 1.24 or newer was not found; videos and the sound of scenes need it" >&2

qtcore=$(ldd "$HERE/bin/reiwand-app" 2>/dev/null | awk '$1 == "libQt6Core.so.6" { print $3 }' || true)
qtcore=$(readlink -f "$qtcore" 2>/dev/null || true)
if [[ -n $qtcore && ! -e ${qtcore%/*}/qt6/plugins/platformthemes/libqxdgdesktopportal.so ]]; then
  echo "WARNING: Qt's desktop portal theme was not found (on Debian and Ubuntu: qt6-xdgdesktopportal-platformtheme); without it the app draws its own file chooser instead of the desktop's" >&2
fi

if ! run=$("$HERE/bin/reiwandd" --version 2>&1) && [[ $run == *GLIBC* ]]; then
  echo "WARNING: the C library of this system is older than the one these binaries need (glibc 2.39 or newer)" >&2
fi

echo "Installing binaries to $BINDIR"
install -Dm755 "$HERE/bin/reiwandd" "$BINDIR/reiwandd"
install -Dm755 "$HERE/bin/reiwand-ctl" "$BINDIR/reiwand-ctl"
install -Dm755 "$HERE/bin/reiwand-app" "$BINDIR/reiwand-app"
install -Dm755 "$HERE/bin/reiwand-web" "$BINDIR/reiwand-web"
install -Dm755 "$HERE/bin/reiwand-upgrade" "$BINDIR/reiwand-upgrade"

echo "Installing the user service"
mkdir -p "$UNIT_DIR" "$DBUS_DIR"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/systemd/reiwandd.service" >"$UNIT_DIR/reiwandd.service"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/dbus/io.github.reiwand.Daemon.service" \
  >"$DBUS_DIR/io.github.reiwand.Daemon.service"

echo "Installing the app"
mkdir -p "$DATA_HOME/applications"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/dbus/$APP_ID.service" >"$DBUS_DIR/$APP_ID.service"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/desktop/$APP_ID.desktop" >"$DATA_HOME/applications/$APP_ID.desktop"
install -Dm644 "$HERE/packaging/icons/$APP_ID.svg" "$DATA_HOME/icons/hicolor/scalable/apps/$APP_ID.svg"
for name in "$APP_ID-symbolic" "$APP_ID-paused-symbolic"; do
  install -Dm644 "$HERE/packaging/icons/$name.svg" "$DATA_HOME/icons/hicolor/symbolic/apps/$name.svg"
done
for png in "$HERE"/packaging/icons/hicolor/*/apps/"$APP_ID.png"; do
  size=$(basename "$(dirname "$(dirname "$png")")")
  install -Dm644 "$png" "$DATA_HOME/icons/hicolor/$size/apps/$APP_ID.png"
done
gtk-update-icon-cache -q -t "$DATA_HOME/icons/hicolor" 2>/dev/null || true
reload_bus

echo "Installing the KWin pointer script, loaded only while a scene follows the pointer"
install -d -m700 "$DATA_HOME/reiwand"
rm -rf "$DATA_HOME/reiwand/kwin-pointer"
cp -r "$HERE/kwin-pointer" "$DATA_HOME/reiwand/kwin-pointer"
install -m644 "$HERE/THIRD-PARTY-NOTICES" "$DATA_HOME/reiwand/THIRD-PARTY-NOTICES"

rm -rf "$DATA_HOME/reiwand/examples"
if [ "$EXAMPLES" = 1 ] && [ -d "$HERE/examples" ]; then
  echo "Installing the example scenes; remove them from the library, or install with --no-examples"
  install -d -m700 "$DATA_HOME/reiwand/examples"
  install -m644 "$HERE"/examples/* "$DATA_HOME/reiwand/examples/"
fi

install_package() {
  if kpackagetool6 --type "$1" --show "$2" >/dev/null 2>&1; then
    kpackagetool6 --type "$1" --upgrade "$3"
    plugin_upgraded=1
  else
    kpackagetool6 --type "$1" --install "$3"
  fi
}

if command -v kpackagetool6 >/dev/null 2>&1; then
  echo "Installing the Plasma wallpaper plugin and tray icon"
  install_package Plasma/Wallpaper io.github.reiwand "$HERE/plasma-wallpaper"
  install_package Plasma/Applet io.github.reiwand.tray "$HERE/plasma-tray"
else
  echo "kpackagetool6 not found: skipping the Plasma plugin and tray icon"
fi

"$BINDIR/reiwand-upgrade" client

if command -v systemctl >/dev/null 2>&1 && systemctl --user show-environment >/dev/null 2>&1; then
  systemctl --user daemon-reload
  systemctl --user enable reiwandd.service
  systemctl --user restart reiwandd.service
  echo "reiwandd is running: systemctl --user status reiwandd"
else
  echo "systemd user session not available: start $BINDIR/reiwandd manually"
fi

if [ "${plugin_upgraded:-0}" = 1 ]; then
  cat <<MSG

The Plasma plugins were upgraded. Plasma keeps the old versions in memory until
it restarts: log out and back in, or run
  systemctl --user restart plasma-plasmashell
MSG
fi

cat <<MSG

Done. In Plasma: right-click the desktop, "Configure Desktop and Wallpaper" and
choose "Reiwand" as the wallpaper type. Manage it with the Reiwand app,
from the Reiwand icon in the system tray (after restarting Plasma) or from a
terminal:
  reiwand-ctl set /path/to/image-or-video
  reiwand-ctl --help
MSG
