#!/usr/bin/env bash
set -euo pipefail
image=${1:?usage: ARCH=<base image> $0 <image name>}
base=${ARCH:?ARCH is not set}
# The packages of the day of the base image (its tag base-YYYYMMDD…), from the Arch archive: the
# same set every run instead of whatever the mirrors hold that day.
day=$(sed -n 's/.*:base-\([0-9]\{4\}\)\([0-9]\{2\}\)\([0-9]\{2\}\).*/\1\/\2\/\3/p' <<<"$base")
[ -n "$day" ] || { echo "plasma-image: $base carries no base-YYYYMMDD tag" >&2; exit 1; }
docker run --name prepare --device /dev/dri -e DAY="$day" "$base" bash -c '
  set -e
  echo "Server = https://archive.archlinux.org/repos/$DAY/\$repo/os/\$arch" > /etc/pacman.d/mirrorlist
  install() {
    pacman -Sy --noconfirm --needed archlinux-keyring >/dev/null
    pacman -Syu --noconfirm --needed \
      plasma-desktop plasma-workspace kwin kpipewire spectacle \
      pipewire wireplumber dbus gst-plugins-base gst-plugins-good \
      mesa vulkan-swrast vulkan-icd-loader \
      imagemagick tar gzip libcap gdb jq qt6-declarative kirigami qqc2-desktop-style >/dev/null
  }
  for attempt in 1 2 3 4; do
    install && break
    [ "$attempt" -lt 4 ] || exit 1
    sleep $((attempt * 20))
  done
  pacman -Q plasma-desktop plasma-workspace kwin kpipewire mesa kirigami
'
docker commit prepare "$image" >/dev/null
docker rm prepare >/dev/null
