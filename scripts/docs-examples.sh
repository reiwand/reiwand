#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
ctl=${1:-target/debug}/reiwand-ctl
count=0
failed=0
for dir in docs/authoring/examples/*/; do
  dir=${dir%/}
  count=$((count + 1))
  if [ -f "$dir/reiwand-provider.json" ]; then
    continue
  fi
  if [ ! -f "$dir/scene.json" ] && [ -f "$dir/manifest.json" ]; then
    if ! "$ctl" pack check "$dir" >/dev/null; then
      echo "does not pass reiwand-ctl pack check: $dir" >&2
      "$ctl" pack check "$dir" >&2 || true
      failed=$((failed + 1))
    fi
  elif [ ! -f "$dir/scene.json" ]; then
    echo "neither scene.json nor manifest.json: $dir" >&2
    failed=$((failed + 1))
  elif ! "$ctl" pack check "$dir" >/dev/null; then
    echo "does not pass reiwand-ctl pack check: $dir" >&2
    "$ctl" pack check "$dir" >&2 || true
    failed=$((failed + 1))
  fi
done
if [ "$count" -eq 0 ]; then
  echo "no example folders under docs/authoring/examples" >&2
  exit 1
fi
if [ "$failed" -gt 0 ]; then
  echo "$failed of $count examples failed" >&2
  exit 1
fi
echo "$count examples pass"
python3 -I - <<'PY'
import os
import re
import sys

root = "docs/authoring"
fence = re.compile(r"^\s*(```|~~~)")
link = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")
head = re.compile(r"^#{1,6}\s+(.*?)\s*#*\s*$")


def slug(text):
    text = re.sub(r"[`*]", "", text).strip().lower()
    text = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"[^\w\- ]", "", text, flags=re.UNICODE)
    return text.replace(" ", "-")


def read(path):
    inside, lines = False, []
    for line in open(path, encoding="utf-8"):
        if fence.match(line):
            inside = not inside
        elif not inside:
            lines.append(line.rstrip("\n"))
    return lines


def anchors(path):
    seen, found = {}, set()
    for line in read(path):
        match = head.match(line)
        if match:
            base = slug(match.group(1))
            found.add(base if base not in seen else f"{base}-{seen[base]}")
            seen[base] = seen.get(base, 0) + 1
    return found


def pages():
    # Every page, the ones the long pages were split into (in folders) included; not the examples.
    found = []
    for folder, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if os.path.join(folder, d) != os.path.join(root, "examples")]
        found += [os.path.join(folder, n) for n in names if n.endswith(".md")]
    return sorted(found)


cache, bad, total = {}, [], 0
for page in pages():
    for number, line in enumerate(read(page), 1):
        for target in link.findall(re.sub(r"`[^`]*`", "", line)):
            if re.match(r"[a-z]+:", target):
                continue
            total += 1
            path, _, frag = target.partition("#")
            dest = os.path.normpath(os.path.join(os.path.dirname(page), path)) if path else page
            if not os.path.exists(dest):
                bad.append(f"{page}:{number}: {target} does not exist")
            elif frag and dest.endswith(".md"):
                cache.setdefault(dest, anchors(dest))
                if frag not in cache[dest]:
                    bad.append(f"{page}:{number}: {target} has no such heading")
for line in bad:
    print(line, file=sys.stderr)
if bad:
    print(f"{len(bad)} of {total} links between the pages are broken", file=sys.stderr)
    sys.exit(1)
print(f"{total} links between the pages work")
PY
python3 -I - "$ctl" <<'PY'
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

root, ctl = "docs/authoring", sys.argv[1]
marker = re.compile(r"^<!-- docs-examples: examples/([a-z0-9-]+)/([a-z0-9-]+\.json) -->$")


def walk(value):
    yield value
    for inner in value.values() if isinstance(value, dict) else value if isinstance(value, list) else []:
        yield from walk(inner)


pieces = []
for folder, _, names in os.walk(os.path.join(root, "examples")):
    for name in sorted(n for n in names if n.endswith(".json")):
        pieces.extend(walk(json.load(open(os.path.join(folder, name), encoding="utf-8"))))
objects = [piece for piece in pieces if isinstance(piece, dict)]


def parse(body):
    try:
        return json.loads(body), False
    except ValueError:
        return json.loads("{" + body.strip().rstrip(",") + "}"), True


def placed(example, file, value, members):
    with tempfile.TemporaryDirectory() as work:
        dest = shutil.copytree(os.path.join(root, "examples", example), os.path.join(work, example))
        path = os.path.join(dest, file)
        doc = json.load(open(path, encoding="utf-8"))
        json.dump({**doc, **value} if members else value, open(path, "w", encoding="utf-8"))
        checked = subprocess.run([ctl, "pack", "check", dest], capture_output=True, text=True)
        return checked.returncode == 0 or print(checked.stdout + checked.stderr, file=sys.stderr)


def pages():
    # Every page, the ones the long pages were split into (in folders) included; not the examples.
    found = []
    for folder, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if os.path.join(folder, d) != os.path.join(root, "examples")]
        found += [os.path.join(folder, n) for n in names if n.endswith(".md")]
    return sorted(found)


bad, total = [], 0
for page in pages():
    lines = open(page, encoding="utf-8").read().split("\n")
    for start, line in enumerate(lines):
        if line.strip() != "```json":
            continue
        end = next(i for i in range(start + 1, len(lines)) if lines[i].strip() == "```")
        total += 1
        where = f"{page}:{start + 1}"
        before = next((text.strip() for text in reversed(lines[:start]) if text.strip()), "")
        try:
            value, members = parse("\n".join(lines[start + 1:end]))
        except ValueError as e:
            bad.append(f"{where}: not JSON: {e}")
            continue
        if match := marker.match(before):
            found = placed(*match.groups(), value, members)
        elif members:
            found = any(all(key in o and o[key] == inner for key, inner in value.items()) for o in objects)
        else:
            found = value in pieces
        if not found:
            bad.append(f"{where}: not a whole piece of an example in {root}/examples")
for line in bad:
    print(line, file=sys.stderr)
if bad:
    print(f"{len(bad)} of {total} JSON snippets do not load", file=sys.stderr)
    sys.exit(1)
print(f"{total} JSON snippets are whole pieces of the examples")
PY
python3 -I scripts/coverage.py check "$ctl"
