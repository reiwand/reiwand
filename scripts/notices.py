import argparse
import os
import sys
import tomllib

from noticelib import *

DESKTOP_ROOTS = ["reiwandd", "reiwand-ctl", "reiwand-web", "reiwand-app", "reiwand-server"]
RULE = "=" * 72
INDEX = "Packages"


def render_crates(crates, roots):
    groups, order, bare, missing = collect(crates, roots)
    leftover = settle(groups, order, bare)
    out = []
    for key in order:
        group = groups[key]
        out += [RULE, "Used by:"]
        for user, holders in sorted(group["users"].items()):
            out.append(f"  {user}" + "".join(f"\n      {h}" for h in dict.fromkeys(holders)))
        out += ["", group["body"], ""]
    if leftover or missing:
        out += [RULE, "Without a licence text: the crate is in no local registry, so neither its text nor its declared licence could be read (see each crate's own source):"]
        out += [f"  {n}" for n in leftover + missing] + [""]
    return "\n".join(out), leftover + missing, groups


def android_libraries():
    with open(os.path.join(ROOT, "android", "gradle", "libs.versions.toml"), "rb") as f:
        data = tomllib.load(f)
    lines = []
    for lib in data["libraries"].values():
        name = lib["module"]
        if name.startswith(("junit", "androidx.test")) or name.endswith(("ui-test-junit4", "ui-test-manifest")):
            continue
        ref = lib.get("version", {}).get("ref")
        lines.append(f"{name} {data['versions'][ref] if ref else '(version of the Compose BOM)'}")
    return sorted(lines)


def assets(*names):
    out = []
    for path in names:
        out += [RULE, os.path.basename(path)[:-4], "", open(os.path.join(ROOT, path)).read().strip(), ""]
    return "\n".join(out)


def build(target, roots):
    if target == "android":
        crates = reachable(lock_packages(), ["reiwand-android"])
        libs = android_libraries()
        listing = [f"{n} {v}" for n, v in crates] + libs
        text, problems, groups = render_crates(crates, roots)
        apache = canonical(groups, "apache license")
        extra = RULE + "\nAndroid libraries (Apache License 2.0, text above):\n" + "".join(f"  {x}\n" for x in libs)
        extra += "\n" + (apache["body"] if apache else "") + "\n"
    else:
        crates = reachable(lock_packages(), DESKTOP_ROOTS)
        listing = [f"{n} {v}" for n, v in crates]
        text, problems, _ = render_crates(crates, roots)
        extra = assets(
            "apps/reiwand-app/look/fonts/OFL-onest.txt",
            "apps/reiwand-app/look/fonts/OFL-red-hat-display.txt",
            "apps/reiwand-app/look/icons/LICENSE-phosphor.txt",
            "crates/reiwand-depth/licenses/depth-anything-v2.txt",
            "crates/reiwand-depth/licenses/lama.txt",
            "crates/reiwand-depth/licenses/onnxruntime.txt",
            "packaging/appimage/appimage-runtime.txt",
            "packaging/appimage/qt-libraries.txt",
        )
    head = ["Third-party notices of Reiwand", "", "The program is free software under the GPL-3.0-or-later.",
            "It carries the libraries, fonts and icons below, each under its own licence.", "", INDEX]
    head += [f"  {x}" for x in listing] + [""]
    return "\n".join(head) + "\n" + text + "\n" + extra, listing, problems


def listed(path):
    lines = open(path).read().splitlines()
    start = lines.index(INDEX) + 1
    block = []
    for line in lines[start:]:
        if not line.startswith("  "):
            break
        block.append(line.strip())
    return block


def expected(target):
    if target == "android":
        return [f"{n} {v}" for n, v in reachable(lock_packages(), ["reiwand-android"])] + android_libraries()
    return [f"{n} {v}" for n, v in reachable(lock_packages(), DESKTOP_ROOTS)]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["write", "check", "print"])
    parser.add_argument("--registry", action="append", default=[])
    parser.add_argument("--target", choices=list(OUTPUTS))
    args = parser.parse_args()
    targets = [args.target] if args.target else list(OUTPUTS)
    failed = False
    for target in targets:
        path = OUTPUTS[target]
        if args.mode == "check":
            if not os.path.exists(path) or listed(path) != expected(target):
                print(f"{os.path.relpath(path, ROOT)} is out of date: run scripts/notices.py write", file=sys.stderr)
                failed = True
            continue
        text, _, problems = build(target, registries(args.registry))
        for p in problems:
            print(f"no text: {p}", file=sys.stderr)
        if args.mode == "print":
            sys.stdout.write(text)
        else:
            with open(path, "w") as out:
                out.write(text)
    sys.exit(1 if failed else 0)


main()
