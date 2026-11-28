#!/usr/bin/env bash
# reiwand --update [--check | --auto]: looks for a newer release on GitHub, checks the signature of
# its SHA256SUMS with the release key carried in this AppImage and the AppImage's checksum against
# it, and installs that version over this one. Never installs an older or equal version. With
# private mode on it contacts nothing (`reiwand-ctl update allowed` reads the configuration).
#   --check  only says whether a newer version exists
#   --auto   what the daily timer runs: nothing while its setting is off (`reiwand-ctl update auto
#            off`), and waits while on battery or on a metered connection
set -euo pipefail

HERE=$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")
VERSION=$(cat "$HERE/VERSION")
VERSION=${VERSION%%+*}
RELEASES=${REIWAND_RELEASES:-https://github.com/reiwand/reiwand/releases}
DATA_HOME=${XDG_DATA_HOME:-$HOME/.local/share}
FILE=reiwand-x86_64.AppImage
SEMVER='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'
MODE=install
case ${1:-} in
  "") ;;
  --check) MODE=check ;;
  --auto) MODE=auto ;;
  *) echo "usage: reiwand --update [--check | --auto]" >&2 && exit 2 ;;
esac
fail() { echo "reiwand --update: $*" >&2; exit 1; }

command -v curl >/dev/null 2>&1 || fail "curl is needed to download the new version"
command -v gpgv >/dev/null 2>&1 || fail "gpgv (GnuPG) is needed to check the signature of the new version"

allowed=(update allowed)
[ "$MODE" != auto ] || allowed+=(--auto)
if ! why=$("$HERE/bin/reiwand-ctl" "${allowed[@]}" 2>&1); then
  # The timer ends quietly when stopped on purpose (private mode or the setting); asked by hand, it fails.
  [ "$MODE" = auto ] || fail "${why#Error: }"
  echo "${why#Error: }"
  exit 0
fi

if [ "$MODE" = auto ]; then
  if command -v systemd-ac-power >/dev/null 2>&1 && ! systemd-ac-power; then
    echo "On battery: the update waits for the next check"
    exit 0
  fi
  metered=$(busctl --system get-property org.freedesktop.NetworkManager /org/freedesktop/NetworkManager \
    org.freedesktop.NetworkManager Metered 2>/dev/null || true)
  if [ "$metered" = "u 1" ] || [ "$metered" = "u 3" ]; then
    echo "On a metered connection: the update waits for the next check"
    exit 0
  fi
fi

latest=$(curl -fsSLI --retry 2 -o /dev/null -w '%{url_effective}' "$RELEASES/latest") ||
  fail "could not reach $RELEASES"
tag=${latest##*/}
NEW=${tag#v}
[[ $tag == v* && $NEW =~ $SEMVER ]] || fail "the latest release has no version name: $latest"
if [ "$(printf '%s\n' "$VERSION" "$NEW" | sort -V | tail -1)" = "$VERSION" ]; then
  echo "Reiwand $VERSION is up to date"
  exit 0
fi
if [ "$MODE" = check ]; then
  echo "Reiwand $NEW is available (this is $VERSION): reiwand --update installs it"
  exit 0
fi

mkdir -p "${XDG_CACHE_HOME:-$HOME/.cache}"
T=$(mktemp -d "${XDG_CACHE_HOME:-$HOME/.cache}/reiwand-update.XXXXXX")
trap 'rm -rf --one-file-system "$T"' EXIT
echo "Downloading Reiwand $NEW"
for name in SHA256SUMS SHA256SUMS.asc "$FILE"; do
  curl -fsSL --retry 2 -o "$T/$name" "$RELEASES/download/$tag/$name" || fail "could not download $name of $tag"
done
# gpgv reads only binary keyrings: the carried key is armored, so take its base64 body.
sed '1,/^$/d; /^=/d; /^-----END/d' "$HERE/release-key.asc" | base64 -d >"$T/release-key.gpg"
gpgv --keyring "$T/release-key.gpg" "$T/SHA256SUMS.asc" "$T/SHA256SUMS" 2>/dev/null ||
  fail "the signature of SHA256SUMS of $tag is not the Reiwand release key: nothing installed"
grep -E "^[0-9a-f]{64}  $FILE\$" "$T/SHA256SUMS" >"$T/sum" || fail "SHA256SUMS of $tag does not list $FILE"
(cd "$T" && sha256sum -c --status sum) || fail "$FILE of $tag does not match its signed checksum: nothing installed"
chmod 755 "$T/$FILE"
# Extracting instead of mounting: no FUSE needed, also from the timer.
got=$(APPIMAGE_EXTRACT_AND_RUN=1 "$T/$FILE" --version)
[ "${got%%+*}" = "reiwand $NEW" ] || fail "$FILE of $tag says it is '$got', not $NEW: nothing installed"

args=()
[ -d "$DATA_HOME/reiwand/examples" ] || args+=(--no-examples)
APPIMAGE_EXTRACT_AND_RUN=1 "$T/$FILE" --install "${args[@]}"
said="Reiwand was updated from $VERSION to $NEW."
# Plasma keeps the tray icon and the wallpaper plugin it already loaded until it starts again.
if command -v kpackagetool6 >/dev/null 2>&1 && pgrep -u "$(id -u)" -x plasmashell >/dev/null 2>&1; then
  said="$said Restart Plasma (or sign out and in) so the tray icon and the wallpaper plugin use it."
fi
if [ "$MODE" = auto ] && command -v notify-send >/dev/null 2>&1; then
  notify-send --app-name=Reiwand --icon=io.github.reiwand "Reiwand $NEW" "$said" 2>/dev/null || true
fi
