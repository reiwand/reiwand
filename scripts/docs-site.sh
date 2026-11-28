#!/usr/bin/env bash
# Builds the documentation website from docs/ into <out-dir> (needs python3 and pandoc); the work
# is in scripts/docs-site.py, the template, styles, search and link filter in packaging/docs/.
#   scripts/docs-site.sh <out-dir>
set -euo pipefail
command -v pandoc >/dev/null || { echo "docs-site: pandoc is needed" >&2; exit 1; }
exec python3 -I "$(dirname "$0")/docs-site.py" "${1:?usage: $0 <out-dir>}"
