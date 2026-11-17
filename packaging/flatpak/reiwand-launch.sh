#!/bin/sh
# Runs the one-time data migration, then the program it was given (the app when given nothing).
set -eu
reiwand-upgrade client
[ "$#" -gt 0 ] || set -- reiwand-app
exec "$@"
