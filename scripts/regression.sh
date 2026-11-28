#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
usage="usage: $0 [--bless] <bin-dir>"
if [ "${1:-}" = --bless ]; then
  export REIWAND_BLESS=1
  shift
fi
bin=${1:?$usage}
for icd in /usr/share/vulkan/icd.d/lvp_icd*.json; do
  [ -e "$icd" ] && export VK_DRIVER_FILES=$icd WGPU_BACKEND=vulkan
  break
done
export REIWAND_NET=1 REIWAND_REQUIRE_GPU=1 REIWAND_REQUIRE_VIDEO=1 LANG=C LC_ALL=C
cargo test --locked -p reiwand-ctl --test net
scripts/integration-idle.sh "$bin"
echo "regression net passed"
