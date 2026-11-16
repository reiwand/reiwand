#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
shards=(
  "reiwand-render reiwand-scene reiwand-core reiwand-ktx2 reiwand-depth reiwand-video reiwand-mix reiwand-audio reiwand-script reiwand-rgb reiwand-schedule reiwand-editor reiwand-type"
  "reiwandd reiwand-ctl reiwand-web reiwand-pipewire reiwand-wayland"
  "reiwand-app reiwand-palette reiwand-android reiwand-server reiwand-qr reiwand-sync reiwand-library reiwand-format reiwand-providers reiwand-client"
)
case ${1:?usage: $0 <shard number>|check} in
  check)
    listed=$(printf '%s\n' "${shards[@]}" | tr ' ' '\n' | sort)
    workspace=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name' | sort)
    if [ "$listed" != "$workspace" ]; then
      diff <(echo "$listed") <(echo "$workspace") >&2 || true
      echo "the test shards and the workspace packages differ" >&2
      exit 1
    fi
    echo "${#shards[@]} shards cover $(echo "$workspace" | wc -l) packages"
    ;;
  [1-9])
    for package in ${shards[$(($1 - 1))]}; do printf '%s\n%s\n' -p "$package"; done
    ;;
  *) echo "unknown shard $1" >&2; exit 2 ;;
esac
