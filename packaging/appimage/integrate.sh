#!/usr/bin/env bash
set -euo pipefail

HERE=$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")
VERSION=$(cat "$HERE/VERSION")
VERSION=${VERSION%%+*}
PREFIX=${PREFIX:-$HOME/.local}
BINDIR=$PREFIX/bin
LIB=$PREFIX/lib/reiwand
DEST=$LIB/$VERSION
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
    *) echo "usage: reiwand --install [--no-examples]" >&2 && exit 2 ;;
  esac
done

gst=$(ldd "$HERE/bin/reiwandd" 2>/dev/null | awk '$1 == "libgstreamer-1.0.so.0" { print $3 }' || true)
gst=$(readlink -f "$gst" 2>/dev/null || true) && gst=${gst##*/libgstreamer-1.0.so.0.} && gst=${gst%%.*}
[[ $gst =~ ^[0-9]+$ ]] && ((gst >= 2400)) ||
  echo "WARNING: GStreamer 1.24 or newer was not found; videos and the sound of scenes need it" >&2

missing=$(ldd "$HERE/bin/reiwand-app" "$HERE/bin/reiwand-web" 2>/dev/null | awk '$3 == "not" { print $1 }' | sort -u | paste -sd' ' || true)
[ -z "$missing" ] ||
  echo "WARNING: the app and the web wallpapers need these system libraries, which were not found: $missing" >&2

if ! run=$("$HERE/bin/reiwandd" --version 2>&1) && [[ $run == *GLIBC* ]]; then
  echo "WARNING: the C library of this system is older than the one these binaries need (glibc 2.39 or newer)" >&2
fi

echo "Installing Reiwand $VERSION to $DEST"
if [ "$(readlink -f "$DEST")" != "$HERE" ]; then
  mkdir -p "$LIB"
  rm -rf "$DEST.new"
  cp -R "$HERE" "$DEST.new"
  rm -rf "$DEST"
  mv "$DEST.new" "$DEST"
fi
ln -sfn "$VERSION" "$LIB/current.new"
mv -T "$LIB/current.new" "$LIB/current"
for old in "$LIB"/*; do
  [ "$old" = "$DEST" ] || [ "$old" = "$LIB/current" ] || rm -rf "$old"
done
HERE=$LIB/current
mkdir -p "$BINDIR"
for bin in reiwandd reiwand-ctl reiwand-app reiwand-web; do
  ln -sfn "$HERE/bin/$bin" "$BINDIR/$bin"
done
ln -sfn "$HERE/AppRun" "$BINDIR/reiwand"

echo "Installing the user service and the daily update check"
mkdir -p "$UNIT_DIR" "$DBUS_DIR"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/systemd/reiwandd.service" >"$UNIT_DIR/reiwandd.service"
# The check is on from the first install; one switched off stays off across upgrades.
[ -e "$UNIT_DIR/reiwand-update.timer" ] || first_update=1
sed "s|@BINDIR@|$BINDIR|g" "$HERE/reiwand-update.service" >"$UNIT_DIR/reiwand-update.service"
install -m644 "$HERE/reiwand-update.timer" "$UNIT_DIR/reiwand-update.timer"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/dbus/io.github.reiwand.Daemon.service" \
  >"$DBUS_DIR/io.github.reiwand.Daemon.service"

echo "Installing the app"
mkdir -p "$DATA_HOME/applications"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/dbus/$APP_ID.service" >"$DBUS_DIR/$APP_ID.service"
sed "s|@BINDIR@|$BINDIR|g" "$HERE/packaging/desktop/$APP_ID.desktop" >"$DATA_HOME/applications/$APP_ID.desktop"
install -Dm644 "$HERE/packaging/mime/$APP_ID.xml" "$DATA_HOME/mime/packages/$APP_ID.xml"
update-mime-database "$DATA_HOME/mime" >/dev/null 2>&1 || true
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

if command -v systemctl >/dev/null 2>&1 && systemctl --user show-environment >/dev/null 2>&1; then
  systemctl --user daemon-reload
  systemctl --user enable reiwandd.service
  systemctl --user restart reiwandd.service
  echo "reiwandd is running: systemctl --user status reiwandd"
  [ "${first_update:-0}" = 0 ] || systemctl --user enable --now reiwand-update.timer
  [ "${first_update:-0}" = 0 ] ||
    echo "Once a day Reiwand asks GitHub (github.com) for its latest release and installs it after checking its signature; nothing else is sent. To stop it: reiwand-ctl update auto off, or Settings › Advanced › Version in the app. Private mode stops it too."
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
To uninstall: reiwand --uninstall
MSG
