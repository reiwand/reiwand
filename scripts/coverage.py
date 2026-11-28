"""Checks that every feature of the scene format is shown by an example or a gallery scene.

The list of features is worked out from the format itself: the frozen schemas of `scene.json` and
of the user properties, the built-in effects, the names of `requires.features`, the file types a
scene may hold and the parts of the script interface that `docs/specs/scene.md` lists. Each one is
checked against the examples under `docs/authoring/examples` and the scenes under `scenes/`.

    coverage.py check CTL   fails when a feature has no example, when an example named in
                            docs/authoring/coverage.md does not use its feature, or when a
                            feature of scripts/coverage-pending.txt is already shown
    coverage.py write CTL   rewrites both files from what the examples use now
"""

import json
import os
import re
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FROZEN = os.path.join(ROOT, "crates/reiwand-fill/frozen")
SPEC = os.path.join(ROOT, "docs/specs/scene.md")
PARTS = os.path.join(ROOT, "docs/authoring/coverage")
PENDING = os.path.join(ROOT, "scripts/coverage-pending.txt")
NOT_ASSETS = {"json", "wgsl", "js", "gltf", "bin"}
ALIASES = {"jpeg": "jpg"}
EMPTY = (None, [], {}, False, "")


def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()


def whole(path):
    """A page with the pages it was split into (the folder of the same name) after it."""
    parts = []
    for folder, _, names in os.walk(os.path.splitext(path)[0]):
        parts += [os.path.join(folder, n) for n in names if n.endswith(".md")]
    return "\n".join(read(page) for page in [path] + sorted(parts))


def norm(name):
    return re.sub(r"_for_.*$", "", name)


def shape(alt):
    """A name for an untagged form of a union: its type, its length and whether it holds values."""
    kind = alt.get("type", "value")
    if kind != "array":
        return kind if isinstance(kind, str) else "value"
    items = alt.get("prefixItems", []) + [alt.get("items") or {}]
    bound = any("Value_for" in json.dumps(i) for i in items)
    size = alt.get("minItems") if alt.get("minItems") == alt.get("maxItems") else ""
    return f"array{size or ''}" + ("_bound" if bound else "")


