#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")"/integration-env.sh "$@"
start_daemon "$WORK/reiwandd.log"
source "$(dirname "$0")"/integration-web.sh
stop_daemon "$daemon" "$WORK/reiwandd.log"
status=0
echo "web test passed"
