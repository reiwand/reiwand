#!/usr/bin/env bash
# discard DIR...: removes the folders a test made for itself. A folder that still has something
# mounted in it stays where it is, and the removal never crosses into another file system.
discard() {
  local dir
  for dir in "$@"; do
    if [ ! -d "$dir" ] || [ -L "$dir" ]; then continue; fi
    dir=$(realpath "$dir")
    if awk -v d="$dir" '$5 == d || index($5, d "/") == 1 { found = 1 } END { exit !found }' /proc/self/mountinfo 2>/dev/null; then
      echo "leaving $dir: something is still mounted in it" >&2
      continue
    fi
    rm -rf --one-file-system "$dir" 2>/dev/null || true
  done
}
