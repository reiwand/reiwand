#!/bin/sh
set -eu
reiwand-upgrade client
exec "$@"
