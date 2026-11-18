#!/usr/bin/env bash
complete() {
  local target dir
  for target in "$@"; do
    if [ -d "$target" ]; then
      "$BIN/reiwand-upgrade" scenes "$target" >/dev/null
      for dir in "$target"/*.wpak; do if [ -e "$dir" ]; then complete "$dir"; fi; done
    else
      dir=$(mktemp -d "$WORK/complete.XXXXXX")
      python3 -c "import sys, zipfile
with zipfile.ZipFile(sys.argv[1]) as z: z.extractall(sys.argv[2])" "$target" "$dir"
      "$BIN/reiwand-upgrade" scenes "$dir" >/dev/null
      grep -rl --include='*.json' --exclude=scene.json --exclude=manifest.json '"stage"' "$dir" |
        xargs -r "$BIN/reiwand-upgrade" effects >/dev/null || true
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
on_copy() { local verb=$1 copy; copy=$(mktemp -d "$WORK/look.XXXXXX"); cp -r "$2/." "$copy"; complete "$copy"; shift 2; ctl scene "$verb" "$copy" "$@"; }
looks_at() { on_copy preview "$@"; }
render_of() { on_copy render "$@"; }
