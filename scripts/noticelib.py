import argparse
import glob
import os
import re
import sys
import tarfile
import tomllib
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUTS = {
    "desktop": os.path.join(ROOT, "packaging", "THIRD-PARTY-NOTICES"),
    "android": os.path.join(ROOT, "android", "app", "src", "main", "assets", "third-party-notices.txt"),
}
FILE = re.compile(r"(?i)^(licen[cs]e|copying|copyright|notice|unlicense)([-_.].*)?$")
HOLDER = re.compile(r"(?i)^\s*(copyright|\(c\))")
ALIAS = {"alloc-stdlib": "brotli"}


def lock_packages():
    with open(os.path.join(ROOT, "Cargo.lock"), "rb") as lock:
        return tomllib.load(lock)["package"]


def reachable(packages, roots):
    by_name = defaultdict(list)
    for package in packages:
        by_name[package["name"]].append(package)
    seen, todo = {}, [p for r in roots for p in by_name[r]]
    while todo:
        package = todo.pop()
        key = (package["name"], package["version"])
        if key in seen:
            continue
        seen[key] = package
        for dep in package.get("dependencies", []):
            parts = dep.split()
            matches = [p for p in by_name[parts[0]] if len(parts) < 2 or p["version"] == parts[1]]
            todo.extend(matches)
    return sorted(k for k, p in seen.items() if "source" in p or k[0] == "naga")


def registries(extra):
    homes = [os.environ.get("CARGO_HOME") or os.path.expanduser("~/.cargo")]
    roots = [os.path.join(h, "registry") for h in homes] + extra
    return [r for r in roots if os.path.isdir(r)]


def crate_files(name, version, roots):
    found, manifest = exact_files(name, version, roots)
    if found:
        return found, manifest
    names = [ALIAS.get(name, name)]
    for root in roots:
        for path in glob.glob(os.path.join(root, "cache", "*", names[0] + "-*.crate")):
            tail = os.path.basename(path)[len(names[0]) + 1:-6]
            if tail[:1].isdigit():
                other, text = exact_files(names[0], tail, roots)
                if other:
                    return other, manifest or text
    return found, manifest


def exact_files(name, version, roots):
    stem = f"{name}-{version}"
    if name == "naga":
        folder = os.path.join(ROOT, "vendor", "naga")
        return {f: open(os.path.join(folder, f), errors="replace").read() for f in os.listdir(folder) if FILE.match(f)}, ""
    for root in roots:
        for folder in glob.glob(os.path.join(root, "src", "*", stem)):
            found = {f: open(os.path.join(folder, f), errors="replace").read() for f in os.listdir(folder) if FILE.match(f) and os.path.isfile(os.path.join(folder, f))}
            manifest = open(os.path.join(folder, "Cargo.toml"), errors="replace").read()
            return found, manifest
        for archive in glob.glob(os.path.join(root, "cache", "*", stem + ".crate")):
            found, manifest = {}, ""
            with tarfile.open(archive) as tar:
                for member in tar:
                    base = member.name[len(stem) + 1:]
                    if "/" in base or not member.isfile():
                        continue
                    if FILE.match(base):
                        found[base] = tar.extractfile(member).read().decode(errors="replace")
                    elif base == "Cargo.toml":
                        manifest = tar.extractfile(member).read().decode(errors="replace")
            return found, manifest
    return None, ""


def declared(manifest):
    match = re.search(r'(?m)^license\s*=\s*"([^"]+)"', manifest)
    return match.group(1) if match else ""


def authors(manifest):
    match = re.search(r"(?ms)^authors\s*=\s*\[(.*?)\]", manifest)
    return re.findall(r'"([^"]+)"', match.group(1)) if match else []


def split(text):
    holders = [" ".join(line.split()) for line in text.splitlines() if HOLDER.match(line)]
    body = [line.rstrip() for line in text.splitlines() if not HOLDER.match(line)]
    return holders, "\n".join(body).strip()


def key_of(body):
    return " ".join(body.lower().split())


def collect(crates, roots):
    groups, order, bare, missing = {}, [], [], []
    for name, version in crates:
        files, manifest = crate_files(name, version, roots)
        if files is None:
            missing.append(f"{name} {version}")
            continue
        if not files:
            bare.append((name, version, declared(manifest), authors(manifest)))
        for text in files.values():
            holders, body = split(text)
            if not body:
                continue
            key = key_of(body)
            if key not in groups:
                groups[key] = {"body": body, "users": {}}
                order.append(key)
            groups[key]["users"].setdefault(f"{name} {version}", []).extend(holders)
    return groups, order, bare, missing


def canonical(groups, needle):
    best = None
    for group in groups.values():
        if needle in group["body"][:200].lower() and (best is None or len(group["users"]) > len(best["users"])):
            best = group
    return best


def settle(groups, order, bare):
    leftover = []
    for name, version, spdx, names in bare:
        target = None
        for token, needle in (("MIT", "mit license"), ("Apache-2.0", "apache license")):
            if token in spdx.split() or token in re.split(r"[ /()]+", spdx):
                target = canonical(groups, needle)
                if target:
                    target["users"].setdefault(f"{name} {version}", []).extend(f"Copyright {n}" for n in names)
        if not target:
            leftover.append(f"{name} {version} ({spdx or 'no licence declared'})")
    return leftover
