#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if compgen -G "/dev/dri/renderD*" >/dev/null; then
  echo "render node present: page tests use it when the helper accepts it as a hardware GPU"
else
  echo "no render node on this runner: page tests run without GPU, through the software path"
fi
export QMAKE=${QMAKE:-/usr/bin/qmake6} CARGO_TARGET_DIR=$PWD/target-web
cargo build --locked -p reiwand-web
page=$(cargo test --locked -p reiwand-web --test page --no-run --message-format=json |
  jq -r 'select(.executable != null and .target.name == "page") | .executable')
cp target-web/debug/reiwand-web target/debug/reiwand-web
cp "$page" target/debug/web-page
export REIWAND_REQUIRE_WEB=1
xvfb-run -a sh -c 'target/debug/web-page --test-threads 1 && scripts/web-test.sh target/debug && REIWAND_REQUIRE_PIPEWIRE_SOUND=1 scripts/video-test.sh target/debug'