class Schema:
    def __init__(self, path, root=None):
        self.doc = json.load(open(path, encoding="utf-8"))
        self.defs = self.doc["$defs"]
        self.root = root

    def target(self, node):
        name = node["$ref"].rsplit("/", 1)[1]
        return self.defs[name], name

    def named(self, ref):
        ref = norm(ref)
        return ref if self.root in (None, ref) else f"{self.root}/{ref}"

    def matches(self, v, node):
        if not isinstance(node, dict):
            return node is not False
        if "$ref" in node:
            return self.matches(v, self.target(node)[0])
        if "const" in node and v != node["const"]:
            return False
        if "enum" in node and v not in node["enum"]:
            return False
        kinds = node.get("type")
        if kinds is not None:
            kinds = kinds if isinstance(kinds, list) else [kinds]
            if not any(typed(v, k) for k in kinds):
                return False
        if "pattern" in node and isinstance(v, str) and not re.search(node["pattern"], v):
            return False
        alts = node.get("oneOf") or node.get("anyOf")
        if alts and not any(self.matches(v, a) for a in alts):
            return False
        if isinstance(v, dict):
            props = node.get("properties", {})
            if any(r not in v for r in node.get("required", [])):
                return False
            for key, inner in v.items():
                if key in props:
                    if not self.matches(inner, props[key]):
                        return False
                elif node.get("additionalProperties") is False:
                    return False
                elif isinstance(node.get("additionalProperties"), dict):
                    if not self.matches(inner, node["additionalProperties"]):
                        return False
        if isinstance(v, list):
            if len(v) < node.get("minItems", 0) or len(v) > node.get("maxItems", len(v)):
                return False
            first = node.get("prefixItems", [])
            for i, inner in enumerate(v):
                item = first[i] if i < len(first) else node.get("items")
                if isinstance(item, dict) and not self.matches(inner, item):
                    return False
        return True

    def alternatives(self, node, name):
        """The alternatives of a union, each with the feature it names (or None) and its own name."""
        alts = node.get("oneOf") or node.get("anyOf")
        alts = [a for a in alts if isinstance(a, dict) and a.get("type") != "null"]
        transparent = name is None or name.startswith("Value") or len(alts) == 1
        out, seen = [], {}
        for alt in alts:
            tag = next((k for k, p in alt.get("properties", {}).items() if "const" in p), None)
            keys = alt.get("required", [])
            if tag:
                label = f"{name}:{alt['properties'][tag]['const']}"
                out.append((label, alt, label, tag))
            elif alt.get("type") == "object" and len(keys) == 1 and len(alt.get("properties", {})) == 1:
                label = f"{name}:{keys[0]}"
                out.append((label, alt, label, None))
            elif "enum" in alt and len(alt["enum"]) == 1:
                out.append((f"{name}:{alt['enum'][0]}", alt, None, None))
            elif transparent or "enum" in alt:
                out.append((None, alt, name, None))
            else:
                kind = norm(self.target(alt)[1]) if "$ref" in alt else shape(alt)
                seen[kind] = seen.get(kind, 0) + 1
                label = f"{name}:{kind}" + (f"#{seen[kind]}" if seen[kind] > 1 else "")
                out.append((label, alt, name, None))
        return out

    def inventory(self):
        found, done = set(), set()

        def scan(node, name):
            if not isinstance(node, dict):
                return
            if "$ref" in node:
                inner, ref = self.target(node)
                ref = self.named(ref)
                if ref not in done:
                    done.add(ref)
                    scan(inner, ref)
                return
            if "enum" in node and name:
                found.update(f"{name}={v}" for v in node["enum"])
            for key, inner in node.get("properties", {}).items():
                if isinstance(inner, dict) and "const" in inner:
                    continue
                if key not in node.get("required", []):
                    found.add(f"{name}.{key}")
                scan(inner, f"{name}.{key}")
            for inner in [node.get("items"), node.get("additionalProperties")] + node.get("prefixItems", []):
                if isinstance(inner, dict):
                    scan(inner, name)
            if node.get("oneOf") or node.get("anyOf"):
                for label, alt, inner, _ in self.alternatives(node, name):
                    found.update([label] if label else [])
                    scan(alt, inner)

        done.add(self.root)
        scan(self.defs[self.root] if self.root else self.doc, self.root or "Scene")
        return found

    def walk(self, v, node, name, out):
        if not isinstance(node, dict):
            return
        if "$ref" in node:
            inner, ref = self.target(node)
            return self.walk(v, inner, self.named(ref), out)
        if "enum" in node and name and v in node["enum"]:
            out.add(f"{name}={v}")
        if isinstance(v, dict) and "properties" in node:
            props = node["properties"]
            for key, inner in v.items():
                if key in props:
                    sub = props[key]
                    if key not in node.get("required", []) and used(inner, sub):
                        out.add(f"{name}.{key}")
                    self.walk(inner, sub, f"{name}.{key}", out)
        if isinstance(v, dict) and isinstance(node.get("additionalProperties"), dict):
            for key, inner in v.items():
                if key not in node.get("properties", {}):
                    self.walk(inner, node["additionalProperties"], name, out)
        if isinstance(v, list):
            first = node.get("prefixItems", [])
            for i, inner in enumerate(v):
                item = first[i] if i < len(first) else node.get("items")
                if isinstance(item, dict):
                    self.walk(inner, item, name, out)
        if node.get("oneOf") or node.get("anyOf"):
            alts = self.alternatives(node, name)
            for label, alt, inner, tag in alts:
                ok = v.get(tag) == alt["properties"][tag]["const"] if tag and isinstance(v, dict) else None
                if ok or (ok is None and (len(alts) == 1 and v is not None or self.matches(v, alt))):
                    out.update([label] if label else [])
                    self.walk(v, alt, inner, out)
                    break


