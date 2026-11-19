#!/bin/bash
set -euo pipefail
cache=$HOME/.cache/ci-apt
mkdir -p "$cache/partial"
sudo chown _apt:root "$cache/partial"
sudo rm -f /var/lib/man-db/auto-update
printf 'Acquire::Retries "5";\nAcquire::http::Timeout "30";\nAcquire::https::Timeout "30";\n' |
  sudo tee /etc/apt/apt.conf.d/80ci >/dev/null
# a mirror that trickles never trips the timeouts above: give each call fifteen minutes and try it again; a call cut
# while dpkg was configuring leaves it half done, so dpkg finishes that before the next try
try() {
  local n
  for n in 1 2 3; do
    timeout 900 "$@" && return 0
    echo "::warning::$1 $2 $3 did not finish (try $n of 3)"
    sudo dpkg --configure -a || true
    sleep 10
  done
  return 1
}
try sudo apt-get update
try sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  -o Dir::Cache::Archives="$cache" -o APT::Keep-Downloaded-Packages=true "$@"
sudo rm -rf --one-file-system "$cache/partial" "$cache/lock"
sudo chown -R "$(id -u):$(id -g)" "$cache"
