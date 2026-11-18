#!/usr/bin/env bash
complete() {
  local target dir
  for target in "$@"; do
    if [ -d "$target" ]; then
      "$BIN/reiwand-upgrade" scenes "$target" >/dev/null
      for dir in "$target"/*.wpak; do [ -e "$dir" ] && complete "$dir"; done
    else
      dir=$(mktemp -d "$WORK/complete.XXXXXX")
      python3 -c "import sys, zipfile
with zipfile.ZipFile(sys.argv[1]) as z: z.extractall(sys.argv[2])" "$target" "$dir"
      "$BIN/reiwand-upgrade" scenes "$dir" >/dev/null
      python3 -c "import sys, zipfile
src = zipfile.ZipFile(sys.argv[1])
items = [(i.filename, i.compress_type) for i in src.infolist()]
src.close()
with zipfile.ZipFile(sys.argv[1], 'w') as z:
    for name, kind in items:
        z.write(sys.argv[2] + '/' + name, name, kind)" "$target" "$dir"
      discard "$dir"
    fi
  done
}