def typed(v, kind):
    return {
        "null": v is None,
        "boolean": isinstance(v, bool),
        "string": isinstance(v, str),
        "object": isinstance(v, dict),
        "array": isinstance(v, list),
        "integer": isinstance(v, int) and not isinstance(v, bool),
        "number": isinstance(v, (int, float)) and not isinstance(v, bool),
    }.get(kind, True)


def used(v, node):
    if isinstance(node, dict) and "default" in node:
        return v is not None and v != node["default"]
    return not any(v == e and type(v) is type(e) for e in EMPTY)


def registry():
    """The names of requires.features that docs/specs/scene.md names, except those always refused."""
    spec = whole(SPEC)
    text = read(os.path.join(ROOT, "crates/reiwand-format/src/scenery.rs"))
    text += read(os.path.join(ROOT, "crates/reiwand-format/src/features.rs"))
    entry = re.compile(r'\(\s*"([a-z0-9_.]+)",\s*"(?:[^"\\]|\\.)*",\s*\[(\w+),\s*(\w+)\]\s*,?\s*\)', re.S)
    scene = read(os.path.join(ROOT, "crates/reiwand-format/src/scenery.rs"))
    names = set()
    for name, desktop, phone in entry.findall(text):
        if (desktop, phone) == ("No", "No"):
            continue
        if f'"{name}"' in scene or f"`{name}`" in spec:
            names.add(f"feature:{name}")
    return names


def refused():
    text = read(os.path.join(ROOT, "crates/reiwand-format/src/features.rs"))
    return set(re.findall(r'\("([a-z0-9_.]+)",\s*"[^"]*",\s*\[No,\s*No\]\)', text))


def formats():
    text = read(os.path.join(ROOT, "crates/reiwand-format/src/validate.rs"))
    found = set()
    for group in ("IMAGES", "VIDEOS", "SCENES"):
        body = re.search(rf"const {group}: &\[&str\] = &\[(.*?)\];", text, re.S).group(1)
        found.update(re.findall(r'"([a-z0-9]+)"', body))
    found -= NOT_ASSETS | refused()
    return {f"format:{ALIASES.get(e, e)}" for e in found}


def section(text, heading):
    """From a heading to the next one; a split page may give the heading another level."""
    found = re.search(rf"^#+ {re.escape(heading.lstrip('#').strip())}\s*$", text, re.M)
    end = text.find("\n#", found.end())
    return text[found.start():end if end >= 0 else len(text)]


def script_parts():
    spec = whole(SPEC)
    found = set()
    for row in section(spec, "### Lifecycle").splitlines():
        if row.startswith("| `"):
            found.add("export:" + re.match(r"\| `(\w+)", row).group(1))
    for row in section(spec, "### The `reiwand` module").splitlines():
        if row.startswith("| `"):
            cell = row.split("|")[1]
            found.update("script:" + name for name in re.findall(r"`(\w+)", cell))
    found.update("script:" + name for name in re.findall(r'"(reiwand/[a-z]+)"', spec))
    return found


def triggers():
    spec = whole(SPEC)
    row = next(line for line in spec.splitlines() if line.startswith("| `play_on` |"))
    kinds = re.findall(r"`([a-z]+(?::<[a-z]+>)?)`", row.split("|")[2])
    media = sorted(set(re.findall(r"`(media:[a-z]+)`", spec)))
    kinds = [k for k in kinds if k != "media:<what>"] + media
    names = {k.split(":<")[0] + (":" if ":<" in k else "") for k in kinds}
    return {f"{slot}:{k}" for slot in ("trigger", "play_on") for k in names}


def effects():
    folder = os.path.join(ROOT, "crates/reiwand-scene/effects")
    ids = [n[:-5] for n in os.listdir(folder) if n.endswith(".json")]
    keys = set()
    for name in ids:
        keys.update(json.load(open(os.path.join(folder, name + ".json"), encoding="utf-8")))
    return {f"effect:{i}" for i in ids} | {f"descriptor.{k}" for k in keys - {"id", "name", "description"}}


def inventory(scene, props):
    found = scene.inventory() | props.inventory()
    return found | registry() | formats() | script_parts() | triggers() | effects()


def trigger_kind(value):
    return value.split(":")[0] + ":" if ":" in value and not value.startswith("media:") else value


def folders():
    for base in ("docs/authoring/examples", "scenes"):
        for name in sorted(os.listdir(os.path.join(ROOT, base))):
            path = os.path.join(base, name)
            if os.path.isfile(os.path.join(ROOT, path, "manifest.json")):
                yield path


def uses(path, scene, props, ctl):
    full = os.path.join(ROOT, path)
    out = set()
    manifest = json.load(open(os.path.join(full, "manifest.json"), encoding="utf-8"))
    for prop in manifest.get("properties", {}).values():
        props.walk(prop, props.defs["Property"], "Property", out)
    checked = subprocess.run([ctl, "pack", "check", "--json", full], capture_output=True, text=True)
    if checked.returncode == 0:
        out.update("feature:" + f["name"] for f in json.loads(checked.stdout)["features"])
    if os.path.isfile(os.path.join(full, "scene.json")):
        doc = json.load(open(os.path.join(full, "scene.json"), encoding="utf-8"))
        scene.walk(doc, scene.doc, "Scene", out)
        for asset in doc.get("assets", {}).values():
            ext = asset.get("path", "").rsplit(".", 1)[-1].lower()
            out.add("format:" + ALIASES.get(ext, ext))
        out.update(slots(doc))
        for files in doc.get("effects", {}).values():
            own = json.load(open(os.path.join(full, files["descriptor"]), encoding="utf-8"))
            out.update(f"descriptor.{k}" for k, v in own.items() if not any(v == e for e in EMPTY))
        out.update(code(full))
    return out


def slots(doc):
    out = set()

    def layers(items):
        for layer in items:
            yield layer
            yield from layers(layer.get("layers", []))

    every = list(layers(doc.get("layers", []))) + list(doc.get("templates", {}).values())
    for layer in every:
        out.update(f"effect:{e['effect']}" for e in layer.get("effects", []))
        if layer.get("type") == "sound" and layer.get("play_on"):
            out.add("play_on:" + trigger_kind(layer["play_on"]))
    for timeline in doc.get("timelines", []):
        if timeline.get("trigger"):
            out.add("trigger:" + trigger_kind(timeline["trigger"]))
    return out


def code(full):
    out = set()
    for folder, _, names in os.walk(os.path.join(full, "scripts")):
        for name in names:
            if not name.endswith((".js", ".mjs")):
                continue
            text = read(os.path.join(folder, name))
            for group in re.findall(r'import\s*\{([^}]*)\}\s*from\s*["\']reiwand["\']', text):
                out.update("script:" + n.split(" as ")[0].strip() for n in group.split(",") if n.strip())
            out.update("script:" + m for m in re.findall(r'from\s*["\'](reiwand/[a-z]+)["\']', text))
            for name_ in re.findall(r"export\s+(?:async\s+)?(?:function\s*\*?|const|let|var)\s+(\w+)", text):
                out.add("export:" + name_)
            for group in re.findall(r"export\s*\{([^}]*)\}", text):
                out.update("export:" + n.split(" as ")[-1].strip() for n in group.split(",") if n.strip())
    return out


HEADINGS = [
    ("feature:", "Features of `requires.features`"),
    ("effect:", "Built-in effects"),
    ("Blend=", "Blend modes"),
    ("Layer", "Layers"),
    ("Binding", "Values"),
    ("trigger:", "Triggers of timelines"),
    ("play_on:", "Triggers of sounds"),
    ("format:", "File types"),
    ("Property", "User properties"),
    ("script:", "Parts of the `reiwand` script module"),
    ("export:", "Functions a script exports"),
    ("descriptor.", "Fields of an effect description"),
    ("", "Everything else in `scene.json`"),
]

# The pages of the list (docs/authoring/coverage.md is their index): file, title, what is on it and
# the groups of HEADINGS it holds. A page with one group has its rows under the title.
PAGES = [
    ("features.md", "Features, effects, blends and layers", "`requires.features`, effects, blend modes and layers", range(0, 4)),
    ("values-and-scripts.md", "Values, triggers, files and scripts", "values, triggers, file types, properties and script interfaces", range(4, 12)),
    ("scene.md", "Everything else in `scene.json`", None, range(12, 13)),
]


def group(feature):
    return next(i for i, (prefix, _) in enumerate(HEADINGS) if feature.startswith(prefix))


def link(path):
    name = os.path.basename(path)
    rel = "../examples/" + name if path.startswith("docs/") else "../../../" + path
    return f"[{'' if path.startswith('docs/') else 'scenes/'}{name}]({rel})"


def render(shown, title, about, groups):
    lines = [f"# {title}", ""] + ([f"On this page: {about}.", ""] if about else [])
    for index in groups:
        rows = sorted(f for f in shown if group(f) == index)
        if rows:
            lines += ([f"## {HEADINGS[index][1]}", ""] if about else []) + ["| Feature | Shown in |", "|---------|----------|"]
            lines += [f"| `{f}` | {link(shown[f])} |" for f in rows]
            lines.append("")
    return "\n".join(lines)


def parse():
    shown, pending = {}, set()
    row = re.compile(r"^\| `([^`]+)` \| \[[^\]]*\]\(([^)]+?)/?\) \|$")
    for name, *_ in PAGES:
        page = os.path.join(PARTS, name)
        for line in read(page).splitlines() if os.path.exists(page) else []:
            if match := row.match(line):
                shown[match.group(1)] = os.path.relpath(os.path.normpath(os.path.join(PARTS, match.group(2))), ROOT)
    if os.path.exists(PENDING):
        pending = {line.strip() for line in read(PENDING).splitlines() if line.strip() and line[0] != "#"}
    return shown, pending


def main():
    mode, ctl = sys.argv[1], sys.argv[2]
    scene = Schema(os.path.join(FROZEN, "scene.json"))
    props = Schema(os.path.join(FROZEN, "manifest.json"), "Property")
    features = inventory(scene, props)
    paths = list(folders())
    with ThreadPoolExecutor(8) as pool:
        usage = dict(zip(paths, pool.map(lambda p: uses(p, scene, props, ctl), paths)))
    shown, pending = parse()
    if mode == "write":
        fresh = {}
        for feature in sorted(features):
            first = next((p for p, found in usage.items() if feature in found), None)
            kept = shown.get(feature) if feature in usage.get(shown.get(feature), ()) else None
            if kept and (kept.startswith("docs/") or not (first or "").startswith("docs/")):
                fresh[feature] = kept
            elif first:
                fresh[feature] = first
        for name, title, about, groups in PAGES:
            with open(os.path.join(PARTS, name), "w", encoding="utf-8") as f:
                f.write(render(fresh, title, about, groups))
        with open(PENDING, "w", encoding="utf-8") as f:
            f.write("# Features of the scene format that no example shows yet (see scripts/coverage.py)\n")
            f.writelines(f"{feature}\n" for feature in sorted(features - set(fresh)))
        print(f"{len(fresh)} of {len(features)} features shown, {len(features) - len(fresh)} pending")
        return
    bad = []
    for feature in sorted(features - set(shown) - pending):
        bad.append(f"{feature}: no example shows it; add one, or list it in scripts/coverage-pending.txt")
    for feature in sorted((set(shown) | pending) - features):
        bad.append(f"{feature}: not a feature of the format any more")
    for feature, path in sorted(shown.items()):
        if feature in features and feature not in usage.get(path, ()):
            bad.append(f"{feature}: {path} does not use it")
    for feature in sorted(pending & features):
        if feature in shown:
            bad.append(f"{feature}: both shown and pending")
        elif first := next((p for p, found in usage.items() if feature in found), None):
            bad.append(f"{feature}: pending, but {first} shows it; run scripts/coverage.py write")
    for line in bad:
        print(line, file=sys.stderr)
    if bad:
        print(f"{len(bad)} problems with the coverage of the {len(features)} features", file=sys.stderr)
        sys.exit(1)
    print(f"{len(shown)} of {len(features)} features have an example, {len(pending)} pending")


if __name__ == "__main__":
    main()
